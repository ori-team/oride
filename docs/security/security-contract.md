# Contrato Pesado e Firme de Segurança

Este documento estabelece as **obrigações invioláveis de segurança** que devem ser satisfeitas por todo contribuinte (humano ou agente de IA) antes de qualquer merge:

1. **Tolerância Zero para Segredos**:
   - Nenhuma chave privada, token de API, segredo de nuvem ou credencial pode ser commitada, mesmo que temporária.
   - Qualquer ocorrência aciona reprovação imediata do commit e rotação compulsória da credencial.

2. **Isolamento e Menor Privilégio**:
   - Subagentes operam exclusivamente com as permissões e ferramentas declaradas em seus contratos de harness.
   - Chamadas de subprocessos devem escapar todos os parâmetros e rejeitar interpolação de shell desprotegida.

3. **Validação Estrita de Limites (Boundaries)**:
   - Todo dado recebido de fontes externas (HTTP, arquivos, variáveis de ambiente, ferramentas MCP) é tratado como não-confiável.
   - Sanitização e validação de schema ocorrem na borda de entrada antes de atingir as camadas de domínio.

4. **Auditoria de Cadeia de Suprimentos**:
   - Dependências externas devem ser auditadas contra vulnerabilidades conhecidas (CVEs).
   - Bloqueio imediato de bibliotecas com avisos críticos ou licenças incompatíveis.
