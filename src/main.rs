use anyhow::Result;
use clap::Parser;
use colored::*;
use std::io::{self, BufRead};
use zsh_alias_tips_rs::{expand_input, find_alias, get_git_aliases, parse_aliases, split_input};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Command to check for aliases")]
    command: String,
}

fn format_tip(alias: &str, prefix: &str) -> String {
    format!("{}{}", prefix.blue(), alias.bright_blue())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prefix = std::env::var("ZSH_PLUGINS_ALIAS_TIPS_TEXT").unwrap_or_else(|_| "Alias tip: ".to_string());
    let expand = std::env::var("ZSH_PLUGINS_ALIAS_TIPS_EXPAND").unwrap_or_else(|_| "1".to_string()) == "1";
    let excludes = std::env::var("ZSH_PLUGINS_ALIAS_TIPS_EXCLUDES").unwrap_or_default();
    let force = std::env::var("ZSH_PLUGINS_ALIAS_TIPS_FORCE").unwrap_or_else(|_| "0".to_string()) == "1";

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let (raw_aliases, functions) = split_input(&lines);

    // Don't suggest alias for functions
    if functions.iter().any(|f| f == &args.command) {
        std::process::exit(1);
    }

    let mut all_aliases = raw_aliases;
    if let Ok(git_aliases) = get_git_aliases() {
        all_aliases.extend(git_aliases);
    }

    let aliases = parse_aliases(&all_aliases);
    let excluded: Vec<&str> = excludes.split_whitespace().collect();
    let aliases: Vec<_> = aliases
        .into_iter()
        .filter(|a| !excluded.contains(&a.name.as_str()))
        .collect();

    let input = if expand {
        expand_input(&args.command, &aliases)
    } else {
        args.command.clone()
    };

    let alias = find_alias(&aliases, &input);
    if alias.len() < input.len() && alias != input {
        println!("{}", format_tip(&alias, &prefix));
        if force {
            std::process::exit(10);
        }
    } else {
        std::process::exit(1);
    }

    Ok(())
}
