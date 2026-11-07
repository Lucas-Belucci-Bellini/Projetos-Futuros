# Plano de execução

## Fase 1 — Mapeamento de lojas

Criar uma lista inicial de lojas de materiais de construção, depósitos, lojas de acabamento, elétrica, hidráulica, tintas, ferramentas, pisos e home centers que atendam Londrina/PR.

Para cada estabelecimento, validar:
- nome comercial;
- site oficial e página de produtos;
- endereço e contato publicados oficialmente;
- categorias vendidas;
- regiões atendidas, entrega e retirada, somente quando confirmadas;
- data da última verificação.

Começar pela Balaroti e ampliar para outras empresas locais. A lista inicial é uma amostra de pesquisa, não uma classificação de qualidade.

## Fase 2 — Taxonomia de produtos

Categorias iniciais sugeridas:
- material básico: cimento, argamassa, areia, brita e blocos;
- hidráulica e tubulações;
- elétrica e iluminação;
- tintas, impermeabilizantes e acessórios;
- pisos, porcelanatos e revestimentos;
- louças, metais, torneiras e chuveiros;
- ferramentas e ferragens;
- madeiras, portas, janelas e telhas;
- construção a seco e isolamento;
- jardinagem e área externa;
- acabamento e decoração.

A taxonomia deverá ser ajustada conforme a oferta real das lojas pesquisadas.

## Fase 3 — Modelo de dados

Separar loja, produto, oferta/preço e fonte. Um mesmo produto pode ter ofertas diferentes em lojas diferentes; não misturar preço do site com preço de loja física sem indicar a modalidade.

## Fase 4 — Protótipo navegável

Priorizar:
1. busca por nome e categoria;
2. filtros;
3. página de detalhes do produto;
4. comparação de ofertas equivalentes;
5. ficha da loja com endereço e link oficial;
6. aviso de que preço e estoque podem mudar;
7. indicação clara da data da última verificação.

## Fase 5 — Atualização dos dados

Definir uma estratégia por fonte:
- API/feed oficial, se existir e seu uso for permitido;
- importação de planilhas autorizadas;
- coleta manual assistida;
- integração comercial autorizada com as lojas.

Antes de automatizar, conferir termos de uso, robots.txt, direitos sobre imagens e limites de acesso. Não contornar controles de acesso.

## Fase 6 — Validação

- verificar se links ainda funcionam;
- detectar preços sem data ou fonte;
- evitar produtos duplicados;
- validar unidade, embalagem e variação;
- sinalizar divergências;
- testar busca e filtros;
- testar acessibilidade e versão móvel.

## Critério de conclusão do MVP

O MVP estará pronto quando houver um conjunto inicial de lojas e produtos com fontes rastreáveis, busca e filtros funcionais, data de atualização visível e nenhuma promessa de preço/estoque em tempo real sem integração que realmente forneça esses dados.
