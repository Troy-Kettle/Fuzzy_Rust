// src/type1/sets/t1mf_cylinder.rs

use crate::type1::sets::t1mf_gaussian::{Tuple, T1MFPrototype};

/// The T1MFCylinder struct serves as a cylindrical extension of a firing strength.
/// In terms of a type‑1 membership function, it represents a singleton over the whole universe of discourse.
pub struct T1MFCylinder {
    name: String,
    membership_degree: f64,
    support: Tuple,
}

impl T1MFCylinder {
    /// Creates a new T1MFCylinder.
    ///
    /// # Panics
    ///
    /// Panics if `membership_degree` is not between 0.0 and 1.0.
    pub fn new(name: String, membership_degree: f64) -> Self {
        if membership_degree < 0.0 || membership_degree > 1.0 {
            panic!("The membership degree should be between 0 and 1.");
        }
        // The support is the entire universe: (-∞, ∞)
        let support = Tuple::new(f64::NEG_INFINITY, f64::INFINITY);
        Self {
            name,
            membership_degree,
            support,
        }
    }

    /// Returns the membership degree for any input x.
    pub fn get_fs(&self, _x: f64) -> f64 {
        self.membership_degree
    }

    /// Returns an alpha-cut as an `Option<Tuple>`.
    ///
    /// If `alpha` is less than or equal to the membership degree, returns the full support;
    /// otherwise, returns `None`.
    pub fn get_alpha_cut(&self, alpha: f64) -> Option<Tuple> {
        if alpha <= self.membership_degree {
            Some(Tuple::new(f64::NEG_INFINITY, f64::INFINITY))
        } else {
            None
        }
    }

    /// Returns a string representation of the cylindrical membership function.
    pub fn to_string_rep(&self) -> String {
        format!(
            "{} - Cylindrical extension at: {}",
            self.name, self.membership_degree
        )
    }

    /// Unsupported method: compare_to.
    #[allow(dead_code)]
    pub fn compare_to(&self, _other: &dyn T1MFPrototype) -> i32 {
        panic!("Unsupported Method");
    }

    /// Unsupported method: get_peak.
    #[allow(dead_code)]
    pub fn get_peak(&self) -> f64 {
        panic!("Unsupported Method");
    }
}

impl T1MFPrototype for T1MFCylinder {
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

