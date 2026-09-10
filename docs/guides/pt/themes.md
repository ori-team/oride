# Guia de Desenvolvimento de Temas no Oride

O **Oride** suporta temas visuais completos em formato declarativo **TOML**, com suporte a **Live Preview** em tempo real na interface e carregamento dinâmico sem necessidade de recompilação.

---

## 1. Onde colocar seu arquivo de tema

O Oride escaneia automaticamente os seguintes diretórios na inicialização:

1. **Global do usuário:** `~/.config/oride/themes/<seu-tema>.toml`
2. **Local do projeto/workspace:** `<workspace>/.oride/themes/<seu-tema>.toml`
3. **Temas embutidos no binário:** `default-dark`, `dracula`, `nord`, `one-dark`, `tokyo-night`, `catppuccin-mocha`, `monokai`, `solarized-light`, `solarized-dark`, `github-dark`.

Qualquer arquivo `.toml` colocado nessas pastas torna-se imediatamente disponível no seletor de temas!

---

## 2. Estrutura do Arquivo `meu-tema.toml`

Um tema é composto por metadados básicos (`name`, `is_dark`), um bloco `[ui]` (para cores de interface e editor) e um bloco `[syntax]` (para o destaque de código Tree-Sitter e léxico).

### Exemplo Completo: Tokyo Night Dark

```toml
name = "Tokyo Night"
is_dark = true

[ui]
background = "#1a1b26"
foreground = "#c0caf5"
line_number = "#565f89"
cursor_bg = "#c0caf5"
cursor_fg = "#1a1b26"
status_bg = "#16161e"
status_fg = "#7aa2f7"
status_dirty = "#e0af68"
gutter_width = 5

[syntax]
comment = "#565f89"
keyword = "#bb9af7"
string = "#9ece6a"
number = "#ff9e64"
type_name = "#2ac3de"
function = "#7aa2f7"
operator = "#89ddff"
punctuation = "#c0caf5"
variable = "#c0caf5"
constant = "#ff9e64"
property = "#7dcfff"
tag = "#f7768e"
attribute = "#bb9af7"
heading = "#7aa2f7"
emphasis = "#e0af68"
strong = "#ff9e64"
link = "#7dcfff"
code = "#7aa2f7"
list_marker = "#bb9af7"
quote = "#565f89"
```

---

## 3. Tabela de Referência de Tokens

### Bloco `[ui]` (Interface do Editor)

| Campo | Tipo | Descrição |
|---|---|---|
| `background` | Hex / Nome | Cor de fundo principal do editor e janelas |
| `foreground` | Hex / Nome | Cor de texto padrão |
| `line_number` | Hex / Nome | Cor dos números de linha na calha (gutter) |
| `cursor_bg` | Hex / Nome | Cor do bloco do cursor |
| `cursor_fg` | Hex / Nome | Cor do caractere sob o cursor |
| `status_bg` | Hex / Nome | Fundo da barra de status no rodapé |
| `status_fg` | Hex / Nome | Texto da barra de status |
| `status_dirty` | Hex / Nome | Indicador de alterações não salvas (`*` / dirty) |
| `gutter_width` | Inteiro | Largura mínima da coluna de números de linha (ex: `5`) |

### Bloco `[syntax]` (Coloração de Sintaxe)

| Token | Descrição | Exemplos de uso |
|---|---|---|
| `keyword` | Palavras-chave da linguagem | `fn`, `let`, `if`, `return`, `pub`, `struct` |
| `function` | Nomes de funções e métodos | `main`, `push`, `calculate_total` |
| `type_name` | Tipos primitivos e structs/classes | `i32`, `String`, `Option`, `User` |
| `string` | Literais de texto entre aspas | `"olá mundo"`, `'c'` |
| `number` | Literais numéricos | `42`, `3.14`, `0xFF` |
| `comment` | Comentários de linha e bloco | `// comentário`, `/* bloco */`, `# bash` |
| `operator` | Operadores matemáticos e lógicos | `+`, `-`, `*`, `==`, `&&`, `\|\|` |
| `punctuation` | Pontuação e delimitadores | `(`, `)`, `{`, `}`, `;`, `,` |
| `variable` | Variáveis locais e parâmetros | `self`, `index`, `payload` |
| `constant` | Constantes e valores imutáveis globais | `MAX_SIZE`, `PI` |
| `property` | Campos de structs e propriedades | `user.name`, `config.theme` |
| `tag` | Tags de marcação (HTML/XML/TSX) | `<div>`, `<span>`, `<main>` |
| `attribute` | Atributos de marcação e decoradores | `class`, `href`, `#[derive]` |
| `heading` | Títulos Markdown | `# Título`, `## Subtítulo` |
| `emphasis` | Itálico Markdown | `*itálico*` |
| `strong` | Negrito Markdown | `**negrito**` |
| `link` | Links Markdown | `[texto](url)` |
| `code` | Código inline em Markdown | `` `código` `` |
| `list_marker` | Marcadores de listas Markdown | `-`, `*`, `1.` |
| `quote` | Citações Markdown | `> citação em bloco` |

---

## 4. Cores Suportadas

Você pode especificar cores de duas maneiras:

1. **Hexadecimal de 24 bits (TrueColor):**
   - `#1a1b26`, `#ff79c6`, `#50fa7b`, `#f8f8f2`
2. **Cores ANSI nomeadas (compatíveis com terminais clássicos):**
   - `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
   - `dark_gray`, `light_red`, `light_green`, `light_yellow`, `light_blue`, `light_magenta`, `light_cyan`

---

## 5. Como Testar e Aplicar com Live Preview

1. Salve o arquivo em `~/.config/oride/themes/meu-tema.toml`.
2. Abra o Oride no terminal:
   ```bash
   oride .
   ```
3. Abra o Seletor de Temas:
   - Pelo menu: clique em `Exibir → Tema de cores…`
   - Pelo teclado: aperte `Ctrl+Shift+P` → selecione `Preferências: Tema de Cores (Live Preview)`
4. Navegue com as setas `↑` e `↓`:
   - Conforme você navega, o Oride **aplica o tema na hora** (Live Preview) para você avaliar o contraste e as cores.
5. Aperte `Enter` para confirmar (o tema é salvo automaticamente em `~/.config/oride/config.toml`) ou `Esc` para voltar ao tema anterior.

---

## 6. Sobrescrita Direta no `config.toml`

Se você gostar de um tema existente (ex: `dracula`), mas quiser apenas mudar a cor do cursor ou do comentário, pode sobrescrever chaves pontuais diretamente em `~/.config/oride/config.toml`:

```toml
theme = "dracula"

[theme_ui]
cursor_bg = "#ff5555"

[syntax]
comment = "#8be9fd"
```
