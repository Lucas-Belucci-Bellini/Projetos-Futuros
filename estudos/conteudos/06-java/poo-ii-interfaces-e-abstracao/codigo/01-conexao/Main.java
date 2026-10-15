public class Main {
    public static void main(String[] args) {
        // O tipo da referência é a interface, não a classe concreta.
        Conectavel impressora = new Impressora("HP LaserJet");
        Conectavel caixaDeSom = new CaixaDeSom("JBL Go");

        impressora.conectar();
        impressora.desconectar();

        System.out.println();

        caixaDeSom.conectar();
        caixaDeSom.desconectar();
    }
}
