use std::collections::HashMap;

/// Customizes content based on user preferences.
///
/// # Arguments
/// * `content` - The base content to customize.
/// * `preferences` - A hashmap of user preferences.
///
/// # Returns
/// The customized content.
pub fn personalize_content(content: &str, preferences: &HashMap<String, String>) -> String {
    let mut personalized_content = content.to_string();
    for (key, value) in preferences {
        personalized_content = personalized_content.replace(&format!("{{{{{}}}}}", key), value);
    }
    personalized_content
}

/// Recommends content based on user interests.
///
/// # Arguments
/// * `interests` - A list of user interests.
///
/// # Returns
/// A list of recommended topics.
pub fn recommend_content(interests: &[String]) -> Vec<String> {
    interests.iter().map(|interest| format!("Explore more about {}!", interest)).collect()
}
