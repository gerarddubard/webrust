// webrust/src/io/print.rs
//! # Rich Text Rendering & Positioning
//!
//! Système de rendu avancé supportant les styles inline, le positionnement absolu,
//! le formatage LaTeX, et les modes de coordonnées multiples.
//!
//! ## Syntaxe des Styles
//!
//! Format : `<attributs>contenu</>`
//!
//! ### Couleurs et Texte
//! - `<red>`, `<blue>`, `<#FF5733>` - Couleur du texte
//! - `<!yellow>` - Couleur de fond
//! - `<b>` - Gras
//! - `<i>` - Italique
//! - `<u>` - Souligné
//! - `<s>` - Barré
//! - `<14>`, `<20>` - Taille en pixels
//!
//! ### Bordures et Boîtes
//! - `<t2>` - Bordure 2px
//! - `<|red>` - Couleur de bordure
//! - `<solid>`, `<dashed>`, `<dotted>`, `<double>` - Style de bordure
//! - `<w200 h100>` - Largeur et hauteur
//! - `<r10>` - Coins arrondis (radius)
//! - `<p10>` - Padding
//! - `<m20>` - Margin
//!
//! ### Positionnement dans la Boîte
//! - `<mc>` - Centré (middle-center)
//! - `<tl>` - En haut à gauche
//! - `<tr>`, `<br>`, `<bl>` - Autres positions
//! - `<j>` - Texte justifié
//!
//! ### Marges Spécifiques
//! - `<mt10>`, `<mb20>` - Margin top/bottom
//! - `<ml5>`, `<mr15>` - Margin left/right
//! - `<pt8>`, `<pb12>` - Padding top/bottom
//!
//! ## Positionnement Absolu
//!
//! La méthode `.at(x, y)` place le texte en coordonnées absolues selon le mode actif :
//!
//! - **CSS** (défaut) : Origine en haut-gauche, +y vers le bas
//! - **Cartesian** : Origine au centre, +y vers le haut
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! coord("css");
//! print("<green>Top Left").at(0, 0);
//!
//! coord("cartesian");
//! print("<red>Center").at(0.0, 0.0);
//! # }
//! ```
//!
//! ## LaTeX Math
//!
//! Syntaxe : `$(expression LaTeX)`
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! println("Formule : $(x = \\frac{-b \\pm \\sqrt{b^2-4ac}}{2a})");
//! println("Inline: $(E=mc^2)$, display: $(\\int_0^\\infty e^{-x}dx)");
//! # }
//! ```
//!
//! ## Constantes d'Écran
//!
//! - `TW` - Largeur totale / 2
//! - `TH` - Hauteur totale / 2
//! - `CW` - Largeur de contenu (TW - 40)
//!
//! Utilisables dans les expressions :
//!
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! println("<black !blue w{*CW} h60 mc>Pleine largeur");
//! # }
//! ```
//!
//! ## Exemples
//!
//! ### Exemple 1 : Styles Combinés
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! println("<blue b i 18>Titre Important");
//! println("<red !yellow t3 |black r10 w300 h50 mc>Boîte Stylée");
//! # }
//! ```
//!
//! ### Exemple 2 : Positionnement
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! coord("css");
//! print("<white !green r10 w120 h24 mc>● Online")
//!     .at(-20, 0.0)
//!     .sticky();  // Reste visible au scroll
//! # }
//! ```
//!
//! ### Exemple 3 : Document Formaté
//! ```rust,no_run
//! # use webrust::prelude::*;
//! # #[gui] fn example() {
//! println("<navy b t4 double |navy !lightcyan r8 w{*CW} h48 mc>TITRE");
//! println(r#"<darkslateblue t1 |darkslateblue !ghostwhite r5 w{*CW} h120 j p10>
//!     Contenu avec texte justifié, bordure fine, et padding uniforme.
//! </>"#);
//! # }
//! ```
//!
//! ## Notes Techniques
//!
//! - **Parser optimisé** : Tokenisation sans regex
//! - **Échappement automatique** : HTML entities pour UTF-8
//! - **Marges intelligentes** : Neutralisation interne pour éviter les doubles marges
//! - **MathJax** : Détection automatique et rendu des expressions LaTeX

use crate::io::gui::{add_output_new_line, add_output_same_line};
use crate::layout::coord::{current, CoordMode};
use std::sync::RwLock;
use std::{process::Command, sync::LazyLock};

fn pshell(cmd: &str, d: u32) -> u32 {
    Command::new("powershell")
        .args(&["-Command", cmd])
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok())
        .unwrap_or(d)
}

pub static TW: LazyLock<u32> = LazyLock::new(|| {
    pshell("Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Width", 800) / 2
});
pub static TH: LazyLock<u32> = LazyLock::new(|| {
    pshell("Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Height", 600) / 2
});
pub static CW: LazyLock<u32> = LazyLock::new(|| *TW - 40);

#[derive(Debug, Clone)]
struct DefaultStyles {
    fg: String,
    font: String,
    size: String,
}
impl Default for DefaultStyles {
    fn default() -> Self {
        Self {
            fg: "black".into(),
            font: "Arial".into(),
            size: "14px".into(),
        }
    }
}
static DEFAULTS: RwLock<Option<DefaultStyles>> = RwLock::new(None);
pub fn set_defaults(fg: String, font: String, size: String) {
    if let Ok(mut d) = DEFAULTS.write() {
        *d = Some(DefaultStyles { fg, font, size });
    }
}
fn get_defaults() -> DefaultStyles {
    DEFAULTS
        .read()
        .ok()
        .and_then(|d| d.as_ref().cloned())
        .unwrap_or_default()
}

fn read_balanced(cs: &[char], i: &mut usize) -> String {
    let (mut d, mut s) = (1, String::new());
    while *i < cs.len() && d > 0 {
        let c = cs[*i];
        if c == '(' {
            d += 1;
        } else if c == ')' {
            d -= 1;
        }
        if d > 0 {
            s.push(c);
        }
        *i += 1;
    }
    s
}

fn latex_from_dollar_paren(t: &str) -> String {
    let mut o = String::new();
    let cs: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if i + 1 < cs.len() && cs[i] == '$' && cs[i + 1] == '(' {
            i += 2;
            let b = read_balanced(&cs, &mut i);
            let disp = b.contains("\\begin{") || b.contains("\\[") || b.len() > 50;
            if disp {
                o.push_str(r#"<div style="text-align:left">$$"#);
                o.push_str(&b);
                o.push_str("$$</div>");
            } else {
                o.push('$');
                o.push_str(&b);
                o.push('$');
            }
        } else {
            o.push(cs[i]);
            i += 1;
        }
    }
    o
}

fn tokenize_head(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut it = raw.chars().peekable();
    while let Some(ch) = it.next() {
        if ch.is_whitespace() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if ch == '!' || ch == '|' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.push(ch);
            while let Some(&nx) = it.peek() {
                if nx.is_whitespace() {
                    break;
                }
                cur.push(nx);
                it.next();
            }
            out.push(std::mem::take(&mut cur));
            continue;
        }
        cur.push(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[derive(Clone, Default)]
struct TextStyle {
    fg: Option<String>,
    bg: Option<String>,
    font: Option<String>,
    size_px: Option<u32>,
    b: bool,
    i: bool,
    u: bool,
    s: bool,
    justify: bool,
}

#[derive(Clone, Default)]
struct BoxStyle {
    t: u32,
    bstyle: String,
    bcolor: String,
    w: Option<u32>,
    h: Option<u32>,
    r: u32,
    pad: Option<u32>,
    mar: Option<u32>,
    ptop: Option<u32>,
    pbottom: Option<u32>,
    pleft: Option<u32>,
    pright: Option<u32>,
    mtop: Option<u32>,
    mbottom: Option<u32>,
    mleft: Option<u32>,
    mright: Option<u32>,
    rm_top: bool,
    rm_bottom: bool,
    rm_left: bool,
    rm_right: bool,
    pos2d: Option<&'static str>,
}

#[derive(Clone, Default)]
struct TagStyle {
    txt: TextStyle,
    bx: BoxStyle,
}

fn is_color_token(tok: &str) -> bool {
    if tok.is_empty() {
        return false;
    }
    let ok = tok
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit());
    ok && !matches!(
        tok,
        "solid"
            | "dashed"
            | "dotted"
            | "double"
            | "groove"
            | "ridge"
            | "inset"
            | "outset"
            | "center"
            | "left"
            | "right"
            | "justify"
    )
}
fn is_font_token(tok: &str) -> bool {
    tok.chars().next().map_or(false, |c| c.is_uppercase())
}
fn num_token(tok: &str) -> Option<u32> {
    tok.parse::<u32>().ok()
}
fn num_px(tok: &str) -> Option<u32> {
    tok.trim_end_matches("px").parse::<u32>().ok()
}
fn two_letter_pos(tok: &str) -> Option<&'static str> {
    match tok {
        "tl" => Some("tl"),
        "tc" => Some("tc"),
        "tr" => Some("tr"),
        "ml" => Some("ml"),
        "mc" => Some("mc"),
        "mr" => Some("mr"),
        "bl" => Some("bl"),
        "bc" => Some("bc"),
        "br" => Some("br"),
        _ => None,
    }
}
fn border_style(tok: &str) -> bool {
    matches!(
        tok,
        "solid" | "dashed" | "dotted" | "double" | "groove" | "ridge" | "inset" | "outset"
    )
}
fn starts_with_num(tok: &str, idx: usize) -> bool {
    tok.len() > idx
        && tok[idx..]
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
}

fn parse_one_tag_tokens(raw: &str, defaults: &DefaultStyles) -> TagStyle {
    let mut t = TagStyle::default();
    t.bx.bstyle = "solid".into();
    t.bx.bcolor = "transparent".into();
    for tok in tokenize_head(raw) {
        if tok == "b" {
            t.txt.b = true;
            continue;
        }
        if tok == "i" {
            t.txt.i = true;
            continue;
        }
        if tok == "u" {
            t.txt.u = true;
            continue;
        }
        if tok == "s" {
            t.txt.s = true;
            continue;
        }
        if tok == "j" {
            t.txt.justify = true;
            continue;
        }
        if let Some(p) = two_letter_pos(&tok) {
            t.bx.pos2d = Some(p);
            continue;
        }
        if let Some(n) = num_token(&tok) {
            t.txt.size_px = Some(n);
            continue;
        }
        if tok.starts_with('!') && tok.len() > 1 {
            t.txt.bg = Some(tok[1..].to_string());
            continue;
        }
        if tok.starts_with('|') && tok.len() > 1 {
            t.bx.bcolor = tok[1..].to_string();
            continue;
        }
        if tok.starts_with("mt") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.mtop = Some(v);
            }
            continue;
        }
        if tok.starts_with("mb") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.mbottom = Some(v);
            }
            continue;
        }
        if tok.starts_with("ml") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.mleft = Some(v);
            }
            continue;
        }
        if tok.starts_with("mr") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.mright = Some(v);
            }
            continue;
        }
        if tok.starts_with("pt") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.ptop = Some(v);
            }
            continue;
        }
        if tok.starts_with("pb") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.pbottom = Some(v);
            }
            continue;
        }
        if tok.starts_with("pl") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.pleft = Some(v);
            }
            continue;
        }
        if tok.starts_with("pr") && starts_with_num(&tok, 2) {
            if let Some(v) = num_px(&tok[2..]) {
                t.bx.pright = Some(v);
            }
            continue;
        }
        if tok.starts_with('m') && starts_with_num(&tok, 1) {
            if let Some(v) = num_px(&tok[1..]) {
                t.bx.mar = Some(v);
            }
            continue;
        }
        if tok.starts_with('p') && starts_with_num(&tok, 1) {
            if let Some(v) = num_px(&tok[1..]) {
                t.bx.pad = Some(v);
            }
            continue;
        }
        if tok.starts_with('t') && starts_with_num(&tok, 1) {
            if let Some(n) = num_px(&tok[1..]) {
                t.bx.t = n;
            }
            continue;
        }
        if tok.starts_with('w') && starts_with_num(&tok, 1) {
            if let Some(n) = num_px(&tok[1..]) {
                t.bx.w = Some(n);
            }
            continue;
        }
        if tok.starts_with('h') && starts_with_num(&tok, 1) {
            if let Some(n) = num_px(&tok[1..]) {
                t.bx.h = Some(n);
            }
            continue;
        }
        if tok.starts_with('r') && starts_with_num(&tok, 1) {
            if let Some(n) = num_px(&tok[1..]) {
                t.bx.r = n;
            }
            continue;
        }
        if border_style(&tok) {
            t.bx.bstyle = tok.to_string();
            continue;
        }
        if tok.len() == 1 {
            match tok.as_str() {
                "T" => {
                    t.bx.rm_top = true;
                }
                "B" => {
                    t.bx.rm_bottom = true;
                }
                "L" => {
                    t.bx.rm_left = true;
                }
                "R" => {
                    t.bx.rm_right = true;
                }
                "H" => {
                    t.bx.rm_top = true;
                    t.bx.rm_bottom = true;
                }
                "V" => {
                    t.bx.rm_left = true;
                    t.bx.rm_right = true;
                }
                _ => {}
            }
            continue;
        }
        if is_font_token(&tok) {
            t.txt.font = Some(tok.replace('_', " "));
            continue;
        }
        if is_color_token(&tok) {
            t.txt.fg = Some(tok.to_string());
            continue;
        }
    }
    if t.txt.fg.is_none() {
        t.txt.fg = Some(defaults.fg.clone());
    }
    if t.txt.font.is_none() {
        t.txt.font = Some(defaults.font.clone());
    }
    if t.txt.size_px.is_none() {
        t.txt.size_px = Some(defaults.size.trim_end_matches("px").parse().unwrap_or(14));
    }
    if t.bx.pos2d.is_none() {
        t.bx.pos2d = Some("mc");
    }
    t
}

fn split_outer_tag(cs: &[char], i0: usize) -> Option<(usize, String, usize, usize, bool)> {
    if cs[i0] != '<' {
        return None;
    }
    let mut i = i0 + 1;
    let mut head_end = None;
    while i < cs.len() {
        if cs[i] == '>' {
            head_end = Some(i);
            break;
        }
        i += 1;
    }
    let head_end = head_end?;
    let head: String = cs[(i0 + 1)..head_end].iter().collect();
    let mut j = head_end + 1;
    let mut close_at = None;
    while j + 2 <= cs.len() {
        if cs[j] == '<' && cs.get(j + 1) == Some(&'/') && cs.get(j + 2) == Some(&'>') {
            close_at = Some(j);
            break;
        }
        if cs[j] == '<' {
            close_at = Some(j);
            break;
        }
        j += 1;
    }
    let content_start = head_end + 1;
    let content_end = close_at.unwrap_or(cs.len());
    let has_explicit_close = (content_end + 2) < cs.len()
        && cs.get(content_end) == Some(&'<')
        && cs.get(content_end + 1) == Some(&'/')
        && cs.get(content_end + 2) == Some(&'>');
    Some((i0, head, content_start, content_end, has_explicit_close))
}

fn pos_to_flex(pos: &str) -> (&'static str, &'static str) {
    match pos {
        "tl" => ("flex-start", "flex-start"),
        "tc" => ("flex-start", "center"),
        "tr" => ("flex-start", "flex-end"),
        "ml" => ("center", "flex-start"),
        "mc" => ("center", "center"),
        "mr" => ("center", "flex-end"),
        "bl" => ("flex-end", "flex-start"),
        "bc" => ("flex-end", "center"),
        "br" => ("flex-end", "flex-end"),
        _ => ("center", "center"),
    }
}

fn render_tag(tag: &TagStyle, inner_html: &str) -> String {
    let mut outer = Vec::with_capacity(24);
    let mut inner = Vec::with_capacity(24);
    let (ai, ji) = tag
        .bx
        .pos2d
        .map(pos_to_flex)
        .unwrap_or(("center", "center"));
    let text_align = match tag.bx.pos2d {
        Some("tl") | Some("ml") | Some("bl") => "left",
        Some("tc") | Some("mc") | Some("bc") => "center",
        Some("tr") | Some("mr") | Some("br") => "right",
        None => "center",
        _ => "center",
    };
    outer.push("display:inline-flex".into());
    outer.push("vertical-align:top".into());
    outer.push(format!("align-items:{ai}"));
    outer.push(format!("justify-content:{ji}"));
    outer.push("box-sizing:border-box".into());
    outer.push("overflow:hidden".into());
    if let Some(bg) = &tag.txt.bg {
        outer.push(format!("background-color:{bg}"));
    }
    if let Some(w) = tag.bx.w {
        outer.push(format!("width:{}px", w));
    }
    if let Some(h) = tag.bx.h {
        outer.push(format!("height:{}px", h));
    }
    if tag.bx.r > 0 {
        outer.push(format!("border-radius:{}px", tag.bx.r));
    }
    if let Some(m) = tag.bx.mar {
        outer.push(format!("margin:{}px", m));
    }
    if let Some(v) = tag.bx.mtop {
        outer.push(format!("margin-top:{}px", v));
    }
    if let Some(v) = tag.bx.mbottom {
        outer.push(format!("margin-bottom:{}px", v));
    }
    if let Some(v) = tag.bx.mleft {
        outer.push(format!("margin-left:{}px", v));
    }
    if let Some(v) = tag.bx.mright {
        outer.push(format!("margin-right:{}px", v));
    }
    if let Some(p) = tag.bx.pad {
        outer.push(format!("padding:{}px", p));
    }
    if let Some(v) = tag.bx.ptop {
        outer.push(format!("padding-top:{}px", v));
    }
    if let Some(v) = tag.bx.pbottom {
        outer.push(format!("padding-bottom:{}px", v));
    }
    if let Some(v) = tag.bx.pleft {
        outer.push(format!("padding-left:{}px", v));
    }
    if let Some(v) = tag.bx.pright {
        outer.push(format!("padding-right:{}px", v));
    }
    if tag.bx.t > 0 {
        outer.push(format!(
            "border:{}px {} {}",
            tag.bx.t, tag.bx.bstyle, tag.bx.bcolor
        ));
    }
    if tag.bx.rm_top {
        outer.push("border-top:none".into());
    }
    if tag.bx.rm_bottom {
        outer.push("border-bottom:none".into());
    }
    if tag.bx.rm_left {
        outer.push("border-left:none".into());
    }
    if tag.bx.rm_right {
        outer.push("border-right:none".into());
    }
    inner.push("white-space:pre-wrap".into());
    inner.push("display:inline-block".into());
    inner.push("box-sizing:border-box".into());
    inner.push(format!(
        "font-family:{}",
        tag.txt.font.as_deref().unwrap_or("Arial")
    ));
    inner.push(format!("font-size:{}px", tag.txt.size_px.unwrap_or(14)));
    if tag.bx.w.is_some() {
        inner.push("width:100%".into());
    }
    if let Some(fg) = &tag.txt.fg {
        inner.push(format!("color:{fg}"));
    }
    if tag.txt.b {
        inner.push("font-weight:bold".into());
    }
    if tag.txt.i {
        inner.push("font-style:italic".into());
    }
    if tag.txt.u && !tag.txt.s {
        inner.push("text-decoration:underline".into());
    } else if !tag.txt.u && tag.txt.s {
        inner.push("text-decoration:line-through".into());
    } else if tag.txt.u && tag.txt.s {
        inner.push("text-decoration:underline line-through".into());
    }
    if tag.txt.justify {
        inner.push("text-align:justify".into());
    } else {
        inner.push(format!("text-align:{text_align}"));
    }
    format!(
        r#"<div class="webrust-box" style="{}"><span style="{}">{}</span></div>"#,
        outer.join(";"),
        inner.join(";"),
        inner_html
    )
}

fn parse_and_render_blocks(src: &str) -> String {
    let defaults = get_defaults();
    let mut out = String::new();
    let cs: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '<' {
            if let Some((_, head, content_start, content_end, has_close)) = split_outer_tag(&cs, i)
            {
                let content: String = cs[content_start..content_end].iter().collect();
                let rendered_inner = process_styles(&content);
                let tag = parse_one_tag_tokens(&head, &defaults);
                out.push_str(&render_tag(&tag, &rendered_inner));
                i = if has_close {
                    content_end + 3
                } else {
                    content_end
                };
                continue;
            }
        }
        out.push(cs[i]);
        i += 1;
    }
    out
}

pub fn html_escape_preserve_utf8(t: &str) -> String {
    t.chars()
        .map(|c| match c {
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            '&' => "&amp;".into(),
            '"' => "&quot;".into(),
            '\'' => "&#x27;".into(),
            _ => c.to_string(),
        })
        .collect()
}

pub fn process_webrust_styles_only(t: &str) -> String {
    parse_and_render_blocks(t)
}
pub fn process_styles(t: &str) -> String {
    parse_and_render_blocks(&latex_from_dollar_paren(t))
}

#[derive(Clone)]
pub struct PrintBox {
    lines: Vec<String>,
    inline: bool,
    block_align: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
    rx: Option<f64>,
    sticky: bool,
    emitted: bool,
}

impl PrintBox {
    fn new(lines: Vec<String>, inline: bool) -> Self {
        Self {
            lines,
            inline,
            block_align: None,
            x: None,
            y: None,
            rx: None,
            sticky: false,
            emitted: false,
        }
    }
    pub fn align<S: AsRef<str>>(mut self, v: S) -> Self {
        let a = v.as_ref().to_ascii_lowercase();
        self.block_align = match a.as_str() {
            "left" | "center" | "right" | "justify" => Some(a),
            _ => None,
        };
        self
    }
    pub fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }
    pub fn at<X: Into<f64>, Y: Into<f64>>(mut self, x: X, y: Y) -> Self {
        let x = x.into();
        let y = y.into();
        match current() {
            CoordMode::Css => {
                if x < 0.0 {
                    self.rx = Some(-x);
                    self.x = None;
                } else {
                    self.x = Some(x);
                    self.rx = None;
                }
            }
            _ => {
                self.x = Some(x);
                self.rx = None;
            }
        }
        self.y = Some(y);
        self
    }
}

fn has_zero_margin_bottom(seg: &str) -> bool {
    seg.contains("margin:0px") || seg.contains("margin-bottom:0px")
}

fn extract_px(seg: &str, key: &str) -> Option<u32> {
    if let Some(i) = seg.find(key) {
        let s = &seg[i + key.len()..];
        let mut n: u32 = 0;
        let mut any = false;
        for ch in s.chars() {
            if ch.is_ascii_digit() {
                any = true;
                n = n * 10 + (ch as u32 - '0' as u32);
            } else {
                break;
            }
        }
        if any {
            return Some(n);
        }
    }
    None
}

fn extract_margin_top_px(seg: &str) -> Option<u32> {
    extract_px(seg, "margin-top:")
}
fn extract_margin_bottom_px(seg: &str) -> Option<u32> {
    extract_px(seg, "margin-bottom:")
}

fn neutralize_inner_vertical_margins(seg: &str) -> String {
    fn zero_key(s: String, key: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut i = 0;
        while let Some(pos) = s[i..].find(key) {
            let abs = i + pos;
            out.push_str(&s[i..abs]);
            out.push_str(key);
            out.push_str("0px");
            let mut j = abs + key.len();
            while j < s.len() && s.as_bytes()[j].is_ascii_whitespace() {
                j += 1;
            }
            while j < s.len() && s.as_bytes()[j].is_ascii_digit() {
                j += 1;
            }
            if s.get(j..j + 2) == Some("px") {
                j += 2;
            }
            i = j;
        }
        out.push_str(&s[i..]);
        out
    }
    let s = zero_key(seg.to_string(), "margin-top:");
    let s = zero_key(s, "margin-bottom:");
    zero_key(s, "margin:")
}

fn emit_flow_line(seg: &str, align: &Option<String>) {
    let mt =
        extract_margin_top_px(seg)
            .unwrap_or_else(|| if has_zero_margin_bottom(seg) { 0 } else { 6 });
    let mb = extract_margin_bottom_px(seg).unwrap_or_else(|| {
        if has_zero_margin_bottom(seg) {
            0
        } else {
            6
        }
    });
    let inner = neutralize_inner_vertical_margins(seg);
    let mut style = String::new();
    style.push_str(&format!(
        "margin-top:{}px;margin-bottom:{}px;line-height:1.2;",
        mt, mb
    ));
    if let Some(a) = align {
        style.push_str(&format!("text-align:{};", a));
    }
    add_output_new_line(format!(
        r#"<div class="webrust-line" style="{}">{}</div>"#,
        style, inner
    ));
}

impl Drop for PrintBox {
    fn drop(&mut self) {
        if self.emitted {
            return;
        }
        let absolute = self.y.is_some() && (self.x.is_some() || self.rx.is_some());
        if absolute {
            let mut css = String::from("position:");
            css.push_str(if self.sticky { "fixed" } else { "absolute" });
            css.push_str(";z-index:1000;");
            match (self.x, self.rx, current()) {
                (Some(x), _, CoordMode::Cartesian) => {
                    let lx = (*TW as f64) / 2.0 + x;
                    let ty = (*TH as f64) / 2.0 - self.y.unwrap();
                    css.push_str(&format!(
                        "left:{}px;top:{}px;",
                        lx.round() as i32,
                        ty.round() as i32
                    ));
                }
                (Some(x), _, _) => {
                    css.push_str(&format!(
                        "left:{}px;top:{}px;",
                        x.round() as i32,
                        self.y.unwrap().round() as i32
                    ));
                }
                (None, Some(r), CoordMode::Css) => {
                    css.push_str(&format!(
                        "right:{}px;top:{}px;",
                        r.round() as i32,
                        self.y.unwrap().round() as i32
                    ));
                }
                (None, Some(r), _) => {
                    let lx = (*TW as f64) / 2.0 - r;
                    let ty = (*TH as f64) / 2.0 - self.y.unwrap();
                    css.push_str(&format!(
                        "left:{}px;top:{}px;",
                        lx.round() as i32,
                        ty.round() as i32
                    ));
                }
                _ => {}
            }
            for seg in &self.lines {
                add_output_new_line(format!(
                    r#"<div class="webrust-abs" style="{css}">{}</div>"#,
                    seg
                ));
            }
        } else if self.inline {
            for seg in &self.lines {
                add_output_same_line(seg.to_string());
            }
        } else {
            for seg in &self.lines {
                emit_flow_line(seg, &self.block_align);
            }
        }
        self.emitted = true;
    }
}

fn make_box<T: std::fmt::Display>(t: T, inline: bool) -> PrintBox {
    let lines: Vec<String> = format!("{}", t).split('\n').map(process_styles).collect();
    PrintBox::new(lines, inline)
}

pub fn print_str<T: std::fmt::Display>(t: T) -> PrintBox {
    make_box(t, true)
}
pub fn println_str<T: std::fmt::Display>(t: T) -> PrintBox {
    make_box(t, false)
}
pub use print_str as print;
pub use println_str as println;
