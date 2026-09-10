//! Arquitetura de Componentes desacoplados para o editor Oride.
//!
//! Permite que painéis e subsistemas (Árvore, Terminal, SCM, Editor, Menus, Barra de Status)
//! sejam desacoplados, testáveis e configuráveis sem quebrar o ciclo de vida da aplicação.

use crate::app::{App, Focus};
use crate::mouse::HitTarget;
use crossterm::event::{KeyEvent, MouseEvent};

/// Identificador de subsistema/painel na TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Editor,
    Tree,
    Terminal,
    Scm,
    MenuBar,
    StatusBar,
}

impl ComponentId {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Tree => "tree",
            Self::Terminal => "terminal",
            Self::Scm => "scm",
            Self::MenuBar => "menu_bar",
            Self::StatusBar => "status_bar",
        }
    }
}

/// Contrato para um componente ou painel desacoplado do Oride.
pub trait Component: Send + Sync {
    /// Identificador único estável do componente.
    fn id(&self) -> &'static str;

    /// Nome legível para UI ou logs.
    fn name(&self) -> &'static str;

    /// Verifica se o componente está visível no estado atual do editor.
    fn is_visible(&self, app: &App) -> bool;

    /// Retorna o foco associado a este componente, se aplicável.
    fn focus(&self) -> Option<Focus>;

    /// Trata evento de teclado direcionado ao componente quando este possui o foco.
    /// Retorna `true` se o evento foi consumido.
    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool;

    /// Trata evento de mouse que atingiu a região visual do componente.
    /// Retorna `true` se o evento foi consumido.
    fn handle_mouse(&self, app: &mut App, event: MouseEvent, target: Option<HitTarget>) -> bool;
}

// ---------------------------------------------------------------------------
// Componente: Árvore de Arquivos
// ---------------------------------------------------------------------------

pub struct TreeComponent;

impl Component for TreeComponent {
    fn id(&self) -> &'static str {
        "tree"
    }

    fn name(&self) -> &'static str {
        "File Tree"
    }

    fn is_visible(&self, app: &App) -> bool {
        app.show_tree && app.tree.is_some()
    }

    fn focus(&self) -> Option<Focus> {
        Some(Focus::Tree)
    }

    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool {
        app.handle_tree_key(key)
    }

    fn handle_mouse(&self, app: &mut App, event: MouseEvent, target: Option<HitTarget>) -> bool {
        matches!(target, Some(HitTarget::Tree | HitTarget::TreeSplitter)) && {
            app.focus = Focus::Tree;
            let _ = (event, app);
            true
        }
    }
}

// ---------------------------------------------------------------------------
// Componente: Terminal Embutido
// ---------------------------------------------------------------------------

pub struct TerminalComponent;

impl Component for TerminalComponent {
    fn id(&self) -> &'static str {
        "terminal"
    }

    fn name(&self) -> &'static str {
        "Embedded Terminal"
    }

    fn is_visible(&self, app: &App) -> bool {
        app.terminal.as_ref().is_some_and(|t| t.height_lines > 0)
    }

    fn focus(&self) -> Option<Focus> {
        Some(Focus::Terminal)
    }

    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool {
        app.handle_terminal_key(key)
    }

    fn handle_mouse(&self, app: &mut App, _event: MouseEvent, target: Option<HitTarget>) -> bool {
        if matches!(target, Some(HitTarget::Terminal)) {
            app.focus = Focus::Terminal;
            return true;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Componente: Controle de Versão (SCM)
// ---------------------------------------------------------------------------

pub struct ScmComponent;

impl Component for ScmComponent {
    fn id(&self) -> &'static str {
        "scm"
    }

    fn name(&self) -> &'static str {
        "Source Control (Git SCM)"
    }

    fn is_visible(&self, app: &App) -> bool {
        app.show_scm
    }

    fn focus(&self) -> Option<Focus> {
        Some(Focus::Scm)
    }

    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool {
        app.handle_scm_key(key)
    }

    fn handle_mouse(&self, app: &mut App, _event: MouseEvent, target: Option<HitTarget>) -> bool {
        if matches!(target, Some(HitTarget::Scm)) {
            app.focus = Focus::Scm;
            return true;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Componente: Editor de Código
// ---------------------------------------------------------------------------

pub struct EditorComponent;

impl Component for EditorComponent {
    fn id(&self) -> &'static str {
        "editor"
    }

    fn name(&self) -> &'static str {
        "Code Editor"
    }

    fn is_visible(&self, _app: &App) -> bool {
        true
    }

    fn focus(&self) -> Option<Focus> {
        Some(Focus::Editor)
    }

    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool {
        if key.code == crossterm::event::KeyCode::Esc && key.modifiers.is_empty() {
            if let Ok(document) = app.store.active_mut() {
                let has_extra = !document.extra_carets().is_empty();
                let has_selection = !document.selection().is_empty();
                if has_extra || has_selection {
                    document.clear_extra_carets();
                    document.collapse_selection();
                    app.set_status("seleção / multi-cursor cancelados");
                    return true;
                }
            }
        }
        false
    }

    fn handle_mouse(&self, app: &mut App, _event: MouseEvent, target: Option<HitTarget>) -> bool {
        if matches!(target, Some(HitTarget::Editor | HitTarget::Tabs)) {
            app.focus = Focus::Editor;
            return true;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Componente: Barra de Menus Superior
// ---------------------------------------------------------------------------

pub struct MenuBarComponent;

impl Component for MenuBarComponent {
    fn id(&self) -> &'static str {
        "menu_bar"
    }

    fn name(&self) -> &'static str {
        "Top Menu Bar"
    }

    fn is_visible(&self, _app: &App) -> bool {
        true
    }

    fn focus(&self) -> Option<Focus> {
        None
    }

    fn handle_key(&self, app: &mut App, key: KeyEvent) -> bool {
        app.handle_menu_key(key);
        true
    }

    fn handle_mouse(&self, app: &mut App, event: MouseEvent, target: Option<HitTarget>) -> bool {
        if matches!(target, Some(HitTarget::Menu)) {
            app.handle_mouse_menu(event);
            return true;
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Componente: Barra de Status Inferior
// ---------------------------------------------------------------------------

pub struct StatusBarComponent;

impl Component for StatusBarComponent {
    fn id(&self) -> &'static str {
        "status_bar"
    }

    fn name(&self) -> &'static str {
        "Bottom Status Bar"
    }

    fn is_visible(&self, _app: &App) -> bool {
        true
    }

    fn focus(&self) -> Option<Focus> {
        None
    }

    fn handle_key(&self, _app: &mut App, _key: KeyEvent) -> bool {
        false
    }

    fn handle_mouse(&self, _app: &mut App, _event: MouseEvent, _target: Option<HitTarget>) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// ComponentRegistry: Registro e Despacho Central dos Componentes
// ---------------------------------------------------------------------------

pub struct ComponentRegistry {
    components: Vec<Box<dyn Component>>,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentRegistry {
    /// Cria um registro com todos os componentes padrão do Oride.
    #[must_use]
    pub fn new() -> Self {
        let mut reg = Self {
            components: Vec::new(),
        };
        reg.register(Box::new(TreeComponent));
        reg.register(Box::new(TerminalComponent));
        reg.register(Box::new(ScmComponent));
        reg.register(Box::new(EditorComponent));
        reg.register(Box::new(MenuBarComponent));
        reg.register(Box::new(StatusBarComponent));
        reg
    }

    /// Registra um novo componente ou substitui componente com mesmo id.
    pub fn register(&mut self, component: Box<dyn Component>) {
        let id = component.id();
        if let Some(pos) = self.components.iter().position(|c| c.id() == id) {
            self.components[pos] = component;
        } else {
            self.components.push(component);
        }
    }

    /// Remove um componente pelo seu identificador (desacoplamento limpo).
    pub fn unregister(&mut self, id: &str) -> Option<Box<dyn Component>> {
        if let Some(pos) = self.components.iter().position(|c| c.id() == id) {
            Some(self.components.remove(pos))
        } else {
            None
        }
    }

    /// Lista os componentes atualmente registrados.
    #[must_use]
    pub fn components(&self) -> &[Box<dyn Component>] {
        &self.components
    }

    /// Localiza um componente pelo id.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&dyn Component> {
        self.components
            .iter()
            .find(|c| c.id() == id)
            .map(AsRef::as_ref)
    }

    /// Despacha uma tecla para o componente que atualmente possui o foco ativo.
    pub fn dispatch_key_by_focus(&self, app: &mut App, key: KeyEvent) -> bool {
        let current_focus = app.focus;
        for c in &self.components {
            if c.focus() == Some(current_focus) && c.handle_key(app, key) {
                return true;
            }
        }
        false
    }

    /// Despacha evento de mouse para os componentes com base no HitTarget.
    pub fn dispatch_mouse(
        &self,
        app: &mut App,
        event: MouseEvent,
        target: Option<HitTarget>,
    ) -> bool {
        for c in &self.components {
            if c.handle_mouse(app, event, target) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oride_config::Config;
    use oride_core::DocumentStore;

    #[test]
    fn component_registry_registration_and_lookup() {
        let mut reg = ComponentRegistry::new();
        assert_eq!(reg.components().len(), 6);
        assert!(reg.get("tree").is_some());
        assert!(reg.get("terminal").is_some());
        assert!(reg.get("scm").is_some());
        assert!(reg.get("editor").is_some());
        assert!(reg.get("menu_bar").is_some());
        assert!(reg.get("status_bar").is_some());

        // Test dynamic unregister (decoupling without break)
        let removed = reg.unregister("scm");
        assert!(removed.is_some());
        assert_eq!(reg.components().len(), 5);
        assert!(reg.get("scm").is_none());

        // Test re-register
        reg.register(removed.unwrap());
        assert_eq!(reg.components().len(), 6);
        assert!(reg.get("scm").is_some());
    }

    #[test]
    fn component_visibility_and_focus_routing() {
        let mut store = DocumentStore::new();
        store.open_empty();
        let mut app = App::from_store_with_config(store, Config::default(), std::env::temp_dir());
        let reg = ComponentRegistry::new();

        let editor_comp = reg.get("editor").unwrap();
        assert!(editor_comp.is_visible(&app));
        assert_eq!(editor_comp.focus(), Some(Focus::Editor));

        let tree_comp = reg.get("tree").unwrap();
        assert_eq!(tree_comp.focus(), Some(Focus::Tree));

        // Test key dispatch through registry when focus is Editor
        app.focus = Focus::Editor;
        let handled = reg.dispatch_key_by_focus(
            &mut app,
            KeyEvent::new(
                crossterm::event::KeyCode::Esc,
                crossterm::event::KeyModifiers::NONE,
            ),
        );
        // Esc when there's no selection returns false (normal behavior)
        assert!(!handled);
    }
}
