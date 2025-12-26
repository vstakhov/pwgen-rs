use console::Term;
use owo_colors::OwoColorize;

use crate::entropy::StrengthLevel;
use crate::generators::GeneratedPassword;

pub struct PasswordDisplay {
    use_colors: bool,
    use_emoji: bool,
    quiet: bool,
}

impl PasswordDisplay {
    pub fn new(use_colors: bool, quiet: bool) -> Self {
        Self {
            use_colors,
            use_emoji: use_colors,
            quiet,
        }
    }

    /// Display a generated password with strength indicator
    pub fn show(&self, password: &GeneratedPassword) {
        if self.quiet {
            println!("{}", password.value);
            return;
        }

        let entropy = &password.entropy;
        let strength = entropy.strength();

        // Password value
        if self.use_colors {
            println!(
                "  {} {}",
                "Password:".bold(),
                password.value.green().bold()
            );
        } else {
            println!("  Password: {}", password.value);
        }

        // Progress bar
        let bar = self.render_progress_bar(entropy.percentage(), 20, strength);

        if self.use_colors {
            let emoji = if self.use_emoji {
                format!(" {}", strength.emoji())
            } else {
                String::new()
            };
            println!(
                "  {} {} {:.1} bits {}{}",
                "Strength:".bold(),
                bar,
                entropy.bits,
                self.colored_strength_label(strength),
                emoji
            );
        } else {
            println!(
                "  Strength: {} {:.1} bits ({})",
                bar,
                entropy.bits,
                strength.label()
            );
        }

        println!();
    }

    fn render_progress_bar(&self, percentage: u8, width: usize, strength: StrengthLevel) -> String {
        let filled = (width * percentage as usize) / 100;
        let empty = width - filled;

        let fill_char = "█";
        let empty_char = "░";

        let bar = format!("{}{}", fill_char.repeat(filled), empty_char.repeat(empty));

        if self.use_colors {
            match strength {
                StrengthLevel::VeryWeak => bar.red().to_string(),
                StrengthLevel::Weak => bar.yellow().to_string(),
                StrengthLevel::Moderate => bar.blue().to_string(),
                StrengthLevel::Strong => bar.green().to_string(),
                StrengthLevel::VeryStrong => bar.bright_green().bold().to_string(),
            }
        } else {
            format!("[{}]", bar)
        }
    }

    fn colored_strength_label(&self, strength: StrengthLevel) -> String {
        let label = strength.label();
        match strength {
            StrengthLevel::VeryWeak => label.red().to_string(),
            StrengthLevel::Weak => label.yellow().to_string(),
            StrengthLevel::Moderate => label.blue().to_string(),
            StrengthLevel::Strong => label.green().to_string(),
            StrengthLevel::VeryStrong => label.bright_green().bold().to_string(),
        }
    }

    /// Show header with generator type
    pub fn show_header(&self, description: &str, count: usize) {
        if self.quiet {
            return;
        }

        let emoji = if self.use_emoji { "🔑 " } else { "" };

        if self.use_colors {
            println!(
                "\n{}{}",
                emoji,
                format!("Generating {} {} password(s):", count, description)
                    .cyan()
                    .bold()
            );
        } else {
            println!("\nGenerating {} {} password(s):", count, description);
        }
        println!();
    }

    /// Detect if we should use colors
    pub fn should_use_colors(no_color_flag: bool) -> bool {
        !no_color_flag && Term::stdout().is_term()
    }
}
