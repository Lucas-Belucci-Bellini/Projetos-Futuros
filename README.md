# Site de Promoções e Orçamentos de PCs

Branch: `site-promocoes-pc-faculdade`

Projeto futuro de um site brasileiro para promoções, comparação de peças, notebooks e montagem de orçamentos de PCs, com foco especial em estudantes de faculdade, programação, game development, jogos e IA.

## Objetivo

O site deve responder: **qual computador comprar para uma finalidade e orçamento específicos?**

A recomendação deve considerar objetivo, orçamento, compatibilidade, desempenho, possibilidade de upgrade, consumo, garantia, manutenção, preço atual e histórico.

## Perfis de usuário

- faculdade / estudo;
- programação;
- engenharia e arquitetura;
- design e 3D;
- edição e criação;
- gaming moderado;
- gaming de alto desempenho;
- IA local/Hermes;
- servidor doméstico.

## Funcionalidades

1. Orçamento por objetivo e orçamento.
2. Comparador de peças.
3. Comparador de notebooks e PCs prontos.
4. Verificador de compatibilidade.
5. Histórico de preços.
6. Detector de estoque.
7. Alertas de queda de preço e retorno ao estoque.
8. Planejador de upgrade.
9. Estimativa de TCO.
10. Links de afiliados identificados.
11. Páginas de promoções.
12. Página explicando por que uma configuração foi recomendada.

## Regra de recomendação

```text
objetivo
+ orçamento
+ compatibilidade
+ desempenho
+ consumo
+ upgrade
+ preço atual
+ histórico
+ estoque
+ garantia
+ manutenção
= recomendação
```

Menor preço não vence automaticamente: confiabilidade, garantia e histórico também entram no cálculo.

## Dados de cada produto

- fabricante;
- modelo;
- part number/SKU;
- categoria;
- especificações completas;
- preço à vista;
- preço parcelado;
- frete;
- estoque;
- loja;
- garantia;
- data/hora da coleta;
- histórico;
- URL oficial;
- URL de afiliado, quando permitida;
- classificação de preço.

## Classificação de preço

- `EXCELENTE`
- `BOM`
- `NORMAL`
- `ALTO`
- `ESGOTADO`

## Futuro bot de preços

O site deve poder consumir dados de um bot separado que monitora preços e estoque. O bot deve registrar mudanças e gerar eventos como:

```text
PRICE_DROP
BACK_IN_STOCK
HISTORICAL_LOW
OUT_OF_STOCK
PRICE_HIGH
```

Toda coleta deve respeitar termos de uso, APIs oficiais, robots.txt e limites das lojas. Não contornar CAPTCHA, bloqueios ou mecanismos anti-bot.

## Monetização

Possíveis fontes:

- programas de afiliados;
- conteúdo/reviews;
- publicidade identificada;
- recursos premium no futuro.

Quando houver comissão, ela deve ser informada ao usuário e nunca deve alterar escondido o ranking técnico.

## Stack sugerida

```text
Next.js + TypeScript
        ↓
API / Server Actions
        ↓
PostgreSQL / Supabase
        ↓
Workers de coleta e normalização
        ↓
Histórico + estoque + alertas
```

Python pode ser usado nos coletores/normalizadores quando apropriado. GitHub Actions pode executar tarefas agendadas simples; com crescimento, usar worker/fila persistente.

## Estrutura documental

- `docs/PRODUTO.md` — requisitos funcionais completos.
- `docs/ARQUITETURA.md` — arquitetura técnica.
- `docs/MODELO-DADOS.md` — produtos, lojas, preços, estoque e histórico.
- `docs/MOTOR-ORCAMENTO.md` — geração de configurações.
- `docs/MOTOR-COMPATIBILIDADE.md` — regras de compatibilidade.
- `docs/BOT-PRECOS.md` — bot e coleta.
- `docs/AFILIADOS.md` — afiliados e transparência.
- `docs/UX.md` — páginas e experiência.
- `docs/SEO.md` — estratégia de páginas e busca.
- `docs/ROADMAP.md` — fases do projeto.

## Comando inicial para Claude Code

> Leia `README.md` e todos os arquivos de `docs/` desta branch antes de alterar ou criar código. Trate os documentos como especificação do produto. Antes de implementar, identifique requisitos, integrações e riscos. Não invente APIs ou dados de lojas. Quando existir diferença entre preço real, preço histórico e preço de afiliado, manter os três conceitos separados.
