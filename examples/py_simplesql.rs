// webrust/examples/py_simplesql.rs

use webrust::prelude::*;

#[gui("Courier New", 12px, black, !navy)]
fn main() {
    println("<cyan,b,i>🦆 WebRust + DuckDB — query-first SQL demo");

    println("<green>→ Create tables / sequences");
    query(
        r#"
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
    "#,
    );

    println("<green>→ Seed data");
    query(
        r#"
        INSERT INTO city(name) VALUES ('Lyon'), ('Cluny'), ('Paris');
        INSERT INTO person(name, age, city_id) VALUES
            ('Alice', 30, 1),
            ('Bob',   25, 2),
            ('Chloé', 28, 1),
            ('David', 34, 3);
    "#,
    );

    println("<purple,b>📋 Base SELECT");
    query(r#"SELECT id, name, age, city_id FROM person ORDER BY id"#);

    println("<orange,b>🔗 Join + projection");
    query(
        r#"
        SELECT p.id, p.name, p.age, c.name AS city
        FROM person p
        JOIN city   c ON p.city_id = c.id
        ORDER BY p.id
    "#,
    );

    println("<yellow,b>📊 Aggregates");
    query(
        r#"
        SELECT
            c.name                    AS city,
            COUNT(*)                  AS n_rows,
            AVG(p.age)                AS avg_age
        FROM person p
        JOIN city c ON p.city_id = c.id
        GROUP BY c.name
        ORDER BY n_rows DESC, city
    "#,
    );

    println("<magenta,b>📈 Window functions");
    query(
        r#"
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
    "#,
    );

    println("<cyan,b>🧱 Simple view");
    query(
        r#"
        DROP VIEW IF EXISTS v_person_city;
        CREATE VIEW v_person_city AS
        SELECT p.id, p.name, p.age, c.name AS city
        FROM person p JOIN city c ON p.city_id = c.id;
    "#,
    );
    query(r#"SELECT * FROM v_person_city ORDER BY id"#);

    println("<bright_yellow,b>🧰 JSON → rows via json_each");
    query(
        r#"
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
    "#,
    );

    println("<bright_cyan,b>🧾 CSV-like data in memory");
    query(
        r#"
        CREATE TEMP TABLE sales(id INTEGER, name TEXT, qty INTEGER);
        INSERT INTO sales VALUES (1,'book',5),(2,'pen',2),(3,'pen',3);
    "#,
    );
    query(
        r#"
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
    "#,
    );

    println("<bright_green,b>🧪 UDF (macro) + usage");
    query(
        r#"
        CREATE OR REPLACE MACRO age_bucket(a) AS (
          CASE
            WHEN a IS NULL THEN 'NA'
            WHEN a <  28   THEN '<28'
            WHEN a <  32   THEN '[28,31]'
            ELSE '>=32'
          END
        );
    "#,
    );
    query(
        r#"
        SELECT name, age, age_bucket(age) AS bucket
        FROM person
        ORDER BY age, name
    "#,
    );

    println("<bright_magenta,b>🧮 Types (SCHEMA)");
    query(r#"SCHEMA SELECT id, name, age, city_id FROM person"#);

    println("\n<bright_green,b>✨ Done");
}
