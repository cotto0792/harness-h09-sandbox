//! Synthetic service for agent-harness smoke tests. Contains no client code or data.

/// Returns the greeting the smoke tests check for.
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("harness"), "Hello, harness!");
    }
}
