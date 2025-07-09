// webrust/examples/py_latex.rs
use webrust::prelude::*;

#[gui]
fn main() {
    latex("\\text{Einstein's equation: } E = mc^2 \\\\[0.5cm]");
    latex("\\text{Integral: } \\int_0^1 x^2 dx = \\frac{1}{3} \\\\[0.5cm]");
    latex("\\text{Identity matrix: } \\begin{pmatrix} 1 & 0 \\\\ 0 & 1 \\end{pmatrix} \\\\[0.5cm]");
    latex("\\text{System of equations: } \\begin{cases} x + y = 5 \\\\ 2x - y = 1 \\end{cases} \\\\[0.5cm]");
    latex("\\text{Quadratic formula: } x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a} \\\\[0.5cm]");
    latex("\\text{Sum: } \\sum_{i=1}^n i = \\frac{n(n+1)}{2} \\\\[0.5cm]");
    latex("\\text{3x3 matrix: } \\begin{bmatrix} a & b & c \\\\ d & e & f \\\\ g & h & i \\end{bmatrix} \\\\[0.5cm]");
    latex("\\text{3x3 determinant: } \\left|\\begin{array}{ccc}1&\\sqrt{3}&4\\\\0&-4&10\\\\2&-6&3\\end{array}\\right| \\\\[0.5cm]");
    latex("\\text{Calculation: } \\begin{align} f(x) &= x^2 + 2x + 1 \\\\ &= (x+1)^2 \\end{align} \\\\[0.8cm]");

    latex("\\text{--- 1D ARRAYS ---} \\\\[0.3cm]");
    latex("\\text{Row vector: } \\begin{bmatrix} 1 & 2 & 3 & 4 & 5 \\end{bmatrix} \\\\[0.5cm]");
    latex("\\text{Column vector: } \\begin{bmatrix} x_1 \\\\ x_2 \\\\ x_3 \\\\ \\vdots \\\\ x_n \\end{bmatrix} \\\\[0.5cm]");

    latex("\\text{Numbered list:} \\\\
    \\begin{array}{ll}
    1. & \\text{First element: } a_1 = 5 \\\\
    2. & \\text{Second element: } a_2 = 10 \\\\  
    3. & \\text{Third element: } a_3 = 15
    \\end{array} \\\\[0.5cm]");

    latex("\\text{Simple data table:} \\\\
    \\begin{array}{|c|c|c|c|}
    \\hline
    \\text{Index} & 1 & 2 & 3 \\\\
    \\hline
    \\text{Value} & 5.2 & 8.7 & 12.1 \\\\
    \\hline
    \\end{array} \\\\[0.8cm]");

    latex("\\text{--- 3D ARRAYS AND COMPLEX STRUCTURES ---} \\\\[0.3cm]");

    latex("\\text{4x4 matrix (3D-like):} \\\\
    \\begin{bmatrix}
    1 & 0 & 0 & t_x \\\\
    0 & 1 & 0 & t_y \\\\
    0 & 0 & 1 & t_z \\\\
    0 & 0 & 0 & 1
    \\end{bmatrix} \\\\[0.5cm]");

    latex("\\text{3D tensor (slices representation):} \\\\
    T_{i,j,k} = \\begin{cases}
    \\text{Slice } k=1: & \\begin{bmatrix} 1 & 2 \\\\ 3 & 4 \\end{bmatrix} \\\\[0.3cm]
    \\text{Slice } k=2: & \\begin{bmatrix} 5 & 6 \\\\ 7 & 8 \\end{bmatrix} \\\\[0.3cm]  
    \\text{Slice } k=3: & \\begin{bmatrix} 9 & 10 \\\\ 11 & 12 \\end{bmatrix}
    \\end{cases} \\\\[0.5cm]");

    latex("\\text{Multidimensional data table:} \\\\
    \\begin{array}{|c|c|c|c|}
    \\hline
    \\text{Product} & \\text{Price (€)} & \\text{Stock} & \\text{Category} \\\\
    \\hline
    \\text{Computer} & 899.99 & 15 & \\text{Tech} \\\\
    \\hline
    \\text{Book} & 24.50 & 50 & \\text{Culture} \\\\
    \\hline
    \\text{Chair} & 129.00 & 8 & \\text{Furniture} \\\\
    \\hline
    \\end{array} \\\\[0.8cm]");

    latex("\\text{--- GRAPHS AND VISUAL REPRESENTATIONS ---} \\\\[0.3cm]");

    latex("\\text{Quadratic function (symbolic):} \\\\
    f(x) = ax^2 + bx + c \\\\
    \\text{Graph: } \\cup \\text{ (parabola)} \\\\[0.5cm]");

    latex("\\text{Simple Cartesian axes:} \\\\
    \\begin{array}{ccc}
     & \\uparrow & \\\\
     & y & \\\\
    \\leftarrow & + & \\rightarrow \\text{ x} \\\\
     & \\downarrow & 
    \\end{array} \\\\[0.5cm]");

    latex("\\text{3D coordinates:} \\\\
    P(x,y,z) = \\begin{pmatrix} 
    \\cos(\\theta)\\cos(\\phi) \\\\ 
    \\cos(\\theta)\\sin(\\phi) \\\\ 
    \\sin(\\theta) 
    \\end{pmatrix} \\\\[0.5cm]");

    latex("\\text{Stylized histogram:} \\\\
    \\begin{array}{c|c}
    \\text{Value} & \\text{Frequency} \\\\
    \\hline
    [0,10) & \\text{||||} \\text{ (4)} \\\\
    [10,20) & \\text{|||||||} \\text{ (7)} \\\\
    [20,30) & \\text{|||||} \\text{ (5)} \\\\
    [30,40) & \\text{||} \\text{ (2)}
    \\end{array} \\\\[0.5cm]");

    latex("\\text{Bar chart with array:} \\\\
    \\begin{array}{ccccc}
    \\text{A} & \\text{B} & \\text{C} & \\text{D} & \\text{E} \\\\
    \\rule{5pt}{20pt} & \\rule{5pt}{35pt} & \\rule{5pt}{15pt} & \\rule{5pt}{40pt} & \\rule{5pt}{25pt} \\\\
    20 & 35 & 15 & 40 & 25
    \\end{array} \\\\[0.8cm]");

    latex("\\text{--- ADVANCED MATHEMATICAL REPRESENTATIONS ---} \\\\[0.3cm]");

    latex("\\text{Fourier series:} \\\\
    f(x) = \\frac{a_0}{2} + \\sum_{n=1}^{\\infty} \\left( a_n \\cos\\left(\\frac{2\\pi nx}{T}\\right) + b_n \\sin\\left(\\frac{2\\pi nx}{T}\\right) \\right) \\\\[0.5cm]");

    latex("\\text{Laplace transform:} \\\\
    \\mathcal{L}\\{f(t)\\} = F(s) = \\int_0^{\\infty} f(t) e^{-st} dt \\\\[0.5cm]");

    latex("\\text{Algorithm in pseudo-code:} \\\\
    \\begin{array}{l}
    \\textbf{function} \\text{ } fibonacci(n) \\\\
    \\quad \\textbf{if} \\text{ } n \\leq 1 \\text{ } \\textbf{then} \\\\
    \\quad \\quad \\textbf{return} \\text{ } n \\\\
    \\quad \\textbf{else} \\\\
    \\quad \\quad \\textbf{return} \\text{ } fibonacci(n-1) + fibonacci(n-2) \\\\
    \\quad \\textbf{end if} \\\\
    \\textbf{end function}
    \\end{array} \\\\[0.8cm]");

    latex("\\text{--- Thank you for exploring LaTeX capabilities! ---} \\\\
    \\text{♥ Coded with love in Rust ♥}");
}
