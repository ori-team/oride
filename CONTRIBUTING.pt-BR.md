# Contribuindo com o Oride

[English](CONTRIBUTING.md) · **Português**

Agradecemos pelo seu interesse em contribuir com o **Oride**! O Oride é um editor de código e mini-IDE de terminal leve, modular e extensível construído em Rust.

Contribuições de todas as naturezas são muito bem-vindas: correções de bugs, melhorias de performance, suporte a novas linguagens/sintaxes, aprimoramento de documentação, novos temas e refinamentos de arquitetura.

---

## 🧭 Invariantes Arquiteturais Fundamentais

Antes de iniciar a codificação, conheça as invariantes centrais que orientam todas as decisões do projeto:

1. **Core sem Interface Gráfica (Headless):** Os crates `oride-core`, `oride-config` e `oride-keymap` devem permanecer 100% testáveis sem TTY. Nunca importe `ratatui` ou `crossterm` nos crates de core. Lógicas de renderização e desenho pertencem exclusivamente a `oride-ui` ou `oride-app`.
2. **Ações como Dados:** Teclas, acordes e itens da Command Palette disparam ações nomeadas e tipadas (`enum Action`), em vez de mutar estado diretamente dentro de manipuladores de eventos de entrada.
3. **Falhas Controladas (Fail-Closed):** Subsistemas como servidores LSP, terminais PTY e processos Git devem falhar de forma elegante com avisos na barra de status — nunca causar pânico (`panic!`) ou encerrar abruptamente o editor.
4. **LSP Desacoplado via `$PATH`:** Servidores de linguagem são executados dinamicamente via binários presentes no `$PATH` (`rust-analyzer`, `clangd`, etc.), mantendo o editor livre de acoplamento estático a compiladores.
5. **Produto Contido e Anti-Bloat:** Todos os recursos operam estritamente dentro do terminal e do processo do Oride. Evitamos dependências externas desnecessárias, renderizadores baseados em navegador ou macros pesadas.
6. **Um Conceito por PR:** Mantenha cada Pull Request focado em um único propósito ou fatia funcional, seguindo as fases definidas em [`docs/design.md`](docs/design.md).

---

## 🛠️ Ambiente de Desenvolvimento & Pré-requisitos

### Requisitos
- **Rust Toolchain:** Stable 1.80+ (`rustup default stable`)
- **Componentes Cargo:** `rustfmt` e `clippy` (`rustup component add rustfmt clippy`)
- **Git**

### Compilando e Executando Localmente

```bash
# Clonar o repositório
git clone https://github.com/ori-team/oride.git
cd oride

# Compilar em modo debug
cargo build

# Executar a demonstração rápida de smoke test
cargo run -p oride -- --demo

# Executar toda a suíte de testes automatizados
cargo test --workspace

# Iniciar o editor no diretório atual
cargo run -p oride -- .
```

---

## 🔄 Fluxo de Trabalho e Padrões

### 1. Nomenclatura de Branches
Crie uma branch de trabalho a partir da `main`:
- `feat/nome-da-feature` — Novas funcionalidades e adições
- `fix/descricao-do-bug` — Correções de defeitos e bugs
- `docs/tema-da-doc` — Atualizações de documentação e guias
- `refactor/escopo` — Refatorações de código sem alteração comportamental
- `chore/nome-da-tarefa` — Ajustes de ferramentas, CI ou dependências

### 2. Desenvolvimento Orientado a Testes (TDD)
- Ao corrigir um bug, escreva primeiro um teste automatizado que reproduza a falha e implemente a correção em seguida para fazê-lo passar.
- Ao adicionar um recurso, inclua testes unitários e de integração validando contratos e casos de borda.
- Os testes devem ser determinísticos, rápidos e isolados.

### 3. Conventional Commits
Todas as mensagens de commit devem seguir o padrão [Conventional Commits](https://www.conventionalcommits.org/pt-br/):

```text
tipo(escopo): descrição no imperativo

[corpo opcional explicando o contexto e decisões]

[rodapé opcional com referências a issues]
```

**Exemplos:**
- `feat(modal): adicionar movimentos de navegacao estilo vim`
- `fix(lsp): tratar spans multilinhas em edicoes utf-16`
- `docs(guides): adicionar tutorial de desenvolvimento de temas`
- `refactor(search): otimizar conversao de filtros glob no ripgrep`

---

## 🧪 Quality Gates Mandatórios

Antes de abrir um Pull Request, seu código deve passar com 100% de sucesso nas três verificações locais:

```bash
# 1. Verificação estrita de formatação
cargo fmt --all -- --check

# 2. Verificação estrita de linter (zero avisos tolerados)
cargo clippy --all-targets -- -D warnings

# 3. Execução completa dos testes em todos os crates
cargo test --workspace
```

---

## 📚 Documentação Canônica Viva e Sincronização Bilíngue

O Oride adota a disciplina de **Documentação Viva**:

1. **`CHANGELOG.md`:** Para qualquer alteração técnica ou funcional, adicione uma entrada sob `## Unreleased` (`### Added`, `### Changed`, `### Fixed` ou `### Removed`).
2. **Guias de Usuário:** Localizados em `docs/guides/`:
   - Português: [`docs/guides/pt/`](docs/guides/pt/)
   - Inglês: [`docs/guides/en/`](docs/guides/en/)
   Se você alterar opções de configuração ou funcionalidades de uso, atualize ambas as versões para manter a paridade.
3. **Especificações Internas:** Documentos de arquitetura, testes e segurança residem em [`docs/`](docs/) e têm seu espelho em [`docs/en/`](docs/en/).

---

## 🚀 Enviando um Pull Request

1. Envie sua branch para o GitHub:
   ```bash
   git push origin feat/sua-feature
   ```
2. Abra um Pull Request contra a branch `main`.
3. Descreva claramente a motivação da alteração, decisões técnicas e cobertura de testes adicionada.
4. Verifique se todas as etapas do pipeline de CI foram concluídas com sucesso.
5. Acompanhe a revisão de código. O merge é realizado utilizando a estratégia `squash`.

Muito obrigado por contribuir com a evolução do **Oride**!
