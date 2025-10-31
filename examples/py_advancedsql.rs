use webrust::prelude::*;

#[gui(Consolas 12px navy !whitesmoke)]
fn main() {
    query("INSTALL httpfs; LOAD httpfs;");

    println("<b !dodgerblue r5 w225 p15>📊 WebRust Data Analysis Demo").align("center");
    println("<gray mt50>Using DuckDB for high-performance analytics\n");

    println("<green,b>🌸 Part 1: Iris Dataset Analysis");

    query("CREATE OR REPLACE TABLE iris AS SELECT * FROM read_csv_auto('https://raw.githubusercontent.com/mwaskom/seaborn-data/master/iris.csv', SAMPLE_SIZE=-1, AUTO_DETECT=TRUE)");

    println("<red>→ First rows");
    query("SELECT * FROM iris LIMIT 10");

    println("<red>→ Dataset shape");
    query("SELECT COUNT(*) AS rows, COUNT(DISTINCT species) AS species FROM iris");

    println("<red>→ Descriptive statistics");
    query(
        r#"
        SELECT
            species,
            COUNT(*) AS count,
            ROUND(AVG(sepal_length), 2) AS avg_sepal_length,
            ROUND(AVG(sepal_width), 2) AS avg_sepal_width,
            ROUND(AVG(petal_length), 2) AS avg_petal_length,
            ROUND(AVG(petal_width), 2) AS avg_petal_width
        FROM iris
        GROUP BY species
        ORDER BY species
    "#,
    );

    println("<red>→ Distribution analysis");
    query(
        r#"
        SELECT
            species,
            MIN(petal_length) AS min_petal,
            percentile_cont(0.25) WITHIN GROUP (ORDER BY petal_length) AS q25,
            percentile_cont(0.50) WITHIN GROUP (ORDER BY petal_length) AS median,
            percentile_cont(0.75) WITHIN GROUP (ORDER BY petal_length) AS q75,
            MAX(petal_length) AS max_petal,
            ROUND(STDDEV(petal_length), 2) AS std_dev
        FROM iris
        GROUP BY species
        ORDER BY species
    "#,
    );

    println("<red>→ Correlation matrix");
    query(
        r#"
        SELECT
            ROUND(CORR(sepal_length, sepal_width), 3) AS sepal_l_w,
            ROUND(CORR(sepal_length, petal_length), 3) AS sepal_l_petal_l,
            ROUND(CORR(sepal_length, petal_width), 3) AS sepal_l_petal_w,
            ROUND(CORR(petal_length, petal_width), 3) AS petal_l_w
        FROM iris
    "#,
    );

    println("\n<blue,b>🚢 Part 2: Titanic Survival Analysis");

    query("CREATE OR REPLACE TABLE titanic AS SELECT * FROM read_csv_auto('https://raw.githubusercontent.com/datasciencedojo/datasets/master/titanic.csv', SAMPLE_SIZE=-1, AUTO_DETECT=TRUE)");
    query("CREATE INDEX IF NOT EXISTS idx_titanic_class ON titanic(Pclass);");
    query("CREATE INDEX IF NOT EXISTS idx_titanic_sex ON titanic(Sex);");

    println("<red>→ Overview");
    query("SELECT * FROM titanic LIMIT 8");

    println("<red>→ Survival rate by class");
    query(
        r#"
        SELECT
            Pclass AS class,
            COUNT(*) AS total,
            SUM(Survived) AS survived,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) AS survival_rate_pct
        FROM titanic
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("<red>→ Survival by gender");
    query(
        r#"
        SELECT
            Sex AS gender,
            COUNT(*) AS total,
            SUM(Survived) AS survived,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) AS survival_rate_pct
        FROM titanic
        GROUP BY Sex
        ORDER BY survival_rate_pct DESC
    "#,
    );

    println("<red>→ Pivot: Survival by class and gender");
    query(
        r#"
        SELECT
            Pclass AS class,
            COUNT(*) AS total,
            SUM(CASE WHEN Sex = 'female' THEN Survived ELSE 0 END) AS female_survived,
            SUM(CASE WHEN Sex = 'female' THEN 1 ELSE 0 END) AS female_total,
            ROUND(100.0 * SUM(CASE WHEN Sex = 'female' THEN Survived ELSE 0 END) /
                  NULLIF(SUM(CASE WHEN Sex = 'female' THEN 1 ELSE 0 END), 0), 1) AS female_pct,
            SUM(CASE WHEN Sex = 'male' THEN Survived ELSE 0 END) AS male_survived,
            SUM(CASE WHEN Sex = 'male' THEN 1 ELSE 0 END) AS male_total,
            ROUND(100.0 * SUM(CASE WHEN Sex = 'male' THEN Survived ELSE 0 END) /
                  NULLIF(SUM(CASE WHEN Sex = 'male' THEN 1 ELSE 0 END), 0), 1) AS male_pct
        FROM titanic
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("<red>→ Age distribution of survivors");
    query(
        r#"
        SELECT
            CASE
                WHEN Age < 18 THEN 'Child'
                WHEN Age < 35 THEN 'Young Adult'
                WHEN Age < 60 THEN 'Adult'
                ELSE 'Senior'
            END AS age_group,
            SUM(Survived) AS survived,
            COUNT(*) - SUM(Survived) AS died,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) AS survival_rate_pct
        FROM titanic
        WHERE Age IS NOT NULL
        GROUP BY age_group
        ORDER BY
            CASE age_group
                WHEN 'Child' THEN 1
                WHEN 'Young Adult' THEN 2
                WHEN 'Adult' THEN 3
                ELSE 4
            END
    "#,
    );

    println("<red>→ Fare analysis");
    query(
        r#"
        SELECT
            Pclass AS class,
            ROUND(MIN(Fare), 2) AS min_fare,
            ROUND(AVG(Fare), 2) AS avg_fare,
            ROUND(MAX(Fare), 2) AS max_fare,
            ROUND(STDDEV(Fare), 2) AS std_fare
        FROM titanic
        WHERE Fare > 0
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("\n<magenta,b>🔬 Part 3: Advanced Analytics");

    println("<red>→ Window functions: Running totals (FIXED)");
    query(
        r#"
        WITH survivors AS (
            SELECT
                Pclass,
                Sex,
                Survived
            FROM titanic
            WHERE Survived = 1
        ),
        grouped AS (
            SELECT
                Pclass,
                Sex,
                COUNT(*) AS count,
                SUM(COUNT(*)) OVER (
                    PARTITION BY Pclass
                    ORDER BY Sex
                    ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                ) AS cumulative_count
            FROM survivors
            GROUP BY Pclass, Sex
        )
        SELECT * FROM grouped
        ORDER BY Pclass, Sex
    "#,
    );

    println("<red>→ Top 3 fares per class (FIXED - avec ROW_NUMBER)");
    query(
        r#"
        WITH ranked AS (
            SELECT
                Name,
                Pclass,
                Fare,
                ROW_NUMBER() OVER (PARTITION BY Pclass ORDER BY Fare DESC, PassengerId) AS rn
            FROM titanic
            WHERE Fare > 0
        )
        SELECT Name, Pclass, Fare, rn AS fare_rank_in_class
        FROM ranked
        WHERE rn <= 3
        ORDER BY Pclass, rn
    "#,
    );

    println("<red>→ Alternative: Distinct top fares per class");
    query(
        r#"
        WITH distinct_fares AS (
            SELECT DISTINCT Pclass, Fare
            FROM titanic
            WHERE Fare > 0
        ),
        ranked_fares AS (
            SELECT
                Pclass,
                Fare,
                ROW_NUMBER() OVER (PARTITION BY Pclass ORDER BY Fare DESC) AS rn
            FROM distinct_fares
        )
        SELECT Pclass, Fare, rn AS fare_rank
        FROM ranked_fares
        WHERE rn <= 5
        ORDER BY Pclass, rn
    "#,
    );

    println("<red>→ Cross-dataset: Compare distributions");
    query(
        r#"
        SELECT
            'Iris petal length' AS metric,
            ROUND(AVG(petal_length), 2) AS mean,
            ROUND(STDDEV(petal_length), 2) AS std_dev,
            ROUND(MIN(petal_length), 2) AS min_val,
            ROUND(MAX(petal_length), 2) AS max_val
        FROM iris
        UNION ALL
        SELECT
            'Titanic age' AS metric,
            ROUND(AVG(Age), 2) AS mean,
            ROUND(STDDEV(Age), 2) AS std_dev,
            ROUND(MIN(Age), 2) AS min_val,
            ROUND(MAX(Age), 2) AS max_val
        FROM titanic
        WHERE Age IS NOT NULL
        UNION ALL
        SELECT
            'Titanic fare' AS metric,
            ROUND(AVG(Fare), 2) AS mean,
            ROUND(STDDEV(Fare), 2) AS std_dev,
            ROUND(MIN(Fare), 2) AS min_val,
            ROUND(MAX(Fare), 2) AS max_val
        FROM titanic
        WHERE Fare > 0
    "#,
    );

    println("\n<cyan,b>💾 Part 4: Export Results");

    query(
        r#"
        CREATE TEMP TABLE analysis_summary AS
        SELECT
            'Iris' AS dataset,
            COUNT(*) AS rows,
            COUNT(DISTINCT species) AS categories
        FROM iris
        UNION ALL
        SELECT
            'Titanic' AS dataset,
            COUNT(*) AS rows,
            COUNT(DISTINCT Pclass) AS categories
        FROM titanic
    "#,
    );

    println("<red>→ Summary table");
    query("SELECT * FROM analysis_summary");

    query(
        "COPY (SELECT * FROM analysis_summary) TO 'summary.csv' WITH (HEADER TRUE, DELIMITER ',');",
    );
    query("COPY analysis_summary TO 'summary.parquet' (FORMAT PARQUET);");
    query("COPY analysis_summary TO 'summary.json' (FORMAT JSON);");

    println("<green>✓ Exported to summary.csv, summary.parquet, summary.json");

    println("\n<bright_magenta,b>🔍 Part 5: Schema Inspection");

    println("<red>→ Iris schema");
    query("SCHEMA SELECT * FROM iris");

    println("<red>→ Titanic schema");
    query("SCHEMA SELECT PassengerId, Survived, Pclass, Name, Sex, Age, Fare FROM titanic");

    println("\n<bright_green,b>✨ Analysis Complete!");
    println("<gray>All data processed in-memory with DuckDB");
}
