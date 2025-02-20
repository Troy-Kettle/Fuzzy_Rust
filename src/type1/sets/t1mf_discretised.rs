#![allow(unused)]  // Suppress warnings for unused code in this module

use crate::type1::sets::t1mf_gaussian::{Tuple, T1MFPrototype};

/// A discretised Type‑1 membership function defined by a set of points.
/// Points are stored as Tuples in (y, x) order.
pub struct T1MFDiscretised {
    name: String,
    sorted: bool,
    left_shoulder: bool,
    right_shoulder: bool,
    left_shoulder_start: f64,
    right_shoulder_start: f64,
    debug: bool,
    alpha_cut_disc_level: usize,
    alpha_cut_precision_limit: f64,
    support: Tuple,
    set: Vec<Tuple>,
}

impl T1MFDiscretised {
    /// Creates a new discretised membership function.
    /// The optional `points` parameter allows you to provide an initial list of points.
    pub fn new(name: String, points: Option<Vec<Tuple>>) -> Self {
        let mut instance = Self {
            name,
            sorted: false,
            left_shoulder: false,
            right_shoulder: false,
            left_shoulder_start: 0.0,
            right_shoulder_start: 0.0,
            debug: false,
            alpha_cut_disc_level: 60,
            alpha_cut_precision_limit: 0.01,
            // Initially set support to a default; it will be updated after sorting.
            support: Tuple::new(0.0, 0.0),
            set: Vec::new(),
        };
        if let Some(ps) = points {
            instance.add_points(ps);
            instance.sort();
        }
        instance
    }

    /// Adds a single point to the discretised set.
    pub fn add_point(&mut self, p: Tuple) {
        self.set.push(p);
        self.sorted = false;
    }

    /// Adds multiple points to the discretised set.
    pub fn add_points(&mut self, ps: Vec<Tuple>) {
        for p in ps {
            self.set.push(p);
        }
        self.sorted = false;
    }

    /// Returns the current alpha cut discretisation level.
    pub fn get_alpha_cut_discretisation_level(&self) -> usize {
        self.alpha_cut_disc_level
    }

    /// Sets a new alpha cut discretisation level.
    pub fn set_alpha_cut_discretisation_level(&mut self, level: usize) {
        self.alpha_cut_disc_level = level;
    }

    /// Returns the number of points in the set.
    pub fn get_number_of_points(&self) -> usize {
        self.set.len()
    }

    /// Returns the membership degree (fuzzy set value) for input `x`.
    pub fn get_fs(&mut self, x: f64) -> f64 {
        if self.set.is_empty() {
            return -1.0;
        }
        if self.left_shoulder && x < self.left_shoulder_start {
            return 1.0;
        }
        if self.right_shoulder && x > self.right_shoulder_start {
            return 1.0;
        }
        let supp = self.get_support();
        if x < supp.left || x > supp.right {
            return 0.0;
        }
        self.sort();

        // Look for the first point whose x (right) value is greater than x.
        for i in 0..self.set.len() {
            if self.set[i].right > x {
                if self.debug {
                    println!("Element at {} was not contained in discretised set - interpolating.", x);
                    println!("Index = {}", i);
                    if i > 0 {
                        println!("Previous point x = {}", self.set[i - 1].right);
                    }
                    println!("Current point x = {}", self.set[i].right);
                }
                // If i is 0, we cannot interpolate; return the value at index 0.
                if i == 0 {
                    return self.set[i].left;
                }
                return self.interpolate(i - 1, x, i);
            } else if (self.set[i].right - x).abs() < std::f64::EPSILON {
                return self.set[i].left;
            }
        }
        -1.0
    }

    /// Returns the x-values where the alpha cut (given by `alpha`) intersects the set.
    /// For alpha = 0 or 1, special rules apply.
    pub fn get_alpha_cut(&mut self, alpha: f64) -> Option<Tuple> {
        if (alpha - 0.0).abs() < std::f64::EPSILON {
            return Some(self.get_support());
        }
        if (alpha - 1.0).abs() < std::f64::EPSILON {
            let mut left = 0.0;
            let mut right = 0.0;
            for p in &self.set {
                if (p.left - 1.0).abs() < std::f64::EPSILON {
                    left = p.right;
                    break;
                }
            }
            for p in self.set.iter().rev() {
                if (p.left - 1.0).abs() < std::f64::EPSILON {
                    right = p.right;
                    break;
                }
            }
            return Some(Tuple::new(left, right));
        }
        let supp = self.get_support();
        let step_size = (supp.right - supp.left) / ((self.alpha_cut_disc_level - 1) as f64);
        let mut left_val = supp.left;
        let mut current_step = supp.left;
        for _ in 0..self.alpha_cut_disc_level {
            let current = self.get_fs(current_step) - alpha;
            if current >= 0.0 {
                left_val = current_step;
                break;
            }
            current_step += step_size;
        }
        let mut right_val = supp.right;
        current_step = supp.right;
        for _ in 0..self.alpha_cut_disc_level {
            let current = self.get_fs(current_step) - alpha;
            if current >= 0.0 {
                right_val = current_step;
                break;
            }
            current_step -= step_size;
        }
        let mut alpha_cut = Tuple::new(left_val, right_val);
        if (left_val - right_val).abs() < self.alpha_cut_precision_limit {
            alpha_cut.right = left_val;
        }
        Some(alpha_cut)
    }

    /// Interpolates the membership value at x using points at indices `x0` and `x2`.
    pub fn interpolate(&self, x0: usize, x1: f64, x2: usize) -> f64 {
        let numerator = self.set[x2].right - self.set[x0].right;
        let denominator = x1 - self.set[x0].right;
        let a = numerator / denominator;
        self.set[x0].left - ((self.set[x0].left - self.set[x2].left) / a)
    }

    /// Returns a reference to the list of points (after sorting).
    pub fn get_points(&mut self) -> &Vec<Tuple> {
        self.sort();
        &self.set
    }

    /// Returns a reference to the point at the given index (after sorting).
    pub fn get_point_at(&mut self, index: usize) -> Option<&Tuple> {
        self.sort();
        self.set.get(index)
    }

    /// Returns the x-coordinate of the peak value (or the midpoint of a flat top).
    pub fn get_peak(&mut self) -> Option<f64> {
        self.sort();
        if self.set.is_empty() {
            return None;
        }
        let mut y_value_at_current_peak = self.set[0].left;
        let mut x_coordinate_of_peak = self.set[0].right;
        let mut i = 1;
        while i < self.get_number_of_points() {
            let point = &self.set[i];
            if point.left > y_value_at_current_peak {
                y_value_at_current_peak = point.left;
                x_coordinate_of_peak = point.right;
            } else if (point.left - y_value_at_current_peak).abs() < std::f64::EPSILON {
                let mut second_x = point.right;
                while i < self.get_number_of_points() &&
                      (self.set[i].left - y_value_at_current_peak).abs() < std::f64::EPSILON {
                    second_x = self.set[i].right;
                    i += 1;
                }
                return Some((x_coordinate_of_peak + second_x) / 2.0);
            }
            i += 1;
        }
        Some(x_coordinate_of_peak)
    }

    /// Returns the support of the set as a Tuple.
    pub fn get_support(&mut self) -> Tuple {
        if self.set.is_empty() {
            return Tuple::new(0.0, 0.0);
        }
        self.sort();
        if self.left_shoulder {
            Tuple::new(f64::NEG_INFINITY, self.set.last().unwrap().right)
        } else if self.right_shoulder {
            Tuple::new(self.set.first().unwrap().right, f64::INFINITY)
        } else {
            Tuple::new(self.set.first().unwrap().right, self.set.last().unwrap().right)
        }
    }

    /// Returns a string representation of all points in the set.
    pub fn to_string_rep(&mut self) -> String {
        self.sort();
        let mut s = String::new();
        for p in &self.set {
            s.push_str(&format!("{} / {}\n", p.left, p.right));
        }
        s
    }

    /// Sorts the points in the set by the x-coordinate (i.e. the `right` field).
    pub fn sort(&mut self) {
        if !self.sorted && !self.set.is_empty() {
            self.set.sort_by(|a, b| a.right.partial_cmp(&b.right).unwrap());
            // Update support based on sorted points.
            if let Some(first) = self.set.first() {
                self.support.left = first.right;
            }
            if let Some(last) = self.set.last() {
                self.support.right = last.right;
            }
            self.sorted = true;
            let mut last_x = self.set[0].right;
            let mut i = 1;
            while i < self.set.len() {
                if (self.set[i].right - last_x).abs() < std::f64::EPSILON {
                    // Merge points with the same x value.
                    self.set[i - 1].left = self.set[i - 1].left.max(self.set[i].left);
                    self.set.remove(i);
                    // Do not increment i, as we removed an element.
                } else {
                    last_x = self.set[i].right;
                    i += 1;
                }
            }
        }
    }

    /// Writes the discretised set to a file.
    pub fn write_to_file(&mut self, filename: &str) -> Result<String, String> {
        self.sort();
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .map_err(|e| format!("Error writing to output file {}: {}", filename, e))?;
        for p in &self.set {
            writeln!(file, "{},{}", p.right, p.left)
                .map_err(|e| format!("Error writing to output file {}: {}", filename, e))?;
        }
        Ok(format!("Discretised set {} was successfully written to {}", self.name, filename))
    }

    /// Writes a high-resolution view of the set to a file using interpolation.
    pub fn write_to_file_high_res(&mut self, filename: &str, resolution: usize) -> Result<String, String> {
        self.sort();
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .map_err(|e| format!("Error writing to output file {}: {}", filename, e))?;
        let supp = self.get_support();
        let step_size = (supp.right - supp.left) / ((resolution - 1) as f64);
        let mut current_step = supp.left;
        for _ in 0..resolution {
            writeln!(file, "{},{}", current_step, self.get_fs(current_step))
                .map_err(|e| format!("Error writing to output file {}: {}", filename, e))?;
            current_step += step_size;
        }
        Ok(format!("Discretised set {} was successfully written to {}", self.name, filename))
    }

    /// Sets this discretised set as a left-shoulder set.
    pub fn set_left_shoulder_set(&mut self, shoulder_start: f64) {
        self.left_shoulder = true;
        self.left_shoulder_start = shoulder_start;
        self.support.left = f64::NEG_INFINITY;
    }

    /// Sets this discretised set as a right-shoulder set.
    pub fn set_right_shoulder_set(&mut self, shoulder_start: f64) {
        self.right_shoulder = true;
        self.right_shoulder_start = shoulder_start;
        self.support.right = f64::INFINITY;
    }

    /// Computes the defuzzified centroid using the centroid algorithm.
    pub fn get_defuzzified_centroid(&mut self) -> f64 {
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        for p in self.get_points().iter() {
            numerator += p.right * p.left;
            denominator += p.left;
        }
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Unsupported method: compare_to.
    pub fn compare_to(&self, _other: &dyn T1MFPrototype) -> i32 {
        panic!("Unsupported Function")
    }
}

impl T1MFPrototype for T1MFDiscretised {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_support(&self) -> &Tuple {
        &self.support
    }

    fn is_left_shoulder(&self) -> bool {
        self.left_shoulder
    }

    fn is_right_shoulder(&self) -> bool {
        self.right_shoulder
    }
}

