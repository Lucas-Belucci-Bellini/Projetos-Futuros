# Testes automatizados

Na raiz do repositório, execute os testes com Python 3:

    python -m unittest discover -s projetos/catalogo-materiais-construcao-londrina/tests -v

Os testes são locais e usam apenas arquivos temporários e dados sintéticos. Eles não acessam lojas, não baixam produtos e não provam que uma fonte real permite reutilização de dados. A suíte cobre o validador de produtos, importador de produtos, relatório SQLite e importador de ofertas.
