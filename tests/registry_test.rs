use arc_registry::{EntryType, Registry};

fn load() -> Registry {
    Registry::load(concat!(env!("CARGO_MANIFEST_DIR"), "/data/registry.json")).expect("registry loads")
}

#[test]
fn loads_and_validates() {
    let reg = load();
    assert_eq!(reg.title, "arc_registry");
    assert!(!reg.entries.is_empty());
}

#[test]
fn handshake_count_matches_declared_total() {
    let reg = load();
    assert_eq!(reg.handshakes().count() as u32, reg.total_handshakes);
}

#[test]
fn every_tag_is_in_taxonomy() {
    let reg = load();
    for e in &reg.entries {
        for t in &e.tags {
            assert!(reg.tags.contains(t), "{} uses unknown tag {t}", e.id);
        }
    }
}

#[test]
fn ids_are_unique_and_kebab() {
    let reg = load();
    let mut seen = std::collections::HashSet::new();
    for e in &reg.entries {
        assert!(seen.insert(&e.id), "duplicate id {}", e.id);
        assert!(
            e.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "id {} is not kebab-case",
            e.id
        );
    }
}

#[test]
fn askjimmy_is_the_forge_launch() {
    let reg = load();
    let aj = reg.get("askjimmy").expect("askjimmy present");
    assert_eq!(aj.entry_type, EntryType::Handshake);
    assert!(aj.tags.iter().any(|t| t == "forge"));
    assert_eq!(aj.token.as_deref(), Some("token_live"));
}

#[test]
fn partners_have_no_token_field_semantics() {
    let reg = load();
    for p in reg.partners() {
        assert!(p.token.is_none(), "partner {} should not declare a token status", p.id);
    }
}

#[test]
fn by_tag_filters() {
    let reg = load();
    let defi: Vec<_> = reg.by_tag("defi").map(|e| e.id.as_str()).collect();
    assert!(defi.contains(&"listen-rs"));
    assert!(defi.contains(&"askjimmy"));
    assert!(!defi.contains(&"ryzome"));
}
