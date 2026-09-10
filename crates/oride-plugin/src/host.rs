use std::path::Path;

use oride_syntax::LanguageId;

use crate::language::{builtin_languages, provider_for, DynamicLang, LanguageProvider};
use crate::plugin::{
    CommandMeta, LifecyclePlugin, Plugin, PluginCtx, PluginHook, PluginResult, ShowPathPlugin,
    WordCountPlugin,
};

/// Host embutido no binário.
pub struct PluginHost {
    languages: Vec<&'static dyn LanguageProvider>,
    plugins: Vec<Box<dyn Plugin>>,
    dynamic_languages: Vec<DynamicLang>,
}

impl PluginHost {
    #[must_use]
    pub fn new(
        languages: Vec<&'static dyn LanguageProvider>,
        plugins: Vec<Box<dyn Plugin>>,
    ) -> Self {
        Self {
            languages,
            plugins,
            dynamic_languages: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_dynamic_languages(
        languages: Vec<&'static dyn LanguageProvider>,
        plugins: Vec<Box<dyn Plugin>>,
        dynamic_languages: Vec<DynamicLang>,
    ) -> Self {
        Self {
            languages,
            plugins,
            dynamic_languages,
        }
    }

    pub fn add_dynamic_language(&mut self, lang: DynamicLang) {
        if let Some(pos) = self
            .dynamic_languages
            .iter()
            .position(|l| l.id.eq_ignore_ascii_case(&lang.id))
        {
            self.dynamic_languages[pos] = lang;
        } else {
            self.dynamic_languages.push(lang);
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    #[must_use]
    pub fn dynamic_languages(&self) -> &[DynamicLang] {
        &self.dynamic_languages
    }

    #[must_use]
    pub fn detect_language_by_path(&self, path: &Path) -> Option<LanguageId> {
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

        for d in &self.dynamic_languages {
            if d.filenames.iter().any(|f| f.eq_ignore_ascii_case(&name)) {
                return Some(d.language_id);
            }
            if !ext.is_empty() && d.extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
                return Some(d.language_id);
            }
        }
        None
    }

    #[must_use]
    pub fn comment_open(&self, id: LanguageId) -> Option<&str> {
        for d in &self.dynamic_languages {
            if d.language_id == id {
                if let Some(open) = &d.comment_open {
                    return Some(open.as_str());
                }
            }
        }
        self.language(id).comment_open()
    }

    #[must_use]
    pub fn comment_close(&self, id: LanguageId) -> Option<&str> {
        for d in &self.dynamic_languages {
            if d.language_id == id {
                if let Some(close) = &d.comment_close {
                    return Some(close.as_str());
                }
            }
        }
        self.language(id).comment_close()
    }

    #[must_use]
    pub fn lsp_command(&self, id: LanguageId) -> Option<Vec<String>> {
        for d in &self.dynamic_languages {
            if d.language_id == id && !d.lsp_command.is_empty() {
                return Some(d.lsp_command.clone());
            }
        }
        self.language(id)
            .lsp_command()
            .map(|cmd| cmd.iter().map(|s| (*s).to_string()).collect())
    }

    #[must_use]
    pub fn completion_words(&self, id: LanguageId) -> Vec<String> {
        let mut words: Vec<String> = self
            .language(id)
            .completion_words()
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        for d in &self.dynamic_languages {
            if d.language_id == id {
                for w in &d.completion_words {
                    if !words.contains(w) {
                        words.push(w.clone());
                    }
                }
            }
        }
        words
    }

    #[must_use]
    pub fn default_soft_wrap(&self, id: LanguageId) -> bool {
        for d in &self.dynamic_languages {
            if d.language_id == id {
                if let Some(sw) = d.soft_wrap {
                    return sw;
                }
            }
        }
        self.language(id).default_soft_wrap()
    }

    #[must_use]
    pub fn language(&self, id: LanguageId) -> &dyn LanguageProvider {
        for p in &self.languages {
            if p.language_id() == id {
                return *p;
            }
        }
        provider_for(id)
    }

    /// Labels para a command palette (`Plugin: word count`).
    #[must_use]
    pub fn palette_commands(&self) -> Vec<CommandMeta> {
        let mut out = Vec::new();
        for p in &self.plugins {
            out.extend_from_slice(p.commands());
        }
        out
    }

    pub fn dispatch_hook(&self, hook: PluginHook, ctx: &mut dyn PluginCtx) {
        for p in &self.plugins {
            p.on_hook(hook, ctx);
        }
    }

    pub fn run_command(&self, command_id: &str, ctx: &mut dyn PluginCtx) -> PluginResult {
        for p in &self.plugins {
            if p.commands().iter().any(|c| c.id == command_id) {
                return p.run_command(command_id, ctx);
            }
            // também aceita label exato
            if p.commands().iter().any(|c| c.label == command_id) {
                let id = p
                    .commands()
                    .iter()
                    .find(|c| c.label == command_id)
                    .map(|c| c.id)
                    .unwrap_or(command_id);
                return p.run_command(id, ctx);
            }
        }
        Err(crate::plugin::PluginError::UnknownCommand(
            command_id.to_string(),
        ))
    }

    /// Resolve id a partir do label da palette.
    #[must_use]
    pub fn command_id_for_label(&self, label: &str) -> Option<&'static str> {
        for p in &self.plugins {
            for c in p.commands() {
                if c.label == label {
                    return Some(c.id);
                }
            }
        }
        None
    }
}

/// Host padrão do Oride.
#[must_use]
pub fn builtin_host() -> PluginHost {
    PluginHost::new(
        builtin_languages(),
        vec![
            Box::new(WordCountPlugin),
            Box::new(ShowPathPlugin),
            Box::new(LifecyclePlugin { announce: false }),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::PluginCtx;
    use std::path::{Path, PathBuf};

    struct Ctx {
        status: String,
        text: String,
    }
    impl PluginCtx for Ctx {
        fn set_status(&mut self, msg: &str) {
            self.status = msg.into();
        }
        fn workspace_root(&self) -> &Path {
            Path::new(".")
        }
        fn active_path(&self) -> Option<PathBuf> {
            None
        }
        fn active_buffer_text(&self) -> String {
            self.text.clone()
        }
        fn active_is_dirty(&self) -> bool {
            false
        }
    }

    #[test]
    fn palette_lists_word_count() {
        let host = builtin_host();
        let cmds = host.palette_commands();
        assert!(cmds.iter().any(|c| c.id == "word_count"));
    }

    #[test]
    fn run_by_label() {
        let host = builtin_host();
        let mut ctx = Ctx {
            status: String::new(),
            text: "a b".into(),
        };
        host.run_command("Plugin: word count", &mut ctx).unwrap();
        assert!(ctx.status.contains("words=2"));
    }
}
