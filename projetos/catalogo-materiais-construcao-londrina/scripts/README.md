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


## Importador SQLite

Depois de criar o banco e cadastrar uma fonte com status aprovado:

    python projetos/catalogo-materiais-construcao-londrina/scripts/importar_catalogo_csv.py produtos.csv --db catalogo-materiais.sqlite3 --source-id ID_DA_FONTE --dry-run

Se o relatório estiver correto, repita sem `--dry-run` para persistir a importação.

O importador exige que o `source_id` de cada linha do CSV seja igual ao argumento `--source-id` e que a fonte esteja aprovada na tabela `sources`. A opção de simulação usa o banco informado para validar as condições, mas reverte as alterações da importação. O schema pode ser criado caso ainda não exista; o banco precisa conter a fonte aprovada para a simulação funcionar.

O importador não busca dados na internet, não cria fontes automaticamente e não comprova a suficiência jurídica de uma autorização. Faça a revisão da evidência da fonte antes de marcar o status como aprovado.
