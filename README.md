# PC-03 — Minecraft / Game Server

Branch: `pc-03-minecraft`

## Objetivo
Hospedar Minecraft, modpacks, mundos persistentes e outros game servers com prioridade para estabilidade, I/O e desempenho por núcleo.

## Configuração recomendada — 128 GB / 32 TB

| Peça | Escolha principal | Qtde | Meta de compra |
|---|---|---:|---:|
| CPU | AMD Ryzen 9 9950X | 1 | R$ 3.200–3.600 |
| Placa-mãe | B650E/X870 de boa qualidade, com 128 GB+ | 1 | R$ 1.500–3.000 |
| RAM | DDR5 128 GB (2x64 GB ou 4x32 GB, conforme QVL) | 1 kit | R$ 2.500–4.500 |
| SSD sistema | NVMe 1–2 TB | 1 | R$ 500–1.200 |
| SSD mundos ativos | NVMe 2–4 TB, alto TBW/endurance | 1 | R$ 1.000–2.500 |
| Dados/arquivo | HDD/SSD para completar até 32 TB | 1 conjunto | R$ 5.000–12.000 |
| GPU | integrada/entrada | 0–1 | R$ 0–2.000 |
| Rede | 10 GbE | 1 | R$ 500–1.500 |
| Fonte | 750–1000 W de boa qualidade | 1 | R$ 600–1.500 |
| Cooler | air cooler premium ou AIO 240/360 | 1 | R$ 300–900 |
| Gabinete | ATX com bom airflow e espaço para armazenamento | 1 | R$ 700–1.500 |

## Referências atuais

- Ryzen 9 9950X: R$ 3.299,99 no PIX na Pichau e R$ 3.399,99 no PIX na KaBuM! nos anúncios consultados.
- O Ryzen 9 9950X tem 16 núcleos/32 threads e suporta até 256 GB de DDR5, segundo a AMD.

## Como atingir 32 TB sem sacrificar o servidor

A melhor estratégia é separar armazenamento de trabalho e arquivo:

```text
NVMe 1–2 TB  -> SO + Docker
NVMe 2–4 TB  -> mundos ativos + cache
HDD/SSD      -> restante da capacidade até 32 TB
STORAGE-01   -> backup externo ao servidor
```

Não usar os 32 TB inteiros em HDD para os mundos ativos quando houver orçamento para NVMe.

## Perfis

### BASICO
64 GB + 4–8 TB.

### PADRAO
128 GB + 16 TB.

### MAX
128 GB + 32 TB + NVMe dedicado para mundos ativos.

## Software

Ubuntu Server 26.04 LTS, Docker ou instalação dedicada, Java conforme cada versão/modpack e backup automático para STORAGE-01.

## Documentação

- `docs/LISTA-DE-COMPRA.md`
- `docs/COMPATIBILIDADE-E-MONTAGEM.md`
- `docs/PERFIS-E-UPGRADES.md`
- `docs/ENERGIA-E-TERMICA.md`
- `docs/MANUTENCAO-E-REPARO.md`
