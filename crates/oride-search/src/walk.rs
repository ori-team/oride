//! Fallback: walk com `ignore` + regex/literal.

use std::fs;
use std::path::Path;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use regex::RegexBuilder;

use crate::{SearchError, SearchHit, SearchQuery};

const SKIP_DIR_NAMES: &[&str] = &["target", "node_modules", ".git", "dist", "build", ".oride"];

pub fn search_walk(
    root: &Path,
    query: &SearchQuery,
    max_hits: usize,
) -> Result<Vec<SearchHit>, SearchError> {
    let re = build_matcher(&query.pattern, query.case_sensitive, query.use_regex)?;
    let mut hits = Vec::new();

    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                return !SKIP_DIR_NAMES.iter().any(|s| *s == name);
            }
            true
        });

    if let Some(ref glob) = query.file_glob {
        let trimmed = glob.trim();
        if !trimmed.is_empty() {
            let mut override_builder = OverrideBuilder::new(root);
            if override_builder.add(trimmed).is_ok() {
                if let Ok(overrides) = override_builder.build() {
                    builder.overrides(overrides);
                }
            }
        }
    }

    let walker = builder.build();

    for entry in walker.flatten() {
        if hits.len() >= max_hits {
            break;
        }
        let path = entry.path();
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        if !is_textish(path) {
            continue;
        }
        let Ok(content) = fs::read_to_string(path) else {
            continue; // binário / invalid utf8
        };
        search_file(path, &content, &re, max_hits, &mut hits);
    }

    Ok(hits)
}

pub(crate) fn build_matcher(
    pattern: &str,
    case_sensitive: bool,
    use_regex: bool,
) -> Result<regex::Regex, SearchError> {
    let pat = if use_regex {
        pattern.to_string()
    } else {
        regex::escape(pattern)
    };
    RegexBuilder::new(&pat)
        .case_insensitive(!case_sensitive)
        .multi_line(false)
        .build()
        .map_err(|e| SearchError::Regex(e.to_string()))
}

fn search_file(
    path: &Path,
    content: &str,
    re: &regex::Regex,
    max_hits: usize,
    hits: &mut Vec<SearchHit>,
) {
    for (i, line) in content.lines().enumerate() {
        if hits.len() >= max_hits {
            return;
        }
        if let Some(m) = re.find(line) {
            let col = line[..m.start()].chars().count() + 1;
            hits.push(SearchHit {
                path: path.to_path_buf(),
                line: i + 1,
                column: col,
                line_text: line.to_string(),
            });
            // um hit por linha (como rg default-ish na UI)
        }
    }
}

pub(crate) fn is_textish(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        return matches!(
            name.as_str(),
            "readme"
                | "license"
                | "makefile"
                | "dockerfile"
                | "cargo.toml"
                | "cargo.lock"
                | "changelog"
                | "authors"
                | "copying"
        );
    };
    let e = ext.to_ascii_lowercase();
    matches!(
        e.as_str(),
        "oris"
            | "rs"
            | "d"
            | "di"
            | "lua"
            | "md"
            | "mdx"
            | "txt"
            | "toml"
            | "json"
            | "yaml"
            | "yml"
            | "html"
            | "htm"
            | "css"
            | "js"
            | "ts"
            | "tsx"
            | "jsx"
            | "sh"
            | "bash"
            | "zsh"
            | "py"
            | "c"
            | "h"
            | "cpp"
            | "go"
            | "java"
            | "kt"
            | "swift"
            | "rb"
            | "php"
            | "sql"
            | "xml"
            | "svg"
            | "csv"
            | "env"
            | "lock"
            | "gitignore"
            | "editorconfig"
    )
}
