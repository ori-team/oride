# Diretório de Orquestração (`.ai/orchestration/`)

## O que é este diretório?
Concentra as diretrizes de governança e parametrização do ecossistema de inteligência artificial do projeto.

## Para que serve?
Garante previsibilidade, controle orçamentário e resiliência nas chamadas aos modelos de linguagem através de regras formais de fallback, limites de tokens e avaliação empírica de acurácia.

## Inventário
- `model-policy.json`: Mapeamento de papéis (geral, arquitetura, testes, revisão) para modelos recomendados.
- `orchestrator.json`: Parâmetros de autonomia e metodologia de contexto (POP / LPC).
- `fallbacks.json`: Regras de degradação graciosa e escalação humana em caso de falha de provedores.
- `model-scorecard.json`: Registro histórico de desempenho empírico dos modelos testados no projeto.
