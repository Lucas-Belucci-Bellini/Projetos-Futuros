public abstract class Funcionario {
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
