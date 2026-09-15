# PC-02 — IA / Hermes / GPU Compute

Branch: `pc-02-ia-hermes`

## Objetivo
Servidor dedicado a IA local, inferência, treinamento, processamento de modelos, Hermes e automações pesadas.

## Configuração de referência

| Componente | Recomendação | Quantidade |
|---|---|---:|
| CPU | AMD Threadripper PRO, preferencialmente classe 7995WX ou equivalente atual | 1 |
| Placa-mãe | WRX90 profissional, múltiplos slots PCIe e suporte a ECC | 1 |
| GPU | NVIDIA RTX 5090 32 GB ou GPU profissional equivalente | 4 |
| RAM | 256 GB DDR5 ECC como alvo; 128 GB como inicial | 1 kit/conjunto |
| Sistema | NVMe empresarial 2 TB | 1 |
| Cache/projetos | NVMe 4–8 TB | 1+ |
| Rede | 25 GbE | 1 |
| Fonte | Plataforma de alta potência, dimensionada pelas GPUs exatas | 1+ |
| Refrigeração | alta capacidade, com fluxo de ar de workstation | 1 |
| Gabinete | workstation full tower para 4 GPUs | 1 |

## Por que workstation
Quatro GPUs e 128–256 GB de RAM exigem muito mais largura de banda, pistas PCIe, espaço físico, energia e capacidade térmica do que uma máquina Ryzen 9 comum.

## Software recomendado
- Ubuntu 26.04 LTS
- NVIDIA Driver
- CUDA
- Docker
- Python
- PyTorch
- ambientes isolados para Hermes e outras cargas

## Perfis
- `128GB`: entrada para IA local.
- `256GB`: configuração recomendada.
- `MAX`: mais RAM, armazenamento NVMe e GPUs profissionais, somente quando a carga justificar.

## Limite importante
Quatro GPUs só devem ser compradas depois de validar placa-mãe, espaçamento dos slots, conectores, fonte, gabinete e refrigeração do modelo exato.

Veja `docs/CONFIGURACAO.md` e `docs/COMPATIBILIDADE-E-MONTAGEM.md`.
