//! Motor de edição modal estilo Vim (Normal, Insert, Visual, VisualLine).

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VimMode {
    #[default]
    Normal,
    Insert,
    Visual,
    VisualLine,
}

impl VimMode {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
            Self::Visual => "VISUAL",
            Self::VisualLine => "V-LINE",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VimState {
    pub mode: VimMode,
    pub pending_operator: Option<char>,
    pub visual_anchor: Option<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VimAction {
    EnterInsert(InsertPosition),
    EnterVisual(bool),
    ExitToNormal,
    OpenCommandLine,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    WordForward,
    WordBackward,
    WordEnd,
    LineStart,
    LineEnd,
    FirstLine,
    LastLine,
    DeleteChar,
    DeleteLine,
    DeleteSelection,
    YankLine,
    YankSelection,
    PasteAfter,
    PasteBefore,
    Undo,
    Redo,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    AtCursor,
    AfterCursor,
    LineStart,
    LineEnd,
    NewLineBelow,
    NewLineAbove,
}

impl VimState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> VimAction {
        match self.mode {
            VimMode::Normal => self.handle_normal_key(key),
            VimMode::Insert => self.handle_insert_key(key),
            VimMode::Visual | VimMode::VisualLine => self.handle_visual_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: &KeyEvent) -> VimAction {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('r') => return VimAction::Redo,
                KeyCode::Char('[') => {
                    self.pending_operator = None;
                    return VimAction::None;
                }
                _ => return VimAction::None,
            }
        }

        if let Some(op) = self.pending_operator.take() {
            match (op, key.code) {
                ('d', KeyCode::Char('d')) => return VimAction::DeleteLine,
                ('y', KeyCode::Char('y')) => return VimAction::YankLine,
                ('g', KeyCode::Char('g')) => return VimAction::FirstLine,
                _ => return VimAction::None,
            }
        }

        match key.code {
            KeyCode::Esc => {
                self.pending_operator = None;
                VimAction::None
            }
            KeyCode::Char(':') => VimAction::OpenCommandLine,
            KeyCode::Char('i') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::AtCursor)
            }
            KeyCode::Char('a') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::AfterCursor)
            }
            KeyCode::Char('I') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::LineStart)
            }
            KeyCode::Char('A') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::LineEnd)
            }
            KeyCode::Char('o') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::NewLineBelow)
            }
            KeyCode::Char('O') => {
                self.mode = VimMode::Insert;
                VimAction::EnterInsert(InsertPosition::NewLineAbove)
            }
            KeyCode::Char('v') => {
                self.mode = VimMode::Visual;
                VimAction::EnterVisual(false)
            }
            KeyCode::Char('V') => {
                self.mode = VimMode::VisualLine;
                VimAction::EnterVisual(true)
            }
            KeyCode::Char('h') | KeyCode::Left => VimAction::MoveLeft,
            KeyCode::Char('l') | KeyCode::Right => VimAction::MoveRight,
            KeyCode::Char('k') | KeyCode::Up => VimAction::MoveUp,
            KeyCode::Char('j') | KeyCode::Down => VimAction::MoveDown,
            KeyCode::Char('w') => VimAction::WordForward,
            KeyCode::Char('b') => VimAction::WordBackward,
            KeyCode::Char('e') => VimAction::WordEnd,
            KeyCode::Char('0') => VimAction::LineStart,
            KeyCode::Char('$') => VimAction::LineEnd,
            KeyCode::Char('G') => VimAction::LastLine,
            KeyCode::Char('g') => {
                self.pending_operator = Some('g');
                VimAction::None
            }
            KeyCode::Char('x') => VimAction::DeleteChar,
            KeyCode::Char('d') => {
                self.pending_operator = Some('d');
                VimAction::None
            }
            KeyCode::Char('y') => {
                self.pending_operator = Some('y');
                VimAction::None
            }
            KeyCode::Char('p') => VimAction::PasteAfter,
            KeyCode::Char('P') => VimAction::PasteBefore,
            KeyCode::Char('u') => VimAction::Undo,
            _ => VimAction::None,
        }
    }

    fn handle_insert_key(&mut self, key: &KeyEvent) -> VimAction {
        if key.code == KeyCode::Esc
            || (key.code == KeyCode::Char('[') && key.modifiers.contains(KeyModifiers::CONTROL))
        {
            self.mode = VimMode::Normal;
            self.pending_operator = None;
            return VimAction::ExitToNormal;
        }
        VimAction::None
    }

    fn handle_visual_key(&mut self, key: &KeyEvent) -> VimAction {
        if key.code == KeyCode::Esc {
            self.mode = VimMode::Normal;
            self.visual_anchor = None;
            return VimAction::ExitToNormal;
        }

        match key.code {
            KeyCode::Char('h') | KeyCode::Left => VimAction::MoveLeft,
            KeyCode::Char('l') | KeyCode::Right => VimAction::MoveRight,
            KeyCode::Char('k') | KeyCode::Up => VimAction::MoveUp,
            KeyCode::Char('j') | KeyCode::Down => VimAction::MoveDown,
            KeyCode::Char('w') => VimAction::WordForward,
            KeyCode::Char('b') => VimAction::WordBackward,
            KeyCode::Char('e') => VimAction::WordEnd,
            KeyCode::Char('0') => VimAction::LineStart,
            KeyCode::Char('$') => VimAction::LineEnd,
            KeyCode::Char('d') | KeyCode::Char('x') => {
                self.mode = VimMode::Normal;
                VimAction::DeleteSelection
            }
            KeyCode::Char('y') => {
                self.mode = VimMode::Normal;
                VimAction::YankSelection
            }
            _ => VimAction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_mode_navigation_and_transitions() {
        let mut vim = VimState::new();
        assert_eq!(vim.mode, VimMode::Normal);

        let act = vim.handle_key(&KeyEvent::from(KeyCode::Char('h')));
        assert_eq!(act, VimAction::MoveLeft);

        let act = vim.handle_key(&KeyEvent::from(KeyCode::Char('i')));
        assert_eq!(act, VimAction::EnterInsert(InsertPosition::AtCursor));
        assert_eq!(vim.mode, VimMode::Insert);

        let act = vim.handle_key(&KeyEvent::from(KeyCode::Esc));
        assert_eq!(act, VimAction::ExitToNormal);
        assert_eq!(vim.mode, VimMode::Normal);
    }

    #[test]
    fn normal_mode_operator_doubling() {
        let mut vim = VimState::new();
        let act = vim.handle_key(&KeyEvent::from(KeyCode::Char('d')));
        assert_eq!(act, VimAction::None);
        assert_eq!(vim.pending_operator, Some('d'));

        let act = vim.handle_key(&KeyEvent::from(KeyCode::Char('d')));
        assert_eq!(act, VimAction::DeleteLine);
        assert_eq!(vim.pending_operator, None);
    }

    #[test]
    fn command_line_trigger() {
        let mut vim = VimState::new();
        let act = vim.handle_key(&KeyEvent::from(KeyCode::Char(':')));
        assert_eq!(act, VimAction::OpenCommandLine);
    }
}
