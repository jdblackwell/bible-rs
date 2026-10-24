//! Local OWL bridge: use bible-rs's core directly and emit JSON for the launcher.
use anyhow::{Context, Result, bail};
use bible_core::{library::Library, reference, search};
use serde_json::{Value, json};
use std::sync::Arc;

fn execute() -> Result<Value> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("read");
    let id = args.get(1).map(String::as_str).unwrap_or("web");
    let input = args.get(2).map(String::as_str).unwrap_or("John 3:16");
    let dir = std::env::var_os("BIBLE_RS_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| dirs::data_dir().unwrap_or_else(std::env::temp_dir).join("bible-rs"));
    let library = Library::open(&dir, Arc::new(|| {}));
    let t = library.load(id)?;
    match command {
        "read" => {
            let r = reference::parse(input).context("Enter a reference such as John 3:16 or Psalm 23")?;
            let chapter = r.chapter_idx();
            let verses: Vec<Value> = t.verses_in(chapter).map(|v| {
                let selected = r.verse.is_some_and(|start| v.number_end >= start && v.number <= r.verse_end.unwrap_or(start));
                json!({"number": v.number, "ref": v.reference().to_string(), "text": v.plain_text(), "selected": selected})
            }).collect();
            if verses.is_empty() {
                bail!("This chapter is unavailable in {}", t.meta.abbrev);
            }
            if r.verse.is_some() && !verses.iter().any(|v| v["selected"] == true) {
                bail!("That verse is unavailable in {}", t.meta.abbrev);
            }
            let selected: Vec<&str> = verses
                .iter()
                .filter(|v| r.verse.is_none() || v["selected"] == true)
                .filter_map(|v| v["text"].as_str())
                .collect();
            let preview = selected.join(" ");
            Ok(json!({"ref": r.display(), "chapter": chapter.display(),
                "translation": t.meta.abbrev, "preview": preview, "verses": verses,
                "previous": chapter.prev().map(|c| c.display()), "next": chapter.next().map(|c| c.display())}))
        }
        "search" => {
            if input.trim().is_empty() {
                return Ok(json!({"total": 0, "results": []}));
            }
            let found = search::run(&t, input, search::Scope::All, false);
            let results: Vec<Value> = found
                .verses
                .iter()
                .take(60)
                .filter_map(|&index| {
                    let v = t.verse_view(index)?;
                    Some(json!({"ref": v.reference().to_string(), "text": v.plain_text()}))
                })
                .collect();
            Ok(json!({"total": found.verses.len(), "results": results}))
        }
        _ => bail!("Use read or search"),
    }
}

fn main() {
    let value = execute().unwrap_or_else(|e| json!({"error": format!("{e:#}")}));
    println!("{value}");
}
