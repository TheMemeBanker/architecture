//! Typed access to the arc_registry data set.
//!
//! The registry tracks three relationship types — projects, handshakes, and
//! ecosystem partners — sourced from `data/registry.json`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub title: String,
    pub total_handshakes: u32,
    pub types: BTreeMap<String, String>,
    pub tags: Vec<String>,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    Project,
    Handshake,
    EcosystemPartner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub entry_type: EntryType,
    #[serde(default)]
    pub token: Option<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub link: Option<String>,
    pub summary: String,
}

/// Error type kept dependency-light: Display and Error are implemented by hand.
#[derive(Debug)]
pub enum RegistryError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Invalid(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::Io(e) => write!(f, "io error: {e}"),
            RegistryError::Parse(e) => write!(f, "parse error: {e}"),
            RegistryError::Invalid(msg) => write!(f, "invalid registry: {msg}"),
        }
    }
}
impl std::error::Error for RegistryError {}
impl From<std::io::Error> for RegistryError {
    fn from(e: std::io::Error) -> Self {
        RegistryError::Io(e)
    }
}
impl From<serde_json::Error> for RegistryError {
    fn from(e: serde_json::Error) -> Self {
        RegistryError::Parse(e)
    }
}

impl Registry {
    /// Load and validate a registry from a JSON file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let raw = std::fs::read_to_string(path)?;
        let reg: Registry = serde_json::from_str(&raw)?;
        reg.validate()?;
        Ok(reg)
    }

    /// Structural invariants that the JSON schema cannot express alone.
    pub fn validate(&self) -> Result<(), RegistryError> {
        let mut seen = std::collections::BTreeSet::new();
        for e in &self.entries {
            if !seen.insert(&e.id) {
                return Err(RegistryError::Invalid(format!("duplicate entry id: {}", e.id)));
            }
            if e.tags.is_empty() {
                return Err(RegistryError::Invalid(format!("entry {} has no tags", e.id)));
            }
            for t in &e.tags {
                if !self.tags.contains(t) {
                    return Err(RegistryError::Invalid(format!(
                        "entry {} uses tag '{t}' missing from the taxonomy",
                        e.id
                    )));
                }
            }
        }
        let handshakes = self.handshakes().count() as u32;
        if handshakes != self.total_handshakes {
            return Err(RegistryError::Invalid(format!(
                "total_handshakes says {} but {} entries carry the handshake type",
                self.total_handshakes, handshakes
            )));
        }
        Ok(())
    }

    pub fn handshakes(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().filter(|e| e.entry_type == EntryType::Handshake)
    }

    pub fn partners(&self) -> impl Iterator<Item = &Entry> {
        self.entries
            .iter()
            .filter(|e| e.entry_type == EntryType::EcosystemPartner)
    }

    pub fn by_tag<'a>(&'a self, tag: &'a str) -> impl Iterator<Item = &'a Entry> {
        self.entries.iter().filter(move |e| e.tags.iter().any(|t| t == tag))
    }

    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Tag histogram across all entries, descending by count.
    pub fn tag_counts(&self) -> Vec<(String, usize)> {
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for e in &self.entries {
            for t in &e.tags {
                *counts.entry(t.as_str()).or_default() += 1;
            }
        }
        let mut out: Vec<(String, usize)> = counts.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        out
    }
}
