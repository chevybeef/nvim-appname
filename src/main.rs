use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use std::env::var;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Simple program to move the nvim XDG directories
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// options are move, restore, create, delete
    #[arg(short, long)]
    operation: String,
    /// the extension to append to the directories
    #[arg(short, long)]
    extension: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let xdg_paths = [
        "/.config/nvim",
        "/.local/share/nvim",
        "/.local/state/nvim",
        "/.cache/nvim",
    ];

    let home = var("HOME")?;
    let mut directories: Vec<PathBuf> = vec![];
    for item in &xdg_paths {
        directories.push((home.clone() + item).into());
    }

    let op = &args.operation;
    let ex = &args.extension;
    let mut from = vec![];
    let mut to = vec![];

    for directory in directories.iter() {
        if op == "move" {
            from.push(directory.clone());
            to.push(Path::new(directory).with_extension(ex));
        } else if op == "restore" || op == "create" || op == "delete" {
            from.push(Path::new(directory).with_extension(ex));
            to.push(directory.clone())
        } else {
            return Err(anyhow!("Operation not supported"));
        }
    }

    Ok(confirm_and_execute(op, &from, &to)?)
}

fn confirm_and_execute(op: &str, from: &[PathBuf], to: &[PathBuf]) -> Result<()> {
    println!("This operation will {op} these directories:");
    print_directories(from);
    if op == "move" || op == "restore" {
        println!("to here:");
        print_directories(to);
    }

    if !confirmed() {
        return Err(anyhow!("Operation cancelled"));
    }

    Ok(execute(op, from, to)?)
}

fn print_directories(dirs: &[PathBuf]) {
    dirs.iter().for_each(|d| {
        println!("{d:?}");
    });
}

fn confirmed() -> bool {
    print!("Are you sure? [y/N] ");
    let _ = io::stdout().flush();
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer.trim() == "y"
}

fn execute(op: &str, from: &[PathBuf], to: &[PathBuf]) -> Result<()> {
    for (from, to) in from.iter().zip(to.iter()) {
        if op == "move" || op == "restore" {
            fs::rename(from, to)?;
        } else if op == "create" {
            fs::create_dir(from)?;
        } else if op == "delete" {
            fs::remove_dir_all(from)?;
        }
    }

    Ok(())
}
