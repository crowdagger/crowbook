/// Equivalent to assert_eq! but with prettier output
pub fn test_eq(actual: &str, expected: &str) {
    if actual != expected {
        panic!(format!("\nexpected:\n{}\nactual:\n{}\n", expected, actual));
    }
}

mod toc;
mod escape;
