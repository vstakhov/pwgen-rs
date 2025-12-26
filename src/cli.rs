use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "pwgen-rs",
    author,
    version,
    about = "Generate secure, memorable passwords",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Number of passwords to generate
    #[arg(short = 'n', long, default_value = "1", global = true)]
    pub count: usize,

    /// Suppress decorative output (only show passwords)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate pronounceable passwords using Markov chains
    Normal {
        /// Password length
        #[arg(short, long, default_value = "12")]
        length: usize,

        /// Include numbers
        #[arg(short = 'd', long, default_value = "true")]
        digits: bool,

        /// Include readable symbols (!@#$%&*-_=+)
        #[arg(short, long, default_value = "false")]
        symbols: bool,

        /// Capitalize first letter
        #[arg(short = 'C', long, default_value = "true")]
        capitalize: bool,
    },

    /// Generate cryptographically secure random passwords
    Secure {
        /// Password length
        #[arg(short, long, default_value = "16")]
        length: usize,

        /// Character set to use
        #[arg(short = 'S', long, value_enum, default_value = "alphanumeric-symbols")]
        charset: CharSet,

        /// Exclude ambiguous characters (0O1lI)
        #[arg(long)]
        no_ambiguous: bool,
    },

    /// Generate diceware passphrases using EFF wordlist
    Phrase {
        /// Number of words
        #[arg(short, long, default_value = "6")]
        words: usize,

        /// Word separator
        #[arg(short, long, value_enum, default_value = "dash")]
        separator: Separator,

        /// Custom separator string (overrides --separator)
        #[arg(long)]
        custom_sep: Option<String>,

        /// Capitalize each word
        #[arg(short = 'C', long)]
        capitalize: bool,
    },

    /// Generate numeric PIN codes
    Pin {
        /// PIN length
        #[arg(short, long, default_value = "6")]
        length: usize,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CharSet {
    /// a-z, A-Z
    Alpha,
    /// a-z, A-Z, 0-9
    Alphanumeric,
    /// a-z, A-Z, 0-9, symbols
    AlphanumericSymbols,
    /// All printable ASCII
    All,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Separator {
    Dash,
    Space,
    Dot,
    Underscore,
    None,
}

impl Separator {
    pub fn as_str(&self) -> &str {
        match self {
            Separator::Dash => "-",
            Separator::Space => " ",
            Separator::Dot => ".",
            Separator::Underscore => "_",
            Separator::None => "",
        }
    }
}
