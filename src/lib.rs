use std::process::Command;
use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub name: String,
    pub expanded: String,
}

pub fn parse_aliases(raw_aliases: &[String]) -> Vec<Alias> {
    let mut aliases = Vec::new();
    for line in raw_aliases {
        if let Some((alias, expanded)) = line.split_once('=') {
            let alias = alias.trim().trim_matches('\'').to_string();
            let expanded = expanded.trim().trim_matches('\'').to_string();
            if !alias.is_empty() && !expanded.is_empty() {
                aliases.push(Alias { name: alias, expanded });
            }
        }
    }
    aliases
}

pub fn split_input(input: &[String]) -> (Vec<String>, Vec<String>) {
    let mut aliases = Vec::new();
    let mut functions = Vec::new();
    
    for line in input {
        let line = line.trim();
        if line.ends_with(" () {") {
            functions.push(line[..line.len() - 5].trim().to_string());
        } else if line.contains('=') {
            aliases.push(line.to_string());
        }
    }
    
    (aliases, functions)
}

pub fn find_alias(aliases: &[Alias], input: &str) -> String {
    let mut aliases = aliases.to_vec();
    aliases.sort_by(|a, b| b.expanded.len().cmp(&a.expanded.len()));

    let mut result = input.to_string();
    let mut prev_result;

    loop {
        prev_result = result.clone();
        for alias in &aliases {
            if result == alias.expanded || result.starts_with(&format!("{} ", alias.expanded)) {
                let idx = alias.expanded.len();
                result = format!("{}{}", alias.name, &result[idx..]);
            }
        }
        if prev_result == result {
            break;
        }
    }

    result
}

pub fn expand_input(input: &str, aliases: &[Alias]) -> String {
    let mut max_exp = 0;
    let mut max_expanded = None;

    for alias in aliases {
        if input.starts_with(&format!("{} ", alias.name)) 
            && alias.expanded.len() > alias.name.len() 
            && alias.expanded.len() > max_exp {
            max_expanded = Some(input.replacen(&alias.name, &alias.expanded, 1));
            max_exp = alias.expanded.len();
        }
    }

    max_expanded.unwrap_or_else(|| input.to_string())
}

pub fn get_git_aliases() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["config", "--get-regexp", "^alias\\."])
        .output()
        .context("Failed to execute git command")?;

    let stdout = String::from_utf8(output.stdout)
        .context("Failed to parse git output as UTF-8")?;

    let re = Regex::new(r"^alias\.([^ ]+) (.+)$").unwrap();
    Ok(stdout
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|caps| {
                format!(
                    "git {} = {}",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str().trim_start_matches('!')
                )
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_aliases() {
        let input = vec![
            "foo='bar'".to_string(),
            "git st='git status -sb'".to_string(),
        ];
        let expected = vec![
            Alias { name: "foo".to_string(), expanded: "bar".to_string() },
            Alias { name: "git st".to_string(), expanded: "git status -sb".to_string() },
        ];
        assert_eq!(parse_aliases(&input), expected);
    }

    #[test]
    fn test_split_input() {
        let input = vec![
            "foo () {".to_string(),
            "}".to_string(),
            "foo=bar".to_string(),
        ];
        let (aliases, functions) = split_input(&input);
        assert_eq!(aliases, vec!["foo=bar"]);
        assert_eq!(functions, vec!["foo"]);
    }

    #[test]
    fn test_find_alias() {
        let aliases = vec![
            Alias { name: "g".to_string(), expanded: "git".to_string() },
            Alias { name: "git st".to_string(), expanded: "git status -sb".to_string() },
        ];
        assert_eq!(find_alias(&aliases, "git status -sb"), "g st");
    }

    #[test]
    fn test_find_alias_with_suffix() {
        let aliases = vec![
            Alias { name: "f".to_string(), expanded: "bar".to_string() },
        ];
        assert_eq!(find_alias(&aliases, "bar -v"), "f -v");
    }

    #[test]
    fn test_expand_input() {
        let aliases = vec![
            Alias { name: "gRv".to_string(), expanded: "git remote -v".to_string() },
            Alias { name: "gR".to_string(), expanded: "git remote".to_string() },
        ];
        assert_eq!(expand_input("gR -v", &aliases), "git remote -v");
    }

    #[test]
    fn test_no_expand() {
        let aliases = vec![
            Alias { name: "gRv".to_string(), expanded: "git remote -v".to_string() },
        ];
        assert_eq!(expand_input("gR -v", &aliases), "gR -v");
    }
}
