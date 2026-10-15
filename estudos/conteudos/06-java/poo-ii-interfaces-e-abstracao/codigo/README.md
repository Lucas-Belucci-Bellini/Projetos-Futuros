# Código dos exercícios de LPOO II

Os dois exercícios estão em diretórios separados. Cada um tem sua própria classe Main.

## Pré-requisitos

- JDK instalado.
- O comando java -version deve funcionar.
- O comando javac -version deve funcionar.

## Exercício 1 — Conexão de dispositivos

Abra o terminal no diretório 01-conexao e execute:

~~~~bash
javac *.java
java Main
~~~~

Saída esperada:

~~~~text
Impressora HP LaserJet conectada por USB.
Impressora HP LaserJet desconectada.

Caixa de som JBL Go conectada por Bluetooth.
Caixa de som JBL Go desconectada.
~~~~

## Exercício 2 — Funcionários de uma escola

Abra o terminal no diretório 02-funcionarios e execute:

~~~~bash
javac *.java
java Main
~~~~

Saída esperada:

~~~~text
=== Professor ===
Nome: Ana Souza
Matrícula: PROF-001
Salário: R$ 3200,00
Presença do professor registrada.

=== Secretário ===
Nome: Carlos Lima
Matrícula: SEC-001
Salário: R$ 2500,00
Presença do secretário registrada.
~~~~

O programa configura a localidade pt-BR para exibir decimais com vírgula. Símbolos de moeda e separadores podem depender do ambiente.

**Importante:** execute cada pasta separadamente. Ambas têm uma classe Main; não compile os dois diretórios juntos no mesmo destino.
