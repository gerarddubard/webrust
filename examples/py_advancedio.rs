// webrust/examples/py_advancedio.rs
use webrust::prelude::*;

#[gui(bg = "pink", fg = "white", font = "Garamond", color = "Dark Pink", size = "12px")]
fn main() {
    println("\n@(darkred, bold)1. Latex with print() and println() methods");

    println("\n@(DarkCyan, bold, italic)Test 1@()");
    println(r"$(E = mc^2)");

    println("\n@(DarkCyan, bold, italic)Test 2@()");
    println(r"Einstein discovered that $(E = mc^2)")
        .weight(1).color("DarkOrange").radius(5).background("MintCream");

    println("\n@(DarkCyan, bold, italic)Test 3@()");
    println("$(E = mc^2)")
        .border(false, true, true, true)
        .weight(2).color("Crimson").background("Lavender");

    println("\n@(DarkCyan, bold, italic)Test 4@()");
    println(r"Einstein: $(E = mc^2)")
        .border(true, false, true, false)
        .weight(2).color("RoyalBlue").background("Azure");

    println("\n@(DarkCyan, bold, italic)Test 5@()");
    println(r"$(\frac{d}{dx}\sin(x) = \cos(x))")
        .border(false, true, false, true)
        .weight(2).color("ForestGreen").background("HoneyDew");

    println("\n@(DarkCyan, bold, italic)Test 6@()\n");
    print("left").align("left").width(80).weight(1).color("SlateGray").background("AliceBlue");
    print("center").align("center").width(80).weight(1).color("SlateGray").background("AliceBlue");
    print("right").align("right").width(80).weight(1).color("SlateGray").background("AliceBlue");

    println("\n@(DarkCyan, bold, italic)Test 7@()");
    let scientist = "Einstein";
    println(r"{scientist} formula: $(E = mc^2)")
        .border(false, false, true, false)
        .weight(2).color("BlueViolet").background("GhostWhite");

    println("\n@(darkred, bold)2. Tables with print() and println() methods");
    
    println("Pascal's Triangle:\n").space(0);
    let rows = 10usize;
    for n in 0..rows {
        let mut c: u64 = 1;
        for k in 0..=n {
            print("{c}")
                .weight(1).style("solid")
                .color("SteelBlue").background("WhiteSmoke")
                .width(12);
            if k < n { c = c * (n - k) as u64 / (k + 1) as u64; }
        }
        println("").space(0);
    }

    println("Multiplication Table:\n").space(0);
    let n = 9u32;
    print("@(red, bold)×@()")
        .weight(1).style("solid").color("SlateGray").background("Ivory")
        .radius(8).width(15).space(0);
    for j in 1..=n {
        print("@(blue, bold){j}@()")
            .weight(1).style("solid").color("SlateGray").background("Ivory")
            .radius(8).width(15).space(0);
    }
    println("").space(0);
    for i in 1..=n {
        print("@(blue, bold){i}@()")
            .weight(1).style("solid").color("SlateGray").background("Ivory")
            .radius(8).width(15);
        for j in 1..=n {
            print("{i*j}")
                .weight(1).style("solid").color("LightSteelBlue").background("White")
                .radius(8).width(15);
        }
        println("").space(0);
    }

    println("\n@(darkred, bold)3. LaTeX Integration - Inline Math");
    println(r"Einstein's famous equation: $(E = mc^2)");
    println(r"@(darkgreen)Quadratic formula:@() $(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})");
    println(r"@(darkblue)Greek letters:@() $(\alpha), $(\beta), $(\gamma), $(\theta), $(\pi), $(\sigma)");
    println(r"@(purple)Trigonometry:@() $(\sin^2(\theta) + \cos^2(\theta) = 1)");
    println(r"@(darkorange)Complex numbers:@() $(e^{i\pi} + 1 = 0) (Euler's identity)");
    println(r"@(darkcyan)Calculus:@() $(\frac{d}{dx}\sin(x) = \cos(x)) and $(\int x^2 dx = \frac{x^3}{3} + C)");

    println("\n@(darkred, bold)4. LaTeX Integration - Display Math");
    println("Matrix representation:");
    println(r"$(\begin{pmatrix} 1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1 \end{pmatrix})");

    println("System of equations:");
    println(r"$(\begin{cases} x + y = 5 \\ 2x - y = 1 \end{cases})");

    println("Fourier transform:");
    println(r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} dt)");

    println("Summation formula:");
    println(r"$(\sum_{i=1}^n i = \frac{n(n+1)}{2})");

    println("\n@(darkred, bold)5. Mixed LaTeX + Colors + Variables");
    let a = 3.0_f64;
    let b = 4.0_f64;
    let c = (a*a + b*b).sqrt();

    println("@(darkgreen)Given:@() a = {a} and b = {b}");
    println(r"@(darkblue)Then:@() c = $(\sqrt{a^2 + b^2}) = {c:.2}");

    let angle = std::f64::consts::PI / 4.0;
    let sin_val = angle.sin();
    let cos_val = angle.cos();
    println(r"@(purple)Angle:@() $(\theta = \frac{\pi}{4}) ≈ {angle:.3} radians");
    println(r"@(darkorange)Values:@() $(\sin(\theta)) = {sin_val:.3}, $(\cos(\theta)) = {cos_val:.3}");

    println("\n@(darkred, bold)6. Scientific Notation with LaTeX");
    println("@(darkcyan)Physical constants:");
    println(r"Planck constant: $(h = 6.62607015 \times 10^{-34}) $(m^2 kg / s)");
    println(r"Speed of light: $(c = 2.998 \times 10^{8}) m/s");
    println(r"Avogadro number: $(N_A = 6.022 \times 10^{23}) $(mol^{-1})");
    println(r"Energy-mass relation: $(E = mc^2 = m \times 8.988 \times 10^{16})");

    println("\n@(darkred, bold)7. Complex Mathematical Expressions");
    println("Schrödinger equation:");
    println(r"$(i\hbar\frac{\partial}{\partial t}\Psi(\mathbf{r},t) = \hat{H}\Psi(\mathbf{r},t))");

    println("Maxwell's equations:");
    println(r"$(\begin{align} \nabla \cdot \mathbf{E} &= \frac{\rho}{\epsilon_0} \\ \nabla \cdot \mathbf{B} &= 0 \\ \nabla \times \mathbf{E} &= -\frac{\partial \mathbf{B}}{\partial t} \\ \nabla \times \mathbf{B} &= \mu_0\mathbf{J} + \mu_0\epsilon_0\frac{\partial \mathbf{E}}{\partial t} \end{align})");
}