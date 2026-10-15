# Resumo — Interfaces e classes abstratas

## 1. Interface: um contrato

Uma interface descreve operações que uma classe promete oferecer. Ela informa **o que pode ser feito**, sem obrigar diferentes classes a executar a operação da mesma maneira.

~~~~java
interface Voador {
    void voar();
}

interface Fotografavel {
    void fotografar();
}

class Drone implements Voador, Fotografavel {
    @Override
    public void voar() {
        System.out.println("Drone voando.");
    }

    @Override
    public void fotografar() {
        System.out.println("Foto registrada.");
    }
}
~~~~

A declaração correta é **class Drone implements Voador, Fotografavel**. A palavra implements aparece uma vez; as interfaces são separadas por vírgulas.

### Regras importantes

- Uma classe pode implementar várias interfaces.
- Uma classe concreta deve implementar os métodos abstratos exigidos, a menos que herde uma implementação válida.
- Um método de interface sem corpo, como void voar();, é implicitamente public abstract.
- A implementação não pode reduzir a visibilidade; use public.
- @Override não cria um método. Ajuda o compilador a verificar se ele sobrescreve ou implementa um método de um supertipo.
- Uma interface não pode ser instanciada diretamente com new.

## 2. Interfaces em Java moderno

Interfaces podem conter:

- **Métodos abstratos:** declaram operações que uma implementação deve fornecer.
- **Métodos default:** têm corpo e podem ser usados sem que cada classe implementadora precise sobrescrevê-los.
- **Métodos static:** pertencem à própria interface e são chamados pelo nome dela.
- **Constantes:** os campos declarados em interfaces são implicitamente public static final.

Uma interface não possui campos de instância para guardar um valor diferente em cada objeto. Se cada objeto precisa armazenar seu próprio nome ou matrícula, esse estado deve ficar em uma classe.

### Exemplo de método default

~~~~java
interface Saudavel {
    default void saudar() {
        System.out.println("Olá!");
    }
}

class Pessoa implements Saudavel {
    // Pessoa pode usar saudar() sem sobrescrevê-lo.
}
~~~~

Se duas interfaces fornecerem métodos default conflitantes com a mesma assinatura, a classe pode precisar sobrescrever o método para resolver a ambiguidade.

## 3. Classe abstrata: uma base compartilhada

Use uma classe abstrata quando objetos compartilham estado ou comportamento, mas a classe-base não deve ser instanciada diretamente.

~~~~java
abstract class Funcionario {
    private final String nome;
    private final String matricula;

    public Funcionario(String nome, String matricula) {
        this.nome = nome;
        this.matricula = matricula;
    }

    public void exibirDados() {
        System.out.println("Nome: " + nome);
        System.out.println("Matrícula: " + matricula);
    }

    public abstract double calcularSalario();
}
~~~~

A classe compartilha os dados e exibirDados(), mas deixa calcularSalario() para cada tipo concreto de funcionário.

- abstract class: declara uma classe abstrata.
- Construtor: inicializa a parte comum do objeto.
- super(nome, matricula): chama o construtor da classe-base.
- Método concreto: tem corpo e pode ser herdado.
- Método abstrato: não tem corpo; uma subclasse concreta precisa implementá-lo, salvo quando herda uma implementação válida.

## 4. Herança e interfaces juntas

Esta declaração é válida:

~~~~java
class Professor extends Funcionario implements Registravel
~~~~

A ordem é: primeiro extends e depois implements.

Java permite apenas uma superclasse direta, mas uma classe pode implementar várias interfaces:

~~~~java
class Exemplo extends ClasseBase implements Voador, Fotografavel, Registravel
~~~~

Não confunda:
- extends em uma classe: herda de uma classe.
- implements em uma classe: assume um ou mais contratos.
- extends em uma interface: uma interface pode estender uma ou mais interfaces.

## 5. Polimorfismo por interface

~~~~java
interface Aviso {
    void emitir();
}

class AvisoSonoro implements Aviso {
    @Override
    public void emitir() {
        System.out.println("Som ativado.");
    }
}

class AvisoVisual implements Aviso {
    @Override
    public void emitir() {
        System.out.println("Luz ativada.");
    }
}
~~~~

Com uma referência do tipo da interface:

~~~~java
Aviso aviso = new AvisoSonoro();
aviso.emitir(); // Som ativado.

aviso = new AvisoVisual();
aviso.emitir(); // Luz ativada.
~~~~

O tipo da referência define quais operações podem ser chamadas; o objeto real determina qual implementação sobrescrita será executada. Isso reduz o acoplamento: o código cliente depende do contrato Aviso, não de uma implementação específica.

## 6. Por que a implementação precisa ser public?

Em uma interface, um método abstrato é implicitamente público. A implementação não pode ter visibilidade mais restrita.

~~~~java
interface Conectavel {
    void conectar();
}

class Impressora implements Conectavel {
    @Override
    public void conectar() {
        System.out.println("Impressora conectada.");
    }
}
~~~~

Declarar conectar() sem public dentro de Impressora reduziria a visibilidade do método e impediria a compilação.

## 7. Comparação rápida

| Critério | Interface | Classe abstrata |
|---|---|---|
| Ideia principal | Contrato/capacidade | Base com estado e comportamento comum |
| Campos de instância | Não | Sim |
| Construtor | Não | Sim |
| Métodos abstratos | Sim | Sim |
| Métodos com corpo | Sim, como default e static | Sim |
| Herança múltipla de tipo | Várias interfaces | Uma superclasse direta |
| Palavra-chave usada pela classe | implements | extends |

## 8. Erros comuns em prova

1. **Usar extends para implementar interfaces:** errado; use implements.
2. **Escrever implements A implements B:** errado; use implements A, B.
3. **Implementar método sem public:** reduz a visibilidade exigida pelo contrato.
4. **Apenas adicionar @Override:** não corrige nome diferente; exibir() não substitui exportar().
5. **Achar que referência de interface não pode apontar para um objeto:** pode, se o objeto implementar a interface.
6. **Achar que classe concreta pode ignorar todos os métodos da interface:** precisa implementar ou herdar implementações válidas.
7. **Achar que todo método de interface precisa ser implementado em cada classe:** métodos default e implementações herdadas podem dispensar isso.
8. **Achar que interface guarda nome individual por objeto:** seus campos são constantes, não atributos de instância.

## 9. Fórmulas para memorizar

- Várias interfaces: class X implements A, B.
- Herança mais interfaces: class X extends Base implements A, B.
- Implementação de método: @Override public void acao() { ... }.
- Salário por hora: quantidadeHoras * valorHora.
