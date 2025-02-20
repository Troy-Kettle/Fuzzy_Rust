// src/type1/sets/t1mf_gaussian.rs
#![allow(dead_code)]


/// A simple Tuple type representing a support interval.
#[derive(Debug, Clone)]
pub struct Tuple {
    pub left: f64,
    pub right: f64,
}

impl Tuple {
    /// Creates a new Tuple.
    pub fn new(left: f64, right: f64) -> Self {
        Self { left, right }
    }
}

/// Trait representing the prototype for a Type‑1 membership function.
/// Allow unused methods because they might be used in the future.
#[allow(dead_code)]
pub trait T1MFPrototype {
    fn name(&self) -> &str;
    fn get_support(&self) -> &Tuple;
    fn is_left_shoulder(&self) -> bool;
    fn is_right_shoulder(&self) -> bool;
}

/// The Gaussian membership function for Type‑1 fuzzy sets.
pub struct T1MFGaussian {
    name: String,
    mean: f64,
    spread: f64,
    support: Tuple,
}

impl T1MFGaussian {
    /// Constructs a new Gaussian membership function.
    /// The support is defined as [mean - 4 * spread, mean + 4 * spread].
    pub fn new(name: String, mean: f64, spread: f64) -> Self {
        let support = Tuple::new(mean - 4.0 * spread, mean + 4.0 * spread);
        Self { name, mean, spread, support }
    }

    /// Returns the fuzzy set value for a given x.
    pub fn get_fs(&self, x: f64) -> f64 {
        if x >= self.support.left && x <= self.support.right {
            if self.is_left_shoulder() && x <= self.mean {
                return 1.0;
            }
            if self.is_right_shoulder() && x >= self.mean {
                return 1.0;
            }
            (-0.5 * ((x - self.mean) / self.spread).powi(2)).exp()
        } else {
            0.0
        }
    }

    /// Returns a string representation of the Gaussian membership function.
    pub fn to_string_rep(&self) -> String {
        let mut s = format!(
            "{} - Gaussian with mean {}, standard deviation: {}",
            self.name, self.mean, self.spread
        );
        if self.is_left_shoulder() {
            s.push_str(" (LeftShoulder)");
        }
        if self.is_right_shoulder() {
            s.push_str(" (RightShoulder)");
        }
        s
    }
}

impl T1MFPrototype for T1MFGaussian {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_support(&self) -> &Tuple {
        &self.support
    }

    fn is_left_shoulder(&self) -> bool {
        false
    }

    fn is_right_shoulder(&self) -> bool {
        false
    }
}

