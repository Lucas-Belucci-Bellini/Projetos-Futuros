# 14. Roadmap de implementação

## Fase 0 — Planejamento e fundação
- [x] Documentação inicial do catálogo.
- [x] Definir React + TypeScript + Vite.
- [x] Definir Tailwind CSS v4.
- [x] Definir Rust + Axum para API.
- [x] Definir Python para automação e PostgreSQL como alvo de produção.
- [x] Criar scaffold inicial da web e health check.
- [x] Dividir o planejamento de arquitetura em documentos.
- [ ] Confirmar builds e testes no GitHub Actions.
- [ ] Resolver falhas reais encontradas na CI.

**Saída:** estrutura técnica compreendida e verificável.

## Fase 1 — Fonte e piloto
- [ ] Revisar lojas e fontes candidatas.
- [ ] Registrar termos, permissões, métodos e campos permitidos.
- [ ] Aprovar manualmente uma fonte.
- [ ] Obter lote real pequeno e autorizado.
- [ ] Rodar dry-run e revisar erros.
- [ ] Importar lote e reconciliar relatório.
- [ ] Revisar amostra manualmente.

**Saída:** primeira carga real reproduzível, com proveniência.

## Fase 2 — Persistência PostgreSQL
- [ ] Escolher driver e ferramenta de migrations.
- [ ] Especificar schema PostgreSQL completo.
- [ ] Criar migrations e constraints.
- [ ] Integrar repositórios Rust.
- [ ] Definir estratégia de staging e importação.
- [ ] Testar migração da amostra.
- [ ] Documentar backup e recuperação inicial.

**Saída:** API e pipeline usando persistência coerente.

## Fase 3 — Catálogo funcional
- [ ] Implementar busca real pela API.
- [ ] Implementar filtros e paginação.
- [ ] Criar páginas de produto, categoria e loja.
- [ ] Exibir ofertas, fontes e datas.
- [ ] Implementar estados de erro e lista vazia.
- [ ] Testar acessibilidade e responsividade.

**Saída:** visitante consegue pesquisar e interpretar dados reais.

## Fase 4 — Administração e qualidade
- [ ] Escolher autenticação e papéis.
- [ ] Criar revisão de fontes, lotes e duplicatas.
- [ ] Implementar trilha de auditoria.
- [ ] Criar fluxo de correção e remoção.
- [ ] Criar relatórios de cobertura e frescor.

**Saída:** operação auditável sem edição manual insegura no banco.

## Fase 5 — Escala
- [ ] Adicionar novas fontes aprovadas gradualmente.
- [ ] Medir qualidade e custo por fonte.
- [ ] Automatizar atualizações permitidas.
- [ ] Testar desempenho e carga.
- [ ] Definir retenção de histórico.
- [ ] Avançar em direção aos 50.000 produtos únicos qualificados.

**Saída:** crescimento medido por qualidade, não só por volume.

## Regra de prioridade
Bloqueadores de permissão, integridade, segurança ou dados falsos têm prioridade sobre novas telas e volume. Não escalar coleta antes de provar que o pipeline preserva qualidade.
