//! Substituição de texto em arquivo individual e no projeto como um todo.

use std::fs;
use std::path::{Path, PathBuf};

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;

use crate::walk::{build_matcher, is_textish};
use crate::{SearchError, SearchQuery};

/// Resumo com contadores de substituições realizadas no projeto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReplaceSummary {
    pub files_modified: usize,
    pub replacements_count: usize,
}

/// Substitui ocorrências do padrão em um único arquivo de forma atômica no disco.
///
/// Retorna a quantidade de substituições efetuadas no arquivo.
pub fn replace_in_file(
    path: &Path,
    query: &SearchQuery,
    replacement: &str,
) -> Result<usize, SearchError> {
    if query.pattern.is_empty() {
        return Err(SearchError::EmptyPattern);
    }
    let content = fs::read_to_string(path)?;
    let matcher = build_matcher(&query.pattern, query.case_sensitive, query.use_regex)?;
    let match_count = matcher.find_iter(&content).count();
    if match_count == 0 {
        return Ok(0);
    }

    let modified_content = if query.use_regex {
        matcher.replace_all(&content, replacement)
    } else {
        matcher.replace_all(&content, regex::NoExpand(replacement))
    };

    fs::write(path, modified_content.as_bytes())?;
    Ok(match_count)
}

/// Substitui ocorrências em todos os arquivos de texto rastreados no projeto.
///
/// Retorna um par contendo o resumo numérico e a lista de caminhos dos arquivos modificados.
pub fn replace_in_project(
    root: &Path,
    query: &SearchQuery,
    replacement: &str,
) -> Result<(ReplaceSummary, Vec<PathBuf>), SearchError> {
    if query.pattern.trim().is_empty() {
        return Err(SearchError::EmptyPattern);
    }
    let matcher = build_matcher(&query.pattern, query.case_sensitive, query.use_regex)?;

    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| {
            let file_name = entry.file_name().to_string_lossy();
            if entry
                .file_type()
                .map(|file_type| file_type.is_dir())
                .unwrap_or(false)
            {
                return !["target", "node_modules", ".git", "dist", "build", ".oride"]
                    .iter()
                    .any(|ignored| *ignored == file_name);
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

    let mut summary = ReplaceSummary::default();
    let mut modified_paths = Vec::new();

    for entry in walker.flatten() {
        let path = entry.path();
        if !entry
            .file_type()
            .map(|file_type| file_type.is_file())
            .unwrap_or(false)
        {
            continue;
        }
        if !is_textish(path) {
            continue;
        }
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        let match_count = matcher.find_iter(&content).count();
        if match_count == 0 {
            continue;
        }

        let modified_content = if query.use_regex {
            matcher.replace_all(&content, replacement)
        } else {
            matcher.replace_all(&content, regex::NoExpand(replacement))
        };

        if fs::write(path, modified_content.as_bytes()).is_ok() {
            summary.files_modified += 1;
            summary.replacements_count += match_count;
            modified_paths.push(path.to_path_buf());
        }
    }

    Ok((summary, modified_paths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_in_file_literal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("code.rs");
        fs::write(&file_path, "let old_name = 10; println!(\"{}\", old_name);").unwrap();

        let query = SearchQuery {
            pattern: "old_name".into(),
            case_sensitive: true,
            use_regex: false,
            max_hits: 100,
            ..Default::default()
        };
        let count = replace_in_file(&file_path, &query, "new_name").unwrap();
        assert_eq!(count, 2);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "let new_name = 10; println!(\"{}\", new_name);");
    }

    #[test]
    fn test_replace_in_file_preserves_dollar_signs_literal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("price.txt");
        fs::write(&file_path, "cost is PRICE USD").unwrap();

        let query = SearchQuery {
            pattern: "PRICE".into(),
            case_sensitive: true,
            use_regex: false,
            max_hits: 100,
            ..Default::default()
        };
        let count = replace_in_file(&file_path, &query, "$100").unwrap();
        assert_eq!(count, 1);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "cost is $100 USD");
    }

    #[test]
    fn test_replace_in_project_multiple_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_a = temp_dir.path().join("a.txt");
        let file_b = temp_dir.path().join("b.rs");
        let ignored_dir = temp_dir.path().join("target");
        fs::create_dir_all(&ignored_dir).unwrap();
        let file_ignored = ignored_dir.join("c.txt");

        fs::write(&file_a, "target_word in first file").unwrap();
        fs::write(&file_b, "target_word and target_word again").unwrap();
        fs::write(&file_ignored, "target_word in target folder").unwrap();

        let query = SearchQuery {
            pattern: "target_word".into(),
            case_sensitive: true,
            use_regex: false,
            max_hits: 100,
            ..Default::default()
        };
        let (summary, modified) =
            replace_in_project(temp_dir.path(), &query, "replacement_word").unwrap();
        assert_eq!(summary.files_modified, 2);
        assert_eq!(summary.replacements_count, 3);
        assert_eq!(modified.len(), 2);

        let content_a = fs::read_to_string(&file_a).unwrap();
        let content_b = fs::read_to_string(&file_b).unwrap();
        let content_ignored = fs::read_to_string(&file_ignored).unwrap();

        assert_eq!(content_a, "replacement_word in first file");
        assert_eq!(content_b, "replacement_word and replacement_word again");
        assert_eq!(content_ignored, "target_word in target folder");
    }

    #[test]
    fn test_replace_in_project_with_glob_filter() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_txt = temp_dir.path().join("doc.txt");
        let file_rs = temp_dir.path().join("lib.rs");

        fs::write(&file_txt, "token_match in txt").unwrap();
        fs::write(&file_rs, "token_match in rs").unwrap();

        let query = SearchQuery {
            pattern: "token_match".into(),
            file_glob: Some("*.rs".into()),
            ..Default::default()
        };
        let (summary, modified) =
            replace_in_project(temp_dir.path(), &query, "token_replaced").unwrap();
        assert_eq!(summary.files_modified, 1);
        assert_eq!(modified.len(), 1);
        assert_eq!(modified[0], file_rs);

        let content_txt = fs::read_to_string(&file_txt).unwrap();
        let content_rs = fs::read_to_string(&file_rs).unwrap();
        assert_eq!(content_txt, "token_match in txt");
        assert_eq!(content_rs, "token_replaced in rs");
    }
}
