# ADR 001: Linha de Base Arquitetural e Princípios de Clean Architecture

## Status
Aceito

## Contexto
O projeto requer alta manutenibilidade, isolamento rigoroso de regras de negócio em relação a frameworks e dependências externas, e suporte a testes unitários e de integração sem dependências de infraestrutura pesada.

## Decisão
Adotamos a Clean Architecture (Portas e Adaptadores). Todas as dependências devem apontar para o domínio central. Comunicações com infraestrutura externa devem ocorrer exclusivamente por meio de interfaces de portas.

## Consequências
- **Positivas**: Testabilidade completa em memória; facilidade de substituição de drivers de persistência; independência de frameworks.
- **Negativas**: Introdução de camadas intermediárias de mapeamento de dados (DTOs e entidades).
