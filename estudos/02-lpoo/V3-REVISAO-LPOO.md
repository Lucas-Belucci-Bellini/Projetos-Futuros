# LPOO — V3 de Revisão para Exame

> Reorganizar a revisão existente em uma sequência de decisão para o exame, reforçando os pontos que o próprio material identifica como lacunas: abstração, exceções e equals/hashCode.

## Método de memória

**PEGADINHA → CONSEQUÊNCIA → REGRA → EXEMPLO → RESULTADO.** Antes de tentar decorar, descubra qual erro a questão está tentando provocar.

## 1. Pilares em uma frase

Encapsulamento protege estado; herança reutiliza especialização; polimorfismo permite comportamentos diferentes pela mesma referência; abstração define o que deve existir sem permitir instâncias inadequadas.

## 2. Overload × Override

**Overload:** mesmo nome, parâmetros diferentes, decisão na compilação. **Override:** mesma assinatura herdada, comportamento redefinido, despacho em execução.

## 3. abstract × interface

Use classe abstrata quando existe uma base com estado/comportamento compartilhado. Use interface para contrato/capacidade. Uma classe pode estender uma classe e implementar várias interfaces.

## 4. Composição

'TEM-UM' aponta para atributo/composição; 'É-UM' aponta para herança. A revisão usa Livro→CodigoLivro como exemplo de composição.

## 5. static × final

static pertence à classe. final impede reatribuição/sobrescrita/herança dependendo do contexto. `final List` não significa lista imutável.

## 6. Exceções

throw lança. catch captura. finally é útil para limpeza. Checked exigem tratamento/declaração; unchecked não exigem isso pelo compilador.

## 7. equals/hashCode

Pense: igualdade lógica primeiro, hash compatível depois. Se `a.equals(b)` é true, os hashes precisam ser iguais.

## 8. Coleções

List aceita repetição e mantém ordem; Set representa conjunto; Map trabalha com chave→valor. HashSet/HashMap dependem de equals/hashCode para objetos usados na estrutura.

## 9. instanceof

É uma ferramenta válida, mas uma sequência excessiva de testes de tipo pode indicar que o polimorfismo não está sendo aproveitado.

## 10. Estratégia anti-branco

Quando esquecer a definição, pergunte: **o objeto é-um ou tem-um?**, **estou em compilação ou execução?**, **estou protegendo estado ou definindo contrato?**, **estou lançando ou capturando exceção?**.

## Checklist de prova

- [ ] Consigo explicar a regra em uma frase.
- [ ] Sei identificar a pegadinha antes de calcular.
- [ ] Consigo fazer um exemplo sem olhar.
- [ ] Sei verificar minha resposta.
