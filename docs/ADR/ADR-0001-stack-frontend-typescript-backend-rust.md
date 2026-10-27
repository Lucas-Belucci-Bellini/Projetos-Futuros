# ADR-0001 — Frontend TypeScript e backend Rust

- **Status:** Proposto
- **Data:** 2026-10-27
- **Escopo:** Site de Promoções e Orçamentos de PCs
- **Branch de trabalho:** `planejamento-stack-typescript-rust-site`

## Contexto

O produto precisa de uma interface web responsiva, páginas indexáveis, busca e comparação de produtos, montagem de orçamentos e uma API com regras de compatibilidade e histórico de preços. A decisão do projeto é usar TypeScript no frontend e Rust no backend sempre que houver uma solução Rust adequada.

## Decisão

- **Frontend:** TypeScript + Next.js + React.
- **Backend:** Rust estável + Axum + Tokio.
- **Persistência:** PostgreSQL + SQLx.
- **Contrato:** OpenAPI versionado em `/api/v1`; tipos de cliente TypeScript gerados ou validados a partir do contrato.
- **Fallback:** um serviço específico pode usar TypeScript/Node.js apenas se a investigação demonstrar que não existe solução Rust suficientemente madura ou compatível para o requisito. A exceção exige ADR próprio.
- **Integrações de preços:** adaptadores separados e coleta somente por meios autorizados.

## Consequências positivas

- Fronteira clara entre interface, API, regras de negócio e persistência.
- Rust fica responsável por regras e serviços centrais; TypeScript oferece a experiência web.
- Contrato explícito reduz divergências entre frontend e backend.
- Adaptadores isolados facilitam trocar uma fonte de preço sem reescrever o domínio.

## Custos e riscos

- Duas linguagens aumentam o custo de desenvolvimento e CI.
- Tipos precisam ser gerados/validados e o contrato precisa de testes.
- O time precisará conhecer Rust async, Next.js e operação PostgreSQL.
- O SEO e o rendering do Next.js devem ser avaliados no protótipo, não presumidos.

## Alternativas consideradas

1. TypeScript no frontend e backend: simples para compartilhar linguagem, mas não é a preferência definida pelo projeto.
2. Rust em frontend e backend: rejeitado para a interface web principal porque a decisão atual exige TypeScript no frontend.
3. Microserviços desde o início: adiado; começar modular e separar serviços quando houver necessidade real.

## Critérios para revisão

Revisar esta decisão se o protótipo revelar limitações importantes, custos operacionais excessivos ou ausência de bibliotecas Rust adequadas para uma necessidade específica. Qualquer mudança deve ser documentada antes de implementação.

## Estado de implementação

Esta ADR registra uma decisão de planejamento. Ela não afirma que frontend, API, banco ou integração já foram implementados.
