// webrust/examples/py_advancedio.rs

use std::collections::HashMap;
use webrust::prelude::*;
#[gui(Times New Roman 10px black !white)]
fn main() {
    println("<blue b i>🚀 WebRust Advanced I/O Features");
    println("\n<green b>1. Collection operations:");
    let numbers: Vec<i32> = (1..=5).collect();
    let numbers_str = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println("Range to Vec: <orange>{numbers_str}");
    let sum_result = numbers.iter().sum::<i32>();
    println("Sum with turbofish: <purple>{sum_result}");
    let doubled_numbers: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let doubled_numbers_str = doubled_numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println("Map and collect: <mediumblue>{doubled_numbers_str}");
    println("\n<pink b>2. Complex type conversions:");
    let age = 30;
    let complex_ternary = if age > 30 { "experienced" } else { "young" };
    println("Complex ternary: <yellow>{complex_ternary}");
    let name = "Alice";
    let reversed_name: String = name.chars().rev().collect();
    println("Chained operations: <green>{reversed_name}");
    let height = 1.75;
    let height_as_int = (height * 100.0) as i32;
    println("Type conversion chain: <blue>{height_as_int}");
    let married = true;
    let bool_parsed = married.to_string().parse::<bool>().unwrap_or(false);
    println("Boolean with turbofish: <red>{bool_parsed}");
    let first_name = "Alice";
    let last_name = "Smith";
    let initials_result = first_name
        .chars()
        .next()
        .zip(last_name.chars().next())
        .map(|(f, l)| {
            [f.to_ascii_uppercase(), l.to_ascii_uppercase()]
                .iter()
                .collect()
        })
        .unwrap_or_else(|| "N/A".to_string());
    println("Initials extraction: <magenta>{initials_result}");
    println("\n<darkcyan b>3. Number formatting:");
    let pi = std::f64::consts::PI;
    let age_fmt = 25i32;
    println("<orange>PI with various formats:");
    println("<dimgray>  - Standard: <black>{pi}");
    println("<dimgray>  - With 2 decimals: <black>{pi:.2}");
    println("<dimgray>  - With 6 decimals: <black>{pi:.6}");
    println("<dimgray>  - Scientific notation: <pink>{pi:e}");
    println("<darkcyan>Integer formats for <darkorange>{age_fmt}<darkcyan>:");
    println("<dimgray>  - Padded with zeros: <black>{age_fmt:04}");
    println("<dimgray>  - Hexadecimal lowercase: <darkorange>{age_fmt:x}");
    println("<dimgray>  - Hexadecimal uppercase: <darkorange>{age_fmt:X}");
    println("<dimgray>  - Binary: <darkgreen>{age_fmt:b}");
    println("<dimgray>  - Octal: <darkblue>{age_fmt:o}");
    println("\n<gold b>4. Container formatting:");
    let vec_3d = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
    println("<darkcyan>3D Array (:c - compact format): {vec_3d:c}");
    println("<darkcyan>3D Array (:j - tree format):\n{vec_3d:j}");
    let mut cities_data = HashMap::new();
    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert(
        "attractions".to_string(),
        "Eiffel Tower, Louvre".to_string(),
    );
    france.insert("Paris".to_string(), paris);
    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert(
        "attractions".to_string(),
        "Statue of Liberty, Times Square".to_string(),
    );
    usa.insert("New York".to_string(), new_york);
    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);
    println("<darkblue>3-level nested structure (:c - compact): {cities_data:c}");
    println("<darkblue>3-level nested structure (:j - tree format):\n{cities_data:j}");

    println("<darkred b mt25>5. Pascal's Triangle");
    let tri: Vec<Vec<u64>> = 0.to(10).then(|n| 0.to(n + 1).then(|k| C(n, k)));
    table(&tri);

    println("<darkred b mt25>6. Multiplication Table");
    let mult_table: Vec<Vec<i32>> = 1.to(10).then(|i| 1.to(10).then(|j| i * j));
    table(&mult_table).header(1.to(10)).pivot().header(1.to(10));

    println("\n<purple b>🎨 Text Alignment & Box Showcase");
    println("<black t1 |blue w{*CW} h40 tc p6>Centered Text");
    println("<black t2 dashed |green w{*CW} h40 ml p6>Left-aligned Block (ml)");
    println("<black t3 dotted |red w{*CW} h40 mr p6>Right-aligned Block (mr)");
    let lorem_text = [
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit,",
        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris",
        "nisi ut aliquip ex ea commodo consequat.",
    ]
    .join(" ");
    println(r#"<black t4 double |darkslategray !linen r12 w{*CW} h120 j p10>{lorem_text}"#);
    println("\n<orange b>Width Variations:");
    println("<black !lightblue w200 h40 mc>Quarter Width Center");
    println("<black !lightgreen w200 h40 ml>Quarter Width Left");
    println("<black !lightyellow w200 h40 mr>Quarter Width Right");
    println("\n<black !lavender w300 h48 mc>Half Width Centered Text");
    println("<black !lightcoral w300 h48 ml>Half Width Left-aligned Text");
    println("<black !lightsteelblue w300 h48 mr>Half Width Right-aligned Text");
    println("\n<teal b>Weights & Styles:");
    println("<black t1 |lightgray !white w200 h32 mc>1px border");
    println("<black t2 |gray !whitesmoke w200 h32 mc>2px border");
    println("<black t3 dashed |darkgray !lightgray w200 h32 mc>3px dashed");
    println("<black t4 dotted |black !silver w200 h32 mc>4px dotted");
    println("<black t5 double |navy !lightsteelblue w200 h32 mc>5px double");
    println("\n<magenta b>Border Style Gallery:");
    println("<black t3 solid |crimson !mistyrose w220 h36 mc>Solid");
    println("<black t3 dashed |forestgreen !honeydew w220 h36 mc>Dashed");
    println("<black t3 dotted |royalblue !aliceblue w220 h36 mc>Dotted");
    println("<black t4 double |darkorange !papayawhip w220 h36 mc>Double");
    println("\n<cyan b>Radius Variations:");
    println("<black t2 |red !lightcoral r0 w200 h36 mc>r0");
    println("<black t2 |orange !peachpuff r5 w200 h36 mc>r5");
    println("<black t2 |gold !lightyellow r10 w200 h36 mc>r10");
    println("<black t2 |limegreen !lightgreen r15 w200 h36 mc>r15");
    println("<black t2 |dodgerblue !lightblue r20 w200 h36 mc>r20");
    println("\n<indigo b>Document Layout Example:");
    println("<navy b t4 double |navy !lightcyan r8 w{*CW} h48 mc>WEBRUST FRAMEWORK v0.7.0");
    println("<darkslateblue t1 |darkslateblue !ghostwhite r5 w{*CW} h120 j p10>Webrust revolutionizes Rust development by providing Python-like simplicity without sacrificing performance. Our framework enables rapid prototyping, elegant GUI creation, and seamless mathematical computing. The innovative styling system demonstrated here showcases just a fraction of webrust's capabilities.");
    println("<crimson t2 dotted |crimson !seashell r10 w{*CW} h44 mr p8>Created with ❤️ by the Webrust Team");
    println("\n<lime b>Creative Combinations:");
    println("<black t1 dashed |teal !lightcyan r25 w{*CW / 2} h48 mc>Thin Dashed + High Radius");
    println("<black t5 double |maroon !mistyrose r0 w{*CW / 2} h48 mc>Thick Double + Sharp");
    println("<black t3 dotted |darkviolet !thistle r12 w{*CW / 2} h48 mc>Medium Dotted + Moderate Radius");
    println("\n<lime b>✨ Advanced I/O demonstration complete!");
}
