import java.util.Locale;

public class Main {
    public static void main(String[] args) {
        Locale.setDefault(Locale.forLanguageTag("pt-BR"));

        Professor professor = new Professor(
                "Ana Souza", "PROF-001", 80, 40.00
        );
        Secretario secretario = new Secretario(
                "Carlos Lima", "SEC-001", 2500.00
        );

        System.out.println("=== Professor ===");
        professor.exibirDados();
        System.out.printf("Salário: R$ %.2f%n", professor.calcularSalario());
        professor.registrarPresenca();

        System.out.println();

        System.out.println("=== Secretário ===");
        secretario.exibirDados();
        System.out.printf("Salário: R$ %.2f%n", secretario.calcularSalario());
        secretario.registrarPresenca();
    }
}
