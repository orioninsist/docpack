use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::document::read_markdown_file;
use crate::merge::merge_documents;
use crate::scanner::find_markdown_files;
use crate::splitter::split_by_tokens;
use crate::token::TokenCounter;

#[derive(Serialize)]
pub struct PackManifest {
    pub files: usize,
    pub tokens: usize,
    pub parts: usize,
}

pub fn pack_documents(input: &Path, output: &Path, max_tokens: usize) -> std::io::Result<()> {
    fs::create_dir_all(output)?;

    let files = find_markdown_files(input);

    let documents = files
        .iter()
        .filter_map(|file| read_markdown_file(file).ok())
        .collect::<Vec<_>>();

    let merged = merge_documents(&documents);

    let counter = TokenCounter::new();

    let parts = split_by_tokens(&merged, max_tokens, &counter);

    for (index, part) in parts.iter().enumerate() {
        let file = output.join(format!("part-{:04}.md", index + 1));

        fs::write(file, part)?;
    }

    let manifest = PackManifest {
        files: documents.len(),
        tokens: counter.count(&merged),
        parts: parts.len(),
    };

    let json = serde_json::to_string_pretty(&manifest).expect("manifest serialize failed");

    fs::write(output.join("manifest.json"), json)?;

    Ok(())
}
