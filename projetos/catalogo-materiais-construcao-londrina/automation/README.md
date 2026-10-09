# Automação Python

Este diretório será a camada de orquestração do pipeline: adaptadores de fontes autorizadas, validação, normalização, relatórios, execução agendada e monitoramento de lotes.

Os scripts existentes continuam em ../scripts/ para evitar duplicação até que os módulos possam ser extraídos com testes de regressão.

## Regras
- Não aprovar fontes automaticamente.
- Não presumir autorização para reutilizar imagens, textos, preços ou estoque.
- Preservar proveniência e data de observação.
- Usar dry-run e relatórios antes de gravar dados.
- Não contornar login, CAPTCHA, bloqueios ou limites de acesso.
- Tornar execuções idempotentes e registrar falhas sem descartar linhas silenciosamente.

## Próximos módulos
- sources/: adaptadores por fonte e método autorizado.
- validation/: regras de schema e qualidade.
- normalization/: unidades, marcas, categorias e identificadores.
- jobs/: tarefas programadas e política de repetição.
- reports/: métricas de cobertura, duplicação, atualidade e erros.

Não adicionar dependências externas sem necessidade identificada e testes correspondentes.
