// examples/py_latex.rs
// Run with: cargo run --example latex
//
// This example demonstrates WebRust's LaTeX rendering capabilities with MathJax:
// - Inline math expressions with $(...)
// - Display math with matrices, systems, integrals
// - Mixed LaTeX with styled text and variables
// - Scientific notation and physical constants
// - Complex mathematical expressions (Schrödinger, Maxwell)
//
// Core features showcased:
// * `$(E = mc^2)` - Inline LaTeX rendering
// * `$(\frac{a}{b})` - Fractions
// * `$(\sum_{i=1}^n)` - Summations and limits
// * `$(\begin{pmatrix}...\end{pmatrix})` - Matrices
// * `$(\int_{a}^{b})` - Integrals
// * Mixed styling with @(color) and LaTeX
//
// LaTeX syntax notes:
// * Greek letters: \alpha, \beta, \gamma, \theta, \pi, \sigma, \omega
// * Superscripts: x^2, e^{i\pi}
// * Subscripts: x_1, a_{ij}
// * Fractions: \frac{numerator}{denominator}
// * Roots: \sqrt{x}, \sqrt[n]{x}
// * Operators: \sum, \prod, \int, \lim
// * Relations: =, \neq, \leq, \geq, \approx
// * Parentheses: \left( ... \right), \left[ ... \right]
//
// Tips:
// * Use raw strings r"..." to avoid escaping backslashes
// * Inline math: $(...)$ for short expressions
// * Display math: $(\begin{...}...\end{...})$ for structured content
// * Combine with color styling: @(blue)Text@() $(formula)$
// * Use separate $(...)$ expressions for clarity (see examples below)

use webrust::prelude::*;

#[gui(bg = "pink", fg = "white", font = "Garamond", color = "DarkPink", size = "12px")]
fn main() {
    println("@(darkred, bold, italic)🧮 WebRust LaTeX Rendering Demo");

    // -------------------------------------------------------------------------
    // 1) Basic inline LaTeX with styling
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)1. Basic LaTeX with print() and println() methods");

    println("\n@(DarkCyan, bold, italic)Einstein's equation@()");
    println(r"$(E = mc^2)");

    println("\n@(DarkCyan, bold, italic)With border styling@()");
    println(r"Einstein discovered that $(E = mc^2)")
        .weight(1)
        .color("DarkOrange")
        .radius(5)
        .background("MintCream");

    println("\n@(DarkCyan, bold, italic)Partial borders@()");
    println("$(E = mc^2)")
        .border(false, true, true, true)
        .weight(2)
        .color("Crimson")
        .background("Lavender");

    println("\n@(DarkCyan, bold, italic)Mixed text and formula@()");
    println(r"Einstein: $(E = mc^2)")
        .border(true, false, true, false)
        .weight(2)
        .color("RoyalBlue")
        .background("Azure");

    println("\n@(DarkCyan, bold, italic)Calculus notation@()");
    println(r"$(\frac{d}{dx} \sin(x) = \cos(x))")
        .border(false, true, false, true)
        .weight(2)
        .color("ForestGreen")
        .background("HoneyDew");

    println("\n@(DarkCyan, bold, italic)With variable interpolation@()");
    let scientist = "Einstein";
    println(r"{scientist} formula: $(E = mc^2)")
        .border(false, false, true, false)
        .weight(2)
        .color("BlueViolet")
        .background("GhostWhite");

    // -------------------------------------------------------------------------
    // 2) LaTeX Integration - Inline Math
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)2. LaTeX Integration - Inline Math");

    println(r"Einstein's famous equation: $(E = mc^2)");

    println(r"@(darkgreen)Quadratic formula:@() $(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})");

    println(r"@(darkblue)Greek letters:@() $(\alpha), $(\beta), $(\gamma), $(\theta), $(\pi), $(\sigma)");

    println(r"@(purple)Trigonometry:@() $(\sin^2(\theta) + \cos^2(\theta) = 1)");

    println(r"@(darkorange)Complex numbers:@() $(e^{i\pi} + 1 = 0) (Euler's identity)");

    println(r"@(darkcyan)Calculus:@() $(\frac{d}{dx}\sin(x) = \cos(x)) and $(\int x^2 dx = \frac{x^3}{3} + C)");

    // -------------------------------------------------------------------------
    // 3) LaTeX Integration - Display Math
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)3. LaTeX Integration - Display Math");

    println("Matrix representation:");
    println(r"$(\begin{pmatrix} 1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1 \end{pmatrix})");

    println("\nSystem of equations:");
    println(r"$(\begin{cases} x + y = 5 \\ 2x - y = 1 \end{cases})");

    println("\nFourier transform:");
    println(r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} \, dt)");

    println("\nSummation formula:");
    println(r"$(\sum_{i=1}^n i = \frac{n(n+1)}{2})");

    // -------------------------------------------------------------------------
    // 4) Mixed LaTeX + Colors + Variables
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)4. Mixed LaTeX + Colors + Variables");

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

    // -------------------------------------------------------------------------
    // 5) Scientific Notation with LaTeX
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)5. Scientific Notation with LaTeX");
    println("@(darkcyan)Physical constants:");

    // ✅ Units inside the LaTeX expression
    println(r"Planck constant: $(h = 6.62607015 \times 10^{-34} \, \text{J·s})");
    println(r"Speed of light: $(c = 2.998 \times 10^{8} \, \text{m/s})");
    println(r"Avogadro number: $(N_A = 6.022 \times 10^{23} \, \text{mol}^{-1})");
    println(r"Gravitational constant: $(G = 6.674 \times 10^{-11} \, \text{N·m}^2\text{/kg}^2)");

    // -------------------------------------------------------------------------
    // 6) Complex Mathematical Expressions
    // -------------------------------------------------------------------------
    println("\n@(darkred, bold)6. Complex Mathematical Expressions");

    println("Schrödinger equation:");
    println(r"$(i\hbar\frac{\partial}{\partial t}\Psi(\mathbf{r},t) = \hat{H}\Psi(\mathbf{r},t))");

    println("\nMaxwell's equations:");
    println(r"$(\begin{align} \nabla \cdot \mathbf{E} &= \frac{\rho}{\epsilon_0} \\ \nabla \cdot \mathbf{B} &= 0 \\ \nabla \times \mathbf{E} &= -\frac{\partial \mathbf{B}}{\partial t} \\ \nabla \times \mathbf{B} &= \mu_0\mathbf{J} + \mu_0\epsilon_0\frac{\partial \mathbf{E}}{\partial t} \end{align})");

    println("\n@(bright_green, bold)✨ LaTeX demonstration complete!");
}