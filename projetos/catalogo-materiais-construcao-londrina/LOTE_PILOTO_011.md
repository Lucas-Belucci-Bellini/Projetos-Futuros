# Lote piloto 011 — argamassas, tintas e impermeabilizantes

- Data de inclusão: 2026-10-09.
- Fonte: [Romani Construção e Acabamentos](https://romaniacabamentos.com.br/), com páginas individuais para os itens abaixo.
- Novos registros incluídos no CSV: 10. Todos ficam como `pending_review`.
- Os códigos usados em `source_product_id` e `manufacturer_code` correspondem ao SKU exibido pela página da loja; GTIN não foi inferido.
- Nenhum preço deste lote foi copiado para `ofertas.csv`. As páginas podem exibir preço e estoque online, mas a oferta comparável precisa registrar observação datada e confirmar atendimento, retirada/entrega, disponibilidade e condições para Londrina.

## Produtos candidatos

| SKU da fonte | Produto | Categoria | Pendências principais |
|---|---|---|---|
| 900799 | Argamassa porcelanato e piso sobre piso interno branca 20 kg Quartzolit | Construção | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 000203 | Argamassa porcelanato e piso sobre piso externo cinza 20 kg Quartzolit | Construção | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 909308 | Rejunte porcelanato flex 5 kg marfim Quartzolit | Pisos e revestimentos | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 911124 | Tinta acrílica fosca para pisos e quadras premium 3,6 L grafite Grafftex | Pintura | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 915818 | Tinta acrílica fosca premium para pisos 18 L azul Suvinil | Pintura | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 914323 | Tinta acrílica fosca standard interna e externa 3,6 L Silverado Grafftex | Pintura | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 916654 | Impermeabilizante Impermax 7000 com fibras flexível 18 kg Votoran | Construção | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 912091 | Viaplus 7000 flexível 18 kg Viapol | Construção | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 915703 | Tinta acrílica fosca Glasu Pintura Essencial 3,6 L palha Suvinil | Pintura | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |
| 914427 | Tinta acrílica fosca para pisos e quadras premium 3,6 L marrom Grafftex | Pintura | Confirmar variante, especificação, disponibilidade e condições locais; deduplicar por código/GTIN quando possível. |

## Regras de revisão

1. Não comparar tintas de cores diferentes como se fossem a mesma variante.
2. Comparar argamassas somente quando uso, classificação, cor e peso forem equivalentes.
3. Para impermeabilizantes, confirmar composição, aplicação indicada e embalagem antes de considerar equivalência.
4. Confirmar se o SKU da loja coincide com o código do fabricante; não presumir que sejam sempre iguais.
5. Não transformar preços apresentados em página de produto em oferta do catálogo sem observação datada e condições de venda verificadas.
