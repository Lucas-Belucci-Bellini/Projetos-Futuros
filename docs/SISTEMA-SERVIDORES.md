# Sistema de Servidores

Cada servidor possui um projeto independente para permitir alteração de peças, orçamento e montagem sem misturar requisitos.

## PC-01 — Gaming
- Ryzen 9
- 64 GB RAM
- 8 TB de armazenamento
- 2x RTX
- Water cooler
- Projeto: branch `pc-01-gaming`

## PC-02 — IA / Hermes
- Plataforma workstation/server AMD
- 128–256 GB RAM ECC
- 4x RTX
- Prioridade para VRAM, PCIe e expansão
- Ambiente isolado para IA
- Projeto: branch `pc-02-ia-hermes`

## PC-03 — Minecraft
- Ryzen 9
- 128 GB RAM
- 32 TB de armazenamento
- Prioridade para CPU, I/O e estabilidade
- Projeto: branch `pc-03-minecraft`

## PC-04 — Serviços
- Ryzen 7/9
- 16 GB RAM
- 8 TB de armazenamento
- DNS, monitoramento, automações, VPN e serviços auxiliares
- Projeto: branch `pc-04-servicos`

## PC-05 — Storage/NAS
O armazenamento principal continua separado da computação, com documentação própria no subsistema `storage`.

## Regra de projeto
Cada PC deve documentar:

1. objetivo e cargas de trabalho;
2. configuração mínima, padrão e máxima;
3. CPU, placa-mãe, RAM, GPU, SSD/HDD, fonte, gabinete e refrigeração;
4. compatibilidade entre peças;
5. sequência de montagem;
6. testes após montagem;
7. sistema operacional e software;
8. orçamento e alternativas;
9. possibilidades de upgrade;
10. limitações conhecidas.

Cada servidor deve poder reiniciar e sofrer manutenção sem derrubar os demais, sempre que possível.
