mod api;
mod cli;
mod models;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Mode};
use colored::Colorize;
use models::CrateInfo;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = api::CratesClient::new()?;

    let (label, results) = match &cli.mode {
        Mode::Search { query } => (
            format!("Search: \"{}\"", query),
            client.search(query).await?,
        ),
        Mode::Category { category } => (
            format!("Category: \"{}\"", category),
            client.by_category(category).await?,
        ),
        Mode::Keyword { keyword } => (
            format!("Keyword: \"{}\"", keyword),
            client.by_keyword(keyword).await?,
        ),
    };

    println!("\n🔎 Kết quả {} — {} crate\n", label, results.len());

    if results.is_empty() {
        println!("(không có kết quả)");
        return Ok(());
    }

    for (i, c) in results.iter().enumerate() {
        print_crate(i + 1, c);
    }

    Ok(())
}

fn print_crate(index: usize, c: &CrateInfo) {
    println!(
        "{}. {}",
        index.to_string().bold(),
        c.name.bright_cyan().bold()
    );

    if let Some(desc) = &c.description {
        // Cắt mô tả dài cho gọn
        let desc = desc.trim();
        let short = if desc.len() > 100 { &desc[..100] } else { desc };
        println!("   {}", short);
    }

    println!("   ⬇ {}  |  📦 v{}", c.downloads, c.max_version);

    if let Some(repo) = &c.repository {
        println!("   🔗 {}", repo.dimmed());
    }
    println!();
}
