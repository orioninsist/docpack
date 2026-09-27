use clap::{Parser, Subcommand};
use std::path::PathBuf;

use docpack::document::read_markdown_file;
use docpack::inspect::inspect_documents;
use docpack::merge::merge_documents;
use docpack::output::write_markdown;
use docpack::pack::pack_documents;
use docpack::progress::create_progress;
use docpack::scanner::find_markdown_files;
use docpack::splitter::split_by_tokens;
use docpack::token::TokenCounter;

#[derive(Parser, Debug)]
#[command(
    name = "docpack",
    version,
    about = "Markdown document packager for AI projects"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Pack {
        #[arg(long)]
        input: PathBuf,

        #[arg(long)]
        output: PathBuf,

        #[arg(long, default_value_t = 12000)]
        max_tokens: usize,
    },

    Inspect {
        #[arg(long)]
        input: PathBuf,
    },

    Merge {
        #[arg(long)]
        input: PathBuf,

        #[arg(long)]
        output: PathBuf,

        #[arg(long, default_value_t = 12000)]
        max_tokens: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pack {
            input,
            output,
            max_tokens,
        } => {
            pack_documents(&input, &output, max_tokens).expect("pack failed");

            println!("pack complete");
        }

        Commands::Inspect { input } => {
            let report = inspect_documents(&input);

            println!("Inspect Report");
            println!("--------------");
            println!("Files:      {}", report.files);
            println!("Bytes:      {}", report.bytes);
            println!("Tokens:     {}", report.tokens);
            println!("Duplicates: {}", report.duplicates);
        }

        Commands::Merge {
            input,
            output,
            max_tokens,
        } => {
            let files = find_markdown_files(&input);

            let progress = create_progress(files.len(), "Reading documents");

            let documents = files
                .iter()
                .filter_map(|file| {
                    let result = read_markdown_file(file).ok();
                    progress.inc(1);
                    result
                })
                .collect::<Vec<_>>();

            progress.finish_with_message("Documents loaded");

            let merge_progress = create_progress(1, "Merging documents");
            let merged = merge_documents(&documents);
            merge_progress.finish_with_message("Merge complete");

            let counter = TokenCounter::new();

            let split_progress = create_progress(1, "Splitting tokens");
            let parts = split_by_tokens(&merged, max_tokens, &counter);
            split_progress.finish_with_message("Split complete");

            let final_content = parts.join("\n\n---\n\n");

            let output_progress = create_progress(1, "Writing output");
            write_markdown(&output, &final_content).expect("failed to write output");
            output_progress.finish_with_message("Output written");

            println!(
                "done files={} parts={} tokens={}",
                documents.len(),
                parts.len(),
                counter.count(&final_content)
            );
        }
    }
}
