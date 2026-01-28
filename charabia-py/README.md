# charabia-py

Python bindings for [charabia](https://github.com/meilisearch/charabia), a library to detect language, tokenize text, and normalize tokens.

## Installation

```bash
pip install charabia
```

### Building from source

```bash
cd charabia-py
uv venv && uv pip install maturin
source .venv/bin/activate
maturin develop  # For development
maturin build --release  # For distribution wheels
```

Requires Rust 1.80.0 or later.

## Usage

### Simple tokenization

```python
import charabia

tokens = charabia.tokenize("Hello world")
for token in tokens:
    print(f"{token.lemma} ({token.kind})")
# hello (TokenKind.Word)
#   (TokenKind.SoftSeparator)
# world (TokenKind.Word)
```

### Custom configuration

```python
from charabia import TokenizerBuilder

builder = TokenizerBuilder()
builder.stop_words(["the", "The", "a", "A"])  # Include case variants
builder.lossy_normalization(True)
builder.create_char_map(True)

tokenizer = builder.build()
tokens = tokenizer.tokenize("The quick brown fox")

for token in tokens:
    if token.is_stopword():
        print(f"Stop word: {token.lemma}")
    elif token.is_word():
        print(f"Word: {token.lemma}")
```

### Reconstruct original text

```python
tokenizer = TokenizerBuilder().build()
for original, token in tokenizer.reconstruct("Thé café"):
    print(f"'{original}' -> '{token.lemma}'")
# 'Thé' -> 'the'
# ' ' -> ' '
# 'café' -> 'cafe'
```

### CJK tokenization

```python
import charabia

# Chinese
for token in charabia.tokenize("我爱北京天安门"):
    if token.is_word():
        print(token.lemma)

# Japanese
for token in charabia.tokenize("東京都は日本の首都です"):
    if token.is_word():
        print(token.lemma)

# Korean
for token in charabia.tokenize("안녕하세요 세계"):
    if token.is_word():
        print(token.lemma)
```

### Language detection

```python
from charabia import Language, TokenizerBuilder

# Restrict to specific languages
builder = TokenizerBuilder()
builder.allow_list([Language.Eng, Language.Fra])
tokenizer = builder.build()

# Language utilities
lang = Language.from_code("eng")
print(lang.code())  # "eng"
```

## API Reference

### Functions

- `tokenize(text: str) -> list[Token]` - Tokenize with default settings

### Classes

#### `Token`
- `lemma: str` - Normalized text
- `kind: TokenKind` - Token classification
- `char_start: int` / `char_end: int` - Character positions
- `byte_start: int` / `byte_end: int` - Byte positions
- `script: Script` - Detected script (Latin, Cj, Arabic, etc.)
- `language: Language | None` - Detected language
- `is_word() -> bool`
- `is_stopword() -> bool`
- `is_separator() -> bool`

#### `TokenizerBuilder`
- `stop_words(words: list[str])` - Set stop words (case-sensitive, include variants)
- `separators(seps: list[str])` - Set custom separators
- `words_dict(words: list[str])` - Set custom word dictionary
- `create_char_map(enable: bool)` - Enable character position mapping
- `lossy_normalization(enable: bool)` - Enable/disable lossy normalization (default: True)
- `allow_list(languages: list[Language])` - Restrict language detection
- `build() -> Tokenizer`

#### `Tokenizer`
- `tokenize(text: str) -> list[Token]`
- `reconstruct(text: str) -> list[tuple[str, Token]]`

### Enums

- `TokenKind`: `Word`, `StopWord`, `SoftSeparator`, `HardSeparator`, `Unknown`
- `SeparatorKind`: `Hard`, `Soft`
- `Language`: 70 language codes (Eng, Fra, Deu, Jpn, Cmn, Ara, etc.)
- `Script`: 24 scripts (Latin, Cj, Arabic, Cyrillic, Hebrew, etc.)

## Notes

- **Stop words are case-sensitive**: They are checked before normalization, so include all case variants (e.g., "the", "The", "THE")
- **Lossy normalization** (enabled by default) includes lowercasing, diacritic removal, and language-specific transformations

## License

MIT
