use rand::Rng;
use rand::RngCore;

use crate::cli::CharSet;
use crate::entropy::EntropyInfo;
use crate::generators::{GeneratedPassword, PasswordGenerator};

pub struct SecureGenerator {
    length: usize,
    charset: Vec<char>,
}

impl SecureGenerator {
    const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &'static str = "0123456789";
    const SYMBOLS: &'static str = "!@#$%^&*()-_=+[]{}|;:,.<>?";
    const AMBIGUOUS: &'static str = "0O1lI";

    pub fn new(length: usize, charset_type: &CharSet, exclude_ambiguous: bool) -> Self {
        let mut charset = String::new();

        match charset_type {
            CharSet::Alpha => {
                charset.push_str(Self::LOWERCASE);
                charset.push_str(Self::UPPERCASE);
            }
            CharSet::Alphanumeric => {
                charset.push_str(Self::LOWERCASE);
                charset.push_str(Self::UPPERCASE);
                charset.push_str(Self::DIGITS);
            }
            CharSet::AlphanumericSymbols => {
                charset.push_str(Self::LOWERCASE);
                charset.push_str(Self::UPPERCASE);
                charset.push_str(Self::DIGITS);
                charset.push_str(Self::SYMBOLS);
            }
            CharSet::All => {
                // All printable ASCII (32-126)
                charset = (32u8..=126).map(|b| b as char).collect();
            }
        }

        let charset: Vec<char> = if exclude_ambiguous {
            charset
                .chars()
                .filter(|c| !Self::AMBIGUOUS.contains(*c))
                .collect()
        } else {
            charset.chars().collect()
        };

        Self { length, charset }
    }
}

impl PasswordGenerator for SecureGenerator {
    fn generate(&self, rng: &mut dyn RngCore) -> GeneratedPassword {
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..self.charset.len());
                self.charset[idx]
            })
            .collect();

        // Entropy = log2(charset_size^length) = length * log2(charset_size)
        let entropy_bits = (self.length as f64) * (self.charset.len() as f64).log2();

        GeneratedPassword {
            value: password,
            entropy: EntropyInfo::new(entropy_bits, "Random"),
        }
    }

    fn description(&self) -> &'static str {
        "Secure random"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_secure_length() {
        let gen = SecureGenerator::new(16, &CharSet::Alphanumeric, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.len(), 16);
    }

    #[test]
    fn test_secure_alphanumeric_charset() {
        let gen = SecureGenerator::new(100, &CharSet::Alphanumeric, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert!(password.value.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_secure_alpha_only() {
        let gen = SecureGenerator::new(100, &CharSet::Alpha, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert!(password.value.chars().all(|c| c.is_ascii_alphabetic()));
    }

    #[test]
    fn test_secure_no_ambiguous() {
        let gen = SecureGenerator::new(1000, &CharSet::Alphanumeric, true);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        let ambiguous = ['0', 'O', '1', 'l', 'I'];
        assert!(!password.value.chars().any(|c| ambiguous.contains(&c)));
    }

    #[test]
    fn test_secure_entropy_alphanumeric() {
        let gen = SecureGenerator::new(16, &CharSet::Alphanumeric, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // 62 chars: 16 * log2(62) ≈ 95.27 bits
        assert!((password.entropy.bits - 95.27).abs() < 0.1);
    }

    #[test]
    fn test_secure_with_symbols_has_special_chars() {
        let gen = SecureGenerator::new(100, &CharSet::AlphanumericSymbols, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // With 100 chars from a set including symbols, we should have some symbols
        assert!(password.value.chars().any(|c| !c.is_ascii_alphanumeric()));
    }
}
