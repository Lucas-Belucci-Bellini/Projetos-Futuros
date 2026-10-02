# Arquitetura do projeto

## 1. Objetivo técnico

Construir uma base documental dos Levels 0–99 e suas sub-seções que possa futuramente alimentar:
- site estático;
- catálogo pesquisável;
- API;
- gerador de páginas;
- banco de dados;
- ferramenta CLI;
- visualizador offline;
- integrações externas.

## 2. Fonte de verdade

- Cada arquivo de nível em `levels/` é uma unidade documental.
- Um nível principal usa `level-NN.md`.
- Uma sub-seção usa um identificador de arquivo derivado do nome, como `level-00-1.md` ou `level-01-5.md`.
- `docs/SUBLEVEL_REGISTRY.md` é a fonte de verdade para cobertura do inventário de sub-seções.
- `docs/LEVEL_CONNECTIVITY_GRAPH.md` é a fonte de verdade para o grafo editorial de entradas/saídas.

## 3. Schema documental

Todo arquivo de nível/sub-seção deve conter:
1. `## Identidade`
2. `## Aparência`
3. `## Estrutura`
4. `## Entidades`
5. `## Recursos`
6. `## Bases`
7. `## Entradas`
8. `## Saídas`
9. `## Mídia`
10. `## Auditoria`

A ordem pode incluir subseções `###`, mas essas dez seções são o contrato mínimo.

## 4. Estado editorial

Estados comuns:
- current;
- trimmed;
- outdated;
- Open for Rewrite;
- Under Rewrite;
- historical;
- não confirmado.

O estado da fonte nunca deve ser substituído por opinião da documentação local.

## 5. Conectividade

Cada aresta deve, quando possível, indicar:
- origem;
- destino;
- método;
- condição;
- direção;
- fonte;
- estado editorial;
- confiança.

Nunca inferir bidirecionalidade apenas porque existe uma rota em uma direção.

## 6. Mídia

A página da wiki e cada imagem incorporada são objetos de direitos diferentes. A branch deve:
- registrar autor;
- registrar URL/origem;
- registrar licença;
- registrar status de confirmação;
- evitar copiar binários quando a licença não estiver clara.

## 7. Linguagens

### Preferência 1 — Rust

Usar Rust para:
- validação;
- parsing;
- geração;
- indexação;
- CLI;
- transformação de Markdown;
- processamento pesado;
- futuras APIs/backend.

### HTML/CSS

Usar para apresentação estática e protótipos.

### TypeScript

Somente quando lógica cliente real não for prática em Rust/WebAssembly ou geração estática.

### JavaScript

Evitar código JavaScript escrito diretamente.

## 8. Ferramentas Rust

- `src/main.rs`: verificador de presença dos 100 níveis principais e consistência de fonte/schema.
- `tools/backrooms_audit.rs`: auditor detalhado de seções, seções vazias e marcadores unresolved.
- `tools/backrooms_inventory.rs`: inventário e relatório de cobertura por arquivo.
- `tools/backrooms_quality.rs`: relatório de lacunas explícitas e qualidade editorial sem transformar desconhecidos em falhas.

## 9. Expansão de sub-seções

O índice oficial lista sub-seções que podem ser:
- pequenas áreas;
- regiões;
- níveis dentro de níveis;
- bases/postos importantes.

Elas devem ser documentadas separadamente quando houver material suficiente. O registro mantém pendências explícitas quando só o título está disponível.

## 10. Evolução para dados estruturados

Uma futura etapa poderá gerar automaticamente:
- JSON;
- SQL;
- índices;
- páginas HTML;
- grafos;
- relatórios de cobertura.

O Markdown continua sendo a fonte editorial até uma migração explícita.
