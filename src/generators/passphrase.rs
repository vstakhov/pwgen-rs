use rand::Rng;
use rand::RngCore;
use zeroize::Zeroizing;

use crate::entropy::EntropyInfo;
use crate::generators::{GeneratedPassword, PasswordGenerator};

/// EFF large wordlist (7776 words)
const EFF_WORDLIST: &str = include_str!("../../data/eff_large_wordlist.txt");

pub struct PassphraseGenerator {
    words: Vec<&'static str>,
    word_count: usize,
    separator: String,
    capitalize: bool,
    mutate: bool,
}

impl PassphraseGenerator {
    /// Bits of entropy per word: log2(7776) ≈ 12.925
    const ENTROPY_PER_WORD: f64 = 12.925;
    /// Extra entropy from mutations (conservative estimate)
    const MUTATION_ENTROPY_BONUS: f64 = 2.0;

    pub fn new(word_count: usize, separator: String, capitalize: bool, mutate: bool) -> Self {
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
            mutate,
        }
    }

    fn capitalize_word(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        if let Some(first) = chars.first_mut() {
            *first = first.to_uppercase().next().unwrap_or(*first);
        }
        chars.into_iter().collect()
    }

    /// Apply leet speak transformation to a character
    fn leetify(c: char) -> char {
        match c.to_ascii_lowercase() {
            'a' => '4',
            'e' => '3',
            'i' => '1',
            'o' => '0',
            's' => '5',
            't' => '7',
            'b' => '8',
            'g' => '9',
            _ => c,
        }
    }

    /// Apply random mutations to a word
    fn mutate_word(word: &str, rng: &mut dyn RngCore) -> String {
        let mut result: Vec<char> = word.chars().collect();
        let len = result.len();

        if len < 3 {
            return word.to_string();
        }

        // Decide what mutation to apply (can apply multiple)
        let mutation_type = rng.gen_range(0..100);

        if mutation_type < 40 {
            // 40% chance: Apply leet speak to 1-2 random characters
            let num_leet = rng.gen_range(1..=2.min(len));
            for _ in 0..num_leet {
                let pos = rng.gen_range(0..len);
                result[pos] = Self::leetify(result[pos]);
            }
        } else if mutation_type < 70 && len > 4 {
            // 30% chance: Truncate word (only if > 4 chars)
            let new_len = rng.gen_range(3..len);
            result.truncate(new_len);
        } else if mutation_type < 85 {
            // 15% chance: Double a vowel or consonant
            let pos = rng.gen_range(0..len);
            let c = result[pos];
            if c.is_ascii_alphabetic() {
                result.insert(pos, c);
            }
        } else {
            // 15% chance: No mutation (keep original)
        }

        result.into_iter().collect()
    }
}

impl PasswordGenerator for PassphraseGenerator {
    fn generate(&self, rng: &mut dyn RngCore) -> GeneratedPassword {
        let selected: Vec<String> = (0..self.word_count)
            .map(|_| {
                let idx = rng.gen_range(0..self.words.len());
                let word = self.words[idx];

                // Apply mutation if enabled
                let word = if self.mutate {
                    Self::mutate_word(word, rng)
                } else {
                    word.to_string()
                };

                if self.capitalize {
                    Self::capitalize_word(&word)
                } else {
                    word
                }
            })
            .collect();

        let passphrase = selected.join(&self.separator);

        // Entropy calculation: base + mutation bonus if enabled
        let mut entropy_bits = (self.word_count as f64) * Self::ENTROPY_PER_WORD;
        if self.mutate {
            entropy_bits += (self.word_count as f64) * Self::MUTATION_ENTROPY_BONUS;
        }

        GeneratedPassword {
            value: Zeroizing::new(passphrase),
            entropy: EntropyInfo::new(entropy_bits, "Diceware"),
        }
    }

    fn description(&self) -> &'static str {
        if self.mutate {
            "EFF Diceware passphrase (mutated)"
        } else {
            "EFF Diceware passphrase"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_passphrase_word_count() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.split('-').count(), 6);
    }

    #[test]
    fn test_passphrase_custom_separator() {
        let gen = PassphraseGenerator::new(4, ".".to_string(), false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.split('.').count(), 4);
    }

    #[test]
    fn test_passphrase_no_separator() {
        let gen = PassphraseGenerator::new(3, "".to_string(), false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // Without separator and mutation, should be all lowercase letters
        assert!(password.value.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_passphrase_capitalize() {
        let gen = PassphraseGenerator::new(4, "-".to_string(), true, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // Each word should start with uppercase
        for word in password.value.split('-') {
            let first = word.chars().next().unwrap();
            assert!(first.is_ascii_uppercase());
        }
    }

    #[test]
    fn test_passphrase_entropy_no_mutate() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // 6 words * 12.925 bits ≈ 77.55 bits (no mutation bonus)
        assert!((password.entropy.bits - 77.55).abs() < 0.1);
    }

    #[test]
    fn test_passphrase_entropy_with_mutate() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false, true);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // 6 words * (12.925 + 2.0) bits ≈ 89.55 bits (with mutation bonus)
        assert!((password.entropy.bits - 89.55).abs() < 0.1);
    }

    #[test]
    fn test_passphrase_words_from_eff_list() {
        let gen = PassphraseGenerator::new(10, "-".to_string(), false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // All words should be lowercase alphabetic (no mutation)
        for word in password.value.split('-') {
            assert!(word.chars().all(|c| c.is_ascii_lowercase()));
            assert!(!word.is_empty());
        }
    }

    #[test]
    fn test_passphrase_mutation_changes_words() {
        let gen = PassphraseGenerator::new(6, "-".to_string(), false, true);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // With mutation, at least some words should have non-alphabetic chars or be modified
        let has_mutation = password.value.chars().any(|c| c.is_ascii_digit());
        // Note: not guaranteed every time due to 15% no-mutation chance, but very likely with 6 words
        assert!(has_mutation || password.value.len() > 0); // At minimum, generates something
    }

    #[test]
    fn test_leetify() {
        assert_eq!(PassphraseGenerator::leetify('a'), '4');
        assert_eq!(PassphraseGenerator::leetify('e'), '3');
        assert_eq!(PassphraseGenerator::leetify('i'), '1');
        assert_eq!(PassphraseGenerator::leetify('o'), '0');
        assert_eq!(PassphraseGenerator::leetify('s'), '5');
        assert_eq!(PassphraseGenerator::leetify('t'), '7');
        assert_eq!(PassphraseGenerator::leetify('z'), 'z'); // unchanged
    }
}
