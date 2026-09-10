//! Detecção de linguagem por path/extensão.

use std::path::Path;

/// Linguagens com highlight nativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LanguageId {
    #[default]
    Plain,
    OriScript,
    Markdown,
    /// MDX / MD com JSX — highlight como Markdown (sem parse JSX completo).
    Mdx,
    Html,
    Css,
    JavaScript,
    TypeScript,
    Tsx,
    Rust,
    C,
    Bash,
    Python,
    Ruby,
    Nim,
    Ori,
    D,
    Lua,
    Custom(&'static str),
}

impl LanguageId {
    #[must_use]
    pub fn from_str_or_custom(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "plain" => Self::Plain,
            "oriscript" | "oris" => Self::OriScript,
            "markdown" | "md" => Self::Markdown,
            "mdx" => Self::Mdx,
            "html" | "htm" => Self::Html,
            "css" => Self::Css,
            "javascript" | "js" | "mjs" | "cjs" => Self::JavaScript,
            "typescript" | "ts" => Self::TypeScript,
            "typescriptreact" | "tsx" => Self::Tsx,
            "rust" | "rs" => Self::Rust,
            "c" | "h" => Self::C,
            "bash" | "sh" | "shell" | "zsh" => Self::Bash,
            "python" | "py" => Self::Python,
            "ruby" | "rb" => Self::Ruby,
            "nim" => Self::Nim,
            "ori" | "orl" => Self::Ori,
            "d" => Self::D,
            "lua" => Self::Lua,
            other => {
                let leaked: &'static str = Box::leak(other.to_string().into_boxed_str());
                Self::Custom(leaked)
            }
        }
    }

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::OriScript => "oriscript",
            Self::Markdown => "markdown",
            Self::Mdx => "mdx",
            Self::Html => "html",
            Self::Css => "css",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Tsx => "typescriptreact",
            Self::Rust => "rust",
            Self::C => "c",
            Self::Bash => "bash",
            Self::Python => "python",
            Self::Ruby => "ruby",
            Self::Nim => "nim",
            Self::Ori => "ori",
            Self::D => "d",
            Self::Lua => "lua",
            Self::Custom(s) => s,
        }
    }

    /// Soft wrap padrão recomendado.
    #[must_use]
    pub fn default_soft_wrap(self) -> bool {
        matches!(self, Self::Markdown | Self::Mdx)
    }

    /// É família Markdown (md / mdx / derivados)?
    #[must_use]
    pub fn is_markdown_family(self) -> bool {
        matches!(self, Self::Markdown | Self::Mdx)
    }

    /// Token de comentário de linha (ou HTML comment open para MD).
    #[must_use]
    pub fn line_comment(self) -> Option<&'static str> {
        match self {
            Self::Markdown | Self::Mdx | Self::Html => Some("<!-- "),
            Self::OriScript
            | Self::JavaScript
            | Self::TypeScript
            | Self::Tsx
            | Self::Rust
            | Self::C
            | Self::D => Some("// "),
            Self::Ori | Self::Lua => Some("-- "),
            Self::Python | Self::Ruby | Self::Nim | Self::Bash => Some("# "),
            Self::Css => Some("/* "),
            Self::Plain | Self::Custom(_) => None,
        }
    }

    /// Sufixo de comentário de bloco (ou HTML/Lua).
    #[must_use]
    pub fn block_comment_close(self) -> Option<&'static str> {
        match self {
            Self::Markdown | Self::Mdx | Self::Html => Some(" -->"),
            Self::Css | Self::D | Self::C => Some(" */"),
            Self::Lua => Some("]]"),
            _ => None,
        }
    }
}

/// Detecta linguagem a partir do path do arquivo.
#[must_use]
pub fn detect_language(path: Option<&Path>) -> LanguageId {
    let Some(path) = path else {
        return LanguageId::Plain;
    };
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    // nomes sem extensão / especiais
    if matches!(
        name.as_str(),
        "readme" | "changelog" | "history" | "license" | "authors" | "contributing"
    ) {
        return LanguageId::Markdown;
    }
    if matches!(
        name.as_str(),
        ".bashrc"
            | ".bash_profile"
            | ".bash_login"
            | ".bash_logout"
            | ".profile"
            | ".zshrc"
            | ".zprofile"
    ) {
        return LanguageId::Bash;
    }
    if name.ends_with(".md") || name.contains("readme") {
        // readme.pt-br etc.
        if name.contains('.') {
            // fall through to extension
        }
    }

    match ext.as_str() {
        "oris" => LanguageId::OriScript,
        // Markdown e derivados
        "md" | "markdown" | "mdown" | "mkd" | "mkdn" | "mdwn" | "mdtxt" | "mdtext" | "rmd"
        | "qmd" => LanguageId::Markdown,
        "mdx" => LanguageId::Mdx,
        "html" | "htm" => LanguageId::Html,
        "css" => LanguageId::Css,
        "js" | "mjs" | "cjs" | "jsx" => LanguageId::JavaScript,
        "ts" => LanguageId::TypeScript,
        "tsx" => LanguageId::Tsx,
        "rs" => LanguageId::Rust,
        "c" | "h" => LanguageId::C,
        "sh" | "bash" | "zsh" => LanguageId::Bash,
        "py" | "pyw" => LanguageId::Python,
        "rb" | "rake" | "gemspec" => LanguageId::Ruby,
        "nim" | "nims" | "nimble" => LanguageId::Nim,
        "orl" => LanguageId::Ori,
        "d" | "di" => LanguageId::D,
        "lua" => LanguageId::Lua,
        _ => {
            // README.md already handled; bare README
            if name.starts_with("readme") {
                LanguageId::Markdown
            } else {
                LanguageId::Plain
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_markdown_derivatives() {
        assert_eq!(
            detect_language(Some(Path::new("a.md"))),
            LanguageId::Markdown
        );
        assert_eq!(
            detect_language(Some(Path::new("doc.markdown"))),
            LanguageId::Markdown
        );
        assert_eq!(detect_language(Some(Path::new("x.mdx"))), LanguageId::Mdx);
        assert_eq!(
            detect_language(Some(Path::new("n.qmd"))),
            LanguageId::Markdown
        );
        assert_eq!(
            detect_language(Some(Path::new("README"))),
            LanguageId::Markdown
        );
        assert!(LanguageId::Markdown.default_soft_wrap());
        assert!(LanguageId::Mdx.is_markdown_family());
    }

    #[test]
    fn detects_l1_languages() {
        let cases = [
            ("main.rs", LanguageId::Rust),
            ("main.c", LanguageId::C),
            ("header.h", LanguageId::C),
            ("deploy.sh", LanguageId::Bash),
            ("script.bash", LanguageId::Bash),
            (".bashrc", LanguageId::Bash),
            ("main.py", LanguageId::Python),
            ("app.ts", LanguageId::TypeScript),
            ("view.tsx", LanguageId::Tsx),
            ("task.rb", LanguageId::Ruby),
            ("main.nim", LanguageId::Nim),
            ("main.orl", LanguageId::Ori),
            ("main.d", LanguageId::D),
            ("module.di", LanguageId::D),
            ("script.lua", LanguageId::Lua),
        ];
        for (path, expected) in cases {
            assert_eq!(detect_language(Some(Path::new(path))), expected, "{path}");
        }
    }
}
