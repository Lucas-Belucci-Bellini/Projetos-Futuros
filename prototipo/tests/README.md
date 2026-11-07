# Testes automatizados

Na raiz do repositório, execute os testes Python:

    python -m unittest discover -s prototipo/tests -v

Para a API Rust e a interface:

    cargo test --manifest-path prototipo/apps/api/Cargo.toml
    npm --prefix prototipo/apps/web run typecheck
    npm --prefix prototipo/apps/web run build

Os testes são locais e usam apenas arquivos temporários e dados sintéticos. Eles não acessam lojas, não baixam produtos e não provam que uma fonte real permite reutilização de dados. A suíte cobre o validador de produtos, importador de produtos, relatório SQLite e importador de ofertas.
