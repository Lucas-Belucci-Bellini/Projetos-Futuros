public class CaixaDeSom implements Conectavel {
    private final String nome;

    public CaixaDeSom(String nome) {
        this.nome = nome;
    }

    @Override
    public void conectar() {
        System.out.println("Caixa de som " + nome + " conectada por Bluetooth.");
    }

    @Override
    public void desconectar() {
        System.out.println("Caixa de som " + nome + " desconectada.");
    }
}
