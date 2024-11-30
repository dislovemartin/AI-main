use std::collections::HashMap;

/// Analyzes user behavior patterns based on interaction data.
///
/// # Arguments
/// * `data` - A vector of user interaction logs as strings.
///
/// # Returns
/// A summary of identified behavior patterns, including counts of each type of interaction.
pub fn analyze_behavior(data: &[String]) -> String {
    if data.is_empty() {
        return "No interaction logs provided.".to_string();
    }

    let mut interaction_counts: HashMap<&str, usize> = HashMap::new();

    // Count occurrences of each interaction type
    for log in data {
        *interaction_counts.entry(log.as_str()).or_insert(0) += 1;
    }

    // Generate a summary report
    let mut summary = String::new();
    summary.push_str(
        "Behavior analysis summary:
",
    );
    for (interaction, count) in interaction_counts {
        summary.push_str(&format!(
            "  {}: {} occurrences
",
            interaction, count
        ));
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_behavior_empty_data() {
        let result = analyze_behavior(&[]);
        assert_eq!(result, "No interaction logs provided.");
    }

    #[test]
    fn test_analyze_behavior_single_interaction() {
        let data = vec!["click".to_string()];
        let result = analyze_behavior(&data);
        assert!(result.contains("click: 1 occurrences"));
    }

    #[test]
    fn test_analyze_behavior_multiple_interactions() {
        let data = vec![
            "click".to_string(),
            "scroll".to_string(),
            "click".to_string(),
            "hover".to_string(),
        ];
        let result = analyze_behavior(&data);
        assert!(result.contains("click: 2 occurrences"));
        assert!(result.contains("scroll: 1 occurrences"));
        assert!(result.contains("hover: 1 occurrences"));
    }
}
