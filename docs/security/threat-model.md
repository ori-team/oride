# Modelo de Ameaças (STRIDE)

| Ameaça | Vetor Potencial | Mitigação Mandatória |
|--------|-----------------|----------------------|
| **Spoofing** | Falsificação de chamadas ou payloads | Autenticação mútua e validação estrita de assinaturas. |
| **Tampering** | Modificação maliciosa de metas ou código | Verificação de integridade via digest criptográfico SHA-256 e regras de branch protegida. |
| **Repudiation** | Ações executadas sem rastreabilidade | Registro imutável de eventos e evidências vinculadas a Goals. |
| **Information Disclosure** | Exposição acidental de credenciais ou dados | Varredura automatizada pré-commit e redação de segredos em logs. |
| **Denial of Service** | Consumo excessivo de recursos ou loops | Limites orçamentários de contexto (LPC), timeouts explícitos e ausência de recursão irrestrita. |
| **Elevation of Privilege** | Escape de isolamento de subagentes | Conectores com políticas de execução estritas e isolamento de permissões. |
