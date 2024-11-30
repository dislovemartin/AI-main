use content_creation_ai::{
    media_generation::{generate_image, generate_video},
    nlg::{generate_article, summarize_text},
    personalization::{personalize_content, recommend_content},
};
use std::collections::HashMap;

#[test]
fn test_media_generation() {
    let description = "A beautiful sunset";
    assert!(generate_image(description).contains(description));
    assert!(generate_video(description).contains(description));
}

#[test]
fn test_nlg_functions() {
    let article = generate_article();
    assert!(!article.is_empty());

    let text = "This is a very long text that needs to be summarized for better readability";
    let summary = summarize_text(text);
    assert!(summary.len() < text.len());
}

#[test]
fn test_personalization() {
    let content = "Hello {{name}}!";
    let mut preferences = HashMap::new();
    preferences.insert("name".to_string(), "John".to_string());

    let personalized = personalize_content(content, &preferences);
    assert_eq!(personalized, "Hello John!");

    let interests = vec!["AI".to_string(), "Machine Learning".to_string()];
    let recommendations = recommend_content(&interests);
    assert_eq!(recommendations.len(), interests.len());
}
