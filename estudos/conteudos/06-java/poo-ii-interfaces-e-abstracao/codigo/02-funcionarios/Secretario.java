public class Secretario extends Funcionario implements Registravel {
    private final double salarioMensal;

    public Secretario(String nome, String matricula, double salarioMensal) {
        super(nome, matricula);
        this.salarioMensal = salarioMensal;
    }

    @Override
    public double calcularSalario() {
        return salarioMensal;
    }

    @Override
    public void registrarPresenca() {
        System.out.println("Presença do secretário registrada.");
    }
}
