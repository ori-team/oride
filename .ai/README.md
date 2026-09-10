# Diretório de Recursos de IA (`.ai/`)

## O que é este diretório?
A pasta `.ai/` armazena as definições canônicas de workforce (agentes, habilidades, receitas de execução) e as metas operacionais (Goals e Planos) vinculadas ao projeto.

## Para que serve?
Permite que o framework Project Atlas resolva deterministicamente quais agentes, skills e receitas de trabalho estão autorizados e disponíveis para executar tarefas no projeto, mantendo políticas de modelos, integridade de locks e rastreabilidade de evidências.

## Inventário de Arquivos e Subdiretórios
- `agents/`: Agentes resolvidos e manifestos de papéis.
- `skills/`: Habilidades e procedimentos atribuídos aos agentes.
- `recipes/`: Fluxos de trabalho e receitas de automação multi-etapas.
- `goals/`: Metas do projeto organizadas por fases (`P00`, `P01`, etc.) com hashes de integridade SHA-256.
- `orchestration/`: Políticas de roteamento de LLMs, tolerância a falhas e scorecards de desempenho.
