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

## Lotes recentes de catalogação

- [Lote piloto 011 — argamassas, tintas e impermeabilizantes](LOTE_PILOTO_011.md) — novos candidatos com páginas individuais da Romani, pendentes de validação.

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

- [Empresas — onda 4](EMPRESAS_ONDA_4.md) — novas fontes locais para hidráulica e elétrica.

## Diretório de pisos e profissionais

- [OBJETIVO_COMPARADOR_DE_PRECOS.md](OBJETIVO_COMPARADOR_DE_PRECOS.md) — visão do produto como comparador de preços, regras de equivalência e métricas.
- [PROFISSIONAIS_E_LOJAS_DE_PISOS.md](PROFISSIONAIS_E_LOJAS_DE_PISOS.md) — lista inicial de lojas e prestadores candidatos, categorias, campos cadastrais e regras de verificação.
- [PROFISSIONAIS_OUTRAS_CATEGORIAS.md](PROFISSIONAIS_OUTRAS_CATEGORIAS.md) — categorias e candidatos de hidráulica, elétrica, pintura, drywall, vidraçaria, telhados, impermeabilização, climatização, arquitetura e engenharia.
- [Planejamento arquitetural do diretório](docs/arquitetura/16_DIRETORIO_LOJAS_E_PROFISSIONAIS.md) — requisitos de site, modelo de dados e critérios para publicação.

Os registros atuais são candidatos de pesquisa e **não devem ser publicados como verificados** antes da confirmação de dados, especialidades, contatos e fontes.

- [Empresas adicionais — onda 3](EMPRESAS_ONDA_3.md) — novos candidatos locais e critérios de validação.
- [Lote piloto 003](LOTE_PILOTO_003.md) — dez candidatos de ferramentas, sem preços considerados atuais.

- [Lote piloto 004 — hidráulica e elétrica](LOTE_PILOTO_004.md) — dez candidatos pendentes, sem ofertas presumidas.

- [Lote piloto 005 — cobertura, madeira, hidráulica e pisos](LOTE_PILOTO_005.md) — novos candidatos pendentes das fontes Romani e Depósito Brasil Sul.
- [Lote piloto 007 — pisos, revestimentos e primeiras ofertas observadas](LOTE_PILOTO_007.md) — candidatos adicionais e preços publicados por páginas individuais da Romani, sujeitos à confirmação comercial.
- [Lote piloto 006 — impermeabilização, limpeza e ferramentas](LOTE_PILOTO_006.md) — candidatos pendentes do Depósito Romani.

- [Lote piloto 010 — areia, ferragens, impermeabilização, pintura e hidráulica](LOTE_PILOTO_010.md) — candidatos com SKU e páginas individuais da Romani.
- [Lote piloto 009 — cimento, tintas e rejuntes especiais](LOTE_PILOTO_009.md) — novos candidatos com SKU e páginas individuais.
- [Lote piloto 008 — argamassas e impermeabilização](LOTE_PILOTO_008.md) — cinco candidatos com páginas individuais da Romani.
- [Empresas — onda 5: madeiras](EMPRESAS_ONDA_5.md) — cadastro pendente da Madeireira Luzitano e duas unidades em Londrina.

- [Lote piloto 012 — preparação de superfícies e rejuntes](LOTE_PILOTO_012.md) — candidatos de catálogo com variantes explícitas e revisão pendente.

## Plano de catalogação e empresas da primeira leva

- [Plano operacional para os 50.000 produtos](PLANO_CATALOGACAO_50000.md) — cotas de planejamento, etapas, qualidade e critérios de contagem.
- [Empresas candidatas da primeira leva](PRIMEIRA_LEVA_EMPRESAS.md) — prioridades de pesquisa, fontes públicas e processo de validação/parceria.
- [Empresas adicionais — onda 2](EMPRESAS_ONDA_2.md) — novas lojas especializadas, fontes e próximos passos.
- [Taxonomia ampliada](TAXONOMIA_AMPLIADA.md) — 36 famílias, subcategorias e atributos de comparação.
- [Lote piloto 001](LOTE_PILOTO_001.md) — cinco produtos identificados em fonte oficial, ainda pendentes de revisão; sem preços tratados como atuais.
- [Lote piloto 002](LOTE_PILOTO_002.md) — dez novos candidatos de produtos locais, ainda pendentes de revisão.

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
