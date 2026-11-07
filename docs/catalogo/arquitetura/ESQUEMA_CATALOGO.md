# Esquema inicial do catálogo

Este é um modelo conceitual para orientar a implementação. Não representa uma base de dados já criada.

## Loja

| Campo | Finalidade |
|---|---|
| id | Identificador interno estável |
| nome | Nome comercial |
| site_oficial | Página oficial |
| pagina_catalogo | Página oficial de produtos |
| telefone_publico | Contato publicado pela empresa |
| endereco | Endereço confirmado |
| bairro | Bairro, se confirmado |
| cidade | Londrina |
| categorias | Categorias atendidas |
| entrega_retirada | Condições publicadas e verificadas |
| fonte_url | Origem do registro |
| verificado_em | Data da última confirmação |
| status_verificacao | Confirmado, pendente ou desatualizado |

## Produto

| Campo | Finalidade |
|---|---|
| id | Identificador interno |
| nome_normalizado | Nome para busca |
| nome_fonte | Nome exibido pela loja |
| categoria | Categoria principal |
| marca | Marca, se publicada |
| modelo_codigo | Modelo, SKU ou código publicado |
| descricao_resumida | Resumo original e objetivo |
| unidade_venda | Unidade, saco, caixa, metro, litro etc. |
| quantidade_embalagem | Quantidade por embalagem, se aplicável |
| especificacoes | Dados técnicos relevantes |
| imagem_url | Apenas quando o uso da imagem for permitido |
| fonte_url | Página de origem |
| verificado_em | Data da última verificação |

## Oferta

| Campo | Finalidade |
|---|---|
| id | Identificador interno |
| produto_id | Referência ao produto normalizado |
| loja_id | Referência à loja |
| preco | Valor publicado, sem inferência |
| moeda | BRL |
| preco_por_unidade | Calcular apenas se unidade e quantidade forem conhecidas |
| modalidade | Online, física, retirada ou outra modalidade declarada |
| condicoes | Promoção, pagamento, CEP ou restrições relevantes |
| disponibilidade | Apenas quando informada pela fonte |
| frete | Valor/condição se a fonte informar; caso contrário, não informado |
| fonte_url | URL exata da oferta |
| verificado_em | Data e hora da consulta |
| status | Atual, vencida, indisponível ou pendente |

## Regras para comparação

1. Comparar somente produtos equivalentes ou explicar diferenças.
2. Normalizar unidade e embalagem antes de calcular preço por unidade.
3. Não assumir que o menor preço final é o menor preço anunciado: frete, quantidade mínima e condições podem alterar o total.
4. Não usar dados antigos como se fossem atuais.
5. Quando não houver informação, exibir “não informado” em vez de adivinhar.
6. Guardar histórico de preços somente quando a origem, a data e a política de retenção estiverem definidas.

## Possíveis extensões futuras

- variações de cor, tamanho, acabamento e voltagem;
- região/bairro de entrega;
- calculadora de materiais para orçamento, identificada como estimativa;
- listas de compra;
- comparação de custo total;
- canal de correção de dados por lojas e usuários.
