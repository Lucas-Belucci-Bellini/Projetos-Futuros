# Arquitetura de dados para catálogo de grande volume

## Componentes lógicos

1. **Registro de fontes:** loja/fabricante, URL, método de aquisição, permissão, frequência e data da última consulta.
2. **Área de entrada (staging):** recebe dados brutos de uma fonte e registra lote, arquivo/URL, data e resultado da importação.
3. **Validação:** verifica campos obrigatórios, tipos, moeda, unidade, URL, identificadores e dados impossíveis.
4. **Normalização:** padroniza nomes, marcas, unidades, medidas e categorias.
5. **Deduplicação:** relaciona ofertas ao produto canônico com base em códigos e especificações; casos ambíguos vão para revisão.
6. **Catálogo canônico:** guarda a identidade do produto e suas variantes.
7. **Ofertas:** liga produto e loja com preço, modalidade, condições, disponibilidade e data de verificação.
8. **Busca:** índice de texto e filtros para consulta rápida.
9. **Monitoramento:** acompanha falhas de importação, itens expirados e qualidade dos dados.

## Entidades recomendadas

- `stores`: lojas e dados comerciais confirmados.
- `sources`: origem e regras de coleta/reutilização.
- `import_batches`: lotes de ingestão e seus resultados.
- `products`: identidade canônica do produto.
- `product_variants`: variações relevantes.
- `offers`: ofertas por loja e período.
- `product_identifiers`: EAN/GTIN, SKU, código do fabricante.
- `categories`: árvore de categorias.
- `product_attributes`: atributos técnicos tipados.
- `media_assets`: imagens e licença/permissão de uso.
- `review_queue`: conflitos e registros que precisam de análise.
- `price_history`: histórico de preços com origem e data, quando permitido.

## Regras de qualidade

- Nunca usar preço zero para significar “preço desconhecido”.
- Separar preço à vista, parcelado, promocional e preço condicionado a CEP.
- Guardar unidade de venda: peça, saco, caixa, metro, m², litro, kg etc.
- Não calcular preço unitário quando a quantidade ou unidade não estiver clara.
- Não confundir uma página de fabricante com uma oferta de loja.
- Guardar o estado de disponibilidade como valor observado na fonte, com data; não prometer estoque em tempo real.
- Não reutilizar imagem ou texto protegido sem permissão/licença.
- Manter logs de erros e reprocessamento idempotente: importar o mesmo lote duas vezes não deve duplicar os registros.

## Escala e desempenho

A arquitetura deve permitir importações em lotes, paginação, índices por nome/código/categoria/marca e atualização incremental. Evitar uma única planilha gigante como banco de dados definitivo.

O banco e a tecnologia concretos serão escolhidos após definir hospedagem, volume de atualizações, orçamento e experiência de desenvolvimento. Não adotar serviços pagos ou infraestrutura complexa sem necessidade comprovada.

## Testes do pipeline

- lote válido importa corretamente;
- lote repetido não duplica produtos/ofertas;
- produto sem fonte é rejeitado ou marcado como pendente;
- preço malformado é rejeitado;
- variantes diferentes não são fundidas indevidamente;
- mesma oferta atualizada mantém histórico conforme política definida;
- erro de uma fonte não interrompe as demais;
- relatório final apresenta importados, aceitos, rejeitados e duplicados.
