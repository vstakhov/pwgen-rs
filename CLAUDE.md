# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

pwgen-x is a CLI password generator with four modes:
- **normal** - Pronounceable passwords using 2nd-order Markov chain trained on EFF wordlist
- **secure** - Cryptographically secure random passwords
- **phrase** - Diceware passphrases using EFF 7776-word list
- **pin** - Numeric PIN codes

Features colored output with emoji strength indicators and entropy visualization.

## Build Commands

```bash
cargo build                  # Debug build
cargo build --release        # Release build
cargo run -- <command>       # Run with subcommand (normal, secure, phrase, pin)
cargo test                   # Run all tests
cargo clippy                 # Lint checks
cargo fmt                    # Format code
```

## Architecture

```
src/
├── main.rs              # CLI entry point, dispatches to generators
├── lib.rs               # Library re-exports
├── cli.rs               # Clap argument definitions (subcommands, options)
├── entropy.rs           # Entropy calculation and strength levels
├── generators/
│   ├── mod.rs           # PasswordGenerator trait definition
│   ├── markov.rs        # Markov chain pronounceable passwords
│   ├── secure.rs        # Secure random password generator
│   ├── passphrase.rs    # EFF diceware passphrase generator
│   └── pin.rs           # Numeric PIN generator
└── output/
    ├── mod.rs
    └── display.rs       # Colored terminal output, progress bars

data/
└── eff_large_wordlist.txt  # EFF diceware wordlist (embedded at compile time)
```

## Key Design Decisions

- **PasswordGenerator trait**: Uses `&mut dyn RngCore` for dyn-compatibility
- **Markov model**: Built at runtime from EFF wordlist trigrams, filters for pronounceability
- **EFF wordlist**: Embedded via `include_str!` for single-binary distribution
- **Entropy display**: Progress bar with color-coded strength levels (Very Weak to Very Strong)
