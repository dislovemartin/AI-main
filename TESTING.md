# Testing Guide

This guide provides instructions on how to run and write tests for the project.

## Running Tests

To execute all tests across the workspace, use the following command:

```bash
cargo test --workspace
```

### Running Clippy

To perform lint checks using Clippy:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Code Coverage

To measure code coverage, you can use tools like `tarpaulin`.

1. **Install Tarpaulin**

   ```bash
   cargo install cargo-tarpaulin
   ```

2. **Run Coverage**

   ```bash
   cargo tarpaulin --workspace
   ```

## Writing Tests

### Unit Tests

Place unit tests within the same file as the code they are testing, inside a `#[cfg(test)]` module.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Create integration tests in the `tests/` directory.

```rust:tests/integration_test.rs
use your_crate::your_module::your_function;

#[test]
fn test_your_function() {
    let result = your_function();
    assert_eq!(result, expected_value);
}
```

### Async Tests

Use `#[tokio::test]` for asynchronous tests.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

## Best Practices

- Write tests for public APIs.
- Cover edge cases and error conditions.
- Keep tests isolated and independent.
``` 
</rewritten_file>