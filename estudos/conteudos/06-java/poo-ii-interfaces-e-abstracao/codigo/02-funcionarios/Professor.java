public class Professor extends Funcionario implements Registravel {
    private final int quantidadeHoras;
    private final double valorHora;

    public Professor(
            String nome,
            String matricula,
            int quantidadeHoras,
            double valorHora
    ) {
        super(nome, matricula);
        this.quantidadeHoras = quantidadeHoras;
        this.valorHora = valorHora;
    }

    @Override
    public double calcularSalario() {
        return quantidadeHoras * valorHora;
    }

    @Override
    public void registrarPresenca() {
        System.out.println("Presença do professor registrada.");
    }
}
