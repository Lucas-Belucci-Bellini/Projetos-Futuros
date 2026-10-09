# Banco SQLite inicial

O schema `001_initial_schema.sql` cria tabelas para lojas, fontes, lotes de importação, produtos observados na fonte, ofertas e erros de importação.

## Criar banco local

Na raiz do repositório, execute:

    sqlite3 catalogo-materiais.sqlite3 < projetos/catalogo-materiais-construcao-londrina/database/001_initial_schema.sql

O banco contém apenas a estrutura. Nenhum produto real é incluído por esta migração.

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
