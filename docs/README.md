# Documentação Canônica do Projeto (`docs/`)

🌐 **Idiomas / Languages:** **Português** · [English (`docs/en/`)](en/README.md)

---

## O que é este diretório?
A pasta `docs/` é a fonte canônica da verdade para todas as especificações de engenharia, produto, arquitetura, testes e governança do projeto **Oride**.

## Para que serve?
Implementa o princípio de **Documentação Canônica Viva**: o repositório é autossuficiente e todo o conhecimento técnico essencial reside diretamente no código e em arquivos Markdown padronizados.

## Guias Rápidos
- 📖 [**Manual do Usuário**](guia-de-uso.md) — Instalação, modos de edição (padrão e modal Vim), splits, atalhos e task runner.
- 🎨 [**Guia de Desenvolvimento de Temas**](themes.md) — Especificação TOML e Live Preview de temas.
- ⚙️ [**Especificação de Configuração**](config.md) — Referência de opções do `config.toml`.
- 🏗️ [**Arquitetura e Design**](design.md) — Topologia e decisões de engenharia.

## Roteador Central
Consulte [`ATLAS.md`](ATLAS.md) como ponto de entrada para navegação guiada por intenção (usuário, desenvolvedor, operador, agente).

## Inventário de Subdiretórios
- `en/`: Versão canônica completa de toda a documentação em **Inglês**.
- `architecture/`: Arquitetura do sistema, boundaries, contratos de Clean Code e ADRs.
- `product/`: Visão de produto, proposta de valor e escopo delimitado.
- `development/`: Padrões de codificação e estratégia exaustiva de testes (TDD/BDD).
- `operations/`: Procedimentos de deploy, runbooks e observabilidade.
- `security/`: Modelagem de ameaças (STRIDE), políticas e contrato de segurança.
- `governance/`: Políticas de branches, pull requests e regras de conformidade.
- `planning/`: Roadmaps e planos de polimento visual e funcional.
