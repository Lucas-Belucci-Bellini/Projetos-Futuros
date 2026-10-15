# Flashcards — Interfaces e classes abstratas

Tente responder antes de ler a resposta.

| Frente | Verso |
|---|---|
| O que uma interface define? | Um contrato de operações ou capacidades. |
| Como uma classe implementa duas interfaces? | class X implements A, B |
| Uma classe pode estender duas classes? | Não. Ela tem uma única superclasse direta. |
| Uma classe pode implementar várias interfaces? | Sim. |
| Como combinar herança e interfaces? | class X extends Base implements A, B |
| Qual é o acesso de um método abstrato declarado em interface? | Implicitamente public abstract. |
| Por que a implementação deve ser public? | Para não reduzir a visibilidade do método do contrato. |
| Para que serve @Override? | Para o compilador verificar a sobrescrita ou implementação pretendida. |
| Uma interface pode ser instanciada com new? | Não; instancie uma classe concreta compatível. |
| O que é polimorfismo por interface? | Uma referência de interface pode apontar para objetos de implementações diferentes. |
| O que é uma classe abstrata? | Uma base que pode compartilhar estado e código e não pode ser instanciada diretamente. |
| Para que serve super(...)? | Chama o construtor da superclasse para inicializar a parte herdada. |
| O que diferencia método concreto de abstrato? | O concreto tem corpo; o abstrato declara a operação sem corpo. |
| O que faz um método default? | Oferece implementação de instância reutilizável pela classe implementadora. |
| Campos de interface podem guardar estado individual? | Não; são implicitamente public static final. |
| O que ocorre se duas interfaces tiverem métodos default conflitantes? | Pode ser necessário sobrescrever o método para resolver a ambiguidade. |
