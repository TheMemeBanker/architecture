//! arc-registry — query the registry from the terminal.
//!
//!   arc-registry list [--tag TAG] [--type project|handshake|ecosystem_partner]
//!   arc-registry show <id>
//!   arc-registry stats
//!   arc-registry validate
//!
//! Reads data/registry.json by default; override with ARC_REGISTRY_PATH.

use arc_registry::{EntryType, Registry};
use std::process::ExitCode;

fn registry_path() -> String {
    std::env::var("ARC_REGISTRY_PATH").unwrap_or_else(|_| "data/registry.json".into())
}

fn type_label(t: &EntryType) -> &'static str {
    match t {
        EntryType::Project => "project",
        EntryType::Handshake => "handshake",
        EntryType::EcosystemPartner => "ecosystem_partner",
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");

    let reg = match Registry::load(registry_path()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    match cmd {
        "list" => {
            let tag = flag_value(&args, "--tag");
            let ty = flag_value(&args, "--type");
            for e in &reg.entries {
                if let Some(t) = &tag {
                    if !e.tags.iter().any(|x| x == t) {
                        continue;
                    }
                }
                if let Some(t) = &ty {
                    if type_label(&e.entry_type) != t {
                        continue;
                    }
                }
                let token = e.token.as_deref().unwrap_or("-");
                println!("{:<20} {:<18} {:<11} {}", e.id, type_label(&e.entry_type), token, e.tags.join(","));
            }
        }
        "show" => match args.get(1).and_then(|id| reg.get(id)) {
            Some(e) => {
                println!("{}  [{}]", e.name, type_label(&e.entry_type));
                if let Some(t) = &e.token {
                    println!("token: {t}");
                }
                if let Some(l) = &e.link {
                    println!("link:  {l}");
                }
                println!("tags:  {}", e.tags.join(", "));
                println!("\n{}", e.summary);
            }
            None => {
                eprintln!("unknown id — try `arc-registry list`");
                return ExitCode::FAILURE;
            }
        },
        "stats" => {
            println!("{}", reg.title);
            println!("entries:    {}", reg.entries.len());
            println!("handshakes: {}", reg.handshakes().count());
            println!("partners:   {}", reg.partners().count());
            println!("\ntag histogram:");
            for (tag, n) in reg.tag_counts() {
                println!("  {n:>2}  {tag}");
            }
        }
        "validate" => {
            // Registry::load already validated; getting here means it passed.
            println!("ok — {} entries, {} handshakes", reg.entries.len(), reg.total_handshakes);
        }
        _ => {
            println!("usage: arc-registry <list|show|stats|validate> [--tag TAG] [--type TYPE]");
        }
    }
    ExitCode::SUCCESS
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}
