# 5. Arquitetura do backend Rust

## Stack
- Rust stable.
- Axum para rotas HTTP.
- Tokio para runtime assíncrono.
- Serde para serialização JSON.
- Tower HTTP para middleware selecionado.
- Driver PostgreSQL e camada de acesso a dados a escolher e registrar antes da integração.
- Tracing para logs estruturados.

## Organização planejada
```text
apps/api/src/
  main.rs
  config.rs
  routes/
    mod.rs
    health.rs
    products.rs
    categories.rs
    stores.rs
  domain/
    mod.rs
    product.rs
    offer.rs
    store.rs
  services/
    mod.rs
    product_search.rs
  repositories/
    mod.rs
    postgres/
  errors.rs
  pagination.rs
```

A árvore acima é um alvo de organização, não uma declaração de que todos os módulos já existem. Extrair módulos conforme o código crescer, sem criar camadas vazias prematuramente.

## Regras de implementação
- Handler HTTP fino: validação, chamada ao serviço e conversão de resposta.
- Regras de negócio no domínio/serviço, não espalhadas pelos handlers.
- SQL parametrizado; nunca concatenar texto do usuário em SQL.
- Paginação obrigatória e limites explícitos.
- Timeout para consultas e limites de corpo da requisição.
- Erros tipados convertidos para JSON público estável.
- Não devolver stack trace, query SQL ou segredos ao cliente.
- Health check não deve revelar versões sensíveis, variáveis ou detalhes de infraestrutura.
- Configuração por ambiente, com validação no início da aplicação.
- Graceful shutdown e logs estruturados.

## Endpoint inicial
`GET /api/v1/health` deve indicar apenas se o processo está vivo. Quando banco e dependências estiverem integrados, adicionar um health/readiness separado que verifique dependências sem vazar credenciais.

## Testes
- Unitários para normalização, filtros e regras de domínio.
- Integração para rotas e banco temporário/isolado.
- Testes de paginação, filtros inválidos, registros ausentes e erros de banco.
- Verificação `cargo fmt --all -- --check` e `cargo test --workspace` na CI.

## Decisões pendentes
Escolher e documentar driver PostgreSQL, estratégia de migrations e abordagem de pesquisa textual antes de codificar o repositório de produção.
