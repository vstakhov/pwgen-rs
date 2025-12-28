use rand::Rng;
use rand::RngCore;
use zeroize::Zeroizing;

use crate::entropy::EntropyInfo;
use crate::generators::{GeneratedPassword, PasswordGenerator};

pub struct PinGenerator {
    length: usize,
}

impl PinGenerator {
    /// Bits of entropy per digit: log2(10)
    const ENTROPY_PER_DIGIT: f64 = std::f64::consts::LOG2_10;

    pub fn new(length: usize) -> Self {
        Self { length }
    }
}

impl PasswordGenerator for PinGenerator {
    fn generate(&self, rng: &mut dyn RngCore) -> GeneratedPassword {
        let pin: String = (0..self.length)
            .map(|_| (b'0' + rng.gen_range(0..10)) as char)
            .collect();

        let entropy_bits = (self.length as f64) * Self::ENTROPY_PER_DIGIT;

        GeneratedPassword {
            value: Zeroizing::new(pin),
            entropy: EntropyInfo::new(entropy_bits, "Numeric"),
        }
    }

    fn description(&self) -> &'static str {
        "Numeric PIN"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_pin_length() {
        let gen = PinGenerator::new(6);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert_eq!(password.value.len(), 6);
    }

    #[test]
    fn test_pin_only_digits() {
        let gen = PinGenerator::new(10);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        assert!(password.value.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_pin_entropy() {
        let gen = PinGenerator::new(6);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let password = gen.generate(&mut rng);
        // 6 digits = 6 * log2(10) ≈ 19.93 bits
        assert!((password.entropy.bits - 19.93).abs() < 0.1);
    }

    #[test]
    fn test_pin_deterministic_with_seed() {
        let gen = PinGenerator::new(6);
        let mut rng1 = ChaCha8Rng::seed_from_u64(12345);
        let mut rng2 = ChaCha8Rng::seed_from_u64(12345);
        let p1 = gen.generate(&mut rng1);
        let p2 = gen.generate(&mut rng2);
        assert_eq!(p1.value, p2.value);
    }
}
