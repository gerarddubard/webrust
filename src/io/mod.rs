// webrust/src/io/mod.rs
//! # I/O & GUI System
//!
//! Système unifié d'entrées/sorties avec serveur web intégré, formatage riche et positionnement.
//!
//! ## Modules
//!
//! - [`gui`] - Serveur web, gestion d'état, configuration des thèmes
//! - [`print`] - Affichage avec styles inline, couleurs, positionnement, LaTeX
//! - [`input`] - Saisie utilisateur avec validation de type en temps réel
//!
//! ## Utilisation
//!
//! L'attribut `#[gui]` démarre automatiquement le serveur et ouvre le navigateur.
//! Le mode de coordonnées défini avec `coord()` s'applique globalement.
pub mod gui;
pub mod input;
pub mod print;

pub use crate::io::gui::*;
pub use input::input;
pub use print::{print, println, process_webrust_styles_only, PrintBox, TH, TW};
