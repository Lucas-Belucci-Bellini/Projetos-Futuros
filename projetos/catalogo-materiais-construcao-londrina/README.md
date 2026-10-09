# Catálogo de Materiais de Construção — Londrina/PR

## Objetivo

Planejar e construir um site independente para pesquisar e comparar materiais de construção vendidos por lojas que atendem Londrina, Paraná. A Balaroti é uma referência inicial de variedade, não a única fonte nem parceira presumida do projeto.

O catálogo deverá separar produtos, variantes, lojas, ofertas e fontes. Preços e disponibilidade sempre terão origem e data de observação.

## Escopo inicial

- Pesquisar lojas locais, depósitos, home centers e lojas especializadas.
- Catalogar produtos apenas com dados rastreáveis e método de uso permitido.
- Filtrar por categoria, loja, marca, unidade e preço disponível.
- Comparar ofertas realmente equivalentes, mostrando modalidade e condições.
- Direcionar o visitante à página oficial da loja para confirmar preço, estoque, frete e compra.
- Planejar atualização periódica sem contornar logins, CAPTCHA, bloqueios ou controles técnicos.
- Construir e validar o pipeline de importação antes de escalar para 50.000 produtos.

## Referências iniciais

- [Balaroti — loja online](https://www.balaroti.com.br/)
- [Balaroti — lojas em Londrina](https://lojas.balaroti.com.br/parana/londrina)
- [Balaroti — sobre a empresa](https://www.balaroti.com.br/sobre-a-balaroti)

Essas páginas são pontos de partida para pesquisa. Não constituem autorização automática para extração em massa ou republicação de imagens e textos.

## Documentação

- [PLANO.md](PLANO.md) — fases de pesquisa e desenvolvimento.
- [FONTES_E_REGRAS.md](FONTES_E_REGRAS.md) — fontes, atribuição e limites de coleta.
- [ESQUEMA_CATALOGO.md](ESQUEMA_CATALOGO.md) — modelo de loja, produto e oferta.
- [META_50000_ITENS.md](META_50000_ITENS.md) — meta de volume e critérios de contagem.
- [ARQUITETURA_DADOS.md](ARQUITETURA_DADOS.md) — arquitetura de ingestão e armazenamento.
- [CATEGORIAS.md](CATEGORIAS.md) — taxonomia inicial.
- [CONTRATO_IMPORTACAO_CSV.md](CONTRATO_IMPORTACAO_CSV.md) — formato e validação de arquivos.
- [REGISTRO_DE_FONTES.md](REGISTRO_DE_FONTES.md) — inventário de fontes e permissões.
- [REQUISITOS_MVP.md](REQUISITOS_MVP.md) — requisitos funcionais e não funcionais.
- [PLANO_DE_TESTES.md](PLANO_DE_TESTES.md) — testes de dados, importação, interface e segurança.
- [ROADMAP_E_BACKLOG.md](ROADMAP_E_BACKLOG.md) — tarefas priorizadas e critérios de saída.

## Princípios

1. Não inventar preços, marcas, estoque, endereços ou disponibilidade.
2. Preços variam e precisam de data da última verificação.
3. Não copiar descrições extensas, imagens ou identidade visual de terceiros sem permissão/licença adequada.
4. Preferir links para páginas originais e descrições curtas próprias.
5. Separar informação confirmada, informação declarada pela loja e informação ainda não verificada.
6. Não publicar avaliações falsas nem apresentar uma amostra como se incluísse todas as lojas da cidade.
7. Contar produto e oferta separadamente: um produto listado em cinco lojas continua sendo um produto, com até cinco ofertas.

## Status real

**Etapa atual: documentação e preparação do pipeline.** A branch ainda não contém site funcional nem catálogo com 50.000 produtos. A próxima entrega técnica é validar importações com dados autorizados, testar qualidade e então desenvolver banco e interface.
