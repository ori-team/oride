use super::state::*;
use super::*;
use crate::app::lsp::{completion_label_matches, identifier_prefix_at, lsp_position_to_offset};
use crate::app::render::{line_visual_height, scroll_origin_for_caret, visual_row_of_caret};
use crate::browser::BrowseMode;
use crate::jump_list::Jump;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use oride_keymap::Action;
use oride_lsp::Position as LspPos;
use std::sync::Mutex;

static CLIPBOARD_TEST_LOCK: Mutex<()> = Mutex::new(());

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

fn key_ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

#[test]
fn typing_and_save_status() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('h'));
    app.apply(KeyCommand::InsertChar('i'));
    let doc = app.store.active().unwrap();
    assert_eq!(doc.buffer().as_string(), "hi");
    assert!(doc.is_dirty());
    app.apply(KeyCommand::Action(Action::Save));
    // untitled → Save As browser (em vez de erro “no path”)
    assert!(matches!(app.overlay, Overlay::Browse(_)));
}

#[test]
fn map_quit_and_save_from_config_defaults() {
    let app = App::new_empty();
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('q'))),
        Some(KeyCommand::Action(Action::Quit))
    );
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('s'))),
        Some(KeyCommand::Action(Action::Save))
    );
    assert_eq!(
        app.map_key(key(KeyCode::Enter)),
        Some(KeyCommand::Action(Action::InsertNewline))
    );
}

#[test]
fn dirty_quit_requires_confirm() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('x'));
    app.apply(KeyCommand::Action(Action::Quit));
    assert!(!app.should_quit);
    app.apply(KeyCommand::Action(Action::Quit));
    assert!(app.should_quit);
}

#[test]
fn dirty_inactive_tab_requires_quit_confirmation() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('x'));
    app.apply(KeyCommand::Action(Action::NewTab));
    assert!(!app.store.active().unwrap().is_dirty());

    app.apply(KeyCommand::Action(Action::Quit));

    assert!(!app.should_quit);
    assert!(app.quit_confirm_pending);
}

#[test]
fn multi_tab_next() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::NewTab));
    assert_eq!(app.store.tab_ids().len(), 2);
    let a = app.store.active_id();
    app.apply(KeyCommand::Action(Action::NextTab));
    assert_ne!(app.store.active_id(), a);
}

#[test]
fn close_dirty_tab_needs_confirm() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('x'));
    app.apply(KeyCommand::Action(Action::CloseTab));
    assert_eq!(app.store.tab_ids().len(), 1);
    assert!(app.store.active().unwrap().is_dirty());
    app.apply(KeyCommand::Action(Action::CloseTab));
    // still one tab (empty recreated) or empty opened
    assert!(!app.store.tab_ids().is_empty());
}

#[test]
fn command_palette_filters() {
    let app = App::new_empty();
    let items = app.command_palette_items("tab");
    assert!(items.iter().any(|i| i.to_lowercase().contains("tab")));
}

#[test]
fn fuzzy_subsequence() {
    assert!(fuzzy_match("ore", "oride-core"));
    assert!(!fuzzy_match("zzz", "oride"));
}

#[test]
fn focus_tree_and_editor_actions() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::FocusTree));
    assert_eq!(app.focus, Focus::Tree);
    assert!(app.show_tree);
    app.apply(KeyCommand::Action(Action::FocusEditor));
    assert_eq!(app.focus, Focus::Editor);
    app.apply(KeyCommand::Action(Action::FocusToggleTreeEditor));
    assert_eq!(app.focus, Focus::Tree);
    app.apply(KeyCommand::Action(Action::FocusToggleTreeEditor));
    assert_eq!(app.focus, Focus::Editor);
}

#[test]
fn open_folder_opens_browser() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::OpenFolder));
    assert!(matches!(app.overlay, Overlay::Browse(_)));
    if let Overlay::Browse(b) = &app.overlay {
        assert_eq!(b.mode, BrowseMode::Folder);
    }
}

#[test]
fn open_file_opens_browser() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::OpenFileFuzzy));
    assert!(matches!(app.overlay, Overlay::Browse(_)));
    if let Overlay::Browse(b) = &app.overlay {
        assert_eq!(b.mode, BrowseMode::File);
    }
}

#[test]
fn tree_enter_updates_focused_pane_and_loads_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("main.oris");
    std::fs::write(&path, "module main\n").unwrap();
    let mut app = App::open_workspace(directory.path().to_path_buf()).unwrap();
    app.config.lsp.enabled = false;
    app.focus = Focus::Tree;
    app.tree.as_mut().unwrap().set_selected(1);

    assert!(app.handle_tree_key(key(KeyCode::Enter)));

    let active_id = app.store.active_id().unwrap();
    assert_eq!(app.split.focused_pane().doc_id, active_id);
    assert_eq!(app.store.active().unwrap().path(), Some(path.as_path()));
    assert_eq!(
        app.store.active().unwrap().buffer().as_string(),
        "module main\n"
    );
}

#[test]
fn toggle_comment_uses_each_l1_language_provider() {
    let directory = tempfile::tempdir().unwrap();
    let fixtures = [
        ("main.rs", "// value"),
        ("main.py", "# value"),
        ("main.ts", "// value"),
        ("main.tsx", "// value"),
        ("main.rb", "# value"),
        ("main.nim", "# value"),
        ("main.orl", "-- value"),
    ];

    for (name, commented) in fixtures {
        let path = directory.path().join(name);
        std::fs::write(&path, "value").unwrap();
        let mut app = App::new_empty();
        app.config.lsp.enabled = false;
        app.open_document_path(&path).unwrap();

        app.apply(KeyCommand::Action(Action::ToggleComment));
        assert_eq!(
            app.store.active().unwrap().buffer().as_string(),
            commented,
            "commenting {name}"
        );

        app.apply(KeyCommand::Action(Action::ToggleComment));
        assert_eq!(
            app.store.active().unwrap().buffer().as_string(),
            "value",
            "uncommenting {name}"
        );
    }
}

#[test]
fn manual_completion_replaces_the_typed_prefix() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("main.rs");
    std::fs::write(&path, "").unwrap();
    let mut app = App::new_empty();
    app.config.editor.completion_auto = false;
    app.open_document_path(&path).unwrap();
    for character in "ret".chars() {
        app.apply(KeyCommand::InsertChar(character));
    }

    app.apply(KeyCommand::Action(Action::LspComplete));
    assert!(matches!(app.overlay, Overlay::Completion { .. }));
    app.handle_completion_key(key(KeyCode::Enter));

    assert_eq!(app.store.active().unwrap().buffer().as_string(), "return");
}

#[test]
fn automatic_completion_keeps_filtering_while_typing() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("main.py");
    std::fs::write(&path, "").unwrap();
    let mut app = App::new_empty();
    app.open_document_path(&path).unwrap();

    for character in "ret".chars() {
        app.handle_key(key(KeyCode::Char(character)));
    }
    let Overlay::Completion { items, .. } = &app.overlay else {
        panic!("expected automatic completion popup");
    };
    assert!(items.iter().any(|item| item.insert_text == "return"));

    app.handle_key(key(KeyCode::Tab));
    assert_eq!(app.store.active().unwrap().buffer().as_string(), "return");
}

#[test]
fn map_focus_and_open_folder_keys() {
    let app = App::new_empty();
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('b'))),
        Some(KeyCommand::Action(Action::FocusTree))
    );
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('e'))),
        Some(KeyCommand::Action(Action::FocusEditor))
    );
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('o'))),
        Some(KeyCommand::Action(Action::OpenFolder))
    );
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('h'))),
        Some(KeyCommand::Action(Action::Replace))
    );
    assert_eq!(
        app.map_key(KeyEvent {
            code: KeyCode::F(1),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }),
        Some(KeyCommand::Action(Action::Help))
    );
    let list_keys = KeyEvent {
        code: KeyCode::Char('/'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(list_keys),
        Some(KeyCommand::Action(Action::Help))
    );
    assert_eq!(
        app.map_key(key_ctrl(KeyCode::Char('"'))),
        Some(KeyCommand::Action(Action::ToggleTerminal))
    );
}

#[test]
fn project_find_opens_and_lists_hits() {
    use std::fs;
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "needle here\n").unwrap();
    fs::write(dir.path().join("b.txt"), "other\n").unwrap();
    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app = App::from_store_with_config(store, Config::default(), dir.path().to_path_buf());
    app.apply(KeyCommand::Action(Action::ProjectFind));
    assert!(matches!(app.overlay, Overlay::ProjectFind { .. }));
    // simula digitar "needle"
    for c in "needle".chars() {
        app.handle_key(KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });
    }
    if let Overlay::ProjectFind { hits, .. } = &app.overlay {
        assert!(!hits.is_empty(), "expected hits for needle");
        assert!(hits.iter().any(|h| h.line_text.contains("needle")));
    } else {
        panic!("expected ProjectFind overlay");
    }
    let pf = KeyEvent {
        code: KeyCode::Char('f'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let app2 = App::new_empty();
    assert_eq!(
        app2.map_key(pf),
        Some(KeyCommand::Action(Action::ProjectFind))
    );
}

#[test]
fn help_lists_all_keybinds() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::Help));
    assert!(matches!(app.overlay, Overlay::Help { .. }));
    let items = app.keybind_list_items("");
    assert!(
        items.len() >= 10,
        "expected full keybind list, got {}",
        items.len()
    );
    assert!(
        items
            .iter()
            .any(|l| l.contains("ctrl+s") && l.contains("Save")),
        "missing ctrl+s save: {items:?}"
    );
    assert!(
        items
            .iter()
            .any(|l| l.contains("f1") && l.contains("keybind")),
        "missing f1 help: {items:?}"
    );
    // filtro
    let filtered = app.keybind_list_items("save");
    assert!(!filtered.is_empty());
    assert!(filtered
        .iter()
        .all(|l| l.to_ascii_lowercase().contains("save") || fuzzy_match("save", l)));
}

#[test]
fn map_save_as_and_save_all() {
    use crossterm::event::KeyModifiers;
    let app = App::new_empty();
    let save_as = KeyEvent {
        code: KeyCode::Char('s'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let save_as_upper = KeyEvent {
        code: KeyCode::Char('S'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let save_as_f12 = KeyEvent {
        code: KeyCode::F(12),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let save_all = KeyEvent {
        code: KeyCode::Char('s'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(save_as),
        Some(KeyCommand::Action(Action::SaveAs))
    );
    assert_eq!(
        app.map_key(save_as_upper),
        Some(KeyCommand::Action(Action::SaveAs))
    );
    assert_eq!(
        app.map_key(save_as_f12),
        Some(KeyCommand::Action(Action::SaveAs))
    );
    assert_eq!(
        app.map_key(save_all),
        Some(KeyCommand::Action(Action::SaveAll))
    );
}

#[test]
fn save_as_opens_path_browser() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::SaveAs));
    assert!(matches!(app.overlay, Overlay::Browse(_)));
    if let Overlay::Browse(b) = &app.overlay {
        assert_eq!(b.mode, BrowseMode::SaveAs);
        assert!(!b.filter.is_empty());
    }
}

#[test]
fn save_without_path_opens_save_as() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('x'));
    app.apply(KeyCommand::Action(Action::Save));
    assert!(
        matches!(app.overlay, Overlay::Browse(_)),
        "Ctrl+S em untitled deve abrir Save As"
    );
    if let Overlay::Browse(b) = &app.overlay {
        assert_eq!(b.mode, BrowseMode::SaveAs);
    }
}

#[test]
fn handle_key_ctrl_shift_s_opens_browser() {
    let mut app = App::new_empty();
    app.handle_key(KeyEvent {
        code: KeyCode::Char('s'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert!(
        matches!(app.overlay, Overlay::Browse(_)),
        "handle_key Ctrl+Shift+S deve abrir browser"
    );
}

#[test]
fn map_tab_navigation_keys() {
    use crossterm::event::KeyModifiers;
    let app = App::new_empty();
    let page_up = KeyEvent {
        code: KeyCode::PageUp,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let page_down = KeyEvent {
        code: KeyCode::PageDown,
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let alt_left = KeyEvent {
        code: KeyCode::Left,
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let alt_right = KeyEvent {
        code: KeyCode::Right,
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(page_up),
        Some(KeyCommand::Action(Action::PrevTab))
    );
    assert_eq!(
        app.map_key(page_down),
        Some(KeyCommand::Action(Action::NextTab))
    );
    assert_eq!(
        app.map_key(alt_left),
        Some(KeyCommand::Action(Action::PrevTab))
    );
    assert_eq!(
        app.map_key(alt_right),
        Some(KeyCommand::Action(Action::NextTab))
    );
}

#[test]
fn selection_extend_and_select_all() {
    use crossterm::event::KeyModifiers;
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('a'));
    app.apply(KeyCommand::InsertChar('b'));
    app.apply(KeyCommand::InsertChar('c'));
    app.apply(KeyCommand::Action(Action::MoveLineStart { extend: false }));
    app.apply(KeyCommand::Action(Action::MoveRight { extend: true }));
    app.apply(KeyCommand::Action(Action::MoveRight { extend: true }));
    let doc = app.store.active().unwrap();
    assert_eq!(doc.selected_text(), "ab");

    app.apply(KeyCommand::Action(Action::SelectAll));
    let doc = app.store.active().unwrap();
    assert_eq!(doc.selected_text(), "abc");

    let shift_end = KeyEvent {
        code: KeyCode::End,
        modifiers: KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(shift_end),
        Some(KeyCommand::Action(Action::MoveLineEnd { extend: true }))
    );
    let ctrl_a = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(ctrl_a),
        Some(KeyCommand::Action(Action::SelectAll))
    );
    let ctrl_shift_end = KeyEvent {
        code: KeyCode::End,
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(ctrl_shift_end),
        Some(KeyCommand::Action(Action::MoveDocEnd { extend: true }))
    );
}

#[test]
fn copy_paste_roundtrip_internal() {
    let _clipboard_guard = CLIPBOARD_TEST_LOCK.lock().unwrap();
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('x'));
    app.apply(KeyCommand::InsertChar('y'));
    app.apply(KeyCommand::Action(Action::SelectAll));
    app.apply(KeyCommand::Action(Action::Copy));
    // Não depende do clipboard do SO (pode ter lixo de outras apps)
    assert_eq!(crate::clipboard::internal_text(), "xy");
    app.apply(KeyCommand::Action(Action::MoveDocEnd { extend: false }));
    // Paste via buffer interno se arboard devolver lixo — simula o path interno
    let pasted = crate::clipboard::internal_text();
    let _ = app.store.active_mut().unwrap().insert_text(&pasted);
    let text = app.store.active().unwrap().buffer().as_string();
    assert_eq!(text, "xyxy");
}

#[test]
fn cut_line_removes_complete_crlf_ending() {
    let _clipboard_guard = CLIPBOARD_TEST_LOCK.lock().unwrap();
    let mut app = App::new_empty();
    app.store
        .active_mut()
        .unwrap()
        .insert_text("one\r\ntwo")
        .unwrap();
    app.store
        .active_mut()
        .unwrap()
        .jump_to_byte(oride_core::ByteOffset::new(0));

    app.apply(KeyCommand::Action(Action::Cut));

    assert_eq!(app.store.active().unwrap().buffer().as_string(), "two");
}

#[test]
fn save_as_enter_confirms_with_name() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = App::new_empty();
    app.apply(KeyCommand::InsertChar('z'));
    let mut browser = crate::browser::PathBrowser::new(dir.path(), BrowseMode::SaveAs);
    browser.filter = "out.txt".into();
    app.overlay = Overlay::Browse(browser);
    // Enter no SaveAs com nome → salva
    app.handle_key(KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert!(matches!(app.overlay, Overlay::None));
    assert!(dir.path().join("out.txt").exists());
}

#[test]
fn toggle_scm_and_buffer_picker() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::ToggleScm));
    assert!(app.show_scm);
    assert_eq!(app.focus, Focus::Scm);
    app.apply(KeyCommand::Action(Action::ToggleScm));
    assert!(!app.show_scm);
    app.apply(KeyCommand::Action(Action::BufferPicker));
    assert!(matches!(app.overlay, Overlay::BufferPicker { .. }));
}

#[test]
fn which_key_and_menu_hotkey() {
    let mut app = App::new_empty();
    app.apply(KeyCommand::Action(Action::WhichKey));
    assert!(app.show_which_key);
    app.handle_key(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert!(!app.show_which_key);

    // Alt+F abre menu File
    app.handle_key(KeyEvent {
        code: KeyCode::Char('f'),
        modifiers: KeyModifiers::ALT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert_eq!(app.menu_open, Some((0, 0)));
    app.handle_key(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert!(app.menu_open.is_none());
}

#[test]
fn terminal_ctrl_c_not_stolen_as_copy() {
    let mut app = App::new_empty();
    app.ensure_terminal();
    if app.terminal.is_none() {
        return; // ambiente sem PTY
    }
    if let Some(t) = app.terminal.as_mut() {
        t.visible = true;
    }
    app.focus = Focus::Terminal;
    // Ctrl+C com foco terminal deve ser consumido pelo handler (não Copy)
    app.handle_key(KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert_eq!(app.focus, Focus::Terminal);
    // Esc volta
    app.handle_key(KeyEvent {
        code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    });
    assert_eq!(app.focus, Focus::Editor);
}

#[test]
fn jump_list_records_and_back() {
    use oride_core::ByteOffset;
    let mut jl = crate::jump_list::JumpList::default();
    jl.push(Jump {
        path: None,
        byte: ByteOffset::new(0),
        line: 0,
    });
    jl.push(Jump {
        path: None,
        byte: ByteOffset::new(10),
        line: 1,
    });
    let b = jl.back().expect("back");
    assert_eq!(b.byte, ByteOffset::new(0));
}

#[test]
fn map_ux_actions() {
    let app = App::new_empty();
    let scm = KeyEvent {
        code: KeyCode::Char('g'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(scm),
        Some(KeyCommand::Action(Action::ToggleScm))
    );
    let pick = KeyEvent {
        code: KeyCode::Char('o'),
        modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    assert_eq!(
        app.map_key(pick),
        Some(KeyCommand::Action(Action::BufferPicker))
    );
}

#[test]
fn scroll_follows_cursor_down() {
    let mut app = App::new_empty();
    app.last_editor_height = 5;
    app.last_editor_text_width = 40;
    app.soft_wrap = false;
    // 20 linhas
    for i in 0..20 {
        if i > 0 {
            app.apply(KeyCommand::Action(Action::InsertNewline));
        }
        app.apply(KeyCommand::InsertChar('x'));
    }
    // caret no fim; com height 5 deve scrollar
    app.ensure_cursor_visible();
    let caret = app.store.active().unwrap().caret().unwrap();
    assert!(
        caret.line >= app.scroll_y,
        "caret.line={} scroll={}",
        caret.line,
        app.scroll_y
    );
    assert!(
        caret.line < app.scroll_y + app.last_editor_height,
        "caret saiu da tela: line={} scroll={} height={}",
        caret.line,
        app.scroll_y,
        app.last_editor_height
    );
    // sobe para o topo
    app.apply(KeyCommand::Action(Action::MoveDocStart { extend: false }));
    app.ensure_cursor_visible();
    assert_eq!(app.scroll_y, 0);
}

#[test]
fn scroll_follows_cursor_with_soft_wrap() {
    let mut app = App::new_empty();
    app.last_editor_height = 4;
    app.last_editor_text_width = 10;
    app.soft_wrap = true;
    // uma linha longa (3 rows visuais) + várias curtas
    let long = "abcdefghij".repeat(3); // 30 chars → 3 visual rows
    for c in long.chars() {
        app.apply(KeyCommand::InsertChar(c));
    }
    app.apply(KeyCommand::Action(Action::InsertNewline));
    for _ in 0..10 {
        app.apply(KeyCommand::InsertChar('z'));
        app.apply(KeyCommand::Action(Action::InsertNewline));
    }
    app.ensure_cursor_visible();
    let caret = app.store.active().unwrap().caret().unwrap();
    let visual = visual_row_of_caret(
        app.store.active().unwrap().buffer(),
        app.scroll_y,
        caret.line,
        caret.column,
        app.last_editor_text_width,
        true,
    );
    assert!(
        visual < app.last_editor_height,
        "soft-wrap: caret visual={visual} height={} scroll={}",
        app.last_editor_height,
        app.scroll_y
    );
}

#[test]
fn scroll_origin_helpers() {
    let buf = oride_core::Buffer::from_text("aa\nbb\ncc\ndd\nee\n");
    assert_eq!(line_visual_height("hello", 10), 1);
    assert_eq!(line_visual_height("hello world!!", 5), 3);
    let origin = scroll_origin_for_caret(&buf, 4, 0, 3, 40, false);
    assert_eq!(origin, 2); // lines 2,3,4 visible
}

#[test]
fn lsp_offsets_use_utf16_columns() {
    let buffer = oride_core::Buffer::from_text("a😀b");

    let offset = lsp_position_to_offset(
        &buffer,
        LspPos {
            line: 0,
            character: 3,
        },
    );

    assert_eq!(offset, Some(oride_core::ByteOffset::new(5)));
}

#[test]
fn completion_prefix_supports_unicode_identifiers() {
    assert_eq!(identifier_prefix_at("let café", 9), (4, "café"));
    assert_eq!(identifier_prefix_at("value.other", 11), (6, "other"));
    assert!(completion_label_matches("ori.string", "st"));
    assert!(completion_label_matches("Vec::with_capacity", "with"));
}

#[test]
fn new_tab_and_close_tab_synchronize_split_doc() {
    let mut app = App::new_empty();
    let first_id = app.store.active_id().unwrap();
    assert_eq!(app.split.focused_pane().doc_id, first_id);

    // Abre nova aba
    app.apply_action(Action::NewTab).unwrap();
    let second_id = app.store.active_id().unwrap();
    assert_ne!(first_id, second_id);
    assert_eq!(app.split.focused_pane().doc_id, second_id);

    // Fecha a nova aba
    app.apply_action(Action::CloseTab).unwrap();
    assert_eq!(app.store.active_id(), Some(first_id));
    assert_eq!(app.split.focused_pane().doc_id, first_id);
}

#[test]
fn multi_lsp_command_resolution_and_fail_closed() {
    let mut app = App::new_empty();

    assert_eq!(
        app.lsp_command_for(oride_syntax::LanguageId::Rust),
        Some(vec!["rust-analyzer".to_string()])
    );
    assert_eq!(
        app.lsp_command_for(oride_syntax::LanguageId::Python),
        Some(vec!["pylsp".to_string()])
    );
    assert_eq!(
        app.lsp_command_for(oride_syntax::LanguageId::D),
        Some(vec!["serve-d".to_string()])
    );
    assert_eq!(
        app.lsp_command_for(oride_syntax::LanguageId::Lua),
        Some(vec!["lua-language-server".to_string()])
    );

    // Custom override in config
    app.config.lsp.servers.insert(
        "rust".to_string(),
        vec!["custom-ra".to_string(), "--stdio".to_string()],
    );
    assert_eq!(
        app.lsp_command_for(oride_syntax::LanguageId::Rust),
        Some(vec!["custom-ra".to_string(), "--stdio".to_string()])
    );

    // Fail-closed behavior on missing binary
    let result = app.ensure_lsp_client(oride_syntax::LanguageId::Rust);
    assert!(result.is_err());
    assert!(app
        .ensure_lsp_client(oride_syntax::LanguageId::Rust)
        .is_err());
}

#[test]
fn theme_picker_live_preview_and_apply() {
    let mut app = App::new_empty();
    let initial_theme = app.config.theme.clone();

    // Trigger action
    app.apply(KeyCommand::Action(Action::SelectTheme));
    assert!(matches!(app.overlay, Overlay::ThemePicker { .. }));

    // Arrow down previews next theme
    app.handle_key(key(KeyCode::Down));
    let preview_theme = app.config.theme.clone();
    assert_ne!(preview_theme, "");

    // Esc cancels and reverts back to initial theme
    app.handle_key(key(KeyCode::Esc));
    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.config.theme, initial_theme);

    // Re-open and apply Tokyo Night
    app.apply(KeyCommand::Action(Action::SelectTheme));
    // Type 't' 'o' 'k' 'y' 'o'
    for ch in "tokyo".chars() {
        app.handle_key(key(KeyCode::Char(ch)));
    }
    app.handle_key(key(KeyCode::Enter));
    assert_eq!(app.overlay, Overlay::None);
    assert!(app.config.theme.to_lowercase().contains("tokyo"));
}

#[test]
fn dynamic_languages_and_lsp_configuration() {
    let mut config = Config::default();
    config.languages.push(oride_config::LanguageConfig {
        id: "zig".into(),
        name: Some("Zig".into()),
        extensions: vec!["zig".into(), "zon".into()],
        filenames: vec!["build.zig".into()],
        line_comment: Some("// ".into()),
        block_comment_close: None,
        lsp_command: vec!["zls".into()],
        tab_size: Some(4),
        insert_spaces: Some(true),
        soft_wrap: Some(false),
        completion_words: vec!["pub".into(), "fn".into(), "const".into()],
    });

    let mut store = DocumentStore::new();
    let temp_dir = std::env::temp_dir();
    let zig_file = temp_dir.join("main.zig");
    let id = store.open_empty();
    store.get_mut(id).unwrap().set_path(zig_file);
    store
        .get_mut(id)
        .unwrap()
        .insert_text("const x = 42;")
        .unwrap();

    let mut app = App::from_store_with_config(store, config, temp_dir);

    // Dynamic language detection
    let lang = app.active_language();
    assert_eq!(lang.as_str(), "zig");

    // Dynamic comment toggle
    app.apply(KeyCommand::Action(Action::ToggleComment));
    let text = app.store.active().unwrap().buffer().as_string();
    assert!(text.contains("// const x = 42;"));

    // Dynamic completion choices
    let completions = app.offline_completion_choices(lang, "co");
    assert!(completions.iter().any(|c| c.insert_text == "const"));

    // Dynamic LSP command resolution
    let lsp_cmd = app.lsp_command_for(lang);
    assert_eq!(lsp_cmd, Some(vec!["zls".to_string()]));
}

#[test]
fn locale_picker_selection_and_switch() {
    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app = App::from_store_with_config(store, Config::default(), std::env::temp_dir());

    assert_eq!(app.locale, oride_i18n::Locale::PtBr);
    assert_eq!(app.menus[0].title, "Arquivo");

    // Open locale picker
    app.apply(KeyCommand::Action(Action::SelectLocale));
    assert!(matches!(app.overlay, Overlay::LocalePicker { .. }));

    // Navigate to English and press Enter
    // Filter by typing 'en'
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('e'),
        crossterm::event::KeyModifiers::NONE,
    ));
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('n'),
        crossterm::event::KeyModifiers::NONE,
    ));
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Enter,
        crossterm::event::KeyModifiers::NONE,
    ));

    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.locale, oride_i18n::Locale::EnUs);
    assert_eq!(app.config.locale, "en-US");
    assert_eq!(app.menus[0].title, "File");
    assert_eq!(app.menus[1].title, "Edit");
    assert_eq!(app.menus[2].title, "View");

    // Reopen and switch back to Portuguese
    app.apply(KeyCommand::Action(Action::SelectLocale));
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('p'),
        crossterm::event::KeyModifiers::NONE,
    ));
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('t'),
        crossterm::event::KeyModifiers::NONE,
    ));
    app.handle_overlay_key(crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Enter,
        crossterm::event::KeyModifiers::NONE,
    ));

    assert_eq!(app.overlay, Overlay::None);
    assert_eq!(app.locale, oride_i18n::Locale::PtBr);
    assert_eq!(app.config.locale, "pt-BR");
    assert_eq!(app.menus[0].title, "Arquivo");
}

#[test]
fn external_plugin_discovery_and_palette_execution() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let plugin_dir = temp_dir.path().join(".oride/plugins/mock-plugin");
    std::fs::create_dir_all(&plugin_dir).expect("create_dir_all");

    let manifest_toml = r#"
[plugin]
name = "mock-tool"
version = "0.1.0"
description = "Mock external tool"

[[commands]]
id = "mock_echo"
label = "Plugin: mock echo"
description = "Echoes message"
executable = "echo"
args = ["executed_for", "$WORKSPACE"]
"#;
    std::fs::write(plugin_dir.join("plugin.toml"), manifest_toml).expect("write manifest");

    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app =
        App::from_store_with_config(store, Config::default(), temp_dir.path().to_path_buf());

    // Check command palette includes external plugin command
    let items = app.command_palette_items("mock");
    assert!(items.iter().any(|item| item.contains("Plugin: mock echo")));

    // Run command directly
    app.run_plugin_command("mock_echo");
    assert!(app
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("mock-tool: executed_for"));
}

#[test]
fn vim_modal_editing_mode_and_command_line() {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    let temp_dir = tempfile::tempdir().expect("tempdir");
    let mut config = Config::default();
    config.editor.modal_mode = true;

    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app = App::from_store_with_config(store, config, temp_dir.path().to_path_buf());

    assert!(app.vim.is_some());
    assert_eq!(
        app.vim.as_ref().unwrap().mode,
        crate::modal::VimMode::Normal
    );

    // Normal mode: typing 'i' switches to Insert mode
    app.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(
        app.vim.as_ref().unwrap().mode,
        crate::modal::VimMode::Insert
    );

    // Insert mode: typing 'h', 'e', 'l', 'l', 'o' writes into buffer
    for c in "hello".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(app.store.active().unwrap().buffer().as_string(), "hello");

    // Esc returns to Normal mode
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(
        app.vim.as_ref().unwrap().mode,
        crate::modal::VimMode::Normal
    );

    // Normal mode: ':' opens VimCommand overlay
    app.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    assert!(matches!(app.overlay, Overlay::VimCommand { .. }));

    // Type 'health' and Enter -> opens HealthCheck
    for c in "health".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(matches!(app.overlay, Overlay::HealthCheck { .. }));

    // Esc closes HealthCheck
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.overlay, Overlay::None);

    // Toggle modal mode off
    app.apply(KeyCommand::Action(Action::ToggleModal));
    assert!(app.vim.is_none());
}

#[test]
fn task_runner_discovery_and_execution() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let oride_dir = temp_dir.path().join(".oride");
    std::fs::create_dir_all(&oride_dir).expect("create .oride dir");

    let tasks_content = r#"
[[tasks]]
name = "build"
command = "cargo build --manifest-path $WORKSPACE/Cargo.toml"
run_in = "background"
key = "F5"
"#;
    std::fs::write(oride_dir.join("tasks.toml"), tasks_content).expect("write tasks.toml");

    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app =
        App::from_store_with_config(store, Config::default(), temp_dir.path().to_path_buf());

    // Execute task by name
    app.execute_task_runner(Some("build"));
    assert!(app
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("tarefa em background: build"));

    // Shortcut F5 also triggers it
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    app.handle_key(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE));
    assert!(app
        .status_message
        .as_deref()
        .unwrap_or("")
        .contains("tarefa em background: build"));
}

#[test]
fn health_check_overlay_inspection() {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    let temp_dir = tempfile::tempdir().expect("tempdir");
    let mut store = DocumentStore::new();
    store.open_empty();
    let mut app =
        App::from_store_with_config(store, Config::default(), temp_dir.path().to_path_buf());

    app.apply(KeyCommand::Action(Action::HealthCheck));
    assert!(matches!(app.overlay, Overlay::HealthCheck { .. }));

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.overlay, Overlay::None);
}
