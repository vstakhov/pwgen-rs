/// Password entropy information
#[derive(Debug, Clone)]
pub struct EntropyInfo {
    /// Entropy in bits
    pub bits: f64,
    /// Type of entropy calculation (for debugging/display)
    #[allow(dead_code)]
    pub source: &'static str,
}

impl EntropyInfo {
    pub fn new(bits: f64, source: &'static str) -> Self {
        Self { bits, source }
    }

    /// Strength category based on entropy bits
    pub fn strength(&self) -> StrengthLevel {
        match self.bits as u32 {
            0..=24 => StrengthLevel::VeryWeak,
            25..=49 => StrengthLevel::Weak,
            50..=74 => StrengthLevel::Moderate,
            75..=99 => StrengthLevel::Strong,
            _ => StrengthLevel::VeryStrong,
        }
    }

    /// Percentage for progress bar (0-100, capped at 128 bits)
    pub fn percentage(&self) -> u8 {
        ((self.bits / 128.0) * 100.0).min(100.0) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrengthLevel {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl StrengthLevel {
    pub fn label(&self) -> &'static str {
        match self {
            StrengthLevel::VeryWeak => "Very Weak",
            StrengthLevel::Weak => "Weak",
            StrengthLevel::Moderate => "Moderate",
            StrengthLevel::Strong => "Strong",
            StrengthLevel::VeryStrong => "Very Strong",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            StrengthLevel::VeryWeak => "💀",
            StrengthLevel::Weak => "😟",
            StrengthLevel::Moderate => "😐",
            StrengthLevel::Strong => "😊",
            StrengthLevel::VeryStrong => "🔒",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_very_weak() {
        let info = EntropyInfo::new(20.0, "test");
        assert_eq!(info.strength(), StrengthLevel::VeryWeak);
    }

    #[test]
    fn test_strength_weak() {
        let info = EntropyInfo::new(40.0, "test");
        assert_eq!(info.strength(), StrengthLevel::Weak);
    }

    #[test]
    fn test_strength_moderate() {
        let info = EntropyInfo::new(60.0, "test");
        assert_eq!(info.strength(), StrengthLevel::Moderate);
    }

    #[test]
    fn test_strength_strong() {
        let info = EntropyInfo::new(80.0, "test");
        assert_eq!(info.strength(), StrengthLevel::Strong);
    }

    #[test]
    fn test_strength_very_strong() {
        let info = EntropyInfo::new(120.0, "test");
        assert_eq!(info.strength(), StrengthLevel::VeryStrong);
    }

    #[test]
    fn test_percentage_capped() {
        let info = EntropyInfo::new(200.0, "test");
        assert_eq!(info.percentage(), 100);
    }

    #[test]
    fn test_percentage_half() {
        let info = EntropyInfo::new(64.0, "test");
        assert_eq!(info.percentage(), 50);
    }

    #[test]
    fn test_strength_labels() {
        assert_eq!(StrengthLevel::VeryWeak.label(), "Very Weak");
        assert_eq!(StrengthLevel::Strong.label(), "Strong");
    }

    #[test]
    fn test_strength_emoji() {
        assert_eq!(StrengthLevel::VeryWeak.emoji(), "💀");
        assert_eq!(StrengthLevel::VeryStrong.emoji(), "🔒");
    }
}
