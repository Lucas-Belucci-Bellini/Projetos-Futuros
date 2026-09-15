# PC-02 — IA / Hermes / GPU Compute

Branch: `pc-02-ia-hermes`

## Objetivo
Servidor dedicado a IA local, inferência, treinamento, processamento de modelos, Hermes e automações pesadas.

## Configuração recomendada — 256 GB

| Peça | Escolha principal | Qtde | Meta de compra |
|---|---|---:|---:|
| CPU | AMD Threadripper PRO 7965WX | 1 | R$ 20.000–24.000 |
| Placa-mãe | ASUS Pro WS WRX90E-SAGE SE | 1 | R$ 10.800–12.000 |
| GPU | NVIDIA RTX 5090 32 GB | 4 | R$ 26.000–32.500 cada |
| RAM | DDR5 ECC RDIMM 256 GB, 8x32 GB ou conjunto equivalente validado na QVL | 1 conjunto | R$ 8.000–15.000 |
| SSD SO | NVMe 2 TB | 1 | R$ 800–1.500 |
| SSD trabalho | NVMe 4–8 TB | 1–2 | R$ 2.000–8.000 por unidade |
| Rede | 25 GbE | 1 | R$ 800–2.000 |
| Fonte | Arquitetura de alimentação profissional, definida pelas GPUs e plataforma | 1 conjunto | R$ 8.000–15.000 |
| Gabinete | Workstation 4U/full tower para 4 GPUs | 1 | R$ 3.000–8.000 |
| Ventilação | Fans de alta pressão/fluxo compatíveis com gabinete | 8–12 | R$ 800–2.000 |
| Refrigeração CPU | Solução certificada para sTR5/Threadripper | 1 | R$ 600–1.500 |

## Referências atuais consultadas

- Threadripper PRO 7965WX: R$ 19.999,99 no PIX na Pichau no anúncio consultado. 
- ASUS Pro WS WRX90E-SAGE SE: R$ 10.799,99 no PIX na Pichau no anúncio consultado; histórico de outra fonte mostrou referências na faixa de R$ 10–12 mil.
- RTX 5090 Gigabyte Gaming OC: R$ 32.299,99 no PIX no anúncio consultado; Gigabyte Windforce OC apareceu por R$ 25.999,99.

## Perfis de compra

### 128 GB / 2 GPUs
Para começar com IA local sem montar a configuração máxima.

- 7965WX
- WRX90
- 128 GB ECC RDIMM
- 2x RTX 5090
- 2 TB + 4 TB NVMe

### PADRÃO — 256 GB / 4 GPUs
A configuração de referência da Base Móvel.

- 7965WX
- WRX90
- 256 GB ECC RDIMM
- 4x RTX 5090
- 2–8 TB NVMe de trabalho
- rede 25 GbE

### MAX
Somente quando a carga realmente exigir mais CPU/RAM/GPU. Avaliar Threadripper PRO superior, mais RAM e GPUs profissionais.

## Regra de compatibilidade

Não comprar as quatro GPUs antes de validar no modelo exato: dimensões, espessura, slots, largura de banda PCIe, alimentação, gabinete, refrigeração e limites térmicos.

## Software

Ubuntu 26.04 LTS, NVIDIA Driver, CUDA, Docker, Python e PyTorch. Hermes fica como workload da camada de aplicações.

## Documentação

- `docs/LISTA-DE-COMPRA.md`
- `docs/COMPATIBILIDADE-E-MONTAGEM.md`
- `docs/PERFIS-E-UPGRADES.md`
- `docs/ENERGIA-E-TERMICA.md`
- `docs/MANUTENCAO-E-REPARO.md`
