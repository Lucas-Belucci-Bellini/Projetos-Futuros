# 11. Estratégia de testes e CI

## Camadas de teste

### Frontend
- Typecheck e build de produção.
- Testes de componentes e estados da busca.
- Testes de parâmetros de URL e paginação.
- Testes de erro HTTP, timeout e API indisponível.
- Acessibilidade automatizada e verificação manual por teclado.

### Backend Rust
- Unitários para regras de domínio, normalização e filtros.
- Testes de handlers e serialização.
- Integração com banco isolado para consultas, transações e constraints.
- Testes para parâmetros inválidos, limites de paginação e erros internos.
- Formatação via rustfmt.

### Automação Python
- Testes de validação e importação.
- CSV vazio, cabeçalhos ausentes, tipos inválidos e campos extras.
- Preço inválido, URL inválida, loja/fonte desconhecida.
- Duplicatas, idempotência, rollback e falha parcial.
- Dry-run não deve alterar os dados publicados.
- Relatórios devem reconciliar lidos, aceitos, rejeitados e importados.

### Integração
- Web consome o contrato real da API.
- API lê apenas dados publicáveis.
- Importador não contorna aprovação de fonte/lote.
- Migrações funcionam em banco limpo e em atualização suportada.
- Dados de teste são claramente sintéticos.

## CI obrigatória
Em pull requests e pushes relevantes:
1. Instalar dependências usando lockfiles quando disponíveis.
2. Typecheck e build web.
3. `cargo fmt --all -- --check`.
4. `cargo test --workspace`.
5. Rodar a suíte Python existente.
6. Validar documentação/formatos de importação quando houver ferramenta apropriada.

Não marcar o build como aprovado até observar execução verde no GitHub Actions. Um arquivo de workflow presente não prova que ele passou.

## Definição de pronto por tarefa
- Requisito e comportamento documentados.
- Implementação sem placeholders ocultos.
- Testes pertinentes adicionados.
- CI verde.
- Documentação e exemplos atualizados.
- Limitações conhecidas registradas.
