# PC-03 — Minecraft / Game Server

Branch: `pc-03-minecraft`

## Objetivo
Hospedar Minecraft, modpacks, mundos persistentes e outros game servers com prioridade para estabilidade, I/O e desempenho por núcleo.

## Configuração de referência

| Componente | Recomendação | Quantidade |
|---|---|---:|
| CPU | AMD Ryzen 9 de alto desempenho por núcleo | 1 |
| Placa-mãe | AM5 estável, suporte a 128 GB+ DDR5 | 1 |
| RAM | DDR5 128 GB (2x64 GB ou 4x32 GB conforme QVL) | 1 conjunto |
| Sistema | NVMe 1–2 TB | 1 |
| Dados ativos | NVMe de alta resistência, capacidade conforme mundos | 1+ |
| Armazenamento total local | até 32 TB, com desenho conforme orçamento | 1 conjunto |
| GPU | integrada/entrada; GPU forte não é prioridade para servidor | 0–1 |
| Rede | 10 GbE | 1 |
| Fonte | qualidade e margem adequada | 1 |
| Refrigeração | tower cooler ou water cooler conforme CPU/carga | 1 |

## Princípio
Minecraft Java costuma valorizar muito desempenho por núcleo, baixa latência e armazenamento responsivo. Mais GPUs não devem ser o foco deste servidor.

## Software recomendado
- Ubuntu Server 26.04 LTS
- Docker ou instalação dedicada
- Java na versão exigida pelo servidor/modpack
- painel de gerenciamento opcional
- backup automático para STORAGE-01

## Organização
```text
SERVER-03
├── Minecraft
│   ├── Survival
│   ├── Modpacks
│   ├── ATM10
│   ├── NEXORA-test
│   └── Worlds
├── Game Servers
└── Backup local
```

## Perfis
- `BASICO`: 64 GB + 4–8 TB.
- `PADRAO`: 128 GB + 16 TB.
- `MAX`: 128 GB + até 32 TB + NVMe dedicado para mundos ativos.

Veja `docs/CONFIGURACAO.md` e `docs/COMPATIBILIDADE-E-MONTAGEM.md`.
