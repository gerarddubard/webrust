// webrust/webrust-macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, visit_mut::{self, VisitMut}, Expr, ExprCall, ExprPath, Lit, ExprLit};
use regex::Regex;

struct FStringTransformer;

impl VisitMut for FStringTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Call(ExprCall { func, args, .. }) = expr {
            if let Expr::Path(ExprPath { path, .. }) = func.as_ref() {
                if path.segments.len() == 1 &&
                    (path.segments[0].ident == "println" || path.segments[0].ident == "print") {
                    if let Some(Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. })) = args.first() {
                        let template = lit_str.value();
                        let transformations = extract_and_process_variables(&template);
                        if !transformations.is_empty() {
                            let func_name = &path.segments[0].ident;
                            let mut replacement_code = quote! {
                                let mut result = String::from(#template);
                            };
                            for (original, expr_str, format_spec) in transformations {
                                // Essayer de parser l'expression, si ça échoue, utiliser une chaîne
                                let expr_tokens = match expr_str.parse::<proc_macro2::TokenStream>() {
                                    Ok(tokens) => tokens,
                                    Err(_) => {
                                        // Si le parsing échoue, traiter comme une chaîne littérale
                                        quote! { #expr_str }
                                    }
                                };

                                let format_expr = match format_spec.as_deref() {
                                    Some(":c") => quote! { format!("{:?}", #expr_tokens) },
                                    Some(":j") => quote! { 
                                        {
                                            let debug_str = format!("{:#?}", #expr_tokens);
                                            webrust_format_json_clean(&debug_str)
                                        }
                                    },
                                    Some(":.2") => quote! { format!("{:.2}", #expr_tokens) },
                                    Some(":.6") => quote! { format!("{:.6}", #expr_tokens) },
                                    Some(":e") => quote! { format!("{:e}", #expr_tokens) },
                                    Some(":.0") => quote! { format!("{:.0}", #expr_tokens) },
                                    Some(":04") => quote! { format!("{:04}", #expr_tokens) },
                                    Some(":x") => quote! { format!("{:x}", #expr_tokens) },
                                    Some(":X") => quote! { format!("{:X}", #expr_tokens) },
                                    Some(":b") => quote! { format!("{:b}", #expr_tokens) },
                                    Some(":o") => quote! { format!("{:o}", #expr_tokens) },
                                    None => quote! { format!("{}", #expr_tokens) },
                                    _ => quote! { format!("{}", #expr_tokens) },
                                };
                                replacement_code = quote! {
                                    #replacement_code
                                    result = result.replace(#original, &#format_expr);
                                };
                            }
                            let new_expr = quote! {
                                #func_name({
                                    #replacement_code
                                    result
                                })
                            };
                            *expr = syn::parse2(new_expr).unwrap();
                            return;
                        }
                    }
                }
            }
        }
        visit_mut::visit_expr_mut(self, expr);
    }
}

fn extract_and_process_variables(template: &str) -> Vec<(String, String, Option<String>)> {
    // Regex pour capturer des expressions complexes, pas seulement des identifiants simples
    let re = Regex::new(r"\{([^:}]+)(:[^}]*)?}").unwrap();
    let mut transformations = Vec::new();
    for cap in re.captures_iter(template) {
        if let Some(full_match) = cap.get(0) {
            if let Some(expr_match) = cap.get(1) {
                let original = full_match.as_str().to_string();
                let expr_str = expr_match.as_str().trim().to_string();
                let format_spec = cap.get(2).map(|m| m.as_str().to_string());
                // Éviter les doublons
                if !transformations.iter().any(|(orig, _, _)| orig == &original) {
                    transformations.push((original, expr_str, format_spec));
                }
            }
        }
    }
    transformations
}

#[proc_macro_attribute]
pub fn gui(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);
    FStringTransformer.visit_item_fn_mut(&mut input_fn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let expanded = quote! {
        #[allow(unused_variables)]
        fn #fn_name() {
            fn webrust_format_json_clean(debug_str: &str) -> String {
                // Version simplifiée sans regex pour éviter les dépendances
                let lines: Vec<&str> = debug_str.lines().collect();
                let mut result = Vec::new();
                let mut i = 0;
                while i < lines.len() {
                    let line = lines[i];
                    let trimmed = line.trim();
                    
                    // Détection simple des valeurs numériques sans regex
                    let is_simple_numbers = trimmed.chars().all(|c| c.is_ascii_digit() || c == ',' || c.is_whitespace()) 
                        && !trimmed.is_empty();
                    
                    if is_simple_numbers && i + 1 < lines.len() {
                        let next_line = lines[i + 1].trim();
                        let next_is_simple = next_line.chars().all(|c| c.is_ascii_digit() || c == ',' || c.is_whitespace()) 
                            && !next_line.is_empty();
                        
                        if next_is_simple {
                            let indent = " ".repeat(line.len() - trimmed.len());
                            let combined = format!("{}[{} {}]", indent, 
                                trimmed.replace(",", "").trim(), 
                                next_line.replace(",", "").trim());
                            result.push(combined);
                            i += 2;
                            continue;
                        }
                    }
                    let cleaned_line = line.replace(",", "");
                    result.push(cleaned_line);
                    i += 1;
                }
                result.join("\n")
            }
            fn println<T: std::fmt::Display>(text: T) { webrust::print::println_str(text); }
            fn print<T: std::fmt::Display>(text: T) { webrust::print::print_str(text); }
            use webrust::input::input_with_validation as input;
            use webrust::latex::{latex, latex_display, latex_inline};
            webrust::gui::start_gui_server(|| { #fn_block });
        }
    };
    TokenStream::from(expanded)
}