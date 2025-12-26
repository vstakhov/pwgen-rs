use crate::entropy::EntropyInfo;

/// Result of password generation
pub struct GeneratedPassword {
    pub value: String,
    pub entropy: EntropyInfo,
}

/// Trait for all password generators
pub trait PasswordGenerator {
    /// Generate a single password
    fn generate(&self, rng: &mut dyn rand::RngCore) -> GeneratedPassword;

    /// Human-readable description of this generator type
    fn description(&self) -> &'static str;
}

pub mod markov;
pub mod passphrase;
pub mod pin;
pub mod secure;

pub use markov::MarkovGenerator;
pub use passphrase::PassphraseGenerator;
pub use pin::PinGenerator;
pub use secure::SecureGenerator;
