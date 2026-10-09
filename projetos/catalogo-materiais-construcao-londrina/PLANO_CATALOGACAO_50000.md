# Plano operacional de catalogação — meta de 50.000 produtos

## Objetivo e definição de contagem

Construir um catálogo de **50.000 produtos canônicos únicos**, com identificação suficiente e origem rastreável. Produto canônico não é oferta: se o mesmo item for vendido por 6 lojas, conta uma vez no catálogo e pode ter 6 ofertas independentes. Cor, medida, voltagem, acabamento ou embalagem só geram um item/variante separado quando forem diferenças comerciais reais.

**Estado inicial:** os templates de produtos e ofertas estão vazios; portanto, a contagem inicial de produtos reais importados por esses templates é 0. Este documento é um plano de distribuição da meta, não afirma que os produtos já foram coletados.

## Distribuição-alvo por família

As cotas são hipóteses de planejamento, não contagens confirmadas de mercado. Serão recalibradas com dados reais após o piloto. A soma é exatamente 50.000.

| Família | Meta de produtos únicos |
|---|---:|
| Pisos, revestimentos, rodapés e acabamentos de superfície | 5.000 |
| Tintas, seladores, vernizes e acessórios de pintura | 3.500 |
| Materiais elétricos e iluminação de instalação | 3.500 |
| Materiais hidráulicos, tubos, conexões e registros | 3.500 |
| Ferragens, fixadores e itens de instalação | 3.000 |
| Ferramentas e equipamentos de uso na obra | 3.000 |
| Cimento, cal, gesso e ligantes | 2.500 |
| Argamassas, rejuntes, colas e impermeabilizantes | 3.000 |
| Tijolos, blocos, canaletas e alvenaria | 3.500 |
| Aço, telas, arames e componentes de estrutura | 3.000 |
| Areia, brita, agregados e materiais a granel | 1.500 |
| Telhas, cumeeiras, calhas e coberturas | 2.000 |
| Portas, janelas, esquadrias e vidros | 2.000 |
| Louças, metais sanitários e acessórios de banheiro | 3.000 |
| Cozinha, cubas, torneiras e acessórios | 1.500 |
| Madeiras, chapas, drywall e forros | 2.500 |
| Jardinagem, drenagem e áreas externas | 1.000 |
| Caixas d'água, bombas e armazenamento | 1.000 |
| EPIs e itens de segurança para trabalho | 1.500 |
| Materiais diversos de acabamento e instalação | 3.000 |
| **Total** | **50.000** |

## Plano de execução

### Fase 0 — Preparação do dado
- Confirmar os campos obrigatórios de produto, variante, loja, oferta e fonte.
- Definir política de equivalência, unidade de venda e deduplicação.
- Manter produtos, variantes e ofertas em conjuntos separados.
- Definir estados: rascunho, pendente de revisão, verificado, rejeitado e descontinuado.

### Fase 1 — Piloto controlado
- Começar com cimento, argamassa, blocos/tijolos, pisos, tintas e tubos/conexões.
- Reunir um lote pequeno por meio de páginas oficiais, catálogos fornecidos ou planilhas autorizadas.
- Registrar fabricante, marca, modelo/código, GTIN quando disponível, unidade/embalagem, URL de origem e data de observação.
- Revisar manualmente os casos de equivalência duvidosa antes de comparar preços.
- Não cadastrar preço quando a fonte não o publica; não inferir estoque nem frete.

### Fase 2 — Crescimento por categoria
- Expandir para as famílias da tabela, priorizando as que tiverem fontes confiáveis.
- Importar em lotes versionados; gerar relatório de aceitos, rejeitados, duplicados e incompletos.
- Fazer amostragem manual por lote e interromper a expansão de uma fonte se a qualidade cair.
- Usar dados de fabricantes para completar especificações técnicas, nunca para presumir que uma loja local vende o item.

### Fase 3 — Cobertura de ofertas
- Relacionar cada oferta ao produto/variante correto e à loja correta.
- Registrar preço, moeda, tipo de preço, unidade, condições, disponibilidade declarada, URL e data/hora.
- Frete desconhecido permanece desconhecido, nunca zero.
- Mostrar o preço como vencido ou não confirmado quando ultrapassar a validade definida para a fonte.

### Fase 4 — Auditoria e meta
- Deduplicar por GTIN/EAN, códigos do fabricante, marca/modelo e especificações.
- Revisar colisões e possíveis produtos diferentes com nomes parecidos.
- Contar apenas produtos canônicos verificados para a meta pública.
- Publicar métricas de cobertura, frescor, qualidade e quantidade de ofertas, sem confundir produto com oferta.

## Critérios mínimos de entrada

Um produto não entra na contagem verificada sem:
1. Nome identificável e categoria;
2. Fonte rastreável;
3. Unidade/embalagem ou declaração explícita de que não se aplica;
4. Marca/modelo/código/GTIN quando disponível, ou especificações suficientes para evitar duplicação;
5. Data de coleta e estado de verificação;
6. Revisão de equivalência quando houver risco de confusão.

## Regras de aquisição

Prioridade: feed/API oficial; arquivo fornecido pela empresa; parceria comercial; consulta manual ou automação permitida após avaliar os termos. Não contornar login, CAPTCHA, limites, bloqueios ou outros controles. Não republicar imagens/textos de terceiros sem base de uso apropriada.

## Relatórios por lote

Cada lote deve registrar: identificador do lote, fonte, data, método autorizado, linhas lidas, produtos novos, atualizações, duplicatas, rejeições por motivo, ofertas novas e responsável pela revisão. O lote deve poder ser reprocessado sem criar duplicatas.

## Próxima ação

Executar a pesquisa e validação do primeiro grupo de empresas, solicitar seus catálogos/feeds e só então montar o primeiro lote real de produtos e ofertas. A meta de 50.000 depende da disponibilidade de fontes adequadas; não será atingida artificialmente com registros genéricos ou inventados.
