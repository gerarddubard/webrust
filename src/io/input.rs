// webrust/src/io/input.rs
//! # Module `io::input` — Entrées utilisateur WebRust
//!
//! Fournit des fonctions de saisie robustes (conversion, validation, annulation).
//! **Rappel :** Les f-strings WebRust utilisent `{var}` (et non `{:?}`).
//!
//! ## Exemple simple (f-string)
//! ```rust
//! use webrust::prelude::*;
//!
//! let age = 42;
//! println("<blue>Age: {age}</blue>");
//! ```
//!
//! ## Exemple : `try_input<T>` avec gestion d’erreur
//!
//! > On importe explicitement la fonction pour les doctests.
//!
//! ```rust
//! use webrust::prelude::*;
//! use webrust::input::try_input;
//!
//! fn demo() {
//!     match try_input::<f64>("Enter a float:") {
//!         Ok(v)  => println("<green>Success: {v}"),
//!         Err(e) => println("<red>Error: {e}"),
//!     };
//! }
//!
//! // Appeler explicitement dans un vrai binaire si souhaité :
//! // demo();
//! ```
//!
//! ## Mélanger format Rust classique (optionnel)
//! ```rust
//! // Si vous voulez le format Rust, utilisez `format!` puis `println!` classique :
//! let age = 30;
//! println!("{}", format!("<gray>Age (debug): {:?}</gray>", age));
//! ```

use crate::io::gui::create_input_request_typed;
use std::str::FromStr;

pub fn input_with_validation<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let user_input = create_input_request_typed::<T>(prompt);
        match user_input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                continue;
            }
        }
    }
}

pub fn input_string(prompt: &str) -> String {
    create_input_request_typed::<String>(prompt)
}

pub fn try_input<T>(prompt: &str) -> Result<T, T::Err>
where
    T: FromStr,
{
    let user_input = create_input_request_typed::<T>(prompt);
    user_input.trim().parse()
}

pub use input_with_validation as input;
