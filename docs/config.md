# Configuração (P0.3)

Oride carrega TOML em camadas (depois sobrescreve o anterior):

1. **Defaults embutidos**
2. **Usuário:** `~/.config/oride/config.toml` (XDG)
3. **Projeto:** primeiro `.oride/config.toml` encontrado subindo a partir do arquivo aberto (ou do CWD)

Exemplo completo: [`assets/config.example.toml`](../assets/config.example.toml).

## Campos

| Campo | Tipo | Default | Efeito |
|-------|------|---------|--------|
| `theme` | string | `"default"` | Nome lógico (cores em `[ui]`) |
| `show_line_numbers` | bool | `true` | Gutter |
| `mouse` | bool | **`false`** | Captura de mouse (clique/drag/scroll). Off por default; também **View → Enable mouse** ou palette |
| `[editor].tab_size` | u8 | `4` | Largura do Tab com espaços |
| `[editor].insert_spaces` | bool | `true` | Tab → espaços |
| `[editor].completion_auto` | bool | `true` | Abre sugestões locais durante a digitação |
| `[editor].completion_min_chars` | u8 | `2` | Prefixo mínimo para sugestões automáticas |
| `[markdown].terminal_images` | bool | `false` | Detecção e exibição de gráficos de terminal (Kitty/Ghostty/WezTerm) |
| `[ui].*` | cor | ver defaults | Tema TUI |
| `[keys]` | map | bindings P0.2 | Rebind de ações |

### Cores

- Nomes: `reset`, `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `darkgray`, `white`, `lightred`, …
- Hex: `#RGB` ou `#RRGGBB`

### Actions (`[keys]`)

| Id | Comportamento |
|----|----------------|
| `quit` | Sair (2× se dirty) |
| `save` | Salvar path atual |
| `undo` / `redo` | Pilha de edits |
| `insert_newline` / `insert_tab` / `backspace` / `delete` | Edição |
| `move_left` … `move_line_end` | Movimento |
| `move_*_extend` | Movimento com seleção |
| `page_up` / `page_down` | Página |
| `toggle_tree` / `toggle_terminal` | Painéis |
| `focus_tree` / `focus_editor` / `focus_terminal` | Foco |
| `next_tab` / `prev_tab` / `close_tab` / `new_tab` | Tabs |
| `command_palette` / `open_file_fuzzy` | Palette |
| `tree_new_file` / `tree_new_dir` / `tree_refresh` | Árvore |

Chords: `ctrl+s`, `shift+left`, `esc`, `pageup`, `ctrl+shift+p`, … (minúsculas, `+` como separador).

Defaults: `ctrl+b` foco árvore, `ctrl+e` foco editor, `ctrl+o` abrir pasta,
`ctrl+shift+b` mostrar/ocultar árvore, `ctrl+\`` terminal, `ctrl+p` arquivo,
`ctrl+shift+p` comandos.

## Validação

```bash
# Rebind em teste de unidade: crates/oride-app
cargo test -p oride-app rebind_ctrl_s
cargo test -p oride-config
cargo test -p oride-keymap
```

## Seções P4+

### `[editor]`
- `tab_size`, `insert_spaces`, `format_on_save`, `use_editorconfig`
- `completion_auto`, `completion_min_chars`

### `[tree]`
- `width`, `show_hidden`, `git_status`

### `[terminal]`
- `shell` (vazio = `$SHELL`), `default_height`

### `[lsp]`
- `enabled`, `oriscript_command` (compatibilidade), `timeout_ms`
- `[lsp.servers]`: mapa `LanguageId → argv`; os processos são iniciados sob
  demanda. OriScript usa `oriscript lsp` e Ori usa `ori-lsp` por default.

```toml
[lsp.servers]
rust = ["rust-analyzer"]
python = ["pylsp"]
```

### `[syntax]`
Cores de highlight (`keyword`, `string`, `comment`, …) — nomes ou `#RRGGBB`.

### Find
- `Alt+R` alterna **regex**; `Alt+C` case; `Alt+A` acentos.

### LSP atalhos
- Sugestões locais aparecem automaticamente após dois caracteres; `↑↓`
  seleciona e `Enter`/`Tab` aceita
- `Ctrl+Space` combina completion semântica do LSP com o fallback local
- `Ctrl+K` hover · `F4` goto · `Ctrl+Shift+I` format
- `Ctrl+Shift+M` painel de diagnostics · `Ctrl+R` reload arquivo
