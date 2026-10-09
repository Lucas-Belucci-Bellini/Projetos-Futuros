# Empresas adicionais para pesquisa — onda 2

Este documento amplia a fila de prospecção. Os negócios abaixo foram encontrados em fontes públicas; a inclusão não significa parceria, aprovação para coleta em massa nem confirmação de que todo produto do site seja entregue em Londrina.

## Prioridade por cobertura

| Prioridade | Empresa | Especialidade | Site/fonte oficial | Próximo passo |
|---|---|---|---|---|
| P1 | [Depósito Londrina](https://depositolondrina.com.br/) | Cimento, cal, argamassa, impermeabilizantes, adesivos e materiais básicos | [Catálogo oficial](https://depositolondrina.com.br/) | Solicitar catálogo estruturado, condições de entrega e frequência de atualização |
| P1 | [Romani Construção e Acabamentos](https://romaniacabamentos.com.br/) | Materiais básicos, arames, areia, madeira e acabamentos | [Catálogo de materiais de construção](https://romaniacabamentos.com.br/categoria-produto/materiais-para-construcao/) | Confirmar quais preços são finais e quais são apenas para orçamento; registrar unidade de venda |
| P1 | [Econolux](https://www.econolux.com.br/) | Elétrica, iluminação, cabos, proteção e componentes | [Loja oficial](https://www.econolux.com.br/) | Solicitar feed ou exportação de catálogo; confirmar preços e entrega para Londrina |
| P1 | [Guaporé Tintas](https://www.guaporetintas.com.br/) | Tintas imobiliárias, automotivas e industriais; acessórios | [Site oficial](https://www.guaporetintas.com.br/) | Separar catálogo de linhas por loja e validar preço por cor, base e embalagem |
| P1 | [Tintas Darka](https://tintasdarka.com.br/lojas) | Tintas e acessórios de pintura | [Localizador oficial](https://tintasdarka.com.br/lojas) | Confirmar filiais de Londrina, preços locais, validade e condições de retirada/entrega |
| P2 | House Pisos e Porcelanatos | Pisos, porcelanatos e revestimentos | [Site oficial](https://housepisoseacabamentos.com.br/) | Obter lista de produtos por caixa, m² por caixa e especificação de acabamento |
| P2 | Lumine Acabamentos | Pisos vinílicos e laminados | Endereço comercial listado publicamente; domínio de catálogo próprio ainda precisa ser confirmado | Confirmar canal oficial, marcas, metragem por embalagem e política de orçamento |
| P2 | Armazém Pisos e Acabamentos | Pisos e acabamentos | Estabelecimento encontrado em pesquisa local; site oficial não confirmado nesta etapa | Validar identidade, canal oficial e catálogo antes de registrar fonte |
| P2 | Guaporé Tintas — linha automotiva | Tintas automotivas e colorimetria | [Site oficial](https://www.guaporetintas.com.br/) | Verificar se a linha automotiva deve ser incluída no escopo inicial ou ficar em categoria secundária |
| P2 | Tintas Darka — filiais de Londrina | Tintas por linha e filial | [Filiais oficiais](https://tintasdarka.com.br/lojas) | Manter cada filial como unidade distinta apenas quando estoque, preço ou atendimento forem diferentes |

## O que já conseguimos observar em catálogos públicos

- O Depósito Londrina apresenta famílias como cimentos, cal, argamassas, impermeabilizantes, adesivos, grautes e aditivos. Isso serve para orientar a busca por produtos, mas cada item precisa de página/identificador específico antes de entrar no catálogo canônico.
- O catálogo da Romani mostra itens de construção como arame recozido, areia por granulometria, madeira por seção e tijolos por dimensão. A própria página explica que alguns preços são para orçamento; não tratar o valor de página como oferta garantida sem confirmar as condições.
- A Econolux possui catálogo online de materiais elétricos e iluminação; preços online e condições de envio precisam ser conferidos para cada produto.
- Guaporé e Tintas Darka ampliam a cobertura de tintas, mas cor, base, volume e tipo de tinta precisam ser normalizados para não comparar produtos diferentes.

## Procedimento de validação

Para cada empresa:
1. Conferir razão comercial, URL oficial, endereço e filial.
2. Registrar a fonte em `templates/fontes.csv` com estado `discovered` ou `under_review`.
3. Não marcar como parceira nem como fonte aprovada sem evidência.
4. Pedir arquivo de catálogo, feed ou autorização de uso dos campos necessários.
5. Validar pelo menos uma amostra de produto e suas unidades.
6. Só publicar oferta quando houver preço/condição, fonte, data e disponibilidade suficientemente claras.
7. Não somar várias filiais como empresas diferentes para medir adesão comercial; distinguir empresa, unidade e oferta no modelo de dados.

## Fontes de pesquisa consultadas

- Depósito Londrina: https://depositolondrina.com.br/
- Romani Construção e Acabamentos: https://romaniacabamentos.com.br/categoria-produto/materiais-para-construcao/
- Econolux: https://www.econolux.com.br/
- Guaporé Tintas: https://www.guaporetintas.com.br/
- Tintas Darka — filiais: https://tintasdarka.com.br/lojas
- House Pisos e Acabamentos: https://housepisoseacabamentos.com.br/
