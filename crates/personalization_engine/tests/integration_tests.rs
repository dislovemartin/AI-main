use personalization_engine::{
    behavior_analysis::analyze_behavior, dynamic_content::adjust_content,
    recommendations::generate_recommendations,
};

#[test]
fn test_recommendations() {
    let preferences = vec!["Books".to_string(), "Movies".to_string(), "Music".to_string()];
    let max_recommendations = 2;
    let filter = Some(|pref: &String| pref.starts_with('M'));

    let recommendations = generate_recommendations(&preferences, max_recommendations, filter);
    assert_eq!(recommendations.len(), 2);
    assert!(recommendations.iter().all(|r| r.contains("Movies") || r.contains("Music")));
}

#[test]
fn test_behavior_analysis() {
    let data = vec!["click".to_string(), "scroll".to_string(), "click".to_string()];
    let analysis = analyze_behavior(&data);
    assert!(analysis.contains("click: 2 occurrences"));
}

#[test]
fn test_dynamic_content() {
    let content = "Welcome to our platform";

    let dark_mode = adjust_content(content, "dark_mode").unwrap();
    assert!(dark_mode.contains("Dark Mode"));

    let empty_content = adjust_content("", "dark_mode");
    assert!(empty_content.is_err());
}
