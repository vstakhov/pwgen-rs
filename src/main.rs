mod cli;
mod entropy;
mod generators;
mod output;

use anyhow::Result;
use clap::Parser;
use rand::thread_rng;

use cli::{Cli, Command};
use generators::{MarkovGenerator, PassphraseGenerator, PasswordGenerator, PinGenerator, SecureGenerator};
use output::PasswordDisplay;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine color support
    let use_colors = PasswordDisplay::should_use_colors(cli.no_color);
    let display = PasswordDisplay::new(use_colors, cli.quiet);

    // Get the appropriate generator
    let generator: Box<dyn PasswordGenerator> = match &cli.command {
        Command::Normal {
            length_pos,
            length,
            digits,
            symbols,
            capitalize,
        } => {
            let len = length_pos.or(*length).unwrap_or(12);
            Box::new(MarkovGenerator::new(len, *digits, *symbols, *capitalize))
        }

        Command::Secure {
            length_pos,
            length,
            charset,
            no_ambiguous,
        } => {
            let len = length_pos.or(*length).unwrap_or(16);
            Box::new(SecureGenerator::new(len, charset, *no_ambiguous))
        }

        Command::Phrase {
            words_pos,
            words,
            separator,
            custom_sep,
            capitalize,
            no_mutate,
        } => {
            let word_count = words_pos.or(*words).unwrap_or(6);
            let sep = custom_sep
                .clone()
                .unwrap_or_else(|| separator.as_str().to_string());
            Box::new(PassphraseGenerator::new(word_count, sep, *capitalize, !*no_mutate))
        }

        Command::Pin { length_pos, length } => {
            let len = length_pos.or(*length).unwrap_or(6);
            Box::new(PinGenerator::new(len))
        }
    };

    // Show header
    display.show_header(generator.description(), cli.count);

    // Generate passwords
    let mut rng = thread_rng();
    for _ in 0..cli.count {
        let password = generator.generate(&mut rng);
        display.show(&password);
    }

    Ok(())
}
