//! Tipos de domínio (independentes de `lsp-types` para manter o crate leve).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverInfo {
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

/// Converte uma coluna em escalares Unicode do core para unidades UTF-16 do LSP.
#[must_use]
pub fn utf16_column(line: &str, character_column: usize) -> u32 {
    line.chars()
        .take(character_column)
        .map(char::len_utf16)
        .sum::<usize>() as u32
}

/// Converte uma coluna UTF-16 do LSP para a coluna em escalares Unicode do core.
#[must_use]
pub fn character_column(line: &str, utf16_column: u32) -> Option<usize> {
    let target = utf16_column as usize;
    let mut units = 0usize;
    for (column, character) in line.chars().enumerate() {
        if units == target {
            return Some(column);
        }
        units += character.len_utf16();
        if units > target {
            return None;
        }
    }
    (units == target).then_some(line.chars().count())
}

#[cfg(test)]
mod tests {
    use super::{character_column, utf16_column};

    #[test]
    fn converts_columns_around_non_bmp_characters() {
        let line = "a😀b";

        assert_eq!(utf16_column(line, 2), 3);
        assert_eq!(character_column(line, 3), Some(2));
        assert_eq!(character_column(line, 2), None);
    }
}
