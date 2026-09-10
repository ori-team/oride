# Diretório Operacional do Atlas (`.atlas/`)

## O que é este diretório?
Contém metadados de execução, caches de desempenho, histórico analítico e estado derivado mantidos pelo Project Atlas CLI.

## Para que serve?
Isola todo estado temporário ou derivado do código-fonte canônico. Dados em `.atlas/runtime/` e `.atlas/cache/` são voláteis e não devem ser versionados no Git (com exceção do histórico estruturado em `history/`).

## Inventário
- `cache/`: Cache de resoluções de esquemas e pacotes para acelerar comandos do CLI.
- `history/`: Histórico cumulativo de inteligência do projeto, tokens consumidos e métricas.
- `runtime/`: Checkpoints de execução, sessões ativas e cápsulas de contexto.
