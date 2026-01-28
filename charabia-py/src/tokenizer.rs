use fst::Set;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use charabia::{Tokenize, TokenizerBuilder};

use crate::enums::PyLanguage;
use crate::token::PyToken;

/// Builder for creating a customized Tokenizer.
///
/// Example:
///     builder = TokenizerBuilder()
///     builder.stop_words(["the", "a", "an"])
///     builder.lossy_normalization(True)
///     tokenizer = builder.build()
#[pyclass(name = "TokenizerBuilder")]
#[derive(Clone, Default)]
pub struct PyTokenizerBuilder {
    stop_words: Option<Vec<String>>,
    separators: Option<Vec<String>>,
    words_dict: Option<Vec<String>>,
    create_char_map: bool,
    lossy: bool,
    allow_list: Option<Vec<PyLanguage>>,
}

#[pymethods]
impl PyTokenizerBuilder {
    /// Create a new TokenizerBuilder with default settings.
    #[new]
    fn new() -> Self {
        PyTokenizerBuilder {
            stop_words: None,
            separators: None,
            words_dict: None,
            create_char_map: false,
            lossy: true,
            allow_list: None,
        }
    }

    /// Set stop words that will be classified as TokenKind.StopWord.
    ///
    /// Args:
    ///     words: List of words to treat as stop words.
    ///
    /// Returns:
    ///     self for method chaining.
    fn stop_words(mut slf: PyRefMut<'_, Self>, words: Vec<String>) -> PyRefMut<'_, Self> {
        let mut words = words;
        words.sort();
        words.dedup();
        slf.stop_words = Some(words);
        slf
    }

    /// Set custom separators.
    ///
    /// Args:
    ///     separators: List of separator strings.
    ///
    /// Returns:
    ///     self for method chaining.
    fn separators(mut slf: PyRefMut<'_, Self>, separators: Vec<String>) -> PyRefMut<'_, Self> {
        slf.separators = Some(separators);
        slf
    }

    /// Set a words dictionary for custom segmentation.
    ///
    /// Words in this dictionary will be recognized as single tokens
    /// even if they would normally be split (e.g., abbreviations).
    ///
    /// Args:
    ///     words: List of words for the dictionary.
    ///
    /// Returns:
    ///     self for method chaining.
    fn words_dict(mut slf: PyRefMut<'_, Self>, words: Vec<String>) -> PyRefMut<'_, Self> {
        slf.words_dict = Some(words);
        slf
    }

    /// Enable or disable char_map creation.
    ///
    /// When enabled, tokens will include a char_map that maps
    /// byte positions between original and normalized text.
    ///
    /// Args:
    ///     enable: Whether to create char maps.
    ///
    /// Returns:
    ///     self for method chaining.
    fn create_char_map(mut slf: PyRefMut<'_, Self>, enable: bool) -> PyRefMut<'_, Self> {
        slf.create_char_map = enable;
        slf
    }

    /// Enable or disable lossy normalization.
    ///
    /// When enabled (default), normalization includes lowercasing,
    /// diacritic removal, and other transformations that may lose
    /// information from the original text.
    ///
    /// Args:
    ///     enable: Whether to use lossy normalization.
    ///
    /// Returns:
    ///     self for method chaining.
    fn lossy_normalization(mut slf: PyRefMut<'_, Self>, enable: bool) -> PyRefMut<'_, Self> {
        slf.lossy = enable;
        slf
    }

    /// Set allowed languages for detection.
    ///
    /// When set, language detection will only consider these languages.
    ///
    /// Args:
    ///     languages: List of Language values to allow.
    ///
    /// Returns:
    ///     self for method chaining.
    fn allow_list(mut slf: PyRefMut<'_, Self>, languages: Vec<PyLanguage>) -> PyRefMut<'_, Self> {
        slf.allow_list = Some(languages);
        slf
    }

    /// Build the Tokenizer with the configured settings.
    ///
    /// Returns:
    ///     A new Tokenizer instance.
    fn build(&self) -> PyResult<PyTokenizer> {
        // Validate stop words can form a valid FST Set
        if let Some(ref words) = self.stop_words {
            Set::from_iter(words.iter()).map_err(|e| {
                PyValueError::new_err(format!("Failed to create stop words set: {}", e))
            })?;
        }

        Ok(PyTokenizer {
            stop_words: self.stop_words.clone(),
            separators: self.separators.clone(),
            words_dict: self.words_dict.clone(),
            create_char_map: self.create_char_map,
            lossy: self.lossy,
            allow_list: self.allow_list.clone(),
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "TokenizerBuilder(stop_words={}, separators={}, create_char_map={}, lossy={})",
            self.stop_words.as_ref().map_or(0, |v| v.len()),
            self.separators.as_ref().map_or(0, |v| v.len()),
            self.create_char_map,
            self.lossy
        )
    }
}

/// A tokenizer for processing text into tokens.
///
/// Use TokenizerBuilder to create a customized tokenizer, or use the
/// default Tokenizer() for standard settings.
///
/// Example:
///     tokenizer = Tokenizer()
///     tokens = tokenizer.tokenize("Hello world")
///     for token in tokens:
///         print(token.lemma)
#[pyclass(name = "Tokenizer")]
#[derive(Clone, Default)]
pub struct PyTokenizer {
    stop_words: Option<Vec<String>>,
    separators: Option<Vec<String>>,
    words_dict: Option<Vec<String>>,
    create_char_map: bool,
    lossy: bool,
    allow_list: Option<Vec<PyLanguage>>,
}

#[pymethods]
impl PyTokenizer {
    /// Create a new Tokenizer with default settings.
    #[new]
    fn new() -> Self {
        PyTokenizer::default()
    }

    /// Tokenize the given text and return a list of tokens.
    ///
    /// Args:
    ///     text: The text to tokenize.
    ///
    /// Returns:
    ///     A list of Token objects.
    fn tokenize(&self, text: &str) -> PyResult<Vec<PyToken>> {
        // Build the tokenizer with stored configuration
        let stop_words_set: Option<Set<Vec<u8>>> = self.stop_words.as_ref().map(|words| {
            Set::from_iter(words.iter()).expect("Stop words already validated in build()")
        });

        let separators_owned: Option<Vec<&str>> = self
            .separators
            .as_ref()
            .map(|s| s.iter().map(|x| x.as_str()).collect());

        let words_dict_owned: Option<Vec<&str>> = self
            .words_dict
            .as_ref()
            .map(|w| w.iter().map(|x| x.as_str()).collect());

        let allow_list_owned: Option<Vec<charabia::Language>> = self
            .allow_list
            .as_ref()
            .map(|langs| langs.iter().map(|&l| l.into()).collect());

        let mut builder = TokenizerBuilder::new();

        if let Some(ref sw) = stop_words_set {
            builder.stop_words(sw);
        }
        if let Some(ref seps) = separators_owned {
            builder.separators(seps.as_slice());
        }
        if let Some(ref words) = words_dict_owned {
            builder.words_dict(words.as_slice());
        }
        if let Some(ref langs) = allow_list_owned {
            builder.allow_list(langs.as_slice());
        }

        builder.create_char_map(self.create_char_map);
        builder.lossy_normalization(self.lossy);

        let tokenizer = builder.build();

        let tokens: Vec<PyToken> = tokenizer.tokenize(text).map(PyToken::from).collect();

        Ok(tokens)
    }

    /// Tokenize text and return tuples of (original_text, token).
    ///
    /// This method returns both the original text span and the
    /// normalized token for each token, useful for highlighting
    /// or reconstruction.
    ///
    /// Args:
    ///     text: The text to tokenize.
    ///
    /// Returns:
    ///     A list of (original_text, token) tuples.
    fn reconstruct(&self, text: &str) -> PyResult<Vec<(String, PyToken)>> {
        let stop_words_set: Option<Set<Vec<u8>>> = self.stop_words.as_ref().map(|words| {
            Set::from_iter(words.iter()).expect("Stop words already validated in build()")
        });

        let separators_owned: Option<Vec<&str>> = self
            .separators
            .as_ref()
            .map(|s| s.iter().map(|x| x.as_str()).collect());

        let words_dict_owned: Option<Vec<&str>> = self
            .words_dict
            .as_ref()
            .map(|w| w.iter().map(|x| x.as_str()).collect());

        let allow_list_owned: Option<Vec<charabia::Language>> = self
            .allow_list
            .as_ref()
            .map(|langs| langs.iter().map(|&l| l.into()).collect());

        let mut builder = TokenizerBuilder::new();

        if let Some(ref sw) = stop_words_set {
            builder.stop_words(sw);
        }
        if let Some(ref seps) = separators_owned {
            builder.separators(seps.as_slice());
        }
        if let Some(ref words) = words_dict_owned {
            builder.words_dict(words.as_slice());
        }
        if let Some(ref langs) = allow_list_owned {
            builder.allow_list(langs.as_slice());
        }

        builder.create_char_map(self.create_char_map);
        builder.lossy_normalization(self.lossy);

        let tokenizer = builder.build();

        let result: Vec<(String, PyToken)> = tokenizer
            .reconstruct(text)
            .map(|(orig, token)| (orig.to_string(), PyToken::from(token)))
            .collect();

        Ok(result)
    }

    fn __repr__(&self) -> String {
        format!(
            "Tokenizer(stop_words={}, separators={}, create_char_map={}, lossy={})",
            self.stop_words.as_ref().map_or(0, |v| v.len()),
            self.separators.as_ref().map_or(0, |v| v.len()),
            self.create_char_map,
            self.lossy
        )
    }
}

/// Tokenize text with default settings.
///
/// This is a convenience function for quick tokenization without
/// needing to create a Tokenizer instance.
///
/// Args:
///     text: The text to tokenize.
///
/// Returns:
///     A list of Token objects.
///
/// Example:
///     tokens = charabia.tokenize("Hello world")
///     for token in tokens:
///         print(token.lemma)
#[pyfunction]
pub fn tokenize(text: &str) -> Vec<PyToken> {
    text.tokenize().map(PyToken::from).collect()
}
