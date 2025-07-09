//! # LaTeX Module - Mathematical and Scientific Rendering
//!
//! This module provides seamless integration of LaTeX mathematical notation
//! with webrust applications. It leverages MathJax for high-quality rendering
//! of mathematical expressions, equations, matrices, and scientific notation.
//!
//! ## Overview
//!
//! Scientific computing and mathematical applications often require the ability
//! to display complex mathematical expressions. Traditional console output is
//! limited to plain text, making it difficult to represent:
//!
//! - Mathematical equations and formulas
//! - Matrices and vectors
//! - Scientific notation and symbols
//! - Complex mathematical structures
//!
//! webrust solves this by integrating LaTeX rendering directly into the web interface,
//! allowing for publication-quality mathematical display.
//!
//! ## Features
//!
//! ### Mathematical Expressions
//!
//! latex("E = mc^2");                          // Einstein's equation
//! latex("\\frac{-b \\pm \\sqrt{b^2-4ac}}{2a}"); // Quadratic formula
//! latex("\\int_0^\\infty e^{-x^2} dx");       // Integral notation
//!
//!
//! ### Matrices and Vectors
//!
//! latex("\\begin{pmatrix} 1 & 0 \\\\ 0 & 1 \\end{pmatrix}");  // Identity matrix
//! latex("\\begin{bmatrix} a \\\\ b \\\\ c \\end{bmatrix}");     // Column vector
//!
//!
//! ### Complex Structures
//!
//! latex("\\begin{cases} x + y = 5 \\\\ 2x - y = 1 \\end{cases}"); // System of equations
//! latex("\\sum_{i=1}^n i = \\frac{n(n+1)}{2}");                  // Summation formula
//!
//!
//! ## Functions
//!
//! ### `latex<T: std::fmt::Display>(formula: T)`
//!
//! The primary LaTeX rendering function:
//! - Accepts any type implementing `Display` (String, &str, formatted strings)
//! - Automatically detects display vs inline mode based on content
//! - Integrates seamlessly with the web interface
//! - Renders using MathJax for high-quality output
//!
//! ### `latex_display<T: std::fmt::Display>(formula: T)`
//!
//! Forces display mode rendering:
//! - Centers the mathematical expression
//! - Uses larger fonts and spacing
//! - Ideal for standalone equations and formulas
//!
//! ### `latex_inline<T: std::fmt::Display>(formula: T)`
//!
//! Forces inline mode rendering:
//! - Integrates with surrounding text
//! - Uses smaller, text-height rendering
//! - Ideal for mathematical expressions within sentences
//!
//! ## MathJax Integration
//!
//! webrust uses MathJax 3.x for LaTeX rendering:
//!
//! ### Configuration
//! - **Input**: TeX/LaTeX notation
//! - **Output**: High-quality HTML/CSS rendering
//! - **Delimiters**: `$$...$$` for display, `$...$` for inline
//! - **Processing**: Automatic detection and rendering
//!
//! ### Performance
//! - **Client-side rendering**: No server-side processing required
//! - **Caching**: MathJax caches rendered expressions
//! - **Progressive enhancement**: Works without JavaScript (shows raw LaTeX)
//!
//! ## Supported LaTeX Features
//!
//! webrust supports the full range of MathJax/LaTeX features:
//!
//! ### Basic Mathematics
//! - Superscripts: `x^2`, `e^{i\pi}`
//! - Subscripts: `x_1`, `H_2O`
//! - Fractions: `\frac{a}{b}`, `\dfrac{x}{y}`
//! - Roots: `\sqrt{x}`, `\sqrt[n]{x}`
//!
//! ### Advanced Features
//! - Integrals: `\int`, `\iint`, `\iiint`
//! - Summations: `\sum`, `\prod`
//! - Limits: `\lim_{x \to \infty}`
//! - Matrices: `pmatrix`, `bmatrix`, `vmatrix`
//!
//! ### Symbols and Notation
//! - Greek letters: `\alpha`, `\beta`, `\gamma`
//! - Mathematical operators: `\times`, `\div`, `\pm`
//! - Set notation: `\in`, `\subset`, `\cup`, `\cap`
//! - Logic symbols: `\land`, `\lor`, `\neg`
//!
//! ## Usage Examples
//!
//! ### Basic Equations
//!
//! latex("F = ma");                    // Newton's second law
//! latex("\\Delta x = v_0 t + \\frac{1}{2}at^2"); // Kinematic equation
//!
//!
//! ### Complex Expressions
//!
//! latex("\\nabla \\times \\vec{E} = -\\frac{\\partial \\vec{B}}{\\partial t}"); // Maxwell's equation
//! latex("\\Psi(x,t) = Ae^{i(kx-\\omega t)}");  // Wave function
//!
//!
//! ### Data Visualization
//!
//! // Display a data table using LaTeX array environment
//! latex("\\begin{array}{|c|c|c|}
//!     \\hline
//!     \\text{Name} & \\text{Age} & \\text{Score} \\\\
//!     \\hline
//!     \\text{Alice} & 25 & 95.5 \\\\
//!     \\text{Bob} & 30 & 87.2 \\\\
//!     \\hline
//!     \\end{array}");
//!
//!
//! ## Integration with F-Strings
//!
//! LaTeX expressions can incorporate Rust variables:
//!
//!
//! let a = 2.0;
//! let b = 3.0;
//! let c = (a * a + b * b).sqrt();
//!
//! latex("\\text{Given } a = {a} \\text{ and } b = {b}");
//! latex("\\text{Then } c = \\sqrt{{{a}^2 + {b}^2}} = {c:.2}");
//!
//!
//! ## Best Practices
//!
//! ### Performance
//! - Group related expressions when possible
//! - Use `latex_inline` for simple expressions in text
//! - Use `latex_display` for standalone equations
//!
//! ### Readability
//! - Add spacing with `\\,` for better visual separation
//! - Use `\\text{}` for non-mathematical text within expressions
//! - Break complex expressions across multiple lines
//!
//! ### Accessibility
//! - Provide alternative text descriptions for complex expressions
//! - Use semantic LaTeX commands when available
//! - Consider providing both symbolic and numeric representations

use crate::gui::add_output;

pub fn latex<T: std::fmt::Display>(formula: T) {
    let latex_content = format!("{}", formula);
    // Envelopper dans des délimiteurs MathJax
    let wrapped = if latex_content.contains("\\begin{") || latex_content.contains("\\[") {
        // Déjà en mode display
        format!("LATEX_DISPLAY:{}", latex_content)
    } else {
        format!("LATEX_INLINE:{}", latex_content)
    };
    add_output(wrapped);
}

pub fn latex_display<T: std::fmt::Display>(formula: T) {
    let latex_content = format!("{}", formula);
    add_output(format!("LATEX_DISPLAY:{}", latex_content));
}

pub fn latex_inline<T: std::fmt::Display>(formula: T) {
    let latex_content = format!("{}", formula);
    add_output(format!("LATEX_INLINE:{}", latex_content));
}