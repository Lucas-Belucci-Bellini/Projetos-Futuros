# 7. Contrato da API REST

## Convenções
- Prefixo: `/api/v1`.
- JSON UTF-8.
- Métodos e códigos HTTP seguem semântica HTTP.
- Paginação explícita, com limite máximo definido no backend.
- Filtros desconhecidos ou malformados retornam erro de validação claro.
- O contrato publicado deve corresponder ao comportamento testado; este documento é a proposta inicial.

## Endpoints planejados
| Método | Caminho | Objetivo |
|---|---|---|
| GET | `/health` | Saúde básica do processo |
| GET | `/products` | Busca, filtros, ordenação e paginação |
| GET | `/products/{id}` | Detalhes do produto/variante |
| GET | `/categories` | Árvore ou lista de categorias |
| GET | `/stores` | Lojas verificadas e publicáveis |
| GET | `/stores/{id}` | Detalhes públicos de uma loja |
| GET | `/products/{id}/offers` | Ofertas rastreáveis do produto |

Rotas administrativas não devem ser públicas e só entram após autenticação, autorização e auditoria estarem definidas.

## Busca de produtos
Parâmetros planejados:
- `q`: texto de busca.
- `category_id`: categoria.
- `store_id`: loja.
- `brand`: marca.
- `min_price` e `max_price`: preço, apenas quando comparável.
- `sort`: relevância, nome ou preço.
- `page` e `page_size`: paginação limitada.
- `updated_since`: filtro opcional se houver necessidade real.

Validar valores, limites, formatos numéricos e combinação de filtros. Não prometer relevância avançada até que ela seja implementada e testada.

## Envelope de sucesso proposto
```json
{
  "data": [],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total_items": 0,
    "total_pages": 0
  }
}
```

O exemplo acima representa formato, não catálogo real. A API deve distinguir uma consulta válida sem resultados de falha interna.

## Envelope de erro proposto
```json
{
  "error": {
    "code": "INVALID_QUERY",
    "message": "Parâmetros de pesquisa inválidos",
    "request_id": "identificador-de-correlação"
  }
}
```

Mensagens públicas não devem incluir SQL, stack trace, tokens ou caminhos internos.

## Regras de compatibilidade
- Mudança incompatível exige nova versão ou estratégia de transição documentada.
- Campos opcionais podem ser adicionados sem exigir preenchimento retroativo imediato.
- Datas devem usar ISO 8601 com fuso explícito.
- Preço deve ser serializado de forma decimal inequívoca.
- Testes de contrato devem verificar paginação, erro, lista vazia e campos obrigatórios.
