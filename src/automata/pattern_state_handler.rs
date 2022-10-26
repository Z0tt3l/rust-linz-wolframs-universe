static PATTERNS: &'static [&'static [bool]] = &[
    &[true, true, true],
    &[true, true, false],
    &[true, false, true],
    &[true, false, false],
    &[false, true, true],
    &[false, true, false],
    &[false, false, true],
    &[false, false, false],
];

#[derive(Clone)]
pub struct PatternStateHandler {
    pub patterns: Vec<Vec<bool>>,
}

impl PatternStateHandler {
    pub fn new(rule: u8) -> Self {
        Self {
            patterns: Self::get_patterns(rule),
            ..Default::default()
        }
    }

    fn get_patterns(rule: u8) -> Vec<Vec<bool>> {
        let mut result = Vec::new();
        let bits = format!("{:08b}", rule);

        for (index, bit) in bits.chars().enumerate() {
            if bit == '1' {
                result.push(PATTERNS[index].to_vec());
            }
        }

        result
    }
}

impl Default for PatternStateHandler {
    fn default() -> Self {
        Self {
            patterns: Self::get_patterns(30),
        }
    }
}
