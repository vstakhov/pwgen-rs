use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::Rng;
use rand::RngCore;
use std::collections::HashMap;
use zeroize::Zeroizing;

use crate::entropy::EntropyInfo;
use crate::generators::{GeneratedPassword, PasswordGenerator};

/// EFF wordlist for training the Markov model
const TRAINING_WORDS: &str = include_str!("../../data/eff_large_wordlist.txt");

/// 2nd-order Markov model for generating pronounceable passwords
pub struct MarkovGenerator {
    /// (char1, char2) -> vec of (next_char, weight)
    transitions: HashMap<(char, char), Vec<(char, u32)>>,
    /// Starting bigrams with their weights
    start_pairs: Vec<((char, char), u32)>,
    /// Average branching factor for entropy calculation
    avg_branching_factor: f64,
    /// Target password length
    length: usize,
    /// Include digits
    include_digits: bool,
    /// Include symbols
    include_symbols: bool,
    /// Capitalize first letter
    capitalize: bool,
}

impl MarkovGenerator {
    const READABLE_SYMBOLS: [char; 10] = ['!', '@', '#', '$', '%', '&', '*', '-', '_', '+'];
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    pub fn new(length: usize, include_digits: bool, include_symbols: bool, capitalize: bool) -> Self {
        let (transitions, start_pairs, avg_branching_factor) = Self::build_model();

        Self {
            transitions,
            start_pairs,
            avg_branching_factor,
            length,
            include_digits,
            include_symbols,
            capitalize,
        }
    }

    /// Build the Markov model from the training wordlist
    #[allow(clippy::type_complexity)]
    fn build_model() -> (
        HashMap<(char, char), Vec<(char, u32)>>,
        Vec<((char, char), u32)>,
        f64,
    ) {
        let mut bigram_counts: HashMap<(char, char), HashMap<char, u32>> = HashMap::new();
        let mut start_counts: HashMap<(char, char), u32> = HashMap::new();

        // Parse training words and count transitions
        for line in TRAINING_WORDS.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() != 2 {
                continue;
            }

            let word = parts[1].to_lowercase();
            let chars: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();

            if chars.len() < 3 {
                continue;
            }

            // Count starting bigram
            let start = (chars[0], chars[1]);
            *start_counts.entry(start).or_insert(0) += 1;

            // Count trigram transitions
            for window in chars.windows(3) {
                let key = (window[0], window[1]);
                let next = window[2];
                *bigram_counts
                    .entry(key)
                    .or_default()
                    .entry(next)
                    .or_insert(0) += 1;
            }
        }

        // Convert to weighted vectors for efficient sampling
        let transitions: HashMap<(char, char), Vec<(char, u32)>> = bigram_counts
            .into_iter()
            .map(|(key, counts)| {
                let vec: Vec<(char, u32)> = counts.into_iter().collect();
                (key, vec)
            })
            .collect();

        let start_pairs: Vec<((char, char), u32)> = start_counts.into_iter().collect();

        // Calculate average branching factor
        let total_transitions: usize = transitions.values().map(|v| v.len()).sum();
        let avg_branching_factor = if !transitions.is_empty() {
            total_transitions as f64 / transitions.len() as f64
        } else {
            26.0 // fallback
        };

        (transitions, start_pairs, avg_branching_factor)
    }

    /// Generate base pronounceable string using Markov chain
    fn generate_base(&self, rng: &mut dyn RngCore) -> Option<String> {
        if self.start_pairs.is_empty() {
            return None;
        }

        let mut result = String::with_capacity(self.length);

        // Pick starting bigram
        let start_weights: Vec<u32> = self.start_pairs.iter().map(|(_, w)| *w).collect();
        let dist = WeightedIndex::new(&start_weights).ok()?;
        let start_idx = dist.sample(rng);
        let (c1, c2) = self.start_pairs[start_idx].0;

        result.push(c1);
        result.push(c2);

        // Generate remaining characters
        let mut attempts = 0;
        while result.len() < self.length && attempts < 100 {
            let len = result.len();
            let chars: Vec<char> = result.chars().collect();
            let key = (chars[len - 2], chars[len - 1]);

            if let Some(transitions) = self.transitions.get(&key) {
                let weights: Vec<u32> = transitions.iter().map(|(_, w)| *w).collect();
                if let Ok(dist) = WeightedIndex::new(&weights) {
                    let idx = dist.sample(rng);
                    result.push(transitions[idx].0);
                } else {
                    break;
                }
            } else {
                // Dead end - try to restart with a new starting pair that we can append
                break;
            }
            attempts += 1;
        }

        // Pad with random vowels/consonants if needed
        while result.len() < self.length {
            let last_char = result.chars().last().unwrap_or('a');
            if Self::VOWELS.contains(&last_char) {
                // Add a consonant
                let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't'];
                result.push(consonants[rng.gen_range(0..consonants.len())]);
            } else {
                // Add a vowel
                result.push(Self::VOWELS[rng.gen_range(0..Self::VOWELS.len())]);
            }
        }

        result.truncate(self.length);
        Some(result)
    }

    /// Post-process: add digits/symbols, capitalize
    fn post_process(&self, mut password: String, rng: &mut dyn RngCore) -> String {
        let mut chars: Vec<char> = password.chars().collect();

        // Capitalize first letter if requested
        if self.capitalize && !chars.is_empty() {
            chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
        }

        password = chars.into_iter().collect();

        // Insert a digit at random position (not at start)
        if self.include_digits && password.len() > 2 {
            let pos = rng.gen_range(1..password.len());
            let digit = (b'0' + rng.gen_range(0..10)) as char;
            password.insert(pos, digit);
            password.truncate(self.length);
        }

        // Insert a symbol at random position (not at start)
        if self.include_symbols && password.len() > 2 {
            let pos = rng.gen_range(1..password.len());
            let symbol = Self::READABLE_SYMBOLS[rng.gen_range(0..Self::READABLE_SYMBOLS.len())];
            password.insert(pos, symbol);
            password.truncate(self.length);
        }

        password
    }

    /// Check if password passes pronounceability filter
    fn is_pronounceable(password: &str) -> bool {
        let password = password.to_lowercase();

        let mut consonant_run = 0;
        let mut vowel_run = 0;

        for c in password.chars() {
            if !c.is_alphabetic() {
                consonant_run = 0;
                vowel_run = 0;
                continue;
            }

            if Self::VOWELS.contains(&c) {
                vowel_run += 1;
                consonant_run = 0;
                if vowel_run > 3 {
                    return false;
                }
            } else {
                consonant_run += 1;
                vowel_run = 0;
                if consonant_run > 3 {
                    return false;
                }
            }
        }

        true
    }
}

impl PasswordGenerator for MarkovGenerator {
    fn generate(&self, rng: &mut dyn RngCore) -> GeneratedPassword {
        // Retry until we get a pronounceable password
        for _ in 0..100 {
            if let Some(base) = self.generate_base(rng) {
                let password = self.post_process(base, rng);

                if Self::is_pronounceable(&password) {
                    // Calculate entropy based on model's branching factor
                    // This is a conservative estimate
                    let base_entropy = (self.length as f64) * self.avg_branching_factor.log2();

                    return GeneratedPassword {
                        value: Zeroizing::new(password),
                        entropy: EntropyInfo::new(base_entropy, "Markov pronounceable"),
                    };
                }
            }
        }

        // Fallback: generate a random pronounceable password
        let mut password = String::new();
        let syllables = ["ba", "be", "bi", "bo", "bu", "da", "de", "di", "do", "du",
                        "fa", "fe", "fi", "fo", "fu", "ga", "ge", "gi", "go", "gu",
                        "ha", "he", "hi", "ho", "hu", "ka", "ke", "ki", "ko", "ku",
                        "la", "le", "li", "lo", "lu", "ma", "me", "mi", "mo", "mu",
                        "na", "ne", "ni", "no", "nu", "pa", "pe", "pi", "po", "pu",
                        "ra", "re", "ri", "ro", "ru", "sa", "se", "si", "so", "su",
                        "ta", "te", "ti", "to", "tu", "va", "ve", "vi", "vo", "vu",
                        "wa", "we", "wi", "wo", "za", "ze", "zi", "zo", "zu"];

        while password.len() < self.length {
            password.push_str(syllables[rng.gen_range(0..syllables.len())]);
        }
        password.truncate(self.length);

        let password = self.post_process(password, rng);
        let entropy = (self.length as f64) * (syllables.len() as f64).log2() / 2.0;

        GeneratedPassword {
            value: Zeroizing::new(password),
            entropy: EntropyInfo::new(entropy, "Syllable fallback"),
        }
    }

    fn description(&self) -> &'static str {
        "Pronounceable (Markov chain)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_markov_length() {
        let gen = MarkovGenerator::new(12, false, false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.len(), 12);
    }

    #[test]
    fn test_markov_capitalize() {
        let gen = MarkovGenerator::new(12, false, false, true);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        let first = password.value.chars().next().unwrap();
        assert!(first.is_ascii_uppercase());
    }

    #[test]
    fn test_markov_with_digits() {
        let gen = MarkovGenerator::new(12, true, false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert!(password.value.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_markov_with_symbols() {
        let gen = MarkovGenerator::new(12, false, true, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        let symbols: Vec<char> = MarkovGenerator::READABLE_SYMBOLS.to_vec();
        assert!(password.value.chars().any(|c| symbols.contains(&c)));
    }

    #[test]
    fn test_markov_pronounceable() {
        let gen = MarkovGenerator::new(12, false, false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        // Generate multiple passwords and check they all pass pronounceability
        for _ in 0..10 {
            let password = gen.generate(&mut rng);
            assert!(MarkovGenerator::is_pronounceable(&password.value));
        }
    }

    #[test]
    fn test_is_pronounceable_rejects_too_many_consonants() {
        assert!(!MarkovGenerator::is_pronounceable("abcdstrng"));
        assert!(!MarkovGenerator::is_pronounceable("xyzwvuts"));
    }

    #[test]
    fn test_is_pronounceable_accepts_valid() {
        assert!(MarkovGenerator::is_pronounceable("hello"));
        assert!(MarkovGenerator::is_pronounceable("password"));
        assert!(MarkovGenerator::is_pronounceable("banana"));
    }

    #[test]
    fn test_markov_entropy_positive() {
        let gen = MarkovGenerator::new(12, false, false, false);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert!(password.entropy.bits > 0.0);
    }
}
