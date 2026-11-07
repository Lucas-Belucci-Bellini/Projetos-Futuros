# API Rust

API HTTP em Rust + Axum, com busca e detalhe de produtos rastreáveis, categorias, biblioteca local de documentos/notas, integração opcional com Hermes e status/commit Git local. O banco SQLite local compartilha o schema e os importadores Python deste projeto. PostgreSQL continua sendo o alvo de produção e exige migração/configuração próprias.

## Executar
Instale Rust stable com Cargo e rode:

    cargo test
    cargo run

A API escuta em `127.0.0.1:8080` por padrão e cria `prototipo/database/catalogo.sqlite3` se ainda não existir. Defina `CATALOG_DB_PATH` para escolher outro banco do catálogo, `LIBRARY_DB_PATH` para escolher o banco da biblioteca pessoal (padrão Windows: `%LOCALAPPDATA%\CatalogoMateriais\biblioteca.sqlite3`) e `PORT` para alterar a porta.

Rotas do catálogo:
- `GET /api/v1/health`
- `GET /api/v1/categories`
- `GET /api/v1/products?q=&category_id=&brand=&store_id=&min_price=&max_price=&page=&page_size=&sort=`
- `GET /api/v1/products/{id}`

Biblioteca local:
- `GET /api/v1/library/documents?q=` — pesquisa em título, conteúdo e etiquetas.
- `POST /api/v1/library/documents` — cria `{ "title": "...", "content": "...", "tags": "..." }`.
- `GET /api/v1/library/documents/{id}` — lê um registro.
- `PUT /api/v1/library/documents/{id}` — atualiza um registro.
- `DELETE /api/v1/library/documents/{id}` — remove um registro.

Cada registro aceita título de até 160 caracteres, conteúdo até 12.000 caracteres
e etiquetas até 300 caracteres. O frontend importa somente arquivos escolhidos
pelo usuário (TXT, Markdown, CSV, JSON e HTML, até 48 KB e 12.000 caracteres por
arquivo). Nenhuma pasta é varrida automaticamente.

Hermes opcional:
- `GET /api/v1/hermes/status` — verifica se o CLI está disponível.
- `POST /api/v1/hermes/research` — aceita `{ "question": "..." }` e executa
  `hermes chat` com o conjunto `web`, limite de três turnos e pergunta de até
  500 caracteres. A chamada pode levar até dois minutos; nenhum conteúdo da
  biblioteca é enviado.

O painel local usa ainda `GET /api/v1/git/status` e `POST /api/v1/git/commit`. O commit exige confirmação, limita a 200 arquivos dentro da pasta do catálogo e recebe a seleção explícita; o push para o GitHub nunca é automático. O serviço não guarda tokens e usa a configuração Git existente no computador.

O SQLite usa `database/001_initial_schema.sql` para o catálogo e
`database/002_library.sql` para o banco separado da biblioteca pessoal. Os dados permanecem locais;
backup ou sincronização GitHub privada ainda não estão implementados.

## Próximas etapas
1. Migrar o repositório SQLite para uma camada PostgreSQL planejada.
2. Adicionar paginação no frontend, filtros de preço/loja e acessibilidade automatizada.
3. Implementar rate limiting e observabilidade adequados ao ambiente público.
4. Testes de integração com banco isolado e validação contínua no GitHub Actions.
