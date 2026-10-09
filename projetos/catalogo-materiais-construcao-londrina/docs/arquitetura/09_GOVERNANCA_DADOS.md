# 9. Governança e qualidade dos dados

## Proveniência obrigatória
Cada dado publicado precisa manter o vínculo com sua fonte e a data em que foi observado. O sistema deve conseguir responder: de onde veio, por qual método, quando foi importado, qual transformação ocorreu e quem aprovou a publicação.

## Estados de fonte
- `discovered`: descoberta, ainda não avaliada.
- `under_review`: em avaliação.
- `approved`: uso permitido dentro de escopo documentado.
- `paused`: coleta suspensa temporariamente.
- `rejected`: uso não autorizado ou risco não aceitável.

Uma fonte aprovada não significa que todos os campos, imagens ou métodos estejam liberados; registrar escopo e limites.

## Estados de lote
- `received`
- `validated`
- `needs_review`
- `approved`
- `imported`
- `rejected`
- `failed`

Transições precisam ser explícitas e auditáveis. Lote rejeitado não publica registros.

## Regras de produto
- Nome e categoria válidos são obrigatórios para publicação.
- Marca, modelo, GTIN/EAN, SKU e especificações só são preenchidos quando comprovados.
- Normalizar não significa apagar o valor original.
- Não deduplicar somente pelo nome.
- Identificadores confiáveis ajudam, mas precisam de escopo e origem.
- Variantes com diferenças comerciais relevantes permanecem separadas.
- Duplicatas incertas entram em fila de revisão.

## Regras de oferta
- Uma oferta precisa de produto, loja, fonte, URL de referência e data de observação quando disponíveis no contrato aprovado.
- Preço, unidade e quantidade devem ser coerentes.
- Preço antigo deve ser marcado como desatualizado, não apresentado como garantido.
- Disponibilidade precisa ser declarada com cautela e vinculada ao momento da observação.
- Não comparar preço por embalagem com preço por unidade sem conversão válida.

## Critérios para a meta de 50.000
Contar somente produtos únicos aceitos, com fonte rastreável, campos mínimos válidos e sem duplicata conhecida. Relatar separadamente:
- produtos canônicos;
- variantes;
- ofertas;
- lojas verificadas;
- fontes aprovadas;
- registros rejeitados e pendentes.

## Indicadores de qualidade
- completude dos campos obrigatórios;
- percentagem com identificador confiável;
- duplicatas confirmadas e candidatas;
- percentagem de ofertas recentes;
- links quebrados;
- erros e rejeições por lote;
- proporção de registros com permissão documentada.

## Correções e remoções
Receber denúncias e pedidos de correção por canal definido. Investigar, registrar decisão e suspender rapidamente dados cuja publicação esteja em dúvida. Preservar apenas o histórico operacional necessário e permitido.
