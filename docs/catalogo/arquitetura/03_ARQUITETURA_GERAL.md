# 3. Arquitetura geral

## Diagrama lógico

```text
Visitante
   |
   v
React + TypeScript + Vite
   | HTTPS / JSON
   v
API Rust + Axum (/api/v1)
   | camada de serviço + repositórios
   v
PostgreSQL
   ^
   | transações de importação
Pipeline Python -> staging -> validação -> revisão -> publicação
   ^
   |
Fontes autorizadas: feeds, APIs, CSVs ou páginas com uso permitido
```

## Responsabilidades
- **Web:** apresentação, navegação, estado da interface e chamadas HTTP. Não conecta diretamente ao banco.
- **API:** valida parâmetros, aplica regras de domínio, autorização, paginação e acesso ao banco.
- **PostgreSQL:** fonte persistente de verdade para dados aprovados e histórico operacional.
- **Python:** coleta permitida, importação, normalização, relatórios e tarefas agendadas.
- **GitHub Actions:** testes e checks automatizados; não é o ambiente de execução das rotinas recorrentes de produção.
- **Armazenamento de objetos (futuro):** somente para arquivos próprios ou assets com direitos adequados; não é necessário no MVP.

## Fluxo de publicação
1. Fonte registrada e avaliada.
2. Método de acesso e campos permitidos documentados.
3. Lote recebido em staging.
4. Validação estrutural e semântica.
5. Normalização e detecção de duplicatas candidatas.
6. Revisão de exceções e aprovação.
7. Gravação transacional e idempotente.
8. API serve apenas registros publicáveis.
9. Monitoramento de atualidade e qualidade.

## Ambientes
- **Local:** dados de teste claramente sintéticos; banco local; segredos em arquivo ignorado.
- **CI:** bancos e fixtures isolados, sem dados de produção.
- **Staging:** valida integração e migrações com conjunto limitado.
- **Produção:** somente fontes e lotes aprovados, backups e monitoramento habilitados.

## Organização de diretórios planejada
- `docs/catalogo/plano/`: objetivo, escopo, requisitos e roadmap.
- `docs/catalogo/arquitetura/`: decisões e desenhos técnicos.
- `docs/catalogo/catalogacao/`: categorias, lotes e métricas.
- `docs/catalogo/fontes/`: pesquisa de fontes, lojas e profissionais.
- `docs/catalogo/operacao/`: checklist, importação e plano de testes.
- `prototipo/apps/web/`: interface React + TypeScript.
- `prototipo/apps/api/`: API Rust + Axum.
- `prototipo/automation/`: espaço para módulos de automação Python.
- `prototipo/database/`: schema local e futuras migrations PostgreSQL.
- `prototipo/scripts/`: ferramentas de importação e manutenção.
- `prototipo/templates/`: contratos de entrada CSV.
- `prototipo/tests/`: testes Python e fixtures.

Não duplicar scripts existentes ao criar módulos novos: antes de mover ou reorganizar, mapear importações, testes e compatibilidade.
