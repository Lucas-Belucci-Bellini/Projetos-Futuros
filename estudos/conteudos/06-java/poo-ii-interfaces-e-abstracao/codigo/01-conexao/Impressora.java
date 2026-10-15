public class Impressora implements Conectavel {
    private final String modelo;

    public Impressora(String modelo) {
        this.modelo = modelo;
    }

    @Override
    public void conectar() {
        System.out.println("Impressora " + modelo + " conectada por USB.");
    }

    @Override
    public void desconectar() {
        System.out.println("Impressora " + modelo + " desconectada.");
    }
}
