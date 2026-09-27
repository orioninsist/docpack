use std::path::Path;

use serde::Serialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::token::TokenCounter;

#[derive(Debug, Serialize)]
pub struct InspectReport {
    pub files: usize,
    pub bytes: u64,
    pub tokens: usize,
    pub duplicates: usize,
}

pub fn inspect_documents(root: &Path) -> InspectReport {
    let counter = TokenCounter::new();

    let mut files = 0;
    let mut bytes = 0;
    let mut tokens = 0;
    let mut hashes: Vec<String> = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if entry.path().extension().is_some_and(|ext| ext == "md")
            && let Ok(content) = std::fs::read_to_string(entry.path())
        {
            files += 1;
            bytes += content.len() as u64;
            tokens += counter.count(&content);

            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());

            let digest = hasher.finalize();

            hashes.push(digest.iter().map(|byte| format!("{:02x}", byte)).collect());
        }
    }

    hashes.sort();

    let duplicates = hashes.windows(2).filter(|pair| pair[0] == pair[1]).count();

    InspectReport {
        files,
        bytes,
        tokens,
        duplicates,
    }
}
