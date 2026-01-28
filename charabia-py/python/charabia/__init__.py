"""Charabia - A library to detect language, tokenize text, and normalize tokens.

Charabia is a Python binding for the Rust charabia library, providing fast
and accurate tokenization with support for 60+ languages including Chinese,
Japanese, Korean, Thai, Arabic, and more.

Quick Start:
    >>> import charabia
    >>> tokens = charabia.tokenize("Hello world")
    >>> for token in tokens:
    ...     print(f"{token.lemma} ({token.kind})")
    hello (TokenKind.Word)
      (TokenKind.SoftSeparator)
    world (TokenKind.Word)

Custom Configuration:
    >>> builder = charabia.TokenizerBuilder()
    >>> builder.stop_words(["the", "The", "a", "A"])  # Include all case variants
    >>> builder.lossy_normalization(True)
    >>> tokenizer = builder.build()
    >>> tokens = tokenizer.tokenize("The quick brown fox")

Note on Stop Words:
    Stop words are checked BEFORE normalization (lowercasing), so you need to
    include all case variants you want to match (e.g., "the", "The", "THE").

CJK Tokenization:
    >>> tokens = charabia.tokenize("東京都は日本の首都です")
    >>> for token in tokens:
    ...     if token.is_word():
    ...         print(token.lemma)
"""

from charabia._charabia import (
    Language,
    Script,
    SeparatorKind,
    Token,
    TokenKind,
    Tokenizer,
    TokenizerBuilder,
    tokenize,
)

__all__ = [
    "Language",
    "Script",
    "SeparatorKind",
    "Token",
    "TokenKind",
    "Tokenizer",
    "TokenizerBuilder",
    "tokenize",
]

__version__ = "0.1.0"
