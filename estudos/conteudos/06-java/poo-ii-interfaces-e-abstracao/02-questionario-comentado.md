# Questionário comentado — LPOO II

Tente responder primeiro. Depois leia a justificativa e explique em voz alta por que a alternativa está correta.

## 1. Uma classe Drone precisa implementar Voador e Fotografavel. Qual declaração é correta?

A) class Drone extends Voador, Fotografavel  
B) class Drone implements Voador implements Fotografavel  
C) interface Drone implements Voador, Fotografavel  
D) class Drone implements Voador, Fotografavel  
E) class Drone extends Voador implements Fotografavel

**Resposta: D.** Uma classe usa implements para assumir contratos de interfaces. Várias interfaces são separadas por vírgulas depois de um único implements. extends não é usado para uma classe implementar interfaces.

## 2. Qual saída é produzida quando uma referência Aviso aponta primeiro para AvisoSonoro e depois para AvisoVisual?

A) Som ativado duas vezes.  
B) Luz ativada duas vezes.  
C) Primeiro Luz ativada e depois Som ativado.  
D) O código não compila porque uma interface não pode ser tipo de referência.  
E) Primeiro Som ativado e depois Luz ativada.

**Resposta: E.** A primeira atribuição cria uma instância de AvisoSonoro, então emitir() imprime Som ativado. Depois a referência recebe uma instância de AvisoVisual, e a segunda chamada imprime Luz ativada. Uma interface pode ser usada como tipo de referência.

## 3. Por que a implementação de conectar() na classe Impressora é public?

A) Todos os métodos de qualquer classe precisam ser públicos.  
B) O método sem corpo da interface é implicitamente público, e a implementação não pode reduzir sua visibilidade.  
C) @Override transforma o método em construtor.  
D) Métodos públicos dispensam correspondência com a interface.  
E) implements torna todos os atributos públicos.

**Resposta: B.** O método abstrato de interface é implicitamente public abstract. Se a classe o implementasse com visibilidade de pacote ou private, reduziria a visibilidade e o programa não compilaria. Nem todos os métodos de uma classe precisam ser públicos.

## 4. Classifique as afirmações como verdadeiras (V) ou falsas (F).

I. Uma interface pode ser instanciada diretamente com new.  
II. Uma classe pode implementar mais de uma interface.  
III. Um método abstrato sem corpo declarado em uma interface é implicitamente public abstract.  
IV. Uma classe concreta pode ignorar um método abstrato exigido por uma interface sem herdar implementação válida.

A) V – V – F – F  
B) F – F – V – V  
C) F – V – V – F  
D) V – F – V – F  
E) F – V – F – V

**Resposta: C — F, V, V, F.**

- I é falsa: instancie uma classe concreta compatível.
- II é verdadeira: por exemplo, class Drone implements Voador, Fotografavel.
- III é verdadeira para métodos de interface sem corpo.
- IV é falsa: uma classe concreta precisa implementar o contrato ou herdar uma implementação válida.

## 5. Relacione elemento e descrição.

1. interface  
2. implements  
3. @Override  
4. Método default  
5. Polimorfismo por interface

Na ordem: método com implementação fornecida pela interface; palavra-chave que declara um contrato; referência que pode acessar diferentes implementações; palavra-chave que assume o contrato; anotação que verifica sobrescrita/implementação.

A) 3 – 1 – 5 – 2 – 4  
B) 4 – 2 – 5 – 1 – 3  
C) 5 – 1 – 4 – 3 – 2  
D) 4 – 1 – 2 – 5 – 3  
E) 4 – 1 – 5 – 2 – 3

**Resposta: E — 4, 1, 5, 2, 3.** default fornece implementação comum; interface declara o contrato; polimorfismo permite trocar a implementação; implements assume o contrato; @Override pede ao compilador que verifique a correspondência da sobrescrita.

## 6. Uma interface define o método enviarMensagem(). Qual é sua finalidade principal?

A) Armazenar os atributos particulares de cada notificação.  
B) Criar diretamente todos os objetos sem classes concretas.  
C) Definir um contrato de comportamento para as classes que a implementarem.  
D) Obrigar todas as notificações a enviar mensagens exatamente da mesma forma interna.  
E) Permitir herdar simultaneamente de várias classes concretas.

**Resposta: C.** A interface garante que exista uma operação com a assinatura definida. Cada classe pode implementá-la conforme sua necessidade, respeitando o contrato.

## 7. Relatorio implementa Exportavel, que declara exportar(), mas só tem o método exibir(). O que deve mudar?

A) Adicionar public void exportar() com uma implementação.  
B) Tornar exibir() privado.  
C) Trocar implements por extends.  
D) Adicionar somente um construtor.  
E) Adicionar @Override a exibir(), mantendo esse nome.

**Resposta: A.** Nome e assinatura precisam corresponder ao método exigido. exibir() não cumpre o contrato de exportar(), e adicionar apenas @Override não corrige essa diferença.

## 8. Classifique as afirmações como V ou F.

I. Interfaces possuem atributos de instância para guardar um nome individual por objeto.  
II. Um método default tem implementação e pode ser usado pela classe sem sobrescrita.  
III. Campos declarados em interfaces são implicitamente public static final.  
IV. Usar interface como tipo de parâmetro pode permitir trocar implementações sem alterar o método cliente.

A) F – V – V – V  
B) V – V – F – V  
C) F – F – V – V  
D) V – F – V – F  
E) F – V – F – F

**Resposta: A — F, V, V, V.** Interfaces não armazenam estado de instância; campos declarados nelas são constantes. Um método default tem corpo, e uma dependência tipada pela interface pode receber qualquer objeto compatível.

## 9. Complete: primeiro declara Autenticavel; depois class Usuario assume Autenticavel; por fim declara o método boolean autenticar(String senha).

A) class – extends – public  
B) interface – extends – private  
C) abstract – implements – protected  
D) interface – implements – public  
E) interface – implements – private

**Resposta: D — interface, implements, public.** A primeira palavra declara uma interface; a classe assume o contrato com implements; a implementação do método deve ser public.

## 10. Qual alternativa descreve os métodos de Professor, que estende Funcionario e implementa Registravel?

A) Os três métodos são exigidos pela interface Registravel.  
B) exibirDados() é herdado com implementação; calcularSalario() implementa o método abstrato; registrarPresenca() atende à interface.  
C) calcularSalario() é herdado com implementação e não pode ser sobrescrito.  
D) registrarPresenca() é um construtor da interface.  
E) Java não permite combinar extends e implements.

**Resposta: B.** exibirDados() já tem corpo em Funcionario e pode ser herdado. calcularSalario() concretiza o método abstrato da classe-base. registrarPresenca() implementa o contrato de Registravel. Uma classe pode combinar extends com implements.

---

## Gabarito rápido

| Questão | Resposta |
|---:|:---:|
| 1 | D |
| 2 | E |
| 3 | B |
| 4 | C |
| 5 | E |
| 6 | C |
| 7 | A |
| 8 | A |
| 9 | D |
| 10 | B |

**Como estudar:** responda novamente sem olhar o gabarito. Se errar, explique qual regra confundiu e escreva um exemplo que mostre a regra correta.
