# PC-04 — Serviços Gerais / Automação

Branch: `pc-04-servicos`

## Objetivo
Nó econômico e estável para DNS, monitoramento, automações, VPN, Git runners, serviços auxiliares e administração da Base Móvel.

## Configuração recomendada

| Peça | Escolha principal | Qtde | Meta de compra |
|---|---|---:|---:|
| CPU | AMD Ryzen 7 9700X | 1 | R$ 1.900–2.700 |
| Placa-mãe | B650/B850 AM5 com bom I/O | 1 | R$ 900–1.800 |
| RAM | DDR5 32 GB (2x16 GB); 16 GB é entrada | 1 kit | R$ 600–1.200 |
| SSD sistema | NVMe 1 TB | 1 | R$ 400–800 |
| Dados | NVMe/SATA para completar até 8 TB | 1 conjunto | R$ 1.500–4.000 |
| GPU | integrada/entrada | 0–1 | R$ 0–1.500 |
| Rede | 2.5 GbE; 10 GbE se necessário | 1 | R$ 0–1.200 |
| Fonte | 650–750 W de boa qualidade | 1 | R$ 450–900 |
| Cooler | air cooler torre | 1 | R$ 200–500 |
| Gabinete | ATX com bom airflow | 1 | R$ 400–900 |
| Fans | 120/140 mm PWM | 3–5 | R$ 150–500 |

## Referências atuais

- Ryzen 7 9700X: comparador consultado indicou R$ 1.889,99 como menor oferta atual e R$ 1.604,55 como menor valor nos últimos 30 dias. Tratar ofertas de marketplace separadamente das lojas com garantia/faturamento mais previsíveis.
- Alternativa mais forte: Ryzen 9 9900X apareceu por R$ 2.199,99 no PIX na Pichau no anúncio consultado.

## Perfis

### MINIMO
16 GB + 1 TB.

### PADRAO
32 GB + 8 TB.

### EXPANDIDO
64 GB + mais SSD/armazenamento + 10 GbE.

## Organização de armazenamento

```text
NVMe 1 TB -> sistema + Docker
SSD/NVMe 2 TB+ -> serviços e cache
capacidade restante -> dados locais
STORAGE-01 -> backup
```

## Software

Debian 13, Docker, Portainer opcional, DNS interno, monitoramento, Git runner, VPN e serviços de administração.

## Documentação

- `docs/LISTA-DE-COMPRA.md`
- `docs/COMPATIBILIDADE-E-MONTAGEM.md`
- `docs/PERFIS-E-UPGRADES.md`
- `docs/ENERGIA-E-TERMICA.md`
- `docs/MANUTENCAO-E-REPARO.md`
