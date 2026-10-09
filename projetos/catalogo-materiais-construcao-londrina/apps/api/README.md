# API Rust

API HTTP inicial em Rust + Axum. Nesta etapa, somente GET /api/v1/health está implementado. Produtos, ofertas, autenticação e conexão com PostgreSQL ainda não estão prontos.

## Executar
Instale Rust stable com Cargo e rode:

    cargo test
    cargo run

A API escuta em 127.0.0.1:8080 por padrão. A variável de ambiente PORT pode alterar a porta.

Teste manual: curl http://127.0.0.1:8080/api/v1/health

## Próximas etapas
1. Configuração e validação de ambiente.
2. Contratos tipados para produto, categoria, loja e oferta.
3. Persistência PostgreSQL por camada de repositório.
4. Busca paginada com limites e filtros validados.
5. CORS restrito por ambiente, rate limiting e tratamento de erros.
6. Testes de integração com banco isolado.
