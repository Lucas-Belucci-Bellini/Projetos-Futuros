# PC-01 — Gaming / Workstation

Branch: `pc-01-gaming`

## Objetivo
Máquina para jogos, game development, Blender/3D, edição e uso geral pesado.

## Configuração de referência

| Componente | Recomendação | Quantidade |
|---|---|---:|
| CPU | AMD Ryzen 9 9950X ou geração equivalente mais recente | 1 |
| Placa-mãe | AM5 de alta qualidade, múltiplos slots PCIe, 10 GbE preferencial | 1 |
| GPU | NVIDIA GeForce RTX 5090 32 GB ou equivalente | 2 |
| RAM | DDR5 64 GB (2x32 GB), expansível | 1 kit |
| SSD sistema | NVMe 2 TB | 1 |
| SSD/Games | NVMe 6 TB | 1 |
| Water cooler | 360 mm ou solução equivalente | 1 |
| Fonte | ATX 3.x de alta potência, dimensionada para as GPUs e CPU | 1 |
| Gabinete | Full tower com espaço real para 2 GPUs grandes | 1 |
| Rede | 10 GbE | 1 |

## Importante
Duas GPUs não significam o dobro de FPS em todos os jogos. Neste PC elas servem também para renderização, IA, criação e cargas que utilizem múltiplas GPUs.

## Como montar
1. Instale CPU, memória e SSD na placa-mãe fora do gabinete.
2. Monte o water cooler conforme o manual da placa e do kit.
3. Instale a placa-mãe e conecte alimentação 24-pin + EPS da CPU.
4. Instale as duas GPUs somente depois de confirmar espaço, espessura, conectores e fluxo de ar.
5. Ligue as GPUs usando a alimentação recomendada pelo fabricante.
6. Instale o SSD adicional e os fans do gabinete.
7. Faça o primeiro boot com uma GPU; depois valide a segunda.
8. Instale Windows 11 Pro, drivers NVIDIA, ferramentas de desenvolvimento e WSL2 Ubuntu.
9. Execute testes de estabilidade antes de considerar o sistema pronto.

## Perfis
- `BASE`: 64 GB + 1 GPU + SSD menor.
- `PADRAO`: 64 GB + 2 GPUs + grande capacidade local.
- `MAX`: 128 GB + 2 GPUs + maior capacidade de SSD.

Veja detalhes em `docs/CONFIGURACAO.md` e `docs/COMPATIBILIDADE-E-MONTAGEM.md`.
