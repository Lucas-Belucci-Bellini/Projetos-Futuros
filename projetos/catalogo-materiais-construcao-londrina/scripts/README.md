# Scripts do catálogo

## Validador CSV inicial

O arquivo `validar_catalogo_csv.py` verifica a estrutura e os campos básicos do CSV de produtos antes de qualquer importação.

Na raiz do repositório, execute:

    python projetos/catalogo-materiais-construcao-londrina/scripts/validar_catalogo_csv.py caminho/para/produtos.csv

Para salvar o relatório:

    python projetos/catalogo-materiais-construcao-londrina/scripts/validar_catalogo_csv.py caminho/para/produtos.csv --report relatorio.json

Código de saída:
- 0: todas as linhas foram aceitas na validação estrutural.
- 1: arquivo ou linhas reprovados.
- 2: falha ao gravar o relatório.

Este validador não importa dados no banco, não verifica se o preço continua atual e não comprova que uma autorização é juridicamente suficiente. A coluna reuse_permission precisa estar como authorized, mas a evidência de autorização ainda deve ser conferida no registro da fonte.
