# Catálogo de Materiais de Construção — Londrina/PR

Este é o índice do projeto de planejamento na branch
`catalogo-materiais-construcao-londrina`. O planejamento fica em
`docs/catalogo/`; o código ainda experimental e os recursos de validação ficam
isolados em `prototipo/` e não representam uma implantação de produção.

## Organização

- [Plano e roadmap](plano/PLANO.md) — objetivo, requisitos e etapas.
- [Plano do organizador de planilhas](plano/PLANO_ORGANIZADOR_PLANILHAS.md) — biblioteca local, análise de Excel, GitHub privado, Hermes e exportações.
- [Arquitetura](arquitetura/00_INDICE_ARQUITETURA.md) — frontend, API, dados,
  segurança, operação e decisões técnicas.
- [Catalogação](catalogacao/CATEGORIAS.md) — taxonomia, lotes e metas.
- [Fontes](fontes/FONTES_E_REGRAS.md) — pesquisa, proveniência e candidatos.
- [Operação](operacao/CHECKLIST_OPERACIONAL_PILOTO.md) — importação, testes e
  revisão dos lotes.
- [Biblioteca local e Hermes](operacao/BIBLIOTECA_LOCAL_E_HERMES.md) — MVP local-first, limites da importação e configuração opcional de pesquisa Hermes.
- `prototipo/` — implementação inicial, scripts e testes usados para validar
  hipóteses; não publicar dados reais sem autorização e revisão.

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

- [Lote piloto 011 — argamassas, tintas e impermeabilizantes](catalogacao/LOTE_PILOTO_011.md) — novos candidatos com páginas individuais da Romani, pendentes de validação.

## Referências iniciais

- [Balaroti — loja online](https://www.balaroti.com.br/)
- [Balaroti — lojas em Londrina](https://lojas.balaroti.com.br/parana/londrina)
- [Balaroti — sobre a empresa](https://www.balaroti.com.br/sobre-a-balaroti)

Essas páginas são pontos de partida para pesquisa. Não constituem autorização automática para extração em massa ou republicação de imagens e textos.

## Arquitetura e aplicações

- [ARQUITETURA_SITE.md](arquitetura/ARQUITETURA_SITE.md) — divisão entre frontend TypeScript, API Rust, automação Python e banco.
- [API_CONTRACT.md](arquitetura/API_CONTRACT.md) — contrato planejado dos endpoints REST.
- [apps/web/README.md](../../prototipo/apps/web/README.md) — scaffold React + TypeScript + Vite + Tailwind CSS.
- [apps/api/README.md](../../prototipo/apps/api/README.md) — API inicial Rust + Axum.
- [Workflow da stack](../../.github/workflows/catalogo-stack.yml) — verifica TypeScript/Vite e compila/testa a API Rust.
- [automation/README.md](../../prototipo/automation/README.md) — responsabilidades da automação Python.

## Planejamento detalhado da arquitetura

O plano de arquitetura foi dividido em documentos menores para facilitar implementação, revisão e atualização por etapa: [índice mestre da arquitetura](arquitetura/00_INDICE_ARQUITETURA.md). A pasta cobre visão geral, requisitos, arquitetura geral, frontend, API Rust, banco de dados, contrato REST, automação Python, governança dos dados, segurança, testes/CI, deploy/operação, UX/acessibilidade, roadmap e decisões pendentes.

- [Empresas — onda 4](fontes/EMPRESAS_ONDA_4.md) — novas fontes locais para hidráulica e elétrica.

## Diretório de pisos e profissionais

- [OBJETIVO_COMPARADOR_DE_PRECOS.md](plano/OBJETIVO_COMPARADOR_DE_PRECOS.md) — visão do produto como comparador de preços, regras de equivalência e métricas.
- [PROFISSIONAIS_E_LOJAS_DE_PISOS.md](fontes/PROFISSIONAIS_E_LOJAS_DE_PISOS.md) — lista inicial de lojas e prestadores candidatos, categorias, campos cadastrais e regras de verificação.
- [PROFISSIONAIS_OUTRAS_CATEGORIAS.md](fontes/PROFISSIONAIS_OUTRAS_CATEGORIAS.md) — categorias e candidatos de hidráulica, elétrica, pintura, drywall, vidraçaria, telhados, impermeabilização, climatização, arquitetura e engenharia.
- [Planejamento arquitetural do diretório](arquitetura/16_DIRETORIO_LOJAS_E_PROFISSIONAIS.md) — requisitos de site, modelo de dados e critérios para publicação.

Os registros atuais são candidatos de pesquisa e **não devem ser publicados como verificados** antes da confirmação de dados, especialidades, contatos e fontes.

- [Empresas adicionais — onda 3](fontes/EMPRESAS_ONDA_3.md) — novos candidatos locais e critérios de validação.
- [Lote piloto 003](catalogacao/LOTE_PILOTO_003.md) — dez candidatos de ferramentas, sem preços considerados atuais.

- [Lote piloto 004 — hidráulica e elétrica](catalogacao/LOTE_PILOTO_004.md) — dez candidatos pendentes, sem ofertas presumidas.

- [Lote piloto 005 — cobertura, madeira, hidráulica e pisos](catalogacao/LOTE_PILOTO_005.md) — novos candidatos pendentes das fontes Romani e Depósito Brasil Sul.
- [Lote piloto 007 — pisos, revestimentos e primeiras ofertas observadas](catalogacao/LOTE_PILOTO_007.md) — candidatos adicionais e preços publicados por páginas individuais da Romani, sujeitos à confirmação comercial.
- [Lote piloto 006 — impermeabilização, limpeza e ferramentas](catalogacao/LOTE_PILOTO_006.md) — candidatos pendentes do Depósito Romani.

- [Lote piloto 010 — areia, ferragens, impermeabilização, pintura e hidráulica](catalogacao/LOTE_PILOTO_010.md) — candidatos com SKU e páginas individuais da Romani.
- [Lote piloto 009 — cimento, tintas e rejuntes especiais](catalogacao/LOTE_PILOTO_009.md) — novos candidatos com SKU e páginas individuais.
- [Lote piloto 008 — argamassas e impermeabilização](catalogacao/LOTE_PILOTO_008.md) — cinco candidatos com páginas individuais da Romani.
- [Empresas — onda 5: madeiras](fontes/EMPRESAS_ONDA_5.md) — cadastro pendente da Madeireira Luzitano e duas unidades em Londrina.

- [Lote piloto 012 — preparação de superfícies e rejuntes](catalogacao/LOTE_PILOTO_012.md) — candidatos de catálogo com variantes explícitas e revisão pendente.

- [Lote piloto 013 — madeiras, forros, decks e assoalhos](catalogacao/LOTE_PILOTO_013.md) — candidatos de catálogo da Madeireste, aguardando SKU e cotação individual.

- [Onda de pesquisa 005 — novas fontes locais e filas de identificação](fontes/PESQUISA_FONTES_ONDA_005.md): Açofer, Hydraulica Comercial, Tintas Diniz e Tintas Aurora.

## Plano de catalogação e empresas da primeira leva

- [Plano operacional para os 50.000 produtos](catalogacao/PLANO_CATALOGACAO_50000.md) — cotas de planejamento, etapas, qualidade e critérios de contagem.
- [Empresas candidatas da primeira leva](fontes/PRIMEIRA_LEVA_EMPRESAS.md) — prioridades de pesquisa, fontes públicas e processo de validação/parceria.
- [Empresas adicionais — onda 2](fontes/EMPRESAS_ONDA_2.md) — novas lojas especializadas, fontes e próximos passos.
- [Taxonomia ampliada](catalogacao/TAXONOMIA_AMPLIADA.md) — 36 famílias, subcategorias e atributos de comparação.
- [Lote piloto 001](catalogacao/LOTE_PILOTO_001.md) — cinco produtos identificados em fonte oficial, ainda pendentes de revisão; sem preços tratados como atuais.
- [Lote piloto 002](catalogacao/LOTE_PILOTO_002.md) — dez novos candidatos de produtos locais, ainda pendentes de revisão.

## Documentação

- [PLANO.md](plano/PLANO.md) — fases de pesquisa e desenvolvimento.
- [FONTES_E_REGRAS.md](fontes/FONTES_E_REGRAS.md) — fontes, atribuição e limites de coleta.
- [ESQUEMA_CATALOGO.md](arquitetura/ESQUEMA_CATALOGO.md) — modelo de loja, produto e oferta.
- [META_50000_ITENS.md](catalogacao/META_50000_ITENS.md) — meta de volume e critérios de contagem.
- [ARQUITETURA_DADOS.md](arquitetura/ARQUITETURA_DADOS.md) — arquitetura de ingestão e armazenamento.
- [CATEGORIAS.md](catalogacao/CATEGORIAS.md) — taxonomia inicial.
- [CONTRATO_IMPORTACAO_CSV.md](operacao/CONTRATO_IMPORTACAO_CSV.md) — formato e validação de arquivos.
- [REGISTRO_DE_FONTES.md](fontes/REGISTRO_DE_FONTES.md) — inventário de fontes e permissões.
- [PESQUISA_FONTES_LOCAIS.md](fontes/PESQUISA_FONTES_LOCAIS.md) — primeiras fontes oficiais encontradas em Londrina.
- [REQUISITOS_MVP.md](plano/REQUISITOS_MVP.md) — requisitos funcionais e não funcionais.
- [PLANO_DE_TESTES.md](operacao/PLANO_DE_TESTES.md) — testes de dados, importação, interface e segurança.
- [ROADMAP_E_BACKLOG.md](plano/ROADMAP_E_BACKLOG.md) — tarefas priorizadas e critérios de saída.
- [CHECKLIST_OPERACIONAL_PILOTO.md](operacao/CHECKLIST_OPERACIONAL_PILOTO.md) — procedimento para validar fontes e executar o primeiro lote real.
- [scripts/README.md](../../prototipo/scripts/README.md) — execução do validador, importador e relatório.
- [DECISOES_TECNICAS.md](arquitetura/DECISOES_TECNICAS.md) — decisões técnicas e pontos pendentes.
- [database/README.md](../../prototipo/database/README.md) — criação do banco SQLite.
- [templates/produtos.csv](../../prototipo/templates/produtos.csv) — modelo vazio de produtos.
- [templates/fontes.csv](../../prototipo/templates/fontes.csv) — fontes descobertas, ainda não aprovadas.
- [templates/lojas.csv](../../prototipo/templates/lojas.csv) — lojas candidatas, ainda pendentes de confirmação.
- [templates/ofertas.csv](../../prototipo/templates/ofertas.csv) — modelo vazio de ofertas por loja.
- [tests/README.md](../../prototipo/tests/README.md) — execução dos testes automatizados.
- [scripts/validar_catalogo_csv.py](../../prototipo/scripts/validar_catalogo_csv.py) — validador estrutural de importação.
- [scripts/relatorio_catalogo.py](../../prototipo/scripts/relatorio_catalogo.py) — relatório de contagens, fontes e categorias.
- [scripts/auditar_catalogo.py](../../prototipo/scripts/auditar_catalogo.py) — auditoria de integridade somente leitura.
- [scripts/importar_ofertas_csv.py](../../prototipo/scripts/importar_ofertas_csv.py) — importador de ofertas com validação de loja, produto e fonte.
- [scripts/registrar_fontes_csv.py](../../prototipo/scripts/registrar_fontes_csv.py) — registra fontes sem aprová-las automaticamente.
- [scripts/registrar_lojas_csv.py](../../prototipo/scripts/registrar_lojas_csv.py) — registra lojas sem confirmá-las automaticamente.
- [tests/test_validar_catalogo_csv.py](../../prototipo/tests/test_validar_catalogo_csv.py) — testes do validador.
- [tests/test_importar_catalogo_csv.py](../../prototipo/tests/test_importar_catalogo_csv.py) — testes do importador SQLite.
- [tests/test_relatorio_catalogo.py](../../prototipo/tests/test_relatorio_catalogo.py) — testes do relatório do banco.
- [tests/test_importar_ofertas_csv.py](../../prototipo/tests/test_importar_ofertas_csv.py) — testes do importador de ofertas.
- [tests/test_registro_catalogo_csv.py](../../prototipo/tests/test_registro_catalogo_csv.py) — testes de cadastro de fontes e lojas.
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

**Etapa atual: MVP local de interface, API e pipeline.** A interface consulta a API para busca e detalhe, e a API lê o banco SQLite local, expondo somente produtos de fontes aprovadas. O painel Git permite selecionar arquivos do subprojeto e criar commits locais após confirmação; envio ao GitHub permanece manual. O catálogo começa vazio: nenhum produto ou preço fictício é publicado. PostgreSQL para produção, autenticação administrativa e escala para 50.000 produtos ainda dependem das próximas fases.

## Executar localmente

Em um terminal, inicie a API:

```powershell
Set-Location prototipo/apps/api
cargo run
```

Em outro terminal, inicie a interface:

```powershell
Set-Location prototipo/apps/web
npm ci
npm run dev
```

Abra `http://127.0.0.1:5173`. A primeira inicialização da API cria
`prototipo/database/catalogo.sqlite3`; os importadores Python devem receber esse mesmo
caminho com `--db`. O painel Git só atua na pasta deste catálogo e não faz push.
