# Plano de testes e qualidade dos dados

## 1. Normalização
- Remover espaços extras e normalizar caixa sem destruir acentos ou códigos.
- Converter decimal brasileiro apenas quando o formato da fonte estiver definido.
- Preservar códigos com zeros à esquerda.
- Normalizar unidades por tabela explícita, sem adivinhar conversões.
- Não fundir automaticamente tamanhos, cores, voltagens ou quantidades diferentes.

## 2. Importador
- CSV válido com todos os campos obrigatórios.
- Cabeçalho ausente ou coluna obrigatória faltando.
- UTF-8 com acentos e nomes longos.
- Aspas e separadores dentro de campos.
- Preço vazio, malformado, negativo ou zero.
- URL ausente ou com esquema não permitido.
- Fonte desconhecida ou sem permissão aprovada.
- Arquivo excessivamente grande.
- Reimportação do mesmo lote.
- Falha parcial sem perder o relatório de processamento.

## 3. Deduplicação
- Mesmo SKU na mesma fonte atualiza o registro correspondente.
- GTIN validado e variante igual podem ser candidatos a produto comum.
- SKU igual em lojas distintas não é globalmente único por padrão.
- Mesmo fabricante com medidas diferentes permanece separado.
- Ofertas de lojas diferentes ficam separadas mesmo quando ligadas ao mesmo produto.
- Correspondência incerta vai para revisão humana.

## 4. Preço e oferta
- Preço desconhecido nunca é representado por zero.
- Parcelado e à vista não são comparados sem destacar a diferença.
- Não calcular preço por metro/quilo/litro sem unidade e quantidade válidas.
- Oferta vencida não aparece como atual.
- Condições por CEP, quantidade mínima ou pagamento ficam visíveis.
- Disponibilidade não observada não aparece como confirmada.

## 5. Interface
- Busca por nome, marca e código.
- Filtros combinados e opção de limpar filtros.
- Paginação e estado sem resultados.
- Links externos identificados.
- Data da observação legível.
- Layout móvel, teclado, foco e leitor de tela.
- Erros de carregamento e fontes fora do ar.
- Campos ausentes e descrições longas.

## 6. Segurança
- Nomes, descrições e condições são tratados como texto, nunca HTML confiável.
- Entradas com tags HTML e caracteres especiais não alteram a página.
- Consultas SQL parametrizadas.
- Validar URLs e restringir protocolos.
- Limites de tamanho e frequência no endpoint de importação.
- Segredos e tokens nunca entram no repositório.
- Logs não incluem segredos nem dados pessoais desnecessários.

## 7. Métricas por lote
Registrar linhas lidas, taxa de aceitação, duplicatas, conflitos em revisão, percentual com fonte/data, percentual de ofertas com preço observado, links quebrados e distribuição por categoria/fonte. Definir metas após medir um lote piloto real, sem incentivar aceitação de registros de baixa qualidade.
