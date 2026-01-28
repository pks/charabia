//! Python bindings for the charabia tokenization library.
//!
//! This crate provides Python bindings for charabia, a library that detects
//! language, tokenizes text, and normalizes tokens. It supports 60+ languages
//! and provides language-specific tokenization for Chinese, Japanese, Korean,
//! Thai, and more.

use pyo3::prelude::*;

mod enums;
mod token;
mod tokenizer;

use enums::{PyLanguage, PyScript, PySeparatorKind, PyTokenKind};
use token::PyToken;
use tokenizer::{tokenize, PyTokenizer, PyTokenizerBuilder};

/// Python bindings for the charabia tokenization library.
///
/// Charabia is a library to detect language, tokenize text, and normalize tokens.
/// It supports 60+ languages and provides specialized tokenization for Chinese,
/// Japanese, Korean, Thai, and other languages.
///
/// Quick Start:
///     import charabia
///
///     # Simple tokenization
///     tokens = charabia.tokenize("Hello world")
///     for token in tokens:
///         print(f"{token.lemma} ({token.kind})")
///
///     # Custom configuration
///     builder = charabia.TokenizerBuilder()
///     builder.stop_words(["the", "a", "an"])
///     tokenizer = builder.build()
///     tokens = tokenizer.tokenize("The quick brown fox")
#[pymodule]
fn _charabia(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register enum classes
    m.add_class::<PyTokenKind>()?;
    m.add_class::<PySeparatorKind>()?;
    m.add_class::<PyLanguage>()?;
    m.add_class::<PyScript>()?;

    // Register main classes
    m.add_class::<PyToken>()?;
    m.add_class::<PyTokenizer>()?;
    m.add_class::<PyTokenizerBuilder>()?;

    // Register convenience function
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;

    Ok(())
}
