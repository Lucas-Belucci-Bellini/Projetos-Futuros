# 17. Comparador de preços

## Objetivo
O recurso central do site é comparar preços de produtos equivalentes entre lojas, começando por Londrina/PR. A meta de 50.000 conta produtos canônicos únicos; preços/ofertas por loja são uma métrica separada.

Documento funcional: [OBJETIVO_COMPARADOR_DE_PRECOS.md](../../OBJETIVO_COMPARADOR_DE_PRECOS.md).

## Requisitos funcionais
1. Buscar por nome, marca, código/GTIN e categoria.
2. Agrupar ofertas somente quando a identidade e a variante do produto forem compatíveis.
3. Exibir loja, preço, unidade/embalagem, data da observação, disponibilidade, condições e link de origem.
4. Distinguir preço observado, confirmado, expirado e indisponível.
5. Mostrar frete e custo total apenas quando houver dados suficientes para a região e quantidade selecionadas.
6. Permitir ordenar por preço anunciado e, quando calculável, custo total.
7. Preservar histórico de preço sem confundi-lo com oferta atual.
8. Informar quando só existe uma oferta ou quando os dados são insuficientes para comparar.
9. Não preencher valores ausentes com estimativas apresentadas como reais.

## Modelo de dados sugerido
- products: identidade canônica do produto.
- product_variants: variantes comercialmente relevantes.
- stores: lojas e unidades comerciais.
- offers: preço, moeda, unidade, disponibilidade, condições e validade.
- price_observations: histórico imutável de observações, fonte e timestamp.
- sources: URL/método, permissão, campos autorizados e frequência permitida.
- delivery_quotes: cotação de entrega por local/quantidade, com validade e origem.

A estrutura já existente deve ser revisada antes de criar migrações, evitando tabelas duplicadas ou incompatíveis.

## API proposta
- GET /api/v1/products?query=...
- GET /api/v1/products/{id}
- GET /api/v1/products/{id}/offers?region=...
- GET /api/v1/products/{id}/price-history
- GET /api/v1/stores
- GET /api/v1/categories

As rotas são propostas, não indicam implementação existente.

## Regras de equivalência
Priorizar GTIN/EAN, código do fabricante, marca, modelo, dimensões, material, acabamento e quantidade/embalagem. Similaridade textual pode sugerir candidatos, mas não deve fundir automaticamente produtos diferentes.

## Regras de atualização
- Cada observação inclui fonte e data/hora.
- Política de expiração é definida por fonte e tipo de produto.
- Preço expirado permanece, quando apropriado, no histórico, mas não é apresentado como atual.
- Falhas de coleta não devem apagar silenciosamente a última observação.
- Importações devem ser auditáveis e idempotentes.

## Critérios de aceite
- Testes comprovam que variantes incompatíveis não são agrupadas.
- Testes comprovam cálculo de preço por unidade e total para quantidades válidas.
- Ofertas expiradas são claramente identificadas.
- Links de origem são preservados.
- Nenhum preço fictício aparece em ambiente de produção.
- O contador de 50.000 usa produtos canônicos únicos, não ofertas.
- Frete desconhecido não é tratado como zero.
