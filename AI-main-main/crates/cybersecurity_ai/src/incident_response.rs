/// Automates responses to detected threats.
///
/// # Arguments
/// * `threats` - A list of detected threats.
///
/// # Returns
/// A vector of responses for each detected threat.
pub fn respond_to_incidents(threats: &[String]) -> Vec<String> {
    threats
        .iter()
        .map(|threat| format!("Response initiated for threat: {}", threat))
        .collect()
}
