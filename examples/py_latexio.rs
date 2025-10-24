// examples/py_latex.rs

use webrust::prelude::*;

#[gui(Garamond 12px darkpink !pink)]
fn main() {
    println("<darkred b i>🧮 WebRust LaTeX Rendering Demo");

    println("\n<darkred b>1. Basic LaTeX with print() and println() methods");

    println("\n<darkcyan b i>Einstein's equation");
    println(r"$(E = mc^2)");

    println("\n<darkcyan b i>With border styling");
    println("<t1 |darkorange r5 !mintcream>Einstein discovered that $(E = mc^2)");

    println("\n<darkcyan b i>Partial borders");
    println("<t2 B |crimson !lavender>$(E = mc^2)");

    println("\n<darkcyan b i>Mixed text and formula");
    println("<t2 H |royalblue !azure>Einstein: $(E = mc^2)");

    println("\n<darkcyan b i>Calculus notation");
    println(r"<t2 V |forestgreen !honeydew>$(\frac{d}{dx} \sin(x) = \cos(x))");

    println("\n<darkcyan b i>With variable interpolation");
    let scientist = "Einstein";
    println("<t2 T |blueviolet !ghostwhite>{scientist} formula: $(E = mc^2)");

    println("\n<darkred b>2. LaTeX Integration - Inline Math");

    println(r"Einstein's famous equation: $(E = mc^2)");

    println(r"<darkgreen>Quadratic formula: $(x = \frac{-b \pm \sqrt{b^2-4ac}}{2a})");

    println(
        r"<darkblue>Greek letters: $(\alpha), $(\beta), $(\gamma), $(\theta), $(\pi), $(\sigma)",
    );

    println(r"<purple>Trigonometry: $(\sin^2(\theta) + \cos^2(\theta) = 1)");

    println(r"<darkorange>Complex numbers: $(e^{i\pi} + 1 = 0) (Euler's identity)");

    println(
        r"<darkcyan>Calculus: $(\frac{d}{dx}\sin(x) = \cos(x)) and $(\int x^2 dx = \frac{x^3}{3} + C)",
    );

    println("\n<darkred b>3. LaTeX Integration - Display Math");

    println("Matrix representation:");
    println(r"$(\begin{pmatrix} 1 & 0 & 0 \\ 0 & 1 & 0 \\ 0 & 0 & 1 \end{pmatrix})");

    println("\nSystem of equations:");
    println(r"$(\begin{cases} x + y = 5 \\ 2x - y = 1 \end{cases})");

    println("\nFourier transform:");
    println(r"$(\mathcal{F}\{f(t)\} = \int_{-\infty}^{\infty} f(t) e^{-2\pi i \xi t} \, dt)");

    println("\nSummation formula:");
    println(r"$(\sum_{i=1}^n i = \frac{n(n+1)}{2})");

    println("\n<darkred b>4. Mixed LaTeX + Colors + Variables");

    let a = 3.0_f64;
    let b = 4.0_f64;
    let c = (a * a + b * b).sqrt();

    println("<darkgreen>Given: a = {a} and b = {b}");
    println(r"<darkblue>Then: c = $(\sqrt{a^2 + b^2}) = {c:.2}");

    let angle = std::f64::consts::PI / 4.0;
    let sin_val = angle.sin();
    let cos_val = angle.cos();

    println(r"<purple>Angle: $(\theta = \frac{\pi}{4}) ≈ {angle:.3} radians");
    println(r"<darkorange>Values: $(\sin(\theta)) = {sin_val:.3}, $(\cos(\theta)) = {cos_val:.3}");

    println("\n<darkred b>5. Scientific Notation with LaTeX");
    println("<darkcyan>Physical constants:");

    println(r"Planck constant: $(h = 6.62607015 \times 10^{-34} \, \text{J·s})");
    println(r"Speed of light: $(c = 2.998 \times 10^{8} \, \text{m/s})");
    println(r"Avogadro number: $(N_A = 6.022 \times 10^{23} \, \text{mol}^{-1})");
    println(r"Gravitational constant: $(G = 6.674 \times 10^{-11} \, \text{N·m}^2\text{/kg}^2)");

    println("\n<darkred b>6. Complex Mathematical Expressions");

    println("Schrödinger equation:");
    println(r"$(i\hbar\frac{\partial}{\partial t}\Psi(\mathbf{r},t) = \hat{H}\Psi(\mathbf{r},t))");

    println("\nMaxwell's equations:");
    println(
        r"$(\begin{align} \nabla \cdot \mathbf{E} &= \frac{\rho}{\epsilon_0} \\ \nabla \cdot \mathbf{B} &= 0 \\ \nabla \times \mathbf{E} &= -\frac{\partial \mathbf{B}}{\partial t} \\ \nabla \times \mathbf{B} &= \mu_0\mathbf{J} + \mu_0\epsilon_0\frac{\partial \mathbf{E}}{\partial t} \end{align})",
    );

    println("\n<limegreen b>✨ LaTeX demonstration complete!");
}
