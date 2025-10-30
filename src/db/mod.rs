// webrust/src/db/mod.rs
//! # Module `db` — Intégration SQL (DuckDB) pour WebRust
//!
//! Moteur analytique en mémoire (DuckDB) avec rendu HTML progressif dans le navigateur.
//! Conçu pour l’exploration de données, les tableaux interactifs et les rapports.
//!
//! ## Points clés
//! - Analyse en mémoire (OLAP) avec DuckDB
//! - Import/Export CSV/JSON/Parquet
//! - Rendu HTML tabulaire en streaming
//! - Types formatés proprement (dates, décimaux, booléens, etc.)
//!
//! ## Exemple minimal d’affichage (MathJax via WebRust)
//! ```rust
//! use webrust::prelude::*;
//!
//! fn demo_math() {
//!     // Pas besoin de fonction `latex(...)` dédiée : WebRust supporte `$...$`
//!     println("LaTeX inline : $a^2 + b^2 = c^2$.");
//!     println("Bloc : $$(\\sum_{i=1}^{n} x_i)/n$$");
//! }
//!
//! // À appeler dans un vrai binaire si souhaité :
//! // demo_math();
//! ```
//!
//! ## Note doctest
//! Pour garder des tests stables sur toutes plateformes, les exemples ci-dessus
//! se limitent à l’affichage. Les exemples de requêtes SQL complètes se trouvent
//! dans `db::sql` (doctests dédiés).

pub mod sql;

pub use sql::*;
