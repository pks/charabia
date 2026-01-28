use pyo3::prelude::*;

use crate::enums::{PyLanguage, PyScript, PySeparatorKind, PyTokenKind};

/// A token produced by the tokenizer.
///
/// Contains the normalized lemma, position information, and metadata
/// about the token's classification, script, and language.
#[pyclass(name = "Token")]
#[derive(Clone, Debug)]
pub struct PyToken {
    /// The classification of this token (Word, StopWord, Separator, Unknown)
    #[pyo3(get)]
    pub kind: PyTokenKind,

    /// The normalized form of the token
    #[pyo3(get)]
    pub lemma: String,

    /// Starting character index in the original text (0-based)
    #[pyo3(get)]
    pub char_start: usize,

    /// Ending character index in the original text (exclusive)
    #[pyo3(get)]
    pub char_end: usize,

    /// Starting byte index in the original text
    #[pyo3(get)]
    pub byte_start: usize,

    /// Ending byte index in the original text
    #[pyo3(get)]
    pub byte_end: usize,

    /// Character mapping from original to normalized text.
    /// Each tuple (a, b) maps a bytes in original to b bytes in normalized.
    #[pyo3(get)]
    pub char_map: Option<Vec<(u8, u8)>>,

    /// The detected script of this token
    #[pyo3(get)]
    pub script: PyScript,

    /// The detected language of this token (if available)
    #[pyo3(get)]
    pub language: Option<PyLanguage>,
}

impl<'o> From<charabia::Token<'o>> for PyToken {
    fn from(token: charabia::Token<'o>) -> Self {
        PyToken {
            kind: token.kind.into(),
            lemma: token.lemma.into_owned(),
            char_start: token.char_start,
            char_end: token.char_end,
            byte_start: token.byte_start,
            byte_end: token.byte_end,
            char_map: token.char_map,
            script: token.script.into(),
            language: token.language.map(Into::into),
        }
    }
}

#[pymethods]
impl PyToken {
    /// Returns the length in bytes of the normalized lemma.
    fn byte_len(&self) -> usize {
        self.lemma.len()
    }

    /// Returns the length in bytes of the original text span.
    fn original_byte_len(&self) -> usize {
        self.byte_end - self.byte_start
    }

    /// Returns the count of characters in the normalized lemma.
    fn char_count(&self) -> usize {
        self.lemma.chars().count()
    }

    /// Returns the count of characters in the original text span.
    fn original_char_count(&self) -> usize {
        self.char_end - self.char_start
    }

    /// Returns True if the token is a word.
    fn is_word(&self) -> bool {
        matches!(self.kind, PyTokenKind::Word)
    }

    /// Returns True if the token is a stop word.
    fn is_stopword(&self) -> bool {
        matches!(self.kind, PyTokenKind::StopWord)
    }

    /// Returns True if the token is a separator.
    fn is_separator(&self) -> bool {
        matches!(
            self.kind,
            PyTokenKind::SoftSeparator | PyTokenKind::HardSeparator
        )
    }

    /// Returns the separator kind if this is a separator token, None otherwise.
    fn separator_kind(&self) -> Option<PySeparatorKind> {
        match self.kind {
            PyTokenKind::SoftSeparator => Some(PySeparatorKind::Soft),
            PyTokenKind::HardSeparator => Some(PySeparatorKind::Hard),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Token(lemma={:?}, kind={:?}, char_start={}, char_end={}, script={:?}, language={:?})",
            self.lemma, self.kind, self.char_start, self.char_end, self.script, self.language
        )
    }

    fn __str__(&self) -> &str {
        &self.lemma
    }
}
