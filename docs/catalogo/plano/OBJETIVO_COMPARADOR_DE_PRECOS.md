# Objetivo do projeto: comparador de preços de materiais de construção

## Visão do produto

O projeto não deve ser tratado apenas como um catálogo de produtos. Seu objetivo principal é ser um **comparador de preços de materiais de construção em Londrina/PR**, permitindo que o usuário pesquise um produto e compare ofertas equivalentes em diferentes lojas.

A meta de 50.000 representa **produtos únicos comparáveis**, não 50.000 preços. Cada produto pode ter várias ofertas, uma por loja e condição comercial.

## Exemplo de experiência (valores fictícios)

O usuário pesquisa um saco de cimento de 50 kg e visualiza ofertas lado a lado. Os valores abaixo são exemplos, não preços reais:

| Loja | Produto equivalente | Preço de exemplo | Atualização |
|---|---|---:|---|
| Loja A | Mesmo fabricante, tipo e embalagem de 50 kg | R$ 32,90 | Exemplo fictício |
| Loja B | Mesmo fabricante, tipo e embalagem de 50 kg | R$ 36,50 | Exemplo fictício |
| Loja C | Mesmo fabricante, tipo e embalagem de 50 kg | R$ 34,99 | Exemplo fictício |

O site deve identificar claramente preços vencidos, indisponíveis ou não confirmados. Nunca gerar preço para preencher uma lacuna.

## Perguntas que o comparador precisa responder

- Qual é o preço anunciado para o mesmo produto em cada loja?
- Quando cada preço foi observado ou confirmado?
- A oferta está disponível para compra ou retirada em Londrina?
- O preço exige cadastro, cupom, quantidade mínima ou pagamento específico?
- Qual é o custo de entrega para o CEP ou região selecionada, quando essa informação estiver disponível?
- Qual é o custo total estimado, sem esconder componentes desconhecidos?
- Qual loja tem o menor preço anunciado e qual tem o menor custo total confirmado?

## Definições de domínio

- **Produto canônico:** identidade normalizada do produto, com marca, modelo/código, especificações e unidade/embalagem.
- **Oferta:** condição comercial do produto por uma loja em um momento, incluindo preço, disponibilidade, URL e restrições.
- **Preço observado:** valor visto em uma fonte numa data e hora; não significa garantia de que ainda esteja válido.
- **Preço confirmado:** valor verificado segundo o método definido para aquela fonte.
- **Custo total:** preço do item mais frete e demais encargos conhecidos para a quantidade e localidade selecionadas.
- **Histórico de preço:** série temporal de observações válidas, preservando a fonte e a data.

Um mesmo produto vendido em cinco lojas conta como **um produto e até cinco ofertas**. Variações de tamanho, material, cor, modelo ou embalagem devem ser produtos separados quando alterarem de forma relevante o item comprado.

## Regras de comparação justa

1. Comparar apenas produtos equivalentes; semelhança no nome não basta.
2. Usar marca, código do fabricante/GTIN, dimensões, material, acabamento, volume e embalagem para identificar equivalência.
3. Mostrar unidade de venda e preço por unidade de medida quando for útil e seguro comparar.
4. Separar preço à vista, parcelado, preço com cupom e preço condicionado a cadastro.
5. Mostrar data/hora da observação e política de validade do preço.
6. Não marcar como disponível sem evidência suficiente.
7. Separar frete, retirada, entrega e quantidade mínima.
8. Se faltar frete ou outro custo, mostrar “não informado” em vez de declarar um vencedor por custo total.
9. Exibir a fonte original e permitir abrir a oferta na loja.
10. Não apresentar preço histórico como preço atual.

## Como adquirir os dados

Prioridade:

1. API, feed ou catálogo disponibilizado oficialmente pela loja.
2. Planilha ou arquivo fornecido com autorização.
3. Parceria ou integração comercial.
4. Consulta pública manual ou automatizada somente quando permitida pelos termos e limites da fonte.
5. Canal comercial para confirmação direta de preços quando não houver integração.

Para cada fonte, registrar método permitido de acesso, campos autorizados, frequência de atualização, evidência de permissão e limitações. Não contornar autenticação, CAPTCHA, bloqueios ou medidas de proteção. Não presumir que dados públicos, fotos e descrições possam ser republicados sem restrições.

## Métricas principais

O painel interno deve separar:
- quantidade de produtos canônicos únicos;
- quantidade de ofertas por loja;
- ofertas com preço observado recente;
- ofertas sem preço ou com preço expirado;
- lojas e fontes aprovadas;
- taxa de duplicidade;
- ofertas com disponibilidade confirmada;
- ofertas com custo de entrega conhecido;
- cobertura de comparação por produto (quantas lojas possuem oferta comparável).

**A porcentagem da meta de 50.000 deve usar apenas produtos canônicos válidos e deduplicados.** Quantidade de lojas, profissionais, arquivos, categorias e ofertas não aumenta esse numerador.

## Escopo das lojas e dos profissionais

O diretório de profissionais continua útil como uma seção separada para encontrar instaladores e prestadores. Entretanto, esses cadastros não contam para a meta de 50.000 produtos e não devem ser misturados à comparação de preços de mercadorias.

## Critério de sucesso

O comparador deve ajudar o usuário a tomar uma decisão com base em ofertas rastreáveis, equivalência real entre produtos e transparência sobre data, disponibilidade, frete e condições. O sistema não deve inventar dados para atingir metas ou declarar uma loja vencedora quando faltarem informações essenciais.
