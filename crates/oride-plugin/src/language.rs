//! LanguageProvider — metadados de linguagem (comentário, soft wrap, LSP).

use oride_syntax::LanguageId;

use crate::completion::completion_words;

/// Provedor de linguagem embutido (sem highlight — isso fica em `oride-syntax`).
pub trait LanguageProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn language_id(&self) -> LanguageId;
    fn extensions(&self) -> &'static [&'static str];
    /// Prefixo de comentário de linha (`// `, `<!-- `).
    fn comment_open(&self) -> Option<&'static str>;
    /// Sufixo (ex. ` -->` em HTML/MD).
    fn comment_close(&self) -> Option<&'static str> {
        None
    }
    /// Comando LSP sugerido (ex. `oriscript lsp`).
    fn lsp_command(&self) -> Option<&'static [&'static str]> {
        None
    }
    /// Keywords e nomes básicos usados quando não há servidor LSP.
    fn completion_words(&self) -> &'static [&'static str] {
        completion_words(self.language_id())
    }
    fn default_soft_wrap(&self) -> bool {
        false
    }
}

/// Definição dinâmica de linguagem (via configuração TOML do usuário ou projeto).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicLang {
    pub id: String,
    pub name: String,
    pub language_id: LanguageId,
    pub extensions: Vec<String>,
    pub filenames: Vec<String>,
    pub comment_open: Option<String>,
    pub comment_close: Option<String>,
    pub lsp_command: Vec<String>,
    pub tab_size: Option<u8>,
    pub insert_spaces: Option<bool>,
    pub soft_wrap: Option<bool>,
    pub completion_words: Vec<String>,
}

impl DynamicLang {
    #[must_use]
    pub fn new(id: impl Into<String>, extensions: Vec<String>) -> Self {
        let id_str = id.into();
        let language_id = LanguageId::from_str_or_custom(&id_str);
        Self {
            id: id_str.clone(),
            name: id_str,
            language_id,
            extensions,
            filenames: Vec::new(),
            comment_open: None,
            comment_close: None,
            lsp_command: Vec::new(),
            tab_size: None,
            insert_spaces: None,
            soft_wrap: None,
            completion_words: Vec::new(),
        }
    }
}

/// Provider trivial por `LanguageId`.
#[derive(Debug, Clone, Copy)]
pub struct BuiltinLang {
    pub id: &'static str,
    pub language_id: LanguageId,
    pub extensions: &'static [&'static str],
    pub comment_open: Option<&'static str>,
    pub comment_close: Option<&'static str>,
    pub lsp: Option<&'static [&'static str]>,
    pub soft_wrap: bool,
}

impl LanguageProvider for BuiltinLang {
    fn id(&self) -> &'static str {
        self.id
    }
    fn language_id(&self) -> LanguageId {
        self.language_id
    }
    fn extensions(&self) -> &'static [&'static str] {
        self.extensions
    }
    fn comment_open(&self) -> Option<&'static str> {
        self.comment_open
    }
    fn comment_close(&self) -> Option<&'static str> {
        self.comment_close
    }
    fn lsp_command(&self) -> Option<&'static [&'static str]> {
        self.lsp
    }
    fn default_soft_wrap(&self) -> bool {
        self.soft_wrap
    }
}

pub static LANG_PLAIN: BuiltinLang = BuiltinLang {
    id: "plain",
    language_id: LanguageId::Plain,
    extensions: &[],
    comment_open: None,
    comment_close: None,
    lsp: None,
    soft_wrap: false,
};

pub static LANG_ORIS: BuiltinLang = BuiltinLang {
    id: "oriscript",
    language_id: LanguageId::OriScript,
    extensions: &["oris"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["oriscript", "lsp"]),
    soft_wrap: false,
};

pub static LANG_MD: BuiltinLang = BuiltinLang {
    id: "markdown",
    language_id: LanguageId::Markdown,
    extensions: &["md", "markdown", "mdown"],
    comment_open: Some("<!-- "),
    comment_close: Some(" -->"),
    lsp: None,
    soft_wrap: true,
};

pub static LANG_MDX: BuiltinLang = BuiltinLang {
    id: "mdx",
    language_id: LanguageId::Mdx,
    extensions: &["mdx"],
    comment_open: Some("<!-- "),
    comment_close: Some(" -->"),
    lsp: None,
    soft_wrap: true,
};

pub static LANG_HTML: BuiltinLang = BuiltinLang {
    id: "html",
    language_id: LanguageId::Html,
    extensions: &["html", "htm"],
    comment_open: Some("<!-- "),
    comment_close: Some(" -->"),
    lsp: None,
    soft_wrap: false,
};

pub static LANG_CSS: BuiltinLang = BuiltinLang {
    id: "css",
    language_id: LanguageId::Css,
    extensions: &["css"],
    comment_open: Some("/* "),
    comment_close: Some(" */"),
    lsp: None,
    soft_wrap: false,
};

pub static LANG_JS: BuiltinLang = BuiltinLang {
    id: "javascript",
    language_id: LanguageId::JavaScript,
    extensions: &["js", "jsx", "mjs", "cjs"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["typescript-language-server", "--stdio"]),
    soft_wrap: false,
};

pub static LANG_TS: BuiltinLang = BuiltinLang {
    id: "typescript",
    language_id: LanguageId::TypeScript,
    extensions: &["ts"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["typescript-language-server", "--stdio"]),
    soft_wrap: false,
};

pub static LANG_TSX: BuiltinLang = BuiltinLang {
    id: "typescriptreact",
    language_id: LanguageId::Tsx,
    extensions: &["tsx"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["typescript-language-server", "--stdio"]),
    soft_wrap: false,
};

pub static LANG_RUST: BuiltinLang = BuiltinLang {
    id: "rust",
    language_id: LanguageId::Rust,
    extensions: &["rs"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["rust-analyzer"]),
    soft_wrap: false,
};

pub static LANG_C: BuiltinLang = BuiltinLang {
    id: "c",
    language_id: LanguageId::C,
    extensions: &["c", "h"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["clangd"]),
    soft_wrap: false,
};

pub static LANG_BASH: BuiltinLang = BuiltinLang {
    id: "bash",
    language_id: LanguageId::Bash,
    extensions: &["sh", "bash", "zsh"],
    comment_open: Some("# "),
    comment_close: None,
    lsp: Some(&["bash-language-server", "start"]),
    soft_wrap: false,
};

pub static LANG_PYTHON: BuiltinLang = BuiltinLang {
    id: "python",
    language_id: LanguageId::Python,
    extensions: &["py", "pyw"],
    comment_open: Some("# "),
    comment_close: None,
    lsp: Some(&["pylsp"]),
    soft_wrap: false,
};

pub static LANG_RUBY: BuiltinLang = BuiltinLang {
    id: "ruby",
    language_id: LanguageId::Ruby,
    extensions: &["rb", "rake", "gemspec"],
    comment_open: Some("# "),
    comment_close: None,
    lsp: Some(&["solargraph", "stdio"]),
    soft_wrap: false,
};

pub static LANG_NIM: BuiltinLang = BuiltinLang {
    id: "nim",
    language_id: LanguageId::Nim,
    extensions: &["nim", "nims", "nimble"],
    comment_open: Some("# "),
    comment_close: None,
    lsp: Some(&["nimlsp"]),
    soft_wrap: false,
};

pub static LANG_ORI: BuiltinLang = BuiltinLang {
    id: "ori",
    language_id: LanguageId::Ori,
    extensions: &["orl"],
    comment_open: Some("-- "),
    comment_close: None,
    lsp: Some(&["ori-lsp"]),
    soft_wrap: false,
};

pub static LANG_D: BuiltinLang = BuiltinLang {
    id: "d",
    language_id: LanguageId::D,
    extensions: &["d", "di"],
    comment_open: Some("// "),
    comment_close: None,
    lsp: Some(&["serve-d"]),
    soft_wrap: false,
};

pub static LANG_LUA: BuiltinLang = BuiltinLang {
    id: "lua",
    language_id: LanguageId::Lua,
    extensions: &["lua"],
    comment_open: Some("-- "),
    comment_close: None,
    lsp: Some(&["lua-language-server"]),
    soft_wrap: false,
};

/// Todos os providers built-in.
pub fn builtin_languages() -> Vec<&'static dyn LanguageProvider> {
    vec![
        &LANG_PLAIN,
        &LANG_ORIS,
        &LANG_MD,
        &LANG_MDX,
        &LANG_HTML,
        &LANG_CSS,
        &LANG_JS,
        &LANG_TS,
        &LANG_TSX,
        &LANG_RUST,
        &LANG_C,
        &LANG_BASH,
        &LANG_PYTHON,
        &LANG_RUBY,
        &LANG_NIM,
        &LANG_ORI,
        &LANG_D,
        &LANG_LUA,
    ]
}

/// Resolve provider pelo `LanguageId` de `oride-syntax`.
#[must_use]
pub fn provider_for(lang: LanguageId) -> &'static dyn LanguageProvider {
    for p in builtin_languages() {
        if p.language_id() == lang {
            return p;
        }
    }
    &LANG_PLAIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oris_has_lsp_hint() {
        let p = provider_for(LanguageId::OriScript);
        assert_eq!(p.id(), "oriscript");
        assert_eq!(p.lsp_command(), Some(&["oriscript", "lsp"][..]));
        assert_eq!(p.comment_open(), Some("// "));
    }

    #[test]
    fn md_soft_wrap_and_html_comment() {
        let p = provider_for(LanguageId::Markdown);
        assert!(p.default_soft_wrap());
        assert_eq!(p.comment_close(), Some(" -->"));
    }

    #[test]
    fn l1_languages_have_expected_comment_syntax() {
        let slash_languages = [
            LanguageId::TypeScript,
            LanguageId::Tsx,
            LanguageId::Rust,
            LanguageId::C,
            LanguageId::D,
        ];
        for language in slash_languages {
            assert_eq!(provider_for(language).comment_open(), Some("// "));
        }

        let hash_languages = [
            LanguageId::Bash,
            LanguageId::Python,
            LanguageId::Ruby,
            LanguageId::Nim,
        ];
        for language in hash_languages {
            assert_eq!(provider_for(language).comment_open(), Some("# "));
        }
        assert_eq!(provider_for(LanguageId::Ori).comment_open(), Some("-- "));
        assert_eq!(
            provider_for(LanguageId::Ori).lsp_command(),
            Some(&["ori-lsp"][..])
        );
        assert_eq!(provider_for(LanguageId::Lua).comment_open(), Some("-- "));
        assert_eq!(
            provider_for(LanguageId::Lua).lsp_command(),
            Some(&["lua-language-server"][..])
        );
        assert_eq!(
            provider_for(LanguageId::D).lsp_command(),
            Some(&["serve-d"][..])
        );
        assert!(provider_for(LanguageId::Rust)
            .completion_words()
            .contains(&"return"));
        assert!(provider_for(LanguageId::D)
            .completion_words()
            .contains(&"unittest"));
        assert!(provider_for(LanguageId::Lua)
            .completion_words()
            .contains(&"local"));
    }

    #[test]
    fn every_l1_language_has_lsp_command() {
        let langs = [
            LanguageId::OriScript,
            LanguageId::Ori,
            LanguageId::D,
            LanguageId::Lua,
            LanguageId::Rust,
            LanguageId::C,
            LanguageId::Bash,
            LanguageId::Python,
            LanguageId::TypeScript,
            LanguageId::Tsx,
            LanguageId::Ruby,
            LanguageId::Nim,
        ];
        for lang in langs {
            let p = provider_for(lang);
            assert!(
                p.lsp_command().is_some(),
                "Language {:?} should have an LSP command defined",
                lang
            );
        }
    }
}
