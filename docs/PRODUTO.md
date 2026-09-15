# Especificação do Produto — Site de Promoções e Orçamentos

## 1. Visão

Plataforma que ajuda usuários brasileiros a montar PCs/notebooks adequados à finalidade e ao orçamento, comparar preços e acompanhar promoções.

## 2. Jornada principal

```text
Usuário
 -> escolhe objetivo
 -> informa orçamento
 -> informa preferências
 -> sistema calcula configurações compatíveis
 -> sistema compara preços reais
 -> mostra opções Melhor Custo, Equilibrada e Melhor Desempenho
 -> usuário acompanha preço/estoque
```

## 3. Entradas do usuário

- orçamento mínimo/máximo;
- finalidade;
- resolução desejada;
- jogos/software;
- necessidade de IA;
- mobilidade;
- preferência AMD/Intel/NVIDIA/qualquer;
- armazenamento mínimo;
- memória mínima;
- necessidade de upgrade;
- compra agora ou pode esperar promoção.

## 4. Saídas

Cada orçamento deve mostrar:

- lista completa de peças;
- preço individual;
- loja;
- frete quando disponível;
- total;
- desempenho esperado como categoria, sem prometer FPS não testado;
- limitações;
- upgrades futuros;
- consumo estimado;
- compatibilidade;
- garantia;
- preço histórico;
- alternativas equivalentes.

## 5. Orçamentos padrão

O banco deve permitir perfis reutilizáveis, por exemplo:

```text
FACULDADE-BASICO
FACULDADE-PROGRAMACAO
FACULDADE-3D
GAMER-1080P
GAMER-1440P
GAMER-ALTO
WORKSTATION
IA-LOCAL
SERVIDOR
NOTEBOOK-MOVIL
```

## 6. Explicabilidade

A página deve responder por que cada peça foi escolhida e quais compromissos existem.

Exemplo:

```text
CPU: escolhida por desempenho por núcleo.
GPU: escolhida por VRAM/TGP/preço.
RAM: 32 GB porque o perfil utiliza VMs.
SSD: 2 TB porque os projetos ocupam muito espaço.
```

## 7. Não induzir compra desnecessária

Se um PC mais barato atender ao objetivo declarado, ele deve aparecer antes de uma opção mais cara. O sistema deve destacar quando o gasto adicional traz benefício pequeno para aquele uso.

## 8. Promoções

Uma promoção deve considerar preço histórico. Um desconto percentual informado pela loja não basta para classificá-la como excelente.

## 9. Página de produto

Campos:

- especificações;
- preço atual;
- histórico;
- lojas;
- estoque;
- garantia;
- pontos fortes;
- limitações;
- produtos alternativos;
- compatibilidade;
- acessórios necessários;
- link normal;
- link afiliado, se existir.

## 10. Página de orçamento

Permitir trocar uma peça sem destruir automaticamente o restante da configuração. O motor deve recalcular:

- compatibilidade;
- preço;
- potência da fonte;
- refrigeração;
- dimensões;
- desempenho estimado;
- custo total.

## 11. Histórico e alertas

Usuário poderá observar um produto e definir preço-alvo. Eventos:

- preço atingiu alvo;
- maior queda recente;
- menor preço histórico registrado;
- retorno ao estoque;
- mudança de loja/vendedor;
- mudança relevante de frete.

## 12. Universidade/faculdade

O site deve poder recomendar por curso e softwares. Não assumir que todo estudante precisa de GPU dedicada.

## 13. Confiabilidade

Produtos e preços devem possuir origem e data de atualização. Dados antigos devem ser marcados como desatualizados.

## 14. Monetização ética

Afiliados, anúncios e patrocínios devem ser identificados. A classificação técnica não pode ser alterada ocultamente para aumentar comissão.

## 15. Critérios de sucesso

- orçamento coerente e compatível;
- preço rastreável;
- estoque atualizável;
- explicação compreensível;
- baixa quantidade de recomendações inválidas;
- histórico útil;
- transparência sobre comissão.
