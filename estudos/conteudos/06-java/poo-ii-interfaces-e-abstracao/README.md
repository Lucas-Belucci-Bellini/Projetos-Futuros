# LPOO II — Interfaces e classes abstratas

Material de estudo para revisar **interfaces, classes abstratas, herança, contratos, sobrescrita e polimorfismo** em Java.

## Por onde começar

1. Leia o [resumo explicado](./01-resumo-interfaces-e-abstracao.md).
2. Rode os dois exemplos da pasta [código](./codigo/README.md).
3. Responda ao [questionário comentado](./02-questionario-comentado.md) antes de consultar o gabarito.
4. Revise os [flashcards](./04-flashcards.md).
5. Use o [roteiro de estudo](./03-roteiro-de-estudo.md) para avaliar seu domínio.

## O que você vai aprender

- Como uma interface estabelece um contrato.
- Por que uma classe pode implementar várias interfaces.
- Quando usar classe abstrata para compartilhar dados e código.
- Como combinar extends e implements.
- Por que métodos de interface precisam de implementação pública.
- Como referências de interface permitem polimorfismo.
- Como analisar questões de múltipla escolha e erros comuns.

## Exemplos executáveis

| Exemplo | Conceitos |
|---|---|
| [01 — Conexão de dispositivos](./codigo/01-conexao/) | Conectavel, Impressora, CaixaDeSom e referências de interface |
| [02 — Funcionários da escola](./codigo/02-funcionarios/) | Registravel, Funcionario abstrato, herança e salários |

Cada exemplo tem seu próprio Main.java. **Compile e execute um diretório por vez** para evitar conflito entre classes chamadas Main.

## Regra mental

- **Interface:** define uma capacidade ou contrato — o que a classe precisa oferecer.
- **Classe abstrata:** fornece uma base comum — dados e código que as subclasses podem reutilizar.
- **Herança (extends):** uma classe herda de uma única classe direta.
- **Implementação (implements):** uma classe pode assumir vários contratos.
- **Polimorfismo:** uma referência comum pode apontar para objetos de implementações diferentes.

> Importante: interfaces modernas também podem conter métodos default e static. Não é correto dizer que toda interface contém exclusivamente assinaturas sem implementação. Veja o resumo para os detalhes.
