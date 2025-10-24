use std::collections::HashMap;
use webrust::prelude::*;

#[gui(Arial, 12px, black, !white)]
fn main() {
    println("<blue,b>🧪 WebRust Table Tests");
    println("");

    println("<green,b>1. Simple Vector");
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

    println("<green,b>2. 2x3 Matrix");
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

    println("<green,b>3. Simple HashMap");
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    println("Basic HashMap:");
    table(&scores);

    println("Pivoted:");
    table(&scores).pivot();

    println("");

    println("<green,b>4. HashMap with Vectors");
    let mut grades = HashMap::new();
    grades.insert("Math", vec![18, 16, 19]);
    grades.insert("Physics", vec![15, 17, 16]);

    println("HashMap with vectors:");
    table(&grades);

    println("Pivoted:");
    table(&grades).pivot();

    println("");

    println("<green,b>5. Textual Data");
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

    println("<green,b>6. Complex Structure");
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

    println("<green,b>7. When .merge() is Useful");
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

    println("<green,b>8. Mathematical Examples");
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

    println("<green,b>9. LaTeX Examples (Simple vs Complex)");

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

    println("<green,b>10. Periodic Table (colored tiles)\n");
    const A: &str = "MistyRose";
    const E: &str = "LemonChiffon";
    const T: &str = "LightSteelBlue";
    const P: &str = "Gainsboro";
    const M: &str = "PaleGreen";
    const N: &str = "HoneyDew";
    const H: &str = "Moccasin";
    const G: &str = "Plum";
    const L: &str = "Lavender";
    const C: &str = "Thistle";
    const B: &str = "#333";
    const X: (&str, &str, &str) = ("", "", "");
    let table = vec![
        vec![
            ("H", N, B),
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
            ("He", G, B),
        ],
        vec![
            ("Li", A, B),
            ("Be", E, B),
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
            ("B", M, B),
            ("C", N, B),
            ("N", N, B),
            ("O", N, B),
            ("F", H, B),
            ("Ne", G, B),
        ],
        vec![
            ("Na", A, B),
            ("Mg", E, B),
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
            ("Al", P, B),
            ("Si", M, B),
            ("P", N, B),
            ("S", N, B),
            ("Cl", H, B),
            ("Ar", G, B),
        ],
        vec![
            ("K", A, B),
            ("Ca", E, B),
            ("Sc", T, B),
            ("Ti", T, B),
            ("V", T, B),
            ("Cr", T, B),
            ("Mn", T, B),
            ("Fe", T, B),
            ("Co", T, B),
            ("Ni", T, B),
            ("Cu", T, B),
            ("Zn", T, B),
            ("Ga", P, B),
            ("Ge", M, B),
            ("As", M, B),
            ("Se", P, B),
            ("Br", H, B),
            ("Kr", G, B),
        ],
        vec![
            ("Rb", A, B),
            ("Sr", E, B),
            ("Y", T, B),
            ("Zr", T, B),
            ("Nb", T, B),
            ("Mo", T, B),
            ("Tc", T, B),
            ("Ru", T, B),
            ("Rh", T, B),
            ("Pd", T, B),
            ("Ag", T, B),
            ("Cd", T, B),
            ("In", P, B),
            ("Sn", P, B),
            ("Sb", M, B),
            ("Te", M, B),
            ("I", H, B),
            ("Xe", G, B),
        ],
        vec![
            ("Cs", A, B),
            ("Ba", E, B),
            ("La", L, B),
            ("Hf", T, B),
            ("Ta", T, B),
            ("W", T, B),
            ("Re", T, B),
            ("Os", T, B),
            ("Ir", T, B),
            ("Pt", T, B),
            ("Au", T, B),
            ("Hg", T, B),
            ("Tl", P, B),
            ("Pb", P, B),
            ("Bi", P, B),
            ("Po", P, B),
            ("At", H, B),
            ("Rn", G, B),
        ],
        vec![
            ("Fr", A, B),
            ("Ra", E, B),
            ("Ac", C, B),
            ("Rf", T, B),
            ("Db", T, B),
            ("Sg", T, B),
            ("Bh", T, B),
            ("Hs", T, B),
            ("Mt", T, B),
            ("Ds", T, B),
            ("Rg", T, B),
            ("Cn", T, B),
            ("Nh", P, B),
            ("Fl", P, B),
            ("Mc", P, B),
            ("Lv", P, B),
            ("Ts", H, B),
            ("Og", G, B),
        ],
        vec![X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X],
        vec![
            X,
            X,
            X,
            ("La", L, B),
            ("Ce", L, B),
            ("Pr", L, B),
            ("Nd", L, B),
            ("Pm", L, B),
            ("Sm", L, B),
            ("Eu", L, B),
            ("Gd", L, B),
            ("Tb", L, B),
            ("Dy", L, B),
            ("Ho", L, B),
            ("Er", L, B),
            ("Tm", L, B),
            ("Yb", L, B),
            ("Lu", L, B),
        ],
        vec![
            X,
            X,
            X,
            ("Ac", C, B),
            ("Th", C, B),
            ("Pa", C, B),
            ("U", C, B),
            ("Np", C, B),
            ("Pu", C, B),
            ("Am", C, B),
            ("Cm", C, B),
            ("Bk", C, B),
            ("Cf", C, B),
            ("Es", C, B),
            ("Fm", C, B),
            ("Md", C, B),
            ("No", C, B),
            ("Lr", C, B),
        ],
    ];
    for row in &table {
        println("<mb0>");
        for &(sym, bg, border) in row {
            if sym.is_empty() {
                print("<transparent !transparent w25 t1 |transparent mc>");
            } else {
                print("<{border} !{bg} w25 t1 |transparent mc>{sym}");
            }
        }
    }

    println("\n<cyan,b>✨ Notice the smart usage of raw strings only when needed for readability!");
    println("<gray,i>Simple LaTeX: $(E = mc^2) - no raw string needed");
    println("<gray,i>Complex LaTeX: r\"$(\\\\frac{...}{...})\" - raw string for clarity");
}
