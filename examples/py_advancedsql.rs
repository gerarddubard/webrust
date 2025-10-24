// webrust/examples/py_advancedsql.rs

use webrust::prelude::*;

#[gui(Consolas, 11px, white, !darkslategray)]
fn main() {
    println("<cyan,b>📊 WebRust Data Analysis Demo");
    println("<gray>Using DuckDB for high-performance analytics\n");

    println("<green,b>🌸 Part 1: Iris Dataset Analysis");

    query(
        "IMPORT 'https://raw.githubusercontent.com/mwaskom/seaborn-data/master/iris.csv' AS iris",
    );

    println("<yellow>→ First rows");
    query("SELECT * FROM iris LIMIT 10");

    println("<yellow>→ Dataset shape");
    query("SELECT COUNT(*) as rows, COUNT(DISTINCT species) as species FROM iris");

    println("<yellow>→ Descriptive statistics");
    query(
        r#"
        SELECT
            species,
            COUNT(*) as count,
            ROUND(AVG(sepal_length), 2) as avg_sepal_length,
            ROUND(AVG(sepal_width), 2) as avg_sepal_width,
            ROUND(AVG(petal_length), 2) as avg_petal_length,
            ROUND(AVG(petal_width), 2) as avg_petal_width
        FROM iris
        GROUP BY species
        ORDER BY species
    "#,
    );

    println("<yellow>→ Distribution analysis");
    query(
        r#"
        SELECT
            species,
            MIN(petal_length) as min_petal,
            percentile_cont(0.25) WITHIN GROUP (ORDER BY petal_length) as q25,
            percentile_cont(0.50) WITHIN GROUP (ORDER BY petal_length) as median,
            percentile_cont(0.75) WITHIN GROUP (ORDER BY petal_length) as q75,
            MAX(petal_length) as max_petal,
            ROUND(STDDEV(petal_length), 2) as std_dev
        FROM iris
        GROUP BY species
        ORDER BY species
    "#,
    );

    println("<yellow>→ Correlation matrix");
    query(
        r#"
        SELECT
            ROUND(CORR(sepal_length, sepal_width), 3) as sepal_l_w,
            ROUND(CORR(sepal_length, petal_length), 3) as sepal_l_petal_l,
            ROUND(CORR(sepal_length, petal_width), 3) as sepal_l_petal_w,
            ROUND(CORR(petal_length, petal_width), 3) as petal_l_w
        FROM iris
    "#,
    );

    println("\n<blue,b>🚢 Part 2: Titanic Survival Analysis");

    query("IMPORT 'https://raw.githubusercontent.com/datasciencedojo/datasets/master/titanic.csv' AS titanic");

    println("<yellow>→ Overview");
    query("SELECT * FROM titanic LIMIT 8");

    println("<yellow>→ Survival rate by class");
    query(
        r#"
        SELECT
            Pclass as class,
            COUNT(*) as total,
            SUM(Survived) as survived,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) as survival_rate_pct
        FROM titanic
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("<yellow>→ Survival by gender");
    query(
        r#"
        SELECT
            Sex as gender,
            COUNT(*) as total,
            SUM(Survived) as survived,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) as survival_rate_pct
        FROM titanic
        GROUP BY Sex
        ORDER BY survival_rate_pct DESC
    "#,
    );

    println("<yellow>→ Pivot: Survival by class and gender");
    query(
        r#"
        SELECT
            Pclass as class,
            COUNT(*) as total,
            SUM(CASE WHEN Sex = 'female' THEN Survived ELSE 0 END) as female_survived,
            SUM(CASE WHEN Sex = 'female' THEN 1 ELSE 0 END) as female_total,
            ROUND(100.0 * SUM(CASE WHEN Sex = 'female' THEN Survived ELSE 0 END) /
                  NULLIF(SUM(CASE WHEN Sex = 'female' THEN 1 ELSE 0 END), 0), 1) as female_pct,
            SUM(CASE WHEN Sex = 'male' THEN Survived ELSE 0 END) as male_survived,
            SUM(CASE WHEN Sex = 'male' THEN 1 ELSE 0 END) as male_total,
            ROUND(100.0 * SUM(CASE WHEN Sex = 'male' THEN Survived ELSE 0 END) /
                  NULLIF(SUM(CASE WHEN Sex = 'male' THEN 1 ELSE 0 END), 0), 1) as male_pct
        FROM titanic
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("<yellow>→ Age distribution of survivors");
    query(
        r#"
        SELECT
            CASE
                WHEN Age < 18 THEN 'Child'
                WHEN Age < 35 THEN 'Young Adult'
                WHEN Age < 60 THEN 'Adult'
                ELSE 'Senior'
            END as age_group,
            SUM(Survived) as survived,
            COUNT(*) - SUM(Survived) as died,
            ROUND(100.0 * SUM(Survived) / COUNT(*), 1) as survival_rate_pct
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

    println("<yellow>→ Fare analysis");
    query(
        r#"
        SELECT
            Pclass as class,
            ROUND(MIN(Fare), 2) as min_fare,
            ROUND(AVG(Fare), 2) as avg_fare,
            ROUND(MAX(Fare), 2) as max_fare,
            ROUND(STDDEV(Fare), 2) as std_fare
        FROM titanic
        WHERE Fare > 0
        GROUP BY Pclass
        ORDER BY Pclass
    "#,
    );

    println("\n<magenta,b>🔬 Part 3: Advanced Analytics");

    println("<yellow>→ Window functions: Running totals (FIXED)");
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
                COUNT(*) as count,
                SUM(COUNT(*)) OVER (
                    PARTITION BY Pclass
                    ORDER BY Sex
                    ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                ) as cumulative_count
            FROM survivors
            GROUP BY Pclass, Sex
        )
        SELECT * FROM grouped
        ORDER BY Pclass, Sex
    "#,
    );

    println("<yellow>→ Top 3 fares per class (FIXED - avec ROW_NUMBER)");
    query(
        r#"
        WITH ranked AS (
            SELECT
                Name,
                Pclass,
                Fare,
                ROW_NUMBER() OVER (PARTITION BY Pclass ORDER BY Fare DESC, PassengerId) as rn
            FROM titanic
            WHERE Fare > 0
        )
        SELECT Name, Pclass, Fare, rn as fare_rank_in_class
        FROM ranked
        WHERE rn <= 3
        ORDER BY Pclass, rn
    "#,
    );

    println("<yellow>→ Alternative: Distinct top fares per class");
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
                ROW_NUMBER() OVER (PARTITION BY Pclass ORDER BY Fare DESC) as rn
            FROM distinct_fares
        )
        SELECT Pclass, Fare, rn as fare_rank
        FROM ranked_fares
        WHERE rn <= 5
        ORDER BY Pclass, rn
    "#,
    );

    println("<yellow>→ Cross-dataset: Compare distributions");
    query(
        r#"
        SELECT
            'Iris petal length' as metric,
            ROUND(AVG(petal_length), 2) as mean,
            ROUND(STDDEV(petal_length), 2) as std_dev,
            ROUND(MIN(petal_length), 2) as min_val,
            ROUND(MAX(petal_length), 2) as max_val
        FROM iris
        UNION ALL
        SELECT
            'Titanic age' as metric,
            ROUND(AVG(Age), 2) as mean,
            ROUND(STDDEV(Age), 2) as std_dev,
            ROUND(MIN(Age), 2) as min_val,
            ROUND(MAX(Age), 2) as max_val
        FROM titanic
        WHERE Age IS NOT NULL
        UNION ALL
        SELECT
            'Titanic fare' as metric,
            ROUND(AVG(Fare), 2) as mean,
            ROUND(STDDEV(Fare), 2) as std_dev,
            ROUND(MIN(Fare), 2) as min_val,
            ROUND(MAX(Fare), 2) as max_val
        FROM titanic
        WHERE Fare > 0
    "#,
    );

    println("\n<cyan,b>💾 Part 4: Export Results");

    query(
        r#"
        CREATE TEMP TABLE analysis_summary AS
        SELECT
            'Iris' as dataset,
            COUNT(*) as rows,
            COUNT(DISTINCT species) as categories
        FROM iris
        UNION ALL
        SELECT
            'Titanic' as dataset,
            COUNT(*) as rows,
            COUNT(DISTINCT Pclass) as categories
        FROM titanic
    "#,
    );

    println("<yellow>→ Summary table");
    query("SELECT * FROM analysis_summary");

    query("EXPORT analysis_summary TO 'summary.csv'");
    query("EXPORT analysis_summary TO 'summary.parquet'");
    query("EXPORT analysis_summary TO 'summary.json'");

    println("<green>✓ Exported to summary.csv, summary.parquet, summary.json");

    println("\n<bright_magenta,b>🔍 Part 5: Schema Inspection");

    println("<yellow>→ Iris schema");
    query("SCHEMA SELECT * FROM iris");

    println("<yellow>→ Titanic schema");
    query("SCHEMA SELECT PassengerId, Survived, Pclass, Name, Sex, Age, Fare FROM titanic");

    println("\n<bright_green,b>✨ Analysis Complete!");
    println("<gray>All data processed in-memory with DuckDB");
}
