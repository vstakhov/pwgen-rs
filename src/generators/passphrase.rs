use rand::Rng;
use rand::RngCore;

use crate::entropy::EntropyInfo;
use crate::generators::{GeneratedPassword, PasswordGenerator};

/// EFF large wordlist (7776 words)
const EFF_WORDLIST: &str = include_str!("../../data/eff_large_wordlist.txt");

pub struct PassphraseGenerator {
    words: Vec<&'static str>,
    word_count: usize,
    separator: String,
    capitalize: bool,
}

impl PassphraseGenerator {
    /// Bits of entropy per word: log2(7776) ≈ 12.925
    const ENTROPY_PER_WORD: f64 = 12.925;

    pub fn new(word_count: usize, separator: String, capitalize: bool) -> Self {
        // Parse EFF wordlist (format: "11111\tabacus")
        let words: Vec<&'static str> = EFF_WORDLIST
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    Some(parts[1])
                } else {
                    None
                }
            })
            .collect();

        Self {
            words,
            word_count,
            separator,
            capitalize,
        }
    }

    fn capitalize_word(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        if let Some(first) = chars.first_mut() {
            *first = first.to_uppercase().next().unwrap_or(*first);
        }
        chars.into_iter().collect()
    }
}

impl PasswordGenerator for PassphraseGenerator {
    fn generate(&self, rng: &mut dyn RngCore) -> GeneratedPassword {
        let selected: Vec<String> = (0..self.word_count)
            .map(|_| {
                let idx = rng.gen_range(0..self.words.len());
                let word = self.words[idx];

                if self.capitalize {
                    Self::capitalize_word(word)
                } else {
                    word.to_string()
                }
            })
            .collect();

        let passphrase = selected.join(&self.separator);

        // Entropy is simply word_count * log2(7776)
        let entropy_bits = (self.word_count as f64) * Self::ENTROPY_PER_WORD;

        GeneratedPassword {
            value: passphrase,
            entropy: EntropyInfo::new(entropy_bits, "Diceware"),
        }
    }

    fn description(&self) -> &'static str {
        "EFF Diceware passphrase"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_passphrase_word_count() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.split('-').count(), 6);
    }

    #[test]
    fn test_passphrase_custom_separator() {
        let gen = PassphraseGenerator::new(4, ".".to_string(), false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.split('.').count(), 4);
    }

    #[test]
    fn test_passphrase_no_separator() {
        let gen = PassphraseGenerator::new(3, "".to_string(), false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // Without separator, should be all lowercase letters
        assert!(password.value.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_passphrase_capitalize() {
        let gen = PassphraseGenerator::new(4, "-".to_string(), true);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // Each word should start with uppercase
        for word in password.value.split('-') {
            let first = word.chars().next().unwrap();
            assert!(first.is_ascii_uppercase());
        }
    }

    #[test]
    fn test_passphrase_entropy() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // 6 words * 12.925 bits ≈ 77.55 bits
        assert!((password.entropy.bits - 77.55).abs() < 0.1);
    }

    #[test]
    fn test_passphrase_words_from_eff_list() {
        let gen = PassphraseGenerator::new(10, "-".to_string(), false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // All words should be lowercase alphabetic
        for word in password.value.split('-') {
            assert!(word.chars().all(|c| c.is_ascii_lowercase()));
            assert!(!word.is_empty());
        }
    }
}
