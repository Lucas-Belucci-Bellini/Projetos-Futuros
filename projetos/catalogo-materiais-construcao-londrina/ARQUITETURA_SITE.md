# Arquitetura do site — Catálogo de Materiais de Construção

## Objetivo
Construir um catálogo independente para pesquisar materiais de construção vendidos em Londrina/PR e comparar ofertas rastreáveis. A meta de 50.000 produtos contará somente produtos reais, únicos, rastreáveis e aceitos nos critérios de qualidade.

## Stack definida
- Frontend: React + TypeScript + Vite.
- CSS: Tailwind CSS v4, com tokens de design e componentes acessíveis.
- Backend: Rust + Axum, API REST versionada em /api/v1.
- Banco de produção: PostgreSQL; SQLite permanece útil para testes e importações locais durante a transição.
- Automação: Python para ingestão autorizada, validação, normalização, relatórios e tarefas agendadas.
- Testes: Vitest/Testing Library no frontend; testes Rust para API e domínio; unittest/pytest no pipeline Python; CI para executar tudo.

## Limites de responsabilidade
### Web
- Exibir catálogo, filtros, página do produto e comparação de ofertas.
- Consumir somente a API; não acessar o banco diretamente.
- Mostrar data de observação e estado da informação.
- Tratar carregamento, erro, lista vazia e paginação.

### API Rust
- Validar parâmetros, limitar paginação e responder com contrato JSON estável.
- Centralizar pesquisa, filtros, comparação e regras de publicação.
- Manter acesso ao PostgreSQL atrás de uma camada de repositórios.
- Expor health check sem credenciais nem detalhes internos.

### Automação Python
- Receber CSVs, feeds ou APIs cujo uso seja permitido.
- Validar e normalizar lotes, preservando erros e proveniência.
- Nunca aprovar fontes automaticamente.
- Executar dry-run antes de gravar dados.

## Organização
- apps/web: React + TypeScript + Vite + Tailwind.
- apps/api: Rust + Axum.
- automation: módulos de automação Python.
- database: esquema SQLite existente e futuras migrations PostgreSQL.
- scripts: ferramentas de manutenção/importação existentes.
- templates: CSVs de entrada.
- tests: testes Python já existentes.
- docs: contratos, decisões e desenhos técnicos.

## Domínios
- Product: identidade canônica, marca, modelo, unidade e atributos.
- SourceProduct: identificação na fonte original.
- Store: loja/filial e estado de verificação.
- Offer: relação entre produto e loja, preço, modalidade, disponibilidade, URL e data de observação.
- Source: origem, método de acesso, permissões e evidências.
- ImportBatch: lote, contagens, estado e relatório.
- Category: taxonomia hierárquica.
- ProductIdentifier: GTIN/EAN, SKU ou código do fabricante quando comprovado.

Não deduplicar apenas pelo nome. Preferir identificadores confiáveis; sem eles, comparar atributos e manter revisão humana e proveniência.

## Fluxo de dados
1. Fonte permitida → adaptador Python.
2. Dados brutos preservados com origem e horário.
3. Validação estrutural e semântica → relatório de erros.
4. Normalização de unidades, categorias e identificadores.
5. Dry-run e revisão do lote.
6. Importação transacional e idempotente.
7. API Rust consulta dados publicáveis.
8. Interface apresenta ofertas com datas e avisos de atualização.

## Requisitos não funcionais
- Paginação obrigatória e limite máximo por requisição.
- Índices para nome normalizado, categoria, marca, identificadores e relação oferta/produto/loja.
- Timeouts, validação de entrada e limites de corpo na API.
- Segredos em variáveis de ambiente ou secret manager.
- CORS restrito aos domínios autorizados em produção.
- Backups testados e migrations versionadas.
- Acessibilidade por teclado, contraste e layout responsivo.
- Não registrar dados sensíveis em logs.
- Não coletar em massa nem republicar imagens/descrições sem autorização compatível.

## Fases
1. Fundação: scaffold web/API, contrato, health check e CI.
2. Pesquisa: busca e filtros ligados à API.
3. Dados: PostgreSQL, migrations, repositórios e integração com importador.
4. Ofertas: comparação, data de observação e política de obsolescência.
5. Administração: revisão de fontes, lotes e erros com controle de acesso.
6. Produção: segurança, monitoramento, backup/restore e testes de carga.

## Critérios de fundação pronta
- Web e API compilam em ambiente limpo.
- GET /api/v1/health retorna JSON previsível.
- A web sinaliza quando a API está indisponível.
- Testes e lint executam em CI.
- Dados demonstrativos nunca são confundidos com dados reais.
- Nenhuma credencial é versionada.
