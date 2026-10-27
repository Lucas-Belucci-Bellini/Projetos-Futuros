# Fase 0 — Descoberta e decisões técnicas

## Objetivo
Fechar as decisões fundamentais antes de criar a aplicação, preservando o escopo e evitando escolher ferramentas sem validar requisitos.

## Decisões já propostas
- Frontend: TypeScript + Next.js + React.
- Backend padrão: Rust + Axum + Tokio.
- Banco: PostgreSQL + SQLx.
- Contrato: OpenAPI em `/api/v1`, com tipos TypeScript gerados/verificados.
- Fallback de backend TypeScript: apenas para serviço específico quando a pesquisa/protótipo provar que a alternativa Rust não atende.
- Coleta: APIs oficiais/fontes autorizadas; sem contornar CAPTCHA, login, bloqueios ou termos de uso.

## Tarefas da Fase 0
- [ ] Revisar e aprovar ADR-0001.
- [ ] Confirmar o escopo do MVP e quais categorias de produtos entram primeiro.
- [ ] Comparar opções de hospedagem para Next.js, API Rust, PostgreSQL e workers, incluindo custo estimado e limites.
- [ ] Selecionar e justificar a estratégia de autenticação; deixar login fora do MVP inicial se não for necessário.
- [ ] Definir a primeira fonte de dados de preço com acesso permitido e termos documentados.
- [ ] Fechar contrato inicial OpenAPI para saúde, busca/listagem de produtos e detalhe de produto.
- [ ] Definir requisitos mínimos de observabilidade, backups, privacidade e atualização de dados.
- [ ] Registrar ADRs adicionais somente para decisões relevantes.

## Critérios de aceite
- Decisões registradas com alternativas e consequências.
- MVP delimitado e fora de escopo explícito.
- Uma fonte de dados inicial identificada com permissão/termos verificados, ou documentada como bloqueio.
- Contrato inicial de API revisável.
- Custos de operação estimados com fontes e data, sem apresentar estimativas como preços garantidos.
- Nenhuma funcionalidade marcada como implementada sem código e validação.

## Fora de escopo nesta fase
- Construir todos os coletores.
- Implementar recomendações avançadas por IA.
- Criar sistema completo de contas/alertas.
- Fazer deploy público antes de existir uma base testada.

## Estado
Planejamento documental em andamento. As caixas só devem ser marcadas após verificação.
