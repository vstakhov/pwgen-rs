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
            length,
            digits,
            symbols,
            capitalize,
        } => Box::new(MarkovGenerator::new(*length, *digits, *symbols, *capitalize)),

        Command::Secure {
            length,
            charset,
            no_ambiguous,
        } => Box::new(SecureGenerator::new(*length, charset, *no_ambiguous)),

        Command::Phrase {
            words,
            separator,
            custom_sep,
            capitalize,
        } => {
            let sep = custom_sep
                .clone()
                .unwrap_or_else(|| separator.as_str().to_string());
            Box::new(PassphraseGenerator::new(*words, sep, *capitalize))
        }

        Command::Pin { length } => Box::new(PinGenerator::new(*length)),
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
