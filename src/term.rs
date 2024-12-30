#[derive(Debug, Clone)]
pub struct Term {
    pub polarity: f64,
    pub exponent: usize,
    pub is_right_side: bool,
    pub current_value: f64,
}

impl Term {
    pub fn new(polarity: f64, exponent: usize, is_right_side: bool, current_value: f64) -> Self {
        Self {
            polarity,
            exponent,
            is_right_side,
            current_value,
        }
    }
}

impl Default for Term {
    fn default() -> Self {
        Self {
            polarity: 1.0,
            exponent: 0,
            is_right_side: false,
            current_value: 1.0,
        }
    }
}
