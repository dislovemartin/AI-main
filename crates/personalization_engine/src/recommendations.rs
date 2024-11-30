/// Generates personalized recommendations based on user preferences.
///
/// # Arguments
/// * `preferences` - A slice of user preferences as strings.
/// * `max_recommendations` - Maximum number of recommendations to return.
/// * `filter` - Optional filter function to apply on preferences.
///
/// # Returns
/// A vector of recommended items as strings.
pub fn generate_recommendations<F>(
    preferences: &[String],
    max_recommendations: usize,
    filter: Option<F>,
) -> Vec<String>
where
    F: Fn(&String) -> bool,
{
    if preferences.is_empty() {
        return vec!["No preferences provided".to_string()];
    }

    let filtered_preferences = if let Some(filter_fn) = filter {
        preferences.iter().filter(|&pref| filter_fn(pref)).collect::<Vec<_>>()
    } else {
        preferences.iter().collect::<Vec<_>>()
    };

    filtered_preferences
        .iter()
        .take(max_recommendations)
        .map(|pref| format!("Recommended item for preference: {}", pref))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_recommendations_no_preferences() {
        let recommendations = generate_recommendations(&[], 5, None);
        assert_eq!(recommendations, vec!["No preferences provided"]);
    }

    #[test]
    fn test_generate_recommendations_with_filter() {
        let preferences = vec![
            "Books".to_string(),
            "Movies".to_string(),
            "Music".to_string(),
            "Games".to_string(),
        ];
        let filter = |pref: &String| pref.contains("M");
        let recommendations = generate_recommendations(&preferences, 3, Some(filter));
        assert_eq!(
            recommendations,
            vec![
                "Recommended item for preference: Movies",
                "Recommended item for preference: Music"
            ]
        );
    }

    #[test]
    fn test_generate_recommendations_limit() {
        let preferences = vec!["Item1".to_string(), "Item2".to_string(), "Item3".to_string()];
        let recommendations = generate_recommendations(&preferences, 2, None);
        assert_eq!(
            recommendations,
            vec![
                "Recommended item for preference: Item1",
                "Recommended item for preference: Item2"
            ]
        );
    }
}
