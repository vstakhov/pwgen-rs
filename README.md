# pwgen-x

A modern, feature-rich password generator CLI written in Rust. Created as an improved alternative to the classic [`pwgen`](https://sourceforge.net/projects/pwgen/) utility, with better entropy visualization, multiple generation modes, and a more user-friendly interface.

**Note:** This project was coded by [Claude](https://claude.ai) (Anthropic's AI assistant).

## Features

- **Pronounceable passwords** - Markov chain-based generation trained on English words
- **Secure random passwords** - Cryptographically secure using ChaCha12 RNG
- **Diceware passphrases** - Using EFF's 7776-word list with optional word mutations (leet speak, truncation)
- **PIN codes** - Numeric-only passwords
- **Entropy visualization** - Colored progress bar with strength rating
- **Flexible output** - Quiet mode for scripts, customizable separators and charsets

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
./target/release/pwgen-x --help
```

## Usage

### Pronounceable Passwords (Markov Chain)

```bash
# Default: 12 characters, capitalized, with digit
pwgen-x normal

# 20 character password (positional shorthand for --length)
pwgen-x normal 20

# With symbols
pwgen-x normal 16 --symbols

# Multiple passwords
pwgen-x normal -n 5
```

### Secure Random Passwords

```bash
# Default: 16 characters with letters, numbers, and symbols
pwgen-x secure

# 32 character password (positional shorthand)
pwgen-x secure 32

# Alphanumeric only, exclude ambiguous chars (0O1lI)
pwgen-x secure 24 --charset alphanumeric --no-ambiguous
```

### Diceware Passphrases

```bash
# Default: 6 words with dashes and mutations (leet speak, truncation)
pwgen-x phrase

# 4 words (positional shorthand for --words)
pwgen-x phrase 4

# With spaces and capitalized
pwgen-x phrase 5 --separator space --capitalize

# Disable mutations for pure diceware words
pwgen-x phrase --no-mutate
```

Word mutations include:
- **Leet speak** - Random letter substitutions (a→4, e→3, s→5, etc.)
- **Truncation** - Shortening longer words
- **Doubling** - Repeating a letter

### PIN Codes

```bash
# Default: 6 digits
pwgen-x pin

# 8-digit PIN (positional shorthand)
pwgen-x pin 8
```

### Global Options

```bash
-n, --count <N>    Generate multiple passwords
-q, --quiet        Output only passwords (no decoration)
--no-color         Disable colored output
```

## Example Output

```
🔑 Generating 3 Secure random password(s):

  Password: D<(=j(|Gu_NT2et|
  Strength: ████████████████░░░░ 103.4 bits Very Strong 🔒

  Password: gr=wMA=gF;5GvDU(
  Strength: ████████████████░░░░ 103.4 bits Very Strong 🔒

  Password: 0qt}m20JBMG()ln(
  Strength: ████████████████░░░░ 103.4 bits Very Strong 🔒
```

## Entropy Reference

| Type | Example | Entropy |
|------|---------|---------|
| PIN (6 digits) | `495531` | ~20 bits |
| Pronounceable (12 chars) | `Engou3ckeduc` | ~36 bits |
| Passphrase (6 words, no mutate) | `correct-horse-battery-staple` | ~78 bits |
| Passphrase (6 words, mutated) | `corr3ct-h0rse-battery-5taple` | ~90 bits |
| Secure (16 chars) | `D<(=j(\|Gu_NT2et\|` | ~103 bits |

## Strength Levels

- 💀 **Very Weak** (0-24 bits) - Easily cracked
- 😟 **Weak** (25-49 bits) - Vulnerable to offline attacks
- 😐 **Moderate** (50-74 bits) - Acceptable for most uses
- 😊 **Strong** (75-99 bits) - Good security
- 🔒 **Very Strong** (100+ bits) - Excellent security

## Why pwgen-x?

The original `pwgen` is a great tool, but `pwgen-x` offers several improvements:

- **Entropy display** - See exactly how strong your password is in bits
- **Diceware passphrases** - Built-in support for memorable word-based passwords
- **Modern CLI** - Colored output, subcommands, and helpful error messages
- **Single binary** - No external dependencies, EFF wordlist embedded at compile time

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Credits

Coded by [Claude](https://claude.ai) (Anthropic's AI assistant) with human guidance.
