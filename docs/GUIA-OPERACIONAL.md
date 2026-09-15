# Guia Operacional da Base Móvel

A `main` é o índice. Cada subsistema deve evoluir em sua própria branch e integração cruzada deve ser registrada em `projeto/base-movel`.

## Princípios

1. Separar função, energia, rede e armazenamento.
2. Toda compra deve registrar modelo/part number, preço à vista, preço parcelado, frete, garantia e data.
3. Toda peça crítica deve ter alternativa compatível.
4. Nenhum servidor individual deve derrubar os demais durante manutenção, quando o desenho permitir.
5. Mudanças que afetam duas ou mais áreas devem ter registro de integração.

## Ordem geral do projeto

Requisitos -> arquitetura -> compatibilidade -> orçamento -> compra -> montagem -> testes -> operação -> manutenção -> upgrade.
