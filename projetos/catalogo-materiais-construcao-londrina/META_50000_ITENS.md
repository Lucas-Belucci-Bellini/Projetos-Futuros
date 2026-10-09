# Meta inicial: 50.000 produtos catalogados

## Objetivo

Projetar a primeira grande carga do catálogo para chegar a **50.000 produtos únicos, rastreáveis e úteis**. A meta é de produtos normalizados, não de linhas repetidas, variações artificiais ou cópias da mesma oferta.

A Balaroti declara em sua página institucional trabalhar com aproximadamente 52 mil itens de construção. Isso mostra que a ordem de grandeza existe em um varejista do setor, mas não comprova que todos esses itens estejam disponíveis em Londrina, tenham preço público ou possam ser reutilizados por este projeto. Fonte consultada em 2026-10-09: https://www.balaroti.com.br/sobre-a-balaroti

## Situação dos lotes de descoberta — 2026-10-09

Os [Lotes Piloto 001](LOTE_PILOTO_001.md), [002](LOTE_PILOTO_002.md), [003](LOTE_PILOTO_003.md), [004](LOTE_PILOTO_004.md), [005](LOTE_PILOTO_005.md), [006](LOTE_PILOTO_006.md), [007](LOTE_PILOTO_007.md) e [008](LOTE_PILOTO_008.md) somam **68 registros candidatos a produto**. O lote 008 adiciona 4 candidatos de argamassas e impermeabilização. Todos continuam pendentes de revisão; a contagem de produtos verificados para a meta permanece em **0**. As ofertas já registradas nos lotes anteriores não substituem a confirmação de preço final, estoque, unidade atendente e entrega. As fontes e permissões de reutilização precisam ser avaliadas antes de qualquer publicação.

## Definições para não inflar os números

- **Produto canônico:** produto normalizado com marca/modelo/código e especificações identificáveis.
- **Variante:** versão distinta do mesmo produto, como tamanho, cor, voltagem, acabamento ou embalagem. Só será um item separado quando a diferença for comercialmente relevante.
- **Oferta:** um produto vendido por uma loja em determinada condição, preço e data.
- **Registro de fonte:** página, feed, arquivo ou integração de onde veio a informação.
- **Produto verificado:** item com fonte válida, identificação suficiente e data de verificação.
- **Produto pendente:** informação parcial que não deve ser apresentada como totalmente confirmada.

A mesma torneira vendida por cinco lojas deve ser, quando realmente equivalente, um produto canônico com cinco ofertas — não cinco produtos únicos artificiais.

## Taxonomia ampliada

A taxonomia detalhada está em [TAXONOMIA_AMPLIADA.md](TAXONOMIA_AMPLIADA.md). Ela organiza 36 famílias e os atributos necessários para normalizar itens, sem presumir disponibilidade local.

## Estratégia de aquisição de dados

Ordem de preferência:

1. Feeds, APIs e catálogos fornecidos oficialmente pelas lojas.
2. Planilhas CSV/XLSX ou exportações fornecidas com autorização.
3. Integrações comerciais e parcerias com varejistas/distribuidores.
4. Páginas públicas consultadas manualmente ou por automação permitida, respeitando os termos e limites de cada fonte.
5. Catálogos de fabricantes, usados para completar especificações técnicas, sem inventar que uma loja local vende ou tem estoque do item.

Não presumir que a informação disponível na internet possa ser copiada em massa. Antes de cada importação, validar os termos de uso, permissões de reutilização, direitos de imagens/textos e limites técnicos. Não contornar login, CAPTCHA, bloqueios ou outras medidas de proteção.

## Plano por etapas

### Etapa A — Descoberta e autorização
- Mapear lojas de materiais básicos, acabamento, pisos, tintas, elétrica, hidráulica, ferramentas, ferragens, madeiras, telhas, jardinagem e climatização.
- Registrar o site oficial, catálogo e forma de contato comercial.
- Solicitar feeds ou planilhas às lojas sempre que possível.
- Registrar por fonte o método permitido de coleta e as condições de uso.

### Etapa B — Modelo e importador
- Definir esquema canônico de produto, variante, loja, oferta e fonte.
- Criar importadores separados por formato/fonte, com testes e logs.
- Validar colunas, moeda, unidade, identificadores, URLs e datas.
- Guardar o arquivo bruto permitido para auditoria, sem misturá-lo diretamente à tabela pública.

### Etapa C — Normalização e deduplicação
- Priorizar GTIN/EAN, SKU do fabricante, código de modelo e marca.
- Comparar nomes normalizados, medidas, material, embalagem e especificações.
- Não unir automaticamente itens parecidos quando a identidade não for clara.
- Enviar conflitos para uma fila de revisão.
- Manter relação entre a fonte original e o produto normalizado.

### Etapa D — Cargas progressivas
- Fazer uma carga piloto pequena e revisar erros.
- Aumentar o volume por fonte após validar o importador.
- Medir quantos itens são válidos, duplicados, incompletos e rejeitados.
- Não publicar a contagem bruta como contagem de produtos válidos.
- Continuar até atingir 50.000 produtos únicos e verificáveis, se as fontes autorizadas forem suficientes.

### Etapa E — Atualização
- Reconsultar as fontes segundo sua frequência e termos permitidos.
- Expirar preços que ultrapassem a janela de atualização definida para a fonte.
- Não apagar o histórico sem política de retenção definida.
- Exibir “última verificação” e “preço não confirmado” quando aplicável.

## Métricas obrigatórias

- total de registros importados;
- total de produtos únicos;
- total de ofertas ativas;
- total de lojas com fonte validada;
- porcentagem com URL de origem;
- porcentagem com data de verificação;
- duplicatas detectadas e resolvidas;
- registros rejeitados e motivo;
- produtos sem preço publicado;
- produtos sem confirmação de venda/entrega em Londrina.

## Critério de sucesso da meta

A meta de 50.000 só será considerada concluída quando houver 50.000 produtos canônicos com identificação suficiente, fonte rastreável, estado de verificação explícito e deduplicação aplicada. Não será permitido preencher a meta com produtos inventados, registros duplicados ou produtos genéricos sem fonte.

## Limite de escopo

O catálogo pode incluir produtos nacionais de fabricantes para descoberta e especificação, mas só deve apresentar uma **oferta local** quando houver evidência de que uma loja identificada oferece aquele item para Londrina, com condições e data claramente informadas.
