# PC-03 — Lista de Compra

Data de referência: setembro de 2026. Valores são metas/faixas de orçamento; confirmar oferta, frete, garantia e part number antes da compra.

## BOM recomendado — 128 GB / 32 TB

| Item | Especificação | Qtde | Meta de preço |
|---|---|---:|---:|
| CPU | AMD Ryzen 9 9950X | 1 | R$ 3.200–3.600 |
| Placa-mãe | B650E/X870 AM5 com suporte a 128 GB+ | 1 | R$ 1.500–3.000 |
| RAM | DDR5 128 GB, 2x64 GB ou 4x32 GB conforme QVL | 1 | R$ 2.500–4.500 |
| NVMe sistema | 1–2 TB | 1 | R$ 500–1.200 |
| NVMe mundos | 2–4 TB, alta resistência | 1 | R$ 1.000–2.500 |
| HDD/SSD dados | conjunto para completar aproximadamente 32 TB | 1 conjunto | R$ 5.000–12.000 |
| GPU | integrada/entrada | 0–1 | R$ 0–2.000 |
| NIC | 10 GbE | 1 | R$ 500–1.500 |
| Fonte | 750–1000 W | 1 | R$ 600–1.500 |
| Cooler | torre premium | 1 | R$ 300–700 |
| Gabinete | ATX com airflow | 1 | R$ 700–1.500 |
| Fans | PWM | 3–6 | R$ 200–600 |

## Custo estimado

- Configuração econômica: aproximadamente R$ 12–18 mil.
- Configuração padrão: aproximadamente R$ 17–26 mil.
- Configuração máxima com 32 TB local bem distribuídos: aproximadamente R$ 24–32 mil.

## Estratégia de armazenamento

```text
NVMe 1–2 TB -> sistema, Docker e ferramentas
NVMe 2–4 TB -> mundos ativos, bancos e cache
HDD/SSD     -> arquivo e conteúdo frio
STORAGE-01  -> backup e cópia independente
```

Os mundos que recebem escrita intensa devem ficar no armazenamento responsivo; os 32 TB totais não precisam ser todos NVMe.

## Referências

O Ryzen 9 9950X apareceu por R$ 3.299,99 no PIX na Pichau e R$ 3.399,99 no PIX na KaBuM! nos anúncios consultados. A AMD informa 16 núcleos/32 threads e suporte a até 256 GB de DDR5.

## Software

Ubuntu Server 26.04 LTS + Docker/instalação dedicada + Java conforme o servidor/modpack + backups automáticos.
