// webrust/examples/py_table.rs

use std::collections::HashMap;
use webrust::prelude::*;

#[gui(Arial 12px black !white)]
fn main() {
    println("<white !fuchsia r8 p6 w150 h25>🧪 WebRust Table Tests").align("center");
    println("");

    println("<green b>1. Simple Vector");
    let numbers = vec![10, 20, 30, 40, 50];
    println("Basic vector:");
    table(&numbers);

    println("With header:");
    table(&numbers).header(["A", "B", "C", "D", "E"]);

    println("Pivoted:");
    table(&numbers).header(["A", "B", "C", "D", "E"]).pivot();

    println("Pivoted with headers:");
    table(&numbers)
        .header(["A", "B", "C", "D", "E"])
        .pivot()
        .header(["Values"]);

    println("");

    println("<green b>2. 2x3 Matrix");
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println("Basic matrix:");
    table(&matrix);

    println("With headers:");
    table(&matrix).header(["$(x)", "$(y)", "$(z)"]);

    println("Pivoted with headers:");
    table(&matrix)
        .header(["$(x)", "$(y)", "$(z)"])
        .pivot()
        .header(["$(\\vec{u})", "$(\\vec{v})"]);

    println("");

    println("<green b>3. Simple HashMap");
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    println("Basic HashMap:");
    table(&scores);

    println("Pivoted:");
    table(&scores).pivot();

    println("");

    println("<green b>4. HashMap with Vectors");
    let mut grades = HashMap::new();
    grades.insert("Math", vec![18, 16, 19]);
    grades.insert("Physics", vec![15, 17, 16]);

    println("HashMap with vectors:");
    table(&grades);

    println("Pivoted:");
    table(&grades).pivot();

    println("");

    println("<green b>5. Textual Data");
    let employees = vec![
        vec!["Alice", "25", "Engineer"],
        vec!["Bob", "30", "Designer"],
        vec!["Charlie", "35", "Manager"],
    ];

    println("Text matrix:");
    table(&employees);

    println("With headers:");
    table(&employees).header(["Name", "Age", "Job"]);

    println("Pivoted with headers:");
    table(&employees).header(["Name", "Age", "Job"]).pivot();

    println("");

    println("<green b>6. Complex Structure");
    let mut complex = HashMap::new();
    let mut paris_data = HashMap::new();
    paris_data.insert("population", 2_200_000);
    paris_data.insert("area ($(km^2))", 105);
    complex.insert("Paris", paris_data);

    let mut lyon_data = HashMap::new();
    lyon_data.insert("population", 515_000);
    lyon_data.insert("area ($(km^2))", 47);
    complex.insert("Lyon", lyon_data);

    println("2-level nested structure:");
    table(&complex);

    println("Pivoted:");
    table(&complex).pivot();

    let mut cities_data = HashMap::new();
    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert(
        "attractions".to_string(),
        "Eiffel Tower, Louvre".to_string(),
    );
    france.insert("Paris".to_string(), paris);
    let mut marseille = HashMap::new();
    marseille.insert("population".to_string(), "870K".to_string());
    marseille.insert("attractions".to_string(), "Old Port, Calanques".to_string());
    france.insert("Marseille".to_string(), marseille);
    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert(
        "attractions".to_string(),
        "Statue of Liberty, Times Square".to_string(),
    );
    usa.insert("New York".to_string(), new_york);
    let mut los_angeles = HashMap::new();
    los_angeles.insert("population".to_string(), "4M".to_string());
    los_angeles.insert(
        "attractions".to_string(),
        "Hollywood, Venice Beach".to_string(),
    );
    usa.insert("Los Angeles".to_string(), los_angeles);
    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);

    println("3-level nested structure:");
    table(&cities_data);

    println("Pivoted:");
    table(&cities_data).pivot();

    println("");

    println("<green b>7. When .merge() is Useful");
    let survey_data = vec![
        vec!["Excellent", "Customer Service"],
        vec!["Excellent", "Product Quality"],
        vec!["Excellent", "Website Design"],
        vec!["Good", "Delivery Speed"],
        vec!["Good", "Ordering Process"],
        vec!["Average", "Price"],
        vec!["Average", "Support Hours"],
    ];

    println("Survey results (default - no grouping):");
    table(&survey_data).header(["Rating", "Aspect"]);

    println("Survey results (with .merge() - visual grouping):");
    table(&survey_data).header(["Rating", "Aspect"]).merge();
    println("Note: .merge() groups adjacent identical ratings");

    let color_matrix = vec![
        vec!["Red", "Red", "Blue"],
        vec!["Red", "Red", "Blue"],
        vec!["Green", "Green", "Blue"],
    ];

    println("Color matrix (default - each cell distinct):");
    table(&color_matrix).header(["1", "2", "3"]);

    println("Color matrix (with .merge() - color blocks):");
    table(&color_matrix).header(["1", "2", "3"]).merge();
    println("Perfect for visualizing color regions or patterns");

    println("");

    println("<green b>8. Mathematical Examples");
    let truth_table = vec![
        vec!["0", "0", "0", "0"],
        vec!["0", "1", "0", "1"],
        vec!["1", "0", "0", "1"],
        vec!["1", "1", "1", "1"],
    ];
    println("Boolean logic truth table:");
    table(&truth_table).header(["$(A)", "$(B)", "$(A \\land B)", "$(A \\lor B)"]);

    println("Pascal's Triangle (triangular structure preserved):");
    let mut pascal_triangle = Vec::new();
    for n in 0i32.to(9) {
        let mut row = Vec::new();
        for k in 0.to(n + 1) {
            let mut c = 1u32;
            for i in 0.to(k) {
                c = c * (n - i) as u32 / (i + 1) as u32;
            }
            row.push(c.to_string());
        }
        pascal_triangle.push(row);
    }
    table(&pascal_triangle);

    println("Multiplication table (9x9):");
    let headers: Vec<String> = (1..=9).map(|i| format!("x{}", i)).collect();
    let data: Vec<Vec<u32>> = (1..=9).map(|i| (1..=9).map(|j| i * j).collect()).collect();
    table(&data).header(headers.clone()).pivot().header(headers);

    println("<green b>9. LaTeX Examples (Simple vs Complex)");

    println("Simple Greek alphabet:");
    let greek = vec![
        vec!["Alpha", "$(\\alpha)", "$(A)"],
        vec!["Beta", "$(\\beta)", "$(B)"],
        vec!["Gamma", "$(\\gamma)", "$(\\Gamma)"],
        vec!["Delta", "$(\\delta)", "$(\\Delta)"],
        vec!["Epsilon", "$(\\epsilon)", "$(E)"],
        vec!["Pi", "$(\\pi)", "$(\\Pi)"],
    ];
    table(&greek).header(["Name", "Lowercase", "Uppercase"]);

    println("Simple trigonometric values:");
    let trig = vec![
        vec!["$(0)", "$(0)", "$(1)", "$(0)"],
        vec![
            r"$(\frac{\pi}{6})",
            r"$(\frac{1}{2})",
            r"$(\frac{\sqrt{3}}{2})",
            r"$(\frac{1}{\sqrt{3}})",
        ],
        vec![
            r"$(\frac{\pi}{4})",
            r"$(\frac{\sqrt{2}}{2})",
            r"$(\frac{\sqrt{2}}{2})",
            "$(1)",
        ],
        vec![
            r"$(\frac{\pi}{3})",
            r"$(\frac{\sqrt{3}}{2})",
            r"$(\frac{1}{2})",
            r"$(\sqrt{3})",
        ],
        vec![r"$(\frac{\pi}{2})", "$(1)", "$(0)", "not defined"],
        vec![r"$(\pi)", "$(0)", "$(-1)", "$(0)"],
        vec![r"$(\frac{3\pi}{2})", "$(-1)", "$(0)", "not defined"],
        vec![r"$2\pi$", "$(0)", "$(1)", "$(0)"],
    ];

    table(&trig).header([
        "$(\\theta)",
        "$(\\sin\\theta)",
        "$(\\cos\\theta)",
        "$(\\tan\\theta)",
    ]);

    println("Complex equations (using raw strings for clarity):");
    let equations = vec![
        vec![
            "Maxwell 1",
            r"$(\nabla \cdot \mathbf{E} = \frac{\rho}{\epsilon_0})",
        ],
        vec!["Maxwell 2", r"$(\nabla \cdot \mathbf{B} = 0)"],
        vec![
            "Maxwell 3",
            r"$(\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t})",
        ],
        vec![
            "Schrödinger",
            r"$(i\hbar\frac{\partial}{\partial t}\Psi = \hat{H}\Psi)",
        ],
    ];
    table(&equations).header(["Equation", "Mathematical Form"]);

    println("2D Transformation matrices (complex - use raw strings):");
    let transforms_2d = vec![
        vec![
            "Rotation",
            r"$(\begin{pmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{pmatrix})",
        ],
        vec![
            "Scaling",
            r"$(\begin{pmatrix} s_x & 0 \\ 0 & s_y \end{pmatrix})",
        ],
        vec![
            "Reflection X",
            r"$(\begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix})",
        ],
    ];
    table(&transforms_2d).header(["Transform", "2D Matrix"]);

    println("3D Homogeneous transformation matrices (very complex - raw strings essential):");
    let transforms_3d = vec![
        vec![
            "3D Rotation Z",
            r"$(\begin{pmatrix} \cos\theta & -\sin\theta & 0 & 0 \\ \sin\theta & \cos\theta & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix})",
        ],
        vec![
            "3D Scaling",
            r"$(\begin{pmatrix} s_x & 0 & 0 & 0 \\ 0 & s_y & 0 & 0 \\ 0 & 0 & s_z & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix})",
        ],
        vec![
            "3D Translation",
            r"$(\begin{pmatrix} 1 & 0 & 0 & t_x \\ 0 & 1 & 0 & t_y \\ 0 & 0 & 1 & t_z \\ 0 & 0 & 0 & 1 \end{pmatrix})",
        ],
    ];
    table(&transforms_3d).header(["Transform", "3D Homogeneous Matrix"]);

    println("Complex mathematical expressions (showcasing when raw strings help):");
    let complex_math = vec![
        vec![
            "Fourier Transform",
            r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} dt)",
        ],
        vec![
            "Gaussian Integral",
            r"$(\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi})",
        ],
        vec!["Euler's Identity", "$(e^{i\\pi} + 1 = 0)"],
        vec![
            "Riemann Zeta",
            r"$(\zeta(s) = \sum_{n=1}^{\infty} \frac{1}{n^s})",
        ],
    ];
    table(&complex_math).header(["Name", "Expression"]);
    println("\n<cyan,b>✨ Notice the smart usage of raw strings only when needed for readability!");
    println("<gray,i>Simple LaTeX: $(E = mc^2) - no raw string needed");
    println("<gray,i>Complex LaTeX: r\"$(\\\\frac{...}{...})\" - raw string for clarity");

    println("<green b>10. Periodic Table\n");
    const COL_ALKALI: &str = "#ff6666";
    const COL_ALKALINE: &str = "#ffd633";
    const COL_TRANSITION: &str = "#6699ff";
    const COL_POST: &str = "#bfbfbf";
    const COL_METALLOID: &str = "#33cc33";
    const COL_NONMETAL: &str = "#99ff99";
    const COL_HALOGEN: &str = "#ff9933";
    const COL_NOBLE: &str = "#cc66ff";
    const COL_LANTH: &str = "#9966ff";
    const COL_ACTIN: &str = "#ff66cc";
    const COL_TEXT: &str = "#fff";
    const COL_BORDER: &str = "#222";
    const X: (&str, &str, &str) = ("", "", "");
    let table = vec![
        vec![
            ("H", COL_NONMETAL, COL_TEXT),
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            ("He", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("Li", COL_ALKALI, COL_TEXT),
            ("Be", COL_ALKALINE, COL_TEXT),
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            ("B", COL_METALLOID, COL_TEXT),
            ("C", COL_NONMETAL, COL_TEXT),
            ("N", COL_NONMETAL, COL_TEXT),
            ("O", COL_NONMETAL, COL_TEXT),
            ("F", COL_HALOGEN, COL_TEXT),
            ("Ne", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("Na", COL_ALKALI, COL_TEXT),
            ("Mg", COL_ALKALINE, COL_TEXT),
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            X,
            ("Al", COL_POST, COL_TEXT),
            ("Si", COL_METALLOID, COL_TEXT),
            ("P", COL_NONMETAL, COL_TEXT),
            ("S", COL_NONMETAL, COL_TEXT),
            ("Cl", COL_HALOGEN, COL_TEXT),
            ("Ar", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("K", COL_ALKALI, COL_TEXT),
            ("Ca", COL_ALKALINE, COL_TEXT),
            ("Sc", COL_TRANSITION, COL_TEXT),
            ("Ti", COL_TRANSITION, COL_TEXT),
            ("V", COL_TRANSITION, COL_TEXT),
            ("Cr", COL_TRANSITION, COL_TEXT),
            ("Mn", COL_TRANSITION, COL_TEXT),
            ("Fe", COL_TRANSITION, COL_TEXT),
            ("Co", COL_TRANSITION, COL_TEXT),
            ("Ni", COL_TRANSITION, COL_TEXT),
            ("Cu", COL_TRANSITION, COL_TEXT),
            ("Zn", COL_TRANSITION, COL_TEXT),
            ("Ga", COL_POST, COL_TEXT),
            ("Ge", COL_METALLOID, COL_TEXT),
            ("As", COL_METALLOID, COL_TEXT),
            ("Se", COL_NONMETAL, COL_TEXT),
            ("Br", COL_HALOGEN, COL_TEXT),
            ("Kr", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("Rb", COL_ALKALI, COL_TEXT),
            ("Sr", COL_ALKALINE, COL_TEXT),
            ("Y", COL_TRANSITION, COL_TEXT),
            ("Zr", COL_TRANSITION, COL_TEXT),
            ("Nb", COL_TRANSITION, COL_TEXT),
            ("Mo", COL_TRANSITION, COL_TEXT),
            ("Tc", COL_TRANSITION, COL_TEXT),
            ("Ru", COL_TRANSITION, COL_TEXT),
            ("Rh", COL_TRANSITION, COL_TEXT),
            ("Pd", COL_TRANSITION, COL_TEXT),
            ("Ag", COL_TRANSITION, COL_TEXT),
            ("Cd", COL_TRANSITION, COL_TEXT),
            ("In", COL_POST, COL_TEXT),
            ("Sn", COL_POST, COL_TEXT),
            ("Sb", COL_METALLOID, COL_TEXT),
            ("Te", COL_METALLOID, COL_TEXT),
            ("I", COL_HALOGEN, COL_TEXT),
            ("Xe", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("Cs", COL_ALKALI, COL_TEXT),
            ("Ba", COL_ALKALINE, COL_TEXT),
            ("La", COL_LANTH, COL_TEXT),
            ("Hf", COL_TRANSITION, COL_TEXT),
            ("Ta", COL_TRANSITION, COL_TEXT),
            ("W", COL_TRANSITION, COL_TEXT),
            ("Re", COL_TRANSITION, COL_TEXT),
            ("Os", COL_TRANSITION, COL_TEXT),
            ("Ir", COL_TRANSITION, COL_TEXT),
            ("Pt", COL_TRANSITION, COL_TEXT),
            ("Au", COL_TRANSITION, COL_TEXT),
            ("Hg", COL_TRANSITION, COL_TEXT),
            ("Tl", COL_POST, COL_TEXT),
            ("Pb", COL_POST, COL_TEXT),
            ("Bi", COL_POST, COL_TEXT),
            ("Po", COL_POST, COL_TEXT),
            ("At", COL_HALOGEN, COL_TEXT),
            ("Rn", COL_NOBLE, COL_TEXT),
        ],
        vec![
            ("Fr", COL_ALKALI, COL_TEXT),
            ("Ra", COL_ALKALINE, COL_TEXT),
            ("Ac", COL_ACTIN, COL_TEXT),
            ("Rf", COL_TRANSITION, COL_TEXT),
            ("Db", COL_TRANSITION, COL_TEXT),
            ("Sg", COL_TRANSITION, COL_TEXT),
            ("Bh", COL_TRANSITION, COL_TEXT),
            ("Hs", COL_TRANSITION, COL_TEXT),
            ("Mt", COL_TRANSITION, COL_TEXT),
            ("Ds", COL_TRANSITION, COL_TEXT),
            ("Rg", COL_TRANSITION, COL_TEXT),
            ("Cn", COL_TRANSITION, COL_TEXT),
            ("Nh", COL_POST, COL_TEXT),
            ("Fl", COL_POST, COL_TEXT),
            ("Mc", COL_POST, COL_TEXT),
            ("Lv", COL_POST, COL_TEXT),
            ("Ts", COL_HALOGEN, COL_TEXT),
            ("Og", COL_NOBLE, COL_TEXT),
        ],
        vec![X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
        vec![
            X,
            X,
            X,
            ("La", COL_LANTH, COL_TEXT),
            ("Ce", COL_LANTH, COL_TEXT),
            ("Pr", COL_LANTH, COL_TEXT),
            ("Nd", COL_LANTH, COL_TEXT),
            ("Pm", COL_LANTH, COL_TEXT),
            ("Sm", COL_LANTH, COL_TEXT),
            ("Eu", COL_LANTH, COL_TEXT),
            ("Gd", COL_LANTH, COL_TEXT),
            ("Tb", COL_LANTH, COL_TEXT),
            ("Dy", COL_LANTH, COL_TEXT),
            ("Ho", COL_LANTH, COL_TEXT),
            ("Er", COL_LANTH, COL_TEXT),
            ("Tm", COL_LANTH, COL_TEXT),
            ("Yb", COL_LANTH, COL_TEXT),
            ("Lu", COL_LANTH, COL_TEXT),
        ],
        vec![
            X,
            X,
            X,
            ("Ac", COL_ACTIN, COL_TEXT),
            ("Th", COL_ACTIN, COL_TEXT),
            ("Pa", COL_ACTIN, COL_TEXT),
            ("U", COL_ACTIN, COL_TEXT),
            ("Np", COL_ACTIN, COL_TEXT),
            ("Pu", COL_ACTIN, COL_TEXT),
            ("Am", COL_ACTIN, COL_TEXT),
            ("Cm", COL_ACTIN, COL_TEXT),
            ("Bk", COL_ACTIN, COL_TEXT),
            ("Cf", COL_ACTIN, COL_TEXT),
            ("Es", COL_ACTIN, COL_TEXT),
            ("Fm", COL_ACTIN, COL_TEXT),
            ("Md", COL_ACTIN, COL_TEXT),
            ("No", COL_ACTIN, COL_TEXT),
            ("Lr", COL_ACTIN, COL_TEXT),
        ],
    ];
    for row in &table {
        println("<mb0>");
        for &(sym, bg, fg) in row {
            if sym.is_empty() {
                print("<transparent !transparent w38 h30 m1 p1 r4 t1 |transparent mc>");
            } else {
                print(&format!(
                    "<{fg} !{bg} w38 h30 m1 p2 r4 t1 |{COL_BORDER} mc>{sym}"
                ));
            }
        }
    }
}
