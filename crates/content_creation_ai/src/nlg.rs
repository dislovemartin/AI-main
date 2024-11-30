use rand::seq::SliceRandom;
use rand::thread_rng;

/// Generates a simple article based on a given topic.
///
/// # Arguments
/// * `topic` - The topic of the article.
///
/// # Returns
/// A generated article as a String.
pub fn generate_article() -> String {
    let mut rng = thread_rng();

    let sentences = vec![
        "Sentence one.".to_string(),
        "Sentence two.".to_string(),
        "Sentence three.".to_string(),
        // Add more sentences as needed
    ];

    let article: Vec<_> = sentences.choose_multiple(&mut rng, 3).cloned().collect();

    article.join(" ")
}

/// Summarizes a given text.
///
/// # Arguments
/// * `text` - The text to summarize.
///
/// # Returns
/// A summarized version of the text.
pub fn summarize_text(text: &str) -> String {
    if text.len() < 50 {
        return text.to_string(); // Return as-is if text is short
    }
    let words: Vec<&str> = text.split_whitespace().collect();
    let half_len = words.len() / 2;
    words[..half_len].join(" ") + " ..."
}
