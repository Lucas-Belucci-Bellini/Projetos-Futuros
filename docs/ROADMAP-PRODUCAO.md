# Roadmap de Produção — Site de Promoções e Orçamentos

**Status:** plano de trabalho; não representa funcionalidades implementadas.  
**Branch:** `planejamento-stack-typescript-rust-site`

## Princípios de execução

- Trabalhar em branches; preservar a branch `main`.
- Fechar cada fase com critérios de aceite verificáveis.
- Não inventar preços, lojas, estoque, APIs, benchmarks ou resultados de teste.
- Começar por dados rastreáveis e orçamento simples; adiar automação complexa até que a base esteja confiável.
- Rust/Axum é o backend padrão; TypeScript no backend é fallback documentado por serviço.
- Cada integração de loja deve respeitar termos, limites e APIs permitidas.

## Fase 0 — Descoberta e decisões

**Entregas**
- [ ] Confirmar público inicial e categorias do MVP.
- [ ] ADR-0001: Next.js + TypeScript no frontend.
- [ ] ADR-0002: Rust + Axum + Tokio no backend.
- [ ] ADR-0003: PostgreSQL + SQLx.
- [ ] Definir contrato OpenAPI e estratégia de geração de tipos.
- [ ] Selecionar estratégia de hospedagem, observabilidade e custos.
- [ ] Identificar fontes de dados permitidas para o primeiro catálogo.

**Aceite:** decisões registradas com alternativas, justificativas, riscos e consequências.

## Fase 1 — Fundação do repositório

**Entregas**
- [ ] Estrutura de pastas para frontend e backend.
- [ ] Configuração de formatação, lint e typecheck.
- [ ] CI para build e testes.
- [ ] Docker Compose local e `.env.example`.
- [ ] Migrações PostgreSQL.
- [ ] Health check e documentação de desenvolvimento.

**Aceite:** projeto compila localmente; CI executa comandos reais e informa falhas corretamente.

## Fase 2 — Catálogo inicial

**Entregas**
- [ ] Modelo de produtos, categorias, lojas e ofertas.
- [ ] Importação manual/administrada de um conjunto pequeno de produtos.
- [ ] API de busca, detalhe, filtros e paginação.
- [ ] Interface responsiva para listagem e página de produto.
- [ ] Proveniência e timestamp visíveis.
- [ ] Estados de carregamento, erro, vazio e dados desatualizados.

**Aceite:** cada item publicado tem fonte, horário e dados mínimos; nenhuma oferta de exemplo é confundida com preço real.

## Fase 3 — Comparação e histórico

**Entregas**
- [ ] Registrar observações de preço sem sobrescrever o histórico.
- [ ] Comparar ofertas por preço, frete, vendedor, garantia e condição.
- [ ] Classificar preço apenas com histórico suficiente e metodologia documentada.
- [ ] Marcar dados antigos e ofertas esgotadas.
- [ ] Página de metodologia de preços.

**Aceite:** totais e classificações podem ser explicados e reproduzidos com dados armazenados.

## Fase 4 — Orçamentos de PC

**Entregas**
- [ ] Perfis iniciais: faculdade básico, programação, 3D, gamer 1080p, workstation e IA local.
- [ ] Adicionar/remover/trocar peças sem perder a configuração.
- [ ] Calcular total e mostrar limitações.
- [ ] Regras iniciais de compatibilidade para CPU, placa-mãe, RAM, GPU, gabinete e fonte.
- [ ] Explicar por que cada peça foi selecionada.
- [ ] Mostrar opções custo-benefício, equilibrada e desempenho quando houver dados suficientes.

**Aceite:** regras cobertas por testes; recomendações incompletas são sinalizadas em vez de apresentadas como totalmente compatíveis.

## Fase 5 — Fontes de preços e workers

**Entregas**
- [ ] Selecionar uma primeira fonte com acesso permitido e termos verificados.
- [ ] Criar adaptador isolado e limites de taxa.
- [ ] Worker separado da API pública.
- [ ] Retentativas controladas, deduplicação e registro de falhas.
- [ ] Eventos de mudança de preço e estoque.
- [ ] Painel/relatório de frescor e falhas dos dados.

**Aceite:** indisponibilidade de uma fonte não interrompe a API; origem e horário de cada observação ficam registrados.

## Fase 6 — Contas, acompanhamento e alertas

**Entregas**
- [ ] Autenticação apenas quando necessária.
- [ ] Lista de produtos acompanhados.
- [ ] Preço-alvo e alerta de retorno ao estoque.
- [ ] Preferências e gestão de notificações.
- [ ] Controles de privacidade e exclusão de dados.

**Aceite:** autorização e preferências são testadas; nenhum usuário acessa os dados privados de outro.

## Fase 7 — SEO, acessibilidade e lançamento

**Entregas**
- [ ] Metadados, URLs canônicas e sitemap.
- [ ] Dados estruturados somente quando corretos e permitidos.
- [ ] Auditoria de acessibilidade e navegação por teclado.
- [ ] Revisão de segurança e dependências.
- [ ] Monitoramento, backups e teste de restauração.
- [ ] Política de privacidade, metodologia, termos e divulgação de afiliados.
- [ ] Beta limitado antes do lançamento público.

**Aceite:** checklist de lançamento assinado; falhas críticas resolvidas ou explicitamente bloqueiam a publicação.

## Ordem de prioridade do MVP

1. Catálogo confiável.
2. Busca e páginas de produto.
3. Fonte e data de cada preço.
4. Comparador simples.
5. Orçamento de PC com compatibilidade parcial e limitações explícitas.
6. Histórico.
7. Coleta automatizada e alertas.
8. Conta de usuário e funcionalidades avançadas.

## Riscos principais

| Risco | Mitigação |
|---|---|
| Preço ou estoque desatualizado | Timestamp, indicador de frescor e rotina de validação |
| Dados incompletos de loja | Proveniência, validação e exclusão do ranking quando necessário |
| Coleta bloqueada ou proibida | APIs autorizadas, termos revisados e fallback manual |
| Compatibilidade incorreta | Regras explícitas, testes e avisos de incerteza |
| Crescimento prematuro da arquitetura | MVP modular, serviços separados apenas quando justificados |
| Dependência de afiliados | Ranking técnico independente e alternativas não afiliadas |
| Framework Rust inadequado a um requisito | Protótipo e ADR; fallback TypeScript isolado somente se necessário |

## Definition of Done para cada entrega

- [ ] Critérios de aceite atendidos.
- [ ] Testes executados e resultados registrados.
- [ ] Documentação atualizada.
- [ ] Sem segredos no diff.
- [ ] Fontes e licenças verificadas.
- [ ] PR revisável e branch base preservada.
