# 8. Automação e pipeline Python

## Responsabilidade
Python é a camada de coleta permitida, transformação, validação, importação, relatórios e tarefas agendadas. A aplicação pública não deve executar scripts de coleta a cada visita.

## Etapas do pipeline
1. **Descoberta:** registrar fonte candidata e responsável.
2. **Aprovação:** documentar termos, licença, API/feed, limites e campos permitidos.
3. **Aquisição:** usar apenas métodos aprovados (API, feed, arquivo ou página acessível conforme as regras aplicáveis).
4. **Staging:** guardar o lote bruto com data, origem e identificador do lote, respeitando limites de retenção.
5. **Validação:** verificar esquema, tipos, URLs, preço, moeda, unidade e campos obrigatórios.
6. **Normalização:** normalizar espaços, unidades e identificadores sem destruir o valor original necessário à auditoria.
7. **Deduplicação:** identificar candidatos e produzir relatório; conflitos ambíguos vão para revisão.
8. **Dry-run:** mostrar inserções, atualizações, rejeições e conflitos sem alterar dados publicados.
9. **Revisão:** pessoa autorizada decide sobre exceções.
10. **Importação:** transação idempotente, com contagens e resultado persistido.
11. **Auditoria:** relatório de qualidade e histórico da execução.

## Organização futura sugerida
- `automation/catalog_sources/`: adaptadores por fonte aprovada.
- `automation/validation/`: regras comuns.
- `automation/normalization/`: unidades, nomes e identificadores.
- `automation/importers/`: importação para staging/publicação.
- `automation/reports/`: relatórios.
- `automation/cli.py`: interface de linha de comando.
- `automation/tests/`: testes próprios, se a organização for separada da suíte atual.

Antes de criar essa árvore, revisar os scripts já existentes em `scripts/` e reutilizar o que funciona.

## Regras operacionais
- Dry-run como padrão para novos adaptadores.
- Limites de requisição e pausas respeitando políticas da fonte.
- Sem contornar login, CAPTCHA, bloqueio, paywall ou controle técnico.
- Não armazenar credenciais de loja no repositório.
- Erros devem identificar lote e registro sem despejar segredos.
- Repetir lote não pode duplicar produto/oferta indevidamente.
- Falhas parciais precisam de estado explícito e relatório recuperável.

## Agendamento
Começar manualmente. Automatizar somente após estabilidade, permissão clara e frequência acordada. A frequência deve refletir a fonte; não assumir que toda loja permite consulta contínua.
