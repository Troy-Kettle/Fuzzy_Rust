// src/main.rs

mod type1 {
    pub mod sets {
        pub mod t1mf_gaussian;
        pub mod t1mf_discretised;  // Ensure this file is named t1mf_discretised.rs
    }
}

use type1::sets::t1mf_discretised::T1MFDiscretised;
use type1::sets::t1mf_gaussian::Tuple;

fn main() {
    println!("--- Testing Discretised Membership Function ---");

    // Create sample points for a triangular MF:
    // Points are in (y, x) order.
    let points = vec![
        Tuple::new(0.0, -1.0),
        Tuple::new(1.0,  0.0),
        Tuple::new(0.0,  1.0),
    ];

    // Create a new T1MFDiscretised instance with these points.
    let mut mf = T1MFDiscretised::new("TriangularMF".to_string(), Some(points));

    // Test get_fs for several x values.
    let test_values = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    for x in test_values {
        let fs = mf.get_fs(x);
        println!("get_fs({}) = {}", x, fs);
    }

    // Test alpha-cuts for alpha = 0.0, 0.5, and 1.0.
    let alphas = vec![0.0, 0.5, 1.0];
    for alpha in alphas {
        match mf.get_alpha_cut(alpha) {
            Some(alpha_cut) => {
                println!("Alpha cut for alpha {}: [{}, {}]", alpha, alpha_cut.left, alpha_cut.right);
            },
            None => {
                println!("Alpha cut for alpha {}: None", alpha);
            }
        }
    }

    // Retrieve and print the peak x-coordinate.
    match mf.get_peak() {
        Some(peak) => println!("Peak x-coordinate: {}", peak),
        None => println!("No peak found."),
    }

    // Compute and print the defuzzified centroid.
    let centroid = mf.get_defuzzified_centroid();
    println!("Defuzzified centroid: {}", centroid);

    // Finally, print a string representation of the full set.
    println!("Set representation:\n{}", mf.to_string_rep());
}

