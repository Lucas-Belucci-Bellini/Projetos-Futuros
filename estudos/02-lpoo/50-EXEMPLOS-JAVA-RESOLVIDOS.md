# LPOO — 50 exemplos Java resolvidos

Este arquivo contém 50 exemplos derivados dos tópicos da revisão correspondente. Cada exemplo traz a situação e o resultado esperado.

## 1. Encapsulamento básico

**Exemplo:** `private String nome; public String getNome(){return nome;}`.

**Resultado:** O acesso ao campo passa pela API da classe.

## 2. Setter com validação

**Exemplo:** `if(nome==null || nome.isBlank()) throw new IllegalArgumentException();`.

**Resultado:** Nome inválido é rejeitado.

## 3. Construtor usando setter

**Exemplo:** `Livro(String t){setTitulo(t);}`.

**Resultado:** A validação vale na criação e na alteração.

## 4. Lista não modificável

**Exemplo:** `return Collections.unmodifiableList(livros);`.

**Resultado:** O consumidor não consegue modificar a lista retornada.

## 5. Herança

**Exemplo:** `class LivroDigital extends Livro {}`.

**Resultado:** LivroDigital herda de Livro.

## 6. super no construtor

**Exemplo:** `super(titulo,autor,ano);`.

**Resultado:** A construção da superclasse ocorre antes do corpo da filha.

## 7. super em método

**Exemplo:** `super.apresentarDados();`.

**Resultado:** Executa a versão da superclasse.

## 8. Polimorfismo

**Exemplo:** `Livro l=new LivroDigital(...); l.apresentarDados();`.

**Resultado:** A implementação sobrescrita de LivroDigital é chamada.

## 9. @Override

**Exemplo:** `@Override public void apresentarDados(){...}`.

**Resultado:** O compilador verifica a sobrescrita.

## 10. Sobrecarga

**Exemplo:** `exibirMensagem()` e `exibirMensagem(String m)`.

**Resultado:** Mesmo nome, parâmetros diferentes.

## 11. Sobrecarga com tipos

**Exemplo:** `buscar(String t)` e `buscar(String t,int ano)`.

**Resultado:** As chamadas são escolhidas pela assinatura.

## 12. Retorno não cria overload

**Exemplo:** `int f()` e `double f()` apenas por retorno.

**Resultado:** Não compila como sobrecarga.

## 13. Classe abstrata

**Exemplo:** `abstract class Forma { abstract double calcularArea(); }`.

**Resultado:** Forma não pode ser instanciada diretamente.

## 14. Método abstrato

**Exemplo:** `abstract double calcularArea();`.

**Resultado:** Uma subclasse concreta precisa implementá-lo.

## 15. Classe concreta

**Exemplo:** `class Circulo extends Forma`.

**Resultado:** Pode ser instanciada após implementar métodos abstratos.

## 16. Referência abstrata

**Exemplo:** `Forma f = new Circulo(2);`.

**Resultado:** Referência abstrata aponta para objeto concreto.

## 17. Interface

**Exemplo:** `interface Alugavel { boolean isDisponivel(); }`.

**Resultado:** Define um contrato.

## 18. implements

**Exemplo:** `class Carro implements Alugavel`.

**Resultado:** Carro deve cumprir os métodos da interface.

## 19. Múltiplas interfaces

**Exemplo:** `class Carro extends Veiculo implements Alugavel, Comparable<Carro>`.

**Resultado:** Uma classe pode implementar várias interfaces.

## 20. Classe + interface

**Exemplo:** `class LivroFisico extends Livro implements Emprestavel`.

**Resultado:** Herança e capacidade podem coexistir.

## 21. Composição

**Exemplo:** `private CodigoLivro codigoLivro;`.

**Resultado:** Livro tem-um CodigoLivro.

## 22. Composição com lista

**Exemplo:** `private List<Livro> livros;`.

**Resultado:** Colecao tem-vários Livro.

## 23. É um vs. tem um

**Exemplo:** `LivroDigital extends Livro` versus atributo CodigoLivro.

**Resultado:** 'É um' aponta para herança; 'tem um' para composição.

## 24. static contador

**Exemplo:** `private static int contador=1;`.

**Resultado:** Uma única variável é compartilhada pela classe.

## 25. static método

**Exemplo:** `static int soma(int a,int b){return a+b;}`.

**Resultado:** Pode ser chamado como `Classe.soma(2,3)` e retorna 5.

## 26. final atributo

**Exemplo:** `private final String isbn;`.

**Resultado:** Depois de inicializado, não pode ser reatribuído.

## 27. final método

**Exemplo:** `final void validar(){}`.

**Resultado:** Não pode ser sobrescrito por subclasses.

## 28. final classe

**Exemplo:** `final class Util {}`.

**Resultado:** Não pode ser estendida.

## 29. final referência

**Exemplo:** `final List<String> nomes=new ArrayList<>();`.

**Resultado:** A referência não muda, mas `add()` ainda é possível.

## 30. throw

**Exemplo:** `throw new IllegalArgumentException("inválido");`.

**Resultado:** Uma exceção é lançada naquele ponto.

## 31. try/catch

**Exemplo:** `try{...}catch(IllegalArgumentException e){...}`.

**Resultado:** A exceção pode ser capturada.

## 32. finally

**Exemplo:** `finally { liberarRecurso(); }`.

**Resultado:** É usado para limpeza após a tentativa.

## 33. Checked exception

**Exemplo:** `void ler() throws IOException`.

**Resultado:** A chamada deve tratar ou declarar a exceção.

## 34. Unchecked exception

**Exemplo:** `throw new IllegalArgumentException();`.

**Resultado:** Não exige tratamento obrigatório pelo compilador.

## 35. Catch específico

**Exemplo:** `catch(IOException e)`.

**Resultado:** Captura o tipo específico.

## 36. Ordem dos catch

**Exemplo:** `catch(IOException e)` antes de `catch(Exception e)`.

**Resultado:** O específico deve vir antes do genérico.

## 37. equals padrão

**Exemplo:** Livro sem sobrescrever `equals`.

**Resultado:** Objetos distintos com mesmos dados continuam diferentes por identidade.

## 38. equals por dados

**Exemplo:** Sobrescrever `equals` comparando título, autor e ano.

**Resultado:** Livros com os mesmos dados podem ser considerados iguais.

## 39. hashCode compatível

**Exemplo:** Usar os mesmos campos de `equals` no `hashCode`.

**Resultado:** Objetos iguais produzem o mesmo hash.

## 40. HashSet

**Exemplo:** `Set<Livro> livros=new HashSet<>();`.

**Resultado:** Duplicatas são controladas com `equals/hashCode`.

## 41. HashMap

**Exemplo:** `Map<String,Livro> porIsbn=new HashMap<>();`.

**Resultado:** Uma chave pode localizar um Livro.

## 42. protected

**Exemplo:** `protected void exibirDadosBasicos(){}`.

**Resultado:** Acesso é ampliado para subclasses e pacote conforme as regras Java.

## 43. private

**Exemplo:** `private double preco;`.

**Resultado:** Acesso direto só dentro da própria classe.

## 44. public

**Exemplo:** `public void salvar(){}`.

**Resultado:** Método pode compor a API pública da classe.

## 45. enum

**Exemplo:** `enum StatusCompra { PENDENTE, PAGA, CANCELADA }`.

**Resultado:** A variável fica restrita aos valores definidos.

## 46. instanceof

**Exemplo:** `if(livro instanceof LivroDigital)`.

**Resultado:** Testa compatibilidade do objeto com o tipo.

## 47. Pattern matching

**Exemplo:** `if(livro instanceof LivroDigital ld) ld.baixar();`.

**Resultado:** Testa e já cria a variável tipada.

## 48. List

**Exemplo:** `List<String> x=new ArrayList<>();`.

**Resultado:** Mantém ordem e aceita repetição.

## 49. Set

**Exemplo:** `Set<String> x=new HashSet<>(); x.add("A"); x.add("A");`.

**Resultado:** O conjunto mantém uma ocorrência de `A`.

## 50. Map

**Exemplo:** `Map<String,Integer> m=Map.of("Lucas",10);`.

**Resultado:** `Lucas` associa-se ao valor 10.

## Observação

Os exemplos complementam a revisão existente e foram organizados para que o resultado apareça imediatamente após cada exercício.
