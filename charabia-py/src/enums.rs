use pyo3::prelude::*;

/// Token classification kind.
///
/// This is a flattened version of charabia's TokenKind enum, where
/// Separator(SeparatorKind) is expanded into SoftSeparator and HardSeparator.
#[pyclass(name = "TokenKind", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyTokenKind {
    /// Regular word token
    Word,
    /// Common word that can be ignored (e.g., "the", "a")
    StopWord,
    /// Soft separator - separates tokens in the same context (e.g., spaces)
    SoftSeparator,
    /// Hard separator - separates tokens in different contexts (e.g., periods)
    HardSeparator,
    /// Unknown/unclassified token
    Unknown,
}

impl From<charabia::TokenKind> for PyTokenKind {
    fn from(kind: charabia::TokenKind) -> Self {
        match kind {
            charabia::TokenKind::Word => PyTokenKind::Word,
            charabia::TokenKind::StopWord => PyTokenKind::StopWord,
            charabia::TokenKind::Separator(charabia::SeparatorKind::Soft) => {
                PyTokenKind::SoftSeparator
            }
            charabia::TokenKind::Separator(charabia::SeparatorKind::Hard) => {
                PyTokenKind::HardSeparator
            }
            charabia::TokenKind::Unknown => PyTokenKind::Unknown,
        }
    }
}

#[pymethods]
impl PyTokenKind {
    fn __repr__(&self) -> &'static str {
        match self {
            PyTokenKind::Word => "TokenKind.Word",
            PyTokenKind::StopWord => "TokenKind.StopWord",
            PyTokenKind::SoftSeparator => "TokenKind.SoftSeparator",
            PyTokenKind::HardSeparator => "TokenKind.HardSeparator",
            PyTokenKind::Unknown => "TokenKind.Unknown",
        }
    }
}

/// Separator context kind.
#[pyclass(name = "SeparatorKind", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PySeparatorKind {
    /// Separates tokens in different contexts (different phrases/sentences)
    Hard,
    /// Separates tokens in the same context (same phrase)
    Soft,
}

impl From<charabia::SeparatorKind> for PySeparatorKind {
    fn from(kind: charabia::SeparatorKind) -> Self {
        match kind {
            charabia::SeparatorKind::Hard => PySeparatorKind::Hard,
            charabia::SeparatorKind::Soft => PySeparatorKind::Soft,
        }
    }
}

#[pymethods]
impl PySeparatorKind {
    fn __repr__(&self) -> &'static str {
        match self {
            PySeparatorKind::Hard => "SeparatorKind.Hard",
            PySeparatorKind::Soft => "SeparatorKind.Soft",
        }
    }
}

/// Supported languages for detection and processing.
#[pyclass(name = "Language", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyLanguage {
    Zho,
    Epo,
    Eng,
    Rus,
    Cmn,
    Spa,
    Por,
    Ita,
    Ben,
    Fra,
    Deu,
    Ukr,
    Kat,
    Ara,
    Hin,
    Jpn,
    Heb,
    Yid,
    Pol,
    Amh,
    Jav,
    Kor,
    Nob,
    Dan,
    Swe,
    Fin,
    Tur,
    Nld,
    Hun,
    Ces,
    Ell,
    Bul,
    Bel,
    Mar,
    Kan,
    Ron,
    Slv,
    Hrv,
    Srp,
    Mkd,
    Lit,
    Lav,
    Est,
    Tam,
    Vie,
    Urd,
    Tha,
    Guj,
    Uzb,
    Pan,
    Aze,
    Ind,
    Tel,
    Pes,
    Mal,
    Ori,
    Mya,
    Nep,
    Sin,
    Khm,
    Tuk,
    Aka,
    Zul,
    Sna,
    Afr,
    Lat,
    Slk,
    Cat,
    Tgl,
    Hye,
}

macro_rules! impl_language_conversion {
    ($($variant:ident),+ $(,)?) => {
        impl From<charabia::Language> for PyLanguage {
            fn from(lang: charabia::Language) -> Self {
                match lang {
                    $(charabia::Language::$variant => PyLanguage::$variant,)+
                }
            }
        }

        impl From<PyLanguage> for charabia::Language {
            fn from(lang: PyLanguage) -> Self {
                match lang {
                    $(PyLanguage::$variant => charabia::Language::$variant,)+
                }
            }
        }
    };
}

impl_language_conversion!(
    Zho, Epo, Eng, Rus, Cmn, Spa, Por, Ita, Ben, Fra, Deu, Ukr, Kat, Ara, Hin, Jpn, Heb, Yid, Pol,
    Amh, Jav, Kor, Nob, Dan, Swe, Fin, Tur, Nld, Hun, Ces, Ell, Bul, Bel, Mar, Kan, Ron, Slv, Hrv,
    Srp, Mkd, Lit, Lav, Est, Tam, Vie, Urd, Tha, Guj, Uzb, Pan, Aze, Ind, Tel, Pes, Mal, Ori, Mya,
    Nep, Sin, Khm, Tuk, Aka, Zul, Sna, Afr, Lat, Slk, Cat, Tgl, Hye
);

#[pymethods]
impl PyLanguage {
    /// Get the ISO 639-3 language code.
    fn code(&self) -> &'static str {
        let lang: charabia::Language = (*self).into();
        lang.code()
    }

    /// Create a Language from ISO 639-3 code.
    #[staticmethod]
    fn from_code(code: &str) -> Option<Self> {
        charabia::Language::from_code(code).map(Into::into)
    }

    fn __repr__(&self) -> String {
        format!("Language.{}", self.code().to_uppercase())
    }

    fn __str__(&self) -> &'static str {
        let lang: charabia::Language = (*self).into();
        lang.code()
    }
}

/// Script/writing system for text.
#[pyclass(name = "Script", eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PyScript {
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Kannada,
    Khmer,
    Latin,
    Malayalam,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai,
    /// Combined CJK script (Chinese, Japanese Hiragana/Katakana)
    Cj,
    /// Other/unknown script
    Other,
}

macro_rules! impl_script_conversion {
    ($($variant:ident),+ $(,)?) => {
        impl From<charabia::Script> for PyScript {
            fn from(script: charabia::Script) -> Self {
                match script {
                    $(charabia::Script::$variant => PyScript::$variant,)+
                }
            }
        }

        impl From<PyScript> for charabia::Script {
            fn from(script: PyScript) -> Self {
                match script {
                    $(PyScript::$variant => charabia::Script::$variant,)+
                }
            }
        }
    };
}

impl_script_conversion!(
    Arabic, Armenian, Bengali, Cyrillic, Devanagari, Ethiopic, Georgian, Greek, Gujarati, Gurmukhi,
    Hangul, Hebrew, Kannada, Khmer, Latin, Malayalam, Myanmar, Oriya, Sinhala, Tamil, Telugu, Thai,
    Cj, Other
);

#[pymethods]
impl PyScript {
    /// Get the script name.
    fn name(&self) -> &'static str {
        let script: charabia::Script = (*self).into();
        script.name()
    }

    /// Create a Script from name.
    #[staticmethod]
    fn from_name(name: &str) -> Self {
        charabia::Script::from_name(name).into()
    }

    fn __repr__(&self) -> String {
        format!("Script.{}", self.name())
    }

    fn __str__(&self) -> &'static str {
        let script: charabia::Script = (*self).into();
        script.name()
    }
}
