// examples/py_sql.rs
// Run with: cargo run --example sql
//
// This example showcases the WebRust + DuckDB "query-first" workflow.
// - We create sequences & tables
// - Insert a few rows
// - Demonstrate SELECT / JOIN / AGGREGATES / WINDOWS
// - Create and query a view
// - Parse inline JSON into rows (json_each)
// - Load CSV-like in-memory data and compute per-item cumulative sums
// - Show SCHEMA on a SELECT
//
// Notes:
// * All SQL is executed via `query(...)` (from webrust::db::sql), which:
//   - Streams SELECT results to the browser as an HTML table (batch-by-batch via Arrow)
//   - Executes DDL/DML statements silently (errors are printed in the terminal area)
//   - Supports `SCHEMA <select ...>` to display {column, arrow_type}
//   - Supports `OPEN 'file.duckdb'` to switch to a file-backed database
//
// Tips:
// * Use raw strings (r#" ... "#) for multi-line SQL.
// * Prefer standard CAST(...) for portability (instead of '::TYPE').

use webrust::prelude::*;

#[gui(bg = "navy", fg = "white", font = "Courier New", color = "black", size = "12px")]
fn main() {
    println("@(cyan, bold, italic)🦆 WebRust + DuckDB — query-first SQL demo");

    // -------------------------------------------------------------------------
    // 1) DDL: sequences & tables
    // -------------------------------------------------------------------------
    println("@(green)→ Create tables / sequences");
    query(r#"
        DROP TABLE IF EXISTS person;
        DROP TABLE IF EXISTS city;
        DROP SEQUENCE IF EXISTS city_seq;
        DROP SEQUENCE IF EXISTS person_seq;

        CREATE SEQUENCE city_seq START 1;
        CREATE TABLE city(
            id INTEGER DEFAULT nextval('city_seq') PRIMARY KEY,
            name TEXT NOT NULL
        );

        CREATE SEQUENCE person_seq START 1;
        CREATE TABLE person(
            id INTEGER DEFAULT nextval('person_seq') PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER,
            city_id INTEGER
        );
    "#);

    // -------------------------------------------------------------------------
    // 2) Seed a few rows
    // -------------------------------------------------------------------------
    println("@(green)→ Seed data");
    query(r#"
        INSERT INTO city(name) VALUES ('Lyon'), ('Cluny'), ('Paris');
        INSERT INTO person(name, age, city_id) VALUES
            ('Alice', 30, 1),
            ('Bob',   25, 2),
            ('Chloé', 28, 1),
            ('David', 34, 3);
    "#);

    // -------------------------------------------------------------------------
    // 3) Simple SELECT
    // -------------------------------------------------------------------------
    println("@(purple, bold)📋 Base SELECT");
    query(r#"SELECT id, name, age, city_id FROM person ORDER BY id"#);

    // -------------------------------------------------------------------------
    // 4) JOIN + projection
    // -------------------------------------------------------------------------
    println("@(orange, bold)🔗 Join + projection");
    query(r#"
        SELECT p.id, p.name, p.age, c.name AS city
        FROM person p
        JOIN city   c ON p.city_id = c.id
        ORDER BY p.id
    "#);

    // -------------------------------------------------------------------------
    // 5) Aggregates
    // -------------------------------------------------------------------------
    println("@(yellow, bold)📊 Aggregates");
    query(r#"
        SELECT
            c.name                    AS city,
            COUNT(*)                  AS n_rows,
            AVG(p.age)                AS avg_age
        FROM person p
        JOIN city c ON p.city_id = c.id
        GROUP BY c.name
        ORDER BY n_rows DESC, city
    "#);

    // -------------------------------------------------------------------------
    // 6) Window functions
    // -------------------------------------------------------------------------
    println("@(magenta, bold)📈 Window functions");
    query(r#"
        SELECT
          p.id,
          p.name,
          p.age,
          c.name AS city,
          RANK()       OVER (PARTITION BY c.name ORDER BY p.age DESC) AS rk_in_city,
          ROW_NUMBER() OVER (ORDER BY p.age DESC, p.id)               AS rn_global
        FROM person p
        JOIN city c ON p.city_id = c.id
        ORDER BY c.name, rk_in_city
    "#);

    // -------------------------------------------------------------------------
    // 7) Simple view + query it
    // -------------------------------------------------------------------------
    println("@(cyan, bold)🧱 Simple view");
    query(r#"
        DROP VIEW IF EXISTS v_person_city;
        CREATE VIEW v_person_city AS
        SELECT p.id, p.name, p.age, c.name AS city
        FROM person p JOIN city c ON p.city_id = c.id;
    "#);
    query(r#"SELECT * FROM v_person_city ORDER BY id"#);

    // -------------------------------------------------------------------------
    // 8) JSON → rows using json_each
    //
    // Columns made explicit:
    //   - key
    //   - value_int           : parsed integer value
    //   - value_cumul         : running sum (ordered by key)
    //   - value_part_pct      : % contribution to grand total
    // -------------------------------------------------------------------------
    println("@(bright_yellow, bold)🧰 JSON → rows via json_each");
    query(r#"
        WITH j(js) AS (
          SELECT CAST('[{"k":"A","v":10},{"k":"B","v":20},{"k":"C","v":30}]' AS JSON)
        )
        SELECT
          je.key AS key,
          CAST(json_extract(je.value, '$.v') AS INTEGER)                              AS value_int,
          SUM(CAST(json_extract(je.value, '$.v') AS INTEGER))
            OVER (ORDER BY je.key)                                                    AS value_cumul,
          ROUND(
            100.0 * CAST(json_extract(je.value, '$.v') AS DOUBLE)
            / SUM(CAST(json_extract(je.value, '$.v') AS DOUBLE)) OVER (), 1
          )                                                                           AS value_part_pct
        FROM j, json_each(js) AS je
        ORDER BY je.key
    "#);

    // -------------------------------------------------------------------------
    // 9) CSV-like in memory
    //
    // We build a small "sales" table with an explicit id to get a stable row
    // order per item, then compute:
    //   - qty_cumul     : running sum per item
    //   - qty_part_pct  : row's % contribution to the item total
    // -------------------------------------------------------------------------
    println("@(bright_cyan, bold)🧾 CSV-like data in memory");
    query(r#"
        CREATE TEMP TABLE sales(id INTEGER, name TEXT, qty INTEGER);
        INSERT INTO sales VALUES (1,'book',5),(2,'pen',2),(3,'pen',3);
    "#);
    query(r#"
        SELECT
           name                                                     AS item,
           qty                                                      AS qty,
           SUM(qty) OVER (
             PARTITION BY name
             ORDER BY id
             ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
           )                                                        AS qty_cumul,
           ROUND(
             100.0 * CAST(qty AS DOUBLE)
             / SUM(CAST(qty AS DOUBLE)) OVER (PARTITION BY name)
           , 1)                                                     AS qty_part_pct
        FROM sales
        ORDER BY item, id
    "#);

    // -------------------------------------------------------------------------
    // 10) Simple macro (DuckDB CREATE MACRO) + usage
    // -------------------------------------------------------------------------
    println("@(bright_green, bold)🧪 UDF (macro) + usage");
    query(r#"
        CREATE OR REPLACE MACRO age_bucket(a) AS (
          CASE
            WHEN a IS NULL THEN 'NA'
            WHEN a <  28   THEN '<28'
            WHEN a <  32   THEN '[28,31]'
            ELSE '>=32'
          END
        );
    "#);
    query(r#"
        SELECT name, age, age_bucket(age) AS bucket
        FROM person
        ORDER BY age, name
    "#);

    // -------------------------------------------------------------------------
    // 11) SCHEMA on a SELECT
    // -------------------------------------------------------------------------
    println("@(bright_magenta, bold)🧮 Types (SCHEMA)");
    query(r#"SCHEMA SELECT id, name, age, city_id FROM person"#);

    // -------------------------------------------------------------------------
    // (Optional) 12) OPEN a file-backed database
    //
    // Note: Uncomment the two lines below to switch from the in-memory DB to a
    // file-backed one (creates the file if needed).
    //
    // query(r#"OPEN 'demo.duckdb'"#);
    // query(r#"SELECT CURRENT_TIMESTAMP AS opened_at"#);

    println("\n@(bright_green, bold)✨ Done");
}