# Banco SQLite inicial

O schema `001_initial_schema.sql` cria tabelas para lojas, fontes, lotes de importação, produtos observados na fonte, ofertas e erros de importação. `002_library.sql` cria o banco separado de notas e documentos pessoais. A API aplica cada schema no respectivo banco local; o banco do catálogo fica em `prototipo/database/catalogo.sqlite3` e o da biblioteca, no perfil do usuário do Windows (`%LOCALAPPDATA%\CatalogoMateriais\biblioteca.sqlite3`). Ambos são ignorados pelo Git.

## Criar banco local

Para criar um banco manualmente:

    sqlite3 prototipo/database/catalogo.sqlite3 < prototipo/database/001_initial_schema.sql

O banco contém apenas a estrutura. Nenhum produto real nem documento pessoal é incluído pelas migrações.

## Cadastrar uma fonte aprovada

Antes de importar, registre a fonte com evidência de permissão e marque-a como `approved` somente depois da revisão. Exemplo de estrutura SQL para ambiente local de teste:

    INSERT INTO sources (
      id, organization_name, official_url, source_type, access_method,
      permission_status, permission_evidence_url, permitted_fields
    ) VALUES (
      'fonte-teste', 'Fonte de teste', 'https://example.com',
      'authorized_file', 'arquivo de teste próprio', 'approved',
      'https://example.com/termos', 'nome,categoria,URL'
    );

Não use o exemplo como declaração de que um site real concedeu autorização.

## Importação

Consulte `../scripts/README.md` para executar o importador. Use primeiro `--dry-run` e inspecione o relatório antes de persistir dados.
