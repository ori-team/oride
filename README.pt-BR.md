# Oride

[English](README.md) · **Português**

**Oride** (Ori + IDE) é um **editor de código e mini-IDE de terminal** modular, leve e extensível construído em Rust, apresentando uma árvore de projeto navegável, terminal embutido colapsável, edição modal estilo Vim, executor de tarefas integrado (`tasks.toml`), diagnósticos de ambiente (`:health`) e suporte sintático para múltiplas linguagens (Rust, C, Bash, Markdown, Ori, HTML, CSS, JavaScript/TypeScript, Python, D, Lua, Nim e Ruby).

Status: **`0.2.0`** — mini-IDE TUI contida (editor, árvore de arquivos, terminal PTY, git/SCM com ahead/behind e pull/push, busca & substituição com globs e regex, multi-LSP sob demanda, preview de Markdown com imagens e links, splits dinâmicos, mouse opt-in).  
Repositório: [ori-team/oride](https://github.com/ori-team/oride).  
Documentação: [Manual do Usuário](docs/guia-de-uso.md) · [Design](docs/design.md) · [Configuração](docs/config.md) · [Temas](docs/themes.md) · [Roadmap](docs/planning/alpha6-roadmap.md).

## Objetivos (Produto Contido)

- **Tudo no TUI:** Sem dependência de browser externo para edição, sem inchaço (anti-bloat), sem macros complexas e sem scripts lentos.
- **Estrutura Completa:** Múltiplas abas, árvore de arquivos com ícones Nerd Fonts, terminal PTY interativo, busca e substituição no buffer e no projeto, status de SCM/Git e sessões leves.
- **Linguagens Suportadas:** Rust, C, Bash, Markdown, Ori, HTML, CSS, JS/TS, Python, D, Lua, Nim e Ruby — detecção automática, realce sintático, toggle de comentários e injeção de fences em Markdown ([detalhes](docs/syntax.md)).
- **LSP sob demanda:** Autocomplete local inteligente + servidores LSP iniciados sob demanda para **Ori** (`ori-lsp`), Rust (`rust-analyzer`), C/C++ (`clangd`), Bash (`bash-language-server`), etc. (totalmente configuráveis via `config.toml`).
- **Markdown Rico no Terminal:** Tabelas desenhadas com caracteres Unicode, blocos de código com realce sintático e imagens renderizadas em terminais compatíveis (Kitty/Ghostty/WezTerm).
- **Links no Preview:** Abertura no navegador do sistema via clique de mouse ou `Alt+Enter`.
- **Mouse Opt-in:** Desativado por padrão (`mouse = false`); quando ativado, permite posicionamento de cursor, seleção por arrasto e redimensionamento de splits.

## Compilação e Execução

```bash
cargo build --release
./scripts/install.sh                    # Instala em ~/.local/bin/oride
./target/release/oride                  # CWD como workspace + buffer vazio
./target/release/oride caminho/arquivo  # Abre arquivo diretamente
./target/release/oride caminho/pasta    # Abre pasta como workspace
./target/release/oride --version
```

### Principais Atalhos de Teclado (Reconfiguráveis via TOML)

| Tecla | Ação |
|---|---|
| Digitação / Enter / Backspace / Delete | Edição direta de texto |
| Setas, Home, End, PgUp, PgDn | Movimentação de cursor |
| `Shift` + Setas / Home / End | Extensão de seleção |
| `Ctrl+Shift+Home` / `End` | Seleciona até o início/fim do documento |
| `Ctrl+A` | Selecionar tudo |
| `Ctrl+S` | Salvar arquivo |
| `Ctrl+Shift+S` / `F12` / `Alt+Shift+S` | **Salvar como…** (navegador de arquivos integrado) |
| `Ctrl+Alt+S` | Salvar todas as abas |
| `Ctrl+Z` / `Ctrl+Y` | Desfazer / Refazer |
| `Ctrl+N` / `Ctrl+W` | Nova aba / Fechar aba atual |
| `Ctrl+PgUp` / `Ctrl+PgDn` / `Alt+←` / `Alt+→` | Aba anterior / próxima aba |
| `Ctrl+B` / `Ctrl+E` | Focar na árvore de arquivos / Focar no editor |
| `Ctrl+O` | **Abrir pasta** como workspace (`F2` confirma) |
| `Ctrl+P` | **Abrir arquivo** |
| `Ctrl+"` / `Ctrl+'` / `Ctrl+\`` | Alternar **terminal embutido** |
| `Ctrl+Shift+G` | **Painel SCM/Git** (`s` stage · `u` unstage · `c` commit · `d` diff) |
| `Ctrl+Shift+O` | **Buffer picker** (alternador rápido de abas) |
| `Alt+F/E/V/G/I/H` | Barra de menus (*File*, *Edit*, *View*, *Go*, *Git*, *Help*) |
| `Alt+/` | Which-key (resumo dos atalhos principais) |
| `F1` / `Ctrl+G` / `Ctrl+Shift+/` | Listar todos os atalhos com filtro |
| `F2` | Git diff do arquivo ativo |
| Digitar 2+ caracteres | Sugestões de autocomplete local automático |
| `Ctrl+Space` / `Ctrl+K` / `F4` | LSP autocomplete / hover / ir para definição |
| `Ctrl+Shift+I` / `Ctrl+Shift+M` | Formatação LSP / painel de diagnósticos |
| `Alt+=` / `Alt+-` | Aumentar / diminuir altura do terminal |
| `Ctrl+R` | Recarregar arquivo do disco |
| `Ctrl+Shift+F` | **Localizar e Substituir no Projeto** (busca recursiva) |
| `Ctrl+Shift+V` / `Alt+P` | **Preview de Markdown** em tempo real |
| `Ctrl+Alt+V` / `Ctrl+Alt+H` | Dividir editor verticalmente / horizontalmente |
| `F6` / `Ctrl+Alt+W` | Próximo painel dividido / fechar painel |
| `Ctrl+Alt+↑/↓` / `U` | Adicionar / limpar multi-cursores |
| `Ctrl+F` / `F3` | Localizar no buffer / próxima ocorrência |
| `Ctrl+H` | Substituir no buffer |
| `Alt+C` / `Alt+A` / `Alt+W` / `Alt+R` | Alternar Case Sensitive / Sem Acentos / Palavra Inteira / Regex |
| `Alt+Enter` / `Ctrl+Alt+Enter` | Substituir ocorrência atual / Substituir todas |
| `Ctrl+C` / `Ctrl+V` / `Ctrl+X` | Copiar / Colar / Recortar |
| `Alt+Z` | Quebra de linha suave (*Soft wrap*) |
| `Ctrl+/` | Comentar / descomentar linha |
| `Esc` ou `Ctrl+Q` | Fechar modal sobreposto / Sair |

### Navegação nos Painéis
- **Navegador de Arquivos (`Ctrl+O` / `Ctrl+P`):** Destaque em ciano indica a seleção · `↑↓` navegam · `Enter` entra/abre · digitação filtra a lista.
- **Árvore de Projeto:** `↑↓` ou `jk` navegam · `Enter` abre/expande pasta · `r` renomeia · `d` deleta · `y`/`c` copia o caminho · `Tab`/`Esc` retorna o foco ao editor.
- **Terminal PTY:** Shell interativo completo; `Ctrl+C` é enviado ao shell quando o terminal está focado; `Esc` retorna ao editor.
- **Modo Modal Vim:** Digite `:normal` para habilitar navegação modal (`h`, `j`, `k`, `l`, `w`, `b`, `gg`, `G`, `x`, `u`, `:w`, `:q`, `:tasks`, `:health`).

### Configuração

```bash
mkdir -p ~/.config/oride
cp assets/config.example.toml ~/.config/oride/config.toml

# Configuração local específica para o projeto:
mkdir -p .oride && cp assets/config.example.toml .oride/config.toml
```

Consulte [`docs/config.md`](docs/config.md) para todos os detalhes de configuração.

## Layout do Workspace

```text
crates/
  oride-core/     # buffers rope, abas de documentos, histórico de undo
  oride-config/   # carregamento e merge hierárquico de TOML
  oride-keymap/   # mapa de acordes de teclado e disparo de ações
  oride-fs/       # árvore de arquivos, criação/remoção, ícones
  oride-git/      # integração com git porcelain e status
  oride-terminal/ # painel embutido PTY
  oride-syntax/   # motor de realce sintático Tree-Sitter e fallback léxico
  tree-sitter-oriscript/  # gramática legada vendored
  oride-ui/       # widgets e renderização Ratatui
  oride-app/      # composição da aplicação e loop de eventos
  oride/          # executável binário CLI
docs/
  guia-de-uso.md  # manual prático do usuário
  design.md       # arquitetura do projeto
  config.md       # especificação do config.toml
  themes.md       # guia para criação de temas visuais
```

## Licença

MIT — consulte [LICENSE](LICENSE).
