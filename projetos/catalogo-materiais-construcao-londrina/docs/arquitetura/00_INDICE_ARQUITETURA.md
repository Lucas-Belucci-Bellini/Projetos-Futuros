# Arquitetura do Catálogo — índice mestre

Este diretório divide o planejamento técnico do **Catálogo de Materiais de Construção de Londrina/PR** em documentos pequenos, rastreáveis e evolutivos. Leia do geral para o específico antes de implementar uma camada.

## Objetivo do produto
Criar um catálogo independente para pesquisar materiais vendidos por lojas que atendem Londrina, comparar ofertas verificáveis e direcionar o visitante à fonte original. A meta de 50.000 produtos é uma meta de qualidade e cobertura, não uma autorização para inventar ou copiar dados sem permissão.

## Stack escolhida
- **Frontend:** React + TypeScript + Vite.
- **CSS:** Tailwind CSS v4, com tokens próprios e componentes acessíveis.
- **Backend:** Rust + Axum, API REST versionada.
- **Banco de produção alvo:** PostgreSQL.
- **Automação e ingestão:** Python.
- **CI:** GitHub Actions; validações independentes para web, API e automação.

## Ordem de leitura
1. [Visão geral e princípios](01_VISAO_GERAL.md)
2. [Requisitos e escopo](02_REQUISITOS_ESCOPO.md)
3. [Arquitetura de alto nível](03_ARQUITETURA_GERAL.md)
4. [Frontend](04_FRONTEND.md)
5. [Backend Rust](05_BACKEND_RUST.md)
6. [Banco de dados](06_BANCO_DADOS.md)
7. [Contrato REST](07_CONTRATO_API.md)
8. [Automação Python](08_AUTOMACAO_PYTHON.md)
9. [Governança e qualidade dos dados](09_GOVERNANCA_DADOS.md)
10. [Segurança e privacidade](10_SEGURANCA.md)
11. [Testes e CI](11_TESTES_CI.md)
12. [Deploy e operação](12_DEPLOY_OPERACAO.md)
13. [UX e acessibilidade](13_UX_ACESSIBILIDADE.md)
14. [Roadmap de implementação](14_ROADMAP.md)
15. [Decisões e perguntas em aberto](15_DECISOES_PENDENTES.md)

## Documentos existentes relacionados
- [Arquitetura resumida](../../ARQUITETURA_SITE.md)
- [Contrato inicial da API](../../API_CONTRACT.md)
- [Modelo do catálogo](../../ESQUEMA_CATALOGO.md)
- [Arquitetura de dados](../../ARQUITETURA_DADOS.md)
- [Meta de 50.000 produtos](../../META_50000_ITENS.md)
- [Fontes e regras](../../FONTES_E_REGRAS.md)
- [Roadmap e backlog existente](../../ROADMAP_E_BACKLOG.md)
- [Requisitos do MVP](../../REQUISITOS_MVP.md)
- [Plano de testes existente](../../PLANO_DE_TESTES.md)
- [Registro de fontes](../../REGISTRO_DE_FONTES.md)

## Status honesto
Esta documentação é o plano-alvo. O scaffold inicial da web e o health check da API já existem no repositório, mas busca real, PostgreSQL integrado, administração, importação de produtos reais, deploy de produção e catálogo de 50.000 ainda exigem implementação e validação. Nenhum build deve ser considerado aprovado até que a CI o confirme.

## Regras para alterar o plano
- Registrar decisões arquiteturais relevantes em [15_DECISOES_PENDENTES.md](15_DECISOES_PENDENTES.md).
- Atualizar este índice e o roadmap quando novos documentos forem criados.
- Marcar tarefas como concluídas somente com evidência verificável: código, teste, workflow verde ou fonte validada.
- Não colocar credenciais, dados pessoais desnecessários ou dados de produto inventados nos exemplos.
## Documentação complementar do diretório profissional

- [Diretório de lojas e profissionais de pisos](../../PROFISSIONAIS_E_LOJAS_DE_PISOS.md)
- [Outras categorias de profissionais da construção](../../PROFISSIONAIS_OUTRAS_CATEGORIAS.md)

- [Comparador de preços](17_COMPARADOR_DE_PRECOS.md) — requisitos, dados, API e regras de comparação.
