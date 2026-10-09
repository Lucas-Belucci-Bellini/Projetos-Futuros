# Catálogo de Materiais de Construção — Londrina/PR

## Objetivo

Planejar e construir um site independente para pesquisar e comparar materiais de construção vendidos por lojas que atendem Londrina, Paraná. A Balaroti é uma referência inicial de variedade, não a única fonte nem parceira presumida do projeto.

O catálogo deverá separar produtos, variantes, lojas, ofertas e fontes. Preços e disponibilidade sempre terão origem e data de observação.

## Escopo inicial

- Pesquisar lojas locais, depósitos, home centers e lojas especializadas.
- Catalogar produtos apenas com dados rastreáveis e método de uso permitido.
- Filtrar por categoria, loja, marca, unidade e preço disponível.
- Comparar ofertas realmente equivalentes, mostrando modalidade e condições.
- Direcionar o visitante à página oficial da loja para confirmar preço, estoque, frete e compra.
- Planejar atualização periódica sem contornar logins, CAPTCHA, bloqueios ou controles técnicos.
- Construir e validar o pipeline de importação antes de escalar para 50.000 produtos.

## Referências iniciais

- [Balaroti — loja online](https://www.balaroti.com.br/)
- [Balaroti — lojas em Londrina](https://lojas.balaroti.com.br/parana/londrina)
- [Balaroti — sobre a empresa](https://www.balaroti.com.br/sobre-a-balaroti)

Essas páginas são pontos de partida para pesquisa. Não constituem autorização automática para extração em massa ou republicação de imagens e textos.

## Arquitetura e aplicações

- [ARQUITETURA_SITE.md](ARQUITETURA_SITE.md) — divisão entre frontend TypeScript, API Rust, automação Python e banco.
- [API_CONTRACT.md](API_CONTRACT.md) — contrato planejado dos endpoints REST.
- [apps/web/README.md](apps/web/README.md) — scaffold React + TypeScript + Vite + Tailwind CSS.
- [apps/api/README.md](apps/api/README.md) — API inicial Rust + Axum.
- [Workflow da stack](../../.github/workflows/catalogo-stack.yml) — verifica TypeScript/Vite e compila/testa a API Rust.
- [automation/README.md](automation/README.md) — responsabilidades da automação Python.

## Planejamento detalhado da arquitetura

O plano de arquitetura foi dividido em documentos menores para facilitar implementação, revisão e atualização por etapa: [índice mestre da arquitetura](docs/arquitetura/00_INDICE_ARQUITETURA.md). A pasta cobre visão geral, requisitos, arquitetura geral, frontend, API Rust, banco de dados, contrato REST, automação Python, governança dos dados, segurança, testes/CI, deploy/operação, UX/acessibilidade, roadmap e decisões pendentes.

## Diretório de pisos e profissionais

- [OBJETIVO_COMPARADOR_DE_PRECOS.md](OBJETIVO_COMPARADOR_DE_PRECOS.md) — visão do produto como comparador de preços, regras de equivalência e métricas.
- [PROFISSIONAIS_E_LOJAS_DE_PISOS.md](PROFISSIONAIS_E_LOJAS_DE_PISOS.md) — lista inicial de lojas e prestadores candidatos, categorias, campos cadastrais e regras de verificação.
- [PROFISSIONAIS_OUTRAS_CATEGORIAS.md](PROFISSIONAIS_OUTRAS_CATEGORIAS.md) — categorias e candidatos de hidráulica, elétrica, pintura, drywall, vidraçaria, telhados, impermeabilização, climatização, arquitetura e engenharia.
- [Planejamento arquitetural do diretório](docs/arquitetura/16_DIRETORIO_LOJAS_E_PROFISSIONAIS.md) — requisitos de site, modelo de dados e critérios para publicação.

Os registros atuais são candidatos de pesquisa e **não devem ser publicados como verificados** antes da confirmação de dados, especialidades, contatos e fontes.

## Documentação

- [PLANO.md](PLANO.md) — fases de pesquisa e desenvolvimento.
- [FONTES_E_REGRAS.md](FONTES_E_REGRAS.md) — fontes, atribuição e limites de coleta.
- [ESQUEMA_CATALOGO.md](ESQUEMA_CATALOGO.md) — modelo de loja, produto e oferta.
- [META_50000_ITENS.md](META_50000_ITENS.md) — meta de volume e critérios de contagem.
- [ARQUITETURA_DADOS.md](ARQUITETURA_DADOS.md) — arquitetura de ingestão e armazenamento.
- [CATEGORIAS.md](CATEGORIAS.md) — taxonomia inicial.
- [CONTRATO_IMPORTACAO_CSV.md](CONTRATO_IMPORTACAO_CSV.md) — formato e validação de arquivos.
- [REGISTRO_DE_FONTES.md](REGISTRO_DE_FONTES.md) — inventário de fontes e permissões.
- [PESQUISA_FONTES_LOCAIS.md](PESQUISA_FONTES_LOCAIS.md) — primeiras fontes oficiais encontradas em Londrina.
- [REQUISITOS_MVP.md](REQUISITOS_MVP.md) — requisitos funcionais e não funcionais.
- [PLANO_DE_TESTES.md](PLANO_DE_TESTES.md) — testes de dados, importação, interface e segurança.
- [ROADMAP_E_BACKLOG.md](ROADMAP_E_BACKLOG.md) — tarefas priorizadas e critérios de saída.
- [CHECKLIST_OPERACIONAL_PILOTO.md](CHECKLIST_OPERACIONAL_PILOTO.md) — procedimento para validar fontes e executar o primeiro lote real.
- [scripts/README.md](scripts/README.md) — execução do validador, importador e relatório.
- [DECISOES_TECNICAS.md](DECISOES_TECNICAS.md) — decisões técnicas e pontos pendentes.
- [database/README.md](database/README.md) — criação do banco SQLite.
- [templates/produtos.csv](templates/produtos.csv) — modelo vazio de produtos.
- [templates/fontes.csv](templates/fontes.csv) — fontes descobertas, ainda não aprovadas.
- [templates/lojas.csv](templates/lojas.csv) — lojas candidatas, ainda pendentes de confirmação.
- [templates/ofertas.csv](templates/ofertas.csv) — modelo vazio de ofertas por loja.
- [tests/README.md](tests/README.md) — execução dos testes automatizados.
- [scripts/validar_catalogo_csv.py](scripts/validar_catalogo_csv.py) — validador estrutural de importação.
- [scripts/relatorio_catalogo.py](scripts/relatorio_catalogo.py) — relatório de contagens, fontes e categorias.
- [scripts/auditar_catalogo.py](scripts/auditar_catalogo.py) — auditoria de integridade somente leitura.
- [scripts/importar_ofertas_csv.py](scripts/importar_ofertas_csv.py) — importador de ofertas com validação de loja, produto e fonte.
- [scripts/registrar_fontes_csv.py](scripts/registrar_fontes_csv.py) — registra fontes sem aprová-las automaticamente.
- [scripts/registrar_lojas_csv.py](scripts/registrar_lojas_csv.py) — registra lojas sem confirmá-las automaticamente.
- [tests/test_validar_catalogo_csv.py](tests/test_validar_catalogo_csv.py) — testes do validador.
- [tests/test_importar_catalogo_csv.py](tests/test_importar_catalogo_csv.py) — testes do importador SQLite.
- [tests/test_relatorio_catalogo.py](tests/test_relatorio_catalogo.py) — testes do relatório do banco.
- [tests/test_importar_ofertas_csv.py](tests/test_importar_ofertas_csv.py) — testes do importador de ofertas.
- [tests/test_registro_catalogo_csv.py](tests/test_registro_catalogo_csv.py) — testes de cadastro de fontes e lojas.
- [Workflow de testes](../../.github/workflows/catalogo-tests.yml) — execução automática dos testes no GitHub Actions.

## Princípios

1. Não inventar preços, marcas, estoque, endereços ou disponibilidade.
2. Preços variam e precisam de data da última verificação.
3. Não copiar descrições extensas, imagens ou identidade visual de terceiros sem permissão/licença adequada.
4. Preferir links para páginas originais e descrições curtas próprias.
5. Separar informação confirmada, informação declarada pela loja e informação ainda não verificada.
6. Não publicar avaliações falsas nem apresentar uma amostra como se incluísse todas as lojas da cidade.
7. Contar produto e oferta separadamente: um produto listado em cinco lojas continua sendo um produto, com até cinco ofertas.

## Status real

**Etapa atual: documentação, pipeline inicial e scaffold de frontend/API.** A interface já tem uma primeira tela de apresentação e a API tem um health check planejado/implementado em código; ainda é necessário executar build e testes. A busca real, o banco PostgreSQL, as ofertas e o catálogo de 50.000 produtos ainda não estão implementados.
