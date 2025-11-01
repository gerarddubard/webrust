// webrust/examples/py_chart.rs
use webrust::prelude::*;
use serde_json::json;

#[gui(Times New Roman 14px navy !white)]
fn main() {
    println("<white !navy r8 p6 w400 h50>WebRust Charts").align("center");

    let months = vec!["Jan","Feb","Mar","Apr","May"];
    let sales = vec![120.0, 200.0, 150.0, 300.0, 250.0];

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>1. Line (category,y)").align("center");
    line(&months, &sales);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>2. Curve (smooth)</dodgerblue>").align("center");
    curve(&months, &sales);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>3. Bar (category,y)</dodgerblue>").align("center");
    bar(&months, &sales);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>4. Area (category,y)</dodgerblue>").align("center");
    area(&months, &sales);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>5. Scatter (x,y)</dodgerblue>").align("center");
    let xs = vec![0.0,0.5,1.0,1.5,2.0];
    let ys = vec![25.0,45.0,35.0,65.0,55.0];
    scatter(&xs, &ys);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>6. Pie (%)</dodgerblue>").align("center");
    let pie_labels = vec!["Smartphones","Laptops","Tablets","Accessories"];
    let pie_values = vec![45.0,30.0,15.0,10.0];
    pie(&pie_labels, &pie_values);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>7. Doughnut</dodgerblue>").align("center");
    doughnut(&pie_labels, &pie_values);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>8. Radar (✅ tooltip)</dodgerblue>").align("center");
    let skills = vec![85.0,90.0,75.0,95.0,80.0];
    let indicators = vec![
        ("Technical",100.0),("Communication",100.0),("Leadership",100.0),("Innovation",100.0),("Quality",100.0),
    ];
    radar(&skills, &indicators);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>9. Gauge</dodgerblue>").align("center");
    gauge(87.5);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w420 h40 m15>10. Funnel</dodgerblue>").align("center");
    let stages = vec![("Visitors",10000.0),("Signups",5000.0),("Active",2500.0),("Purchases",350.0)];
    funnel(&stages);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>11. Matrix</dodgerblue>").align("center");
    let matrix_data = vec![
        vec![10.0, 20.0, 30.0, 40.0, 50.0],
        vec![15.0, 18.0, 22.0, 35.0, 45.0],
        vec![12.0, 25.0, 28.0, 31.0, 60.0],
    ];
    matrix(&matrix_data, &["A","B","C","D","E"], &["R1","R2","R3"]);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>12. Treemap (flat)</dodgerblue>").align("center");
    treemap(&[
        ("Smartphones", 45.0),
        ("Laptops", 30.0),
        ("Tablets", 15.0),
        ("Accessories", 10.0),
    ]);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>13. Treemap (paths)</dodgerblue>").align("center");
    treemap(&[
        ("Europe/France/Paris", 120.0),
        ("Europe/France/Lyon", 60.0),
        ("Europe/Italy/Rome", 80.0),
        ("USA/California/SF", 150.0),
    ]);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>14. Tree (chemins with &[&str])</dodgerblue>").align("center");
    tree(&[
        "Company/Engineering/Backend",
        "Company/Engineering/Frontend",
        "Company/Design/UX",
        "Company/Design/UI",
    ] as &[&str]);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>15. Tree (JSON with json! macro)</dodgerblue>").align("center");
    let org = json!({
        "Company": {
            "Engineering": {
                "Backend": null,
                "Frontend": null
            },
            "Design": {
                "UX": null,
                "UI": null
            }
        }
    });
    tree(&org);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>16. Tree (JSON string)</dodgerblue>").align("center");
    let org_json = r#"{
        "Company": {
            "Engineering": {
                "Backend": null,
                "Frontend": null
            },
            "Design": {
                "UX": null,
                "UI": null
            }
        }
    }"#;
    tree(org_json);
    println("<gray t1 dashed |silver w{*TW} m20>").align("center");

    println("<dodgerblue t2 |dodgerblue !aliceblue r6 w460 h40 m15>17. Function f(x)</dodgerblue>").align("center");
    function(|x| x.sin()*50.0+100.0, -4.0, 4.0, 0.25);
}
