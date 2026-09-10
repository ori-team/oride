# Padrões de Codificação e Engenharia

1. **Formatação e Estilo**:
   - Todo código deve passar pelo formatador oficial (`cargo fmt` / `rustfmt`) e pelo linter (`cargo clippy --all-targets -- -D warnings`) sem exceção.
   - Linhas mantidas em até 100 caracteres quando razoável.

2. **Tipagem e Erros**:
   - Tipagem estrita em todas as assinaturas públicas.
   - Retornos de erro devem ser explícitos e incluir contexto da operação.

3. **Documentação no Código**:
   - Comentários explicam o *porquê*, nunca o *o quê*.
   - Todas as funções e interfaces públicas devem conter docstrings/comentários descritivos.
