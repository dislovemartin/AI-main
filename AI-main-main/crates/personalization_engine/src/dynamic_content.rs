/// Dynamically adjusts content delivery based on user context.
///
/// # Arguments
/// * `content` - The base content to adjust.
/// * `context` - The user's current context.
///
/// # Returns
/// Adjusted content as a string or an error message if inputs are invalid.
pub fn adjust_content(content: &str, context: &str) -> Result<String, String> {
    if content.trim().is_empty() {
        return Err("Content cannot be empty.".to_string());
    }
    if context.trim().is_empty() {
        return Err("Context cannot be empty.".to_string());
    }

    // Example: Contextual adjustments
    let adjusted_content = match context {
        "dark_mode" => format!("{} (Adjusted for Dark Mode)", content),
        "light_mode" => format!("{} (Adjusted for Light Mode)", content),
        "en_US" => format!("{} (Localized for English - US)", content),
        "es_ES" => format!("{} (Localized for Spanish - ES)", content),
        _ => format!("{} (Adjusted for context: {})", content, context),
    };

    Ok(adjusted_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_content_valid() {
        let result = adjust_content("Welcome", "dark_mode").unwrap();
        assert_eq!(result, "Welcome (Adjusted for Dark Mode)");
    }

    #[test]
    fn test_adjust_content_empty_content() {
        let result = adjust_content("", "dark_mode");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Content cannot be empty.");
    }

    #[test]
    fn test_adjust_content_empty_context() {
        let result = adjust_content("Welcome", "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Context cannot be empty.");
    }

    #[test]
    fn test_adjust_content_unknown_context() {
        let result = adjust_content("Welcome", "custom_context").unwrap();
        assert_eq!(result, "Welcome (Adjusted for context: custom_context)");
    }
}
