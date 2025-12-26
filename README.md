# pwgen-rs

A modern, feature-rich password generator CLI written in Rust. Created as an improved alternative to the classic [`pwgen`](https://sourceforge.net/projects/pwgen/) utility, with better entropy visualization, multiple generation modes, and a more user-friendly interface.

**Note:** This project was coded by [Claude](https://claude.ai) (Anthropic's AI assistant).

## Features

- **Pronounceable passwords** - Markov chain-based generation trained on English words
- **Secure random passwords** - Cryptographically secure using ChaCha12 RNG
- **Diceware passphrases** - Using EFF's 7776-word list (~12.9 bits/word)
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
./target/release/pwgen-rs --help
```

## Usage

### Pronounceable Passwords (Markov Chain)

```bash
# Default: 12 characters, capitalized, with digit
pwgen-rs normal

# Longer password with symbols
pwgen-rs normal --length 16 --symbols

# Multiple passwords
pwgen-rs normal -n 5
```

### Secure Random Passwords

```bash
# Default: 16 characters with letters, numbers, and symbols
pwgen-rs secure

# Alphanumeric only, exclude ambiguous chars (0O1lI)
pwgen-rs secure --charset alphanumeric --no-ambiguous

# 32 character password
pwgen-rs secure --length 32
```

### Diceware Passphrases

```bash
# Default: 6 words with dashes
pwgen-rs phrase

# 4 words with spaces, capitalized
pwgen-rs phrase --words 4 --separator space --capitalize

# Custom separator
pwgen-rs phrase --custom-sep "."
```

### PIN Codes

```bash
# Default: 6 digits
pwgen-rs pin

# 8-digit PIN
pwgen-rs pin --length 8
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
| Passphrase (6 words) | `correct-horse-battery-staple-pizza-ninja` | ~78 bits |
| Secure (16 chars) | `D<(=j(\|Gu_NT2et\|` | ~103 bits |

## Strength Levels

- 💀 **Very Weak** (0-24 bits) - Easily cracked
- 😟 **Weak** (25-49 bits) - Vulnerable to offline attacks
- 😐 **Moderate** (50-74 bits) - Acceptable for most uses
- 😊 **Strong** (75-99 bits) - Good security
- 🔒 **Very Strong** (100+ bits) - Excellent security

## Why pwgen-rs?

The original `pwgen` is a great tool, but `pwgen-rs` offers several improvements:

- **Entropy display** - See exactly how strong your password is in bits
- **Diceware passphrases** - Built-in support for memorable word-based passwords
- **Modern CLI** - Colored output, subcommands, and helpful error messages
- **Single binary** - No external dependencies, EFF wordlist embedded at compile time

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Credits

Coded by [Claude](https://claude.ai) (Anthropic's AI assistant) with human guidance.
