# Roteiro de estudo — 30 a 40 minutos

Uma sessão curta e ativa vale mais do que só reler o texto.

## Etapa 1 — Entenda a diferença (5 minutos)

Sem consultar o resumo, complete em voz alta:

- Interface é um __________.
- Classe abstrata permite compartilhar __________ e __________.
- implements é usado quando uma classe __________ uma interface.
- extends é usado quando uma classe __________ outra classe.
- Polimorfismo por interface permite que a mesma referência use __________ diferentes.

Confira no [resumo](./01-resumo-interfaces-e-abstracao.md).

## Etapa 2 — Leia o código como uma história (10 minutos)

No exercício de dispositivos, responda:
1. Qual é o contrato comum?
2. O que muda entre impressora e caixa de som?
3. Por que as variáveis de Main têm tipo Conectavel?
4. Qual método de cada objeto é chamado?

No exercício dos funcionários, responda:
1. O que Professor e Secretario recebem de Funcionario?
2. Por que Funcionario é abstrata?
3. Como o salário de cada tipo é calculado?
4. Por que as duas classes implementam Registravel?

## Etapa 3 — Rode e altere (10 minutos)

1. Compile cada pasta separadamente, seguindo [as instruções do código](./codigo/README.md).
2. Mude o modelo da impressora e o nome da caixa de som.
3. Mude o valor da hora do professor e confira o salário.
4. Crie uma terceira classe de aviso que implemente a mesma interface Aviso.

## Etapa 4 — Recuperação ativa (5–10 minutos)

- Responda às 10 questões sem consultar.
- Explique cada resposta errada com uma frase.
- Crie um flashcard para cada confusão recorrente.

## Critério de domínio

Você está pronto para avançar quando conseguir, sem copiar:

- explicar interface versus classe abstrata;
- escrever class X extends Base implements A, B;
- explicar por que a implementação de método de interface é public;
- prever a saída de um exemplo de polimorfismo;
- compilar os dois exercícios e explicar os salários de R$ 3.200,00 e R$ 2.500,00.
