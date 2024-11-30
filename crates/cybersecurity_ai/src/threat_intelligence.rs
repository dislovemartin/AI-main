use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatIntelligence {
    pub threat_signatures: HashMap<String, ThreatSignature>,
    pub last_updated: DateTime<Utc>,
    pub confidence_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatSignature {
    pub pattern: String,
    pub severity: SeverityLevel,
    pub description: String,
    pub mitigation_steps: Vec<String>,
    pub indicators: Vec<String>,
    pub false_positive_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SeverityLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl ThreatIntelligence {
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            threat_signatures: HashMap::new(),
            last_updated: Utc::now(),
            confidence_threshold,
        }
    }

    pub fn add_signature(&mut self, name: String, signature: ThreatSignature) {
        self.threat_signatures.insert(name, signature);
        self.last_updated = Utc::now();
    }

    pub fn match_threat_patterns(&self, data: &str) -> Vec<(String, &ThreatSignature)> {
        self.threat_signatures
            .iter()
            .filter(|(_, sig)| {
                data.contains(&sig.pattern) && sig.false_positive_rate < self.confidence_threshold
            })
            .map(|(name, sig)| (name.clone(), sig))
            .collect()
    }

    pub fn get_mitigation_plan(&self, threat_name: &str) -> Option<&Vec<String>> {
        self.threat_signatures
            .get(threat_name)
            .map(|sig| &sig.mitigation_steps)
    }
}
