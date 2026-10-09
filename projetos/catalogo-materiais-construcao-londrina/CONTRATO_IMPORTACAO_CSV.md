# Contrato de importação CSV

## Objetivo
Definir um formato estável para importar catálogos autorizados de lojas e fabricantes. Este documento é uma especificação; nenhum catálogo foi importado apenas por existir este arquivo.

## Formato
- CSV UTF-8, com cabeçalho obrigatório.
- Separador padrão: vírgula. Ponto e vírgula só quando a configuração da fonte declarar isso.
- Datas em ISO 8601, preferencialmente com fuso horário.
- Preservar códigos como texto para não perder zeros à esquerda.
- A regra de conversão de preço brasileiro deve ser definida por fonte.
- Tratar todo conteúdo importado como dado não confiável; nunca executar HTML ou scripts recebidos.

## Colunas de produto

| Coluna | Obrigatória | Regra |
|---|---|---|
| source_id | Sim | ID existente no registro de fontes |
| source_product_id | Recomendada | SKU/código publicado pela fonte |
| product_name | Sim | Nome real publicado |
| brand | Não | Marca publicada |
| category_path | Sim | Caminho existente na taxonomia |
| manufacturer_code | Não | Código confirmado do fabricante |
| gtin | Não | EAN/GTIN validado |
| variant | Não | Cor, tamanho, voltagem ou variação relevante |
| sale_unit | Sim | peça, saco, caixa, m, m², kg, L etc. |
| package_quantity | Não | Quantidade por embalagem, se conhecida |
| package_unit | Não | Unidade da quantidade |
| short_description | Não | Resumo permitido; não copiar texto extenso sem licença |
| product_url | Sim | URL da página de origem |
| image_url | Não | Apenas se a reutilização estiver permitida |
| observed_at | Sim | Data/hora em que o dado foi observado |
| reuse_permission | Sim | authorized, public_link_only, manual_review ou blocked |

## Colunas de oferta

| Coluna | Obrigatória | Regra |
|---|---|---|
| source_id | Sim | Fonte que publicou a oferta |
| source_offer_id | Não | ID/SKU da oferta na origem |
| store_id | Sim | Loja que realmente vende a oferta |
| source_product_id | Recomendada | Relação com o produto importado |
| price_amount | Não | Decimal positivo; vazio se não houver preço publicado |
| currency | Sim | BRL para reais |
| price_type | Sim | regular, sale, cash, installment, conditional ou unknown |
| availability | Sim | in_stock, out_of_stock, unknown ou not_applicable |
| fulfillment | Sim | online, store, pickup, delivery ou unknown |
| conditions | Não | Condições factuais, como exigência de CEP |
| offer_url | Sim | URL específica da oferta |
| observed_at | Sim | Data/hora da observação |
| expires_at | Não | Expiração publicada ou definida pela política interna |

## Cabeçalho de exemplo sem produtos reais

    source_id,source_product_id,product_name,brand,category_path,manufacturer_code,gtin,variant,sale_unit,package_quantity,package_unit,short_description,product_url,image_url,observed_at,reuse_permission

## Validação
Rejeitar ou encaminhar para revisão registros sem fonte, nome, categoria válida ou URL rastreável; preço inválido/negativo; unidade desconhecida; permissão ausente; ou possível duplicata sem evidência suficiente. Preço ausente não significa zero. Não converter parcelamento em preço à vista sem base explícita.

## Relatório por lote
Informar linhas lidas, aceitas, criadas, atualizadas, duplicatas, rejeições por motivo, itens em revisão, duração, fonte e versão das regras aplicadas.

## Idempotência
Reimportar o mesmo arquivo não pode duplicar registros. Preferir chave composta por fonte e código da origem. Sem código estável, usar URL e atributos controlados; ambiguidades devem ir para revisão humana.
