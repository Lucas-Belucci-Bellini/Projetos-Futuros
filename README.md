# PC-04 — Serviços Gerais / Automação

Branch: `pc-04-servicos`

## Objetivo
Nó econômico e estável para DNS, monitoramento, automações, VPN, Git runners, serviços auxiliares e administração da Base Móvel.

## Configuração de referência

| Componente | Recomendação | Quantidade |
|---|---|---:|
| CPU | AMD Ryzen 7 ou Ryzen 9 eficiente | 1 |
| Placa-mãe | AM5 com boa conectividade | 1 |
| RAM | DDR5 16 GB inicialmente | 1 kit |
| SSD sistema | NVMe 1 TB | 1 |
| Armazenamento | 8 TB total, preferencialmente dividido entre sistema e dados | 1 conjunto |
| GPU | integrada ou placa de entrada | 0–1 |
| Rede | 2.5/10 GbE | 1 |
| Fonte | 80 Plus de boa qualidade | 1 |
| Refrigeração | air cooler eficiente | 1 |

## Software recomendado
- Debian 13
- Docker
- Portainer opcional
- DNS interno
- monitoramento
- Git runner
- serviços de administração
- VM ou container para Home Assistant, quando adequado

## Perfis
- `MINIMO`: 16 GB + 1 TB.
- `PADRAO`: 16–32 GB + 8 TB.
- `EXPANDIDO`: 32–64 GB + mais SSD/armazenamento.

## Montagem
1. Instalar CPU, RAM e NVMe.
2. Montar o cooler e a placa-mãe.
3. Instalar o armazenamento secundário.
4. Configurar rede e IP de gerenciamento.
5. Instalar Debian 13.
6. Instalar Docker e serviços.
7. Integrar monitoramento ao restante da Base.

Veja `docs/COMPATIBILIDADE-E-MONTAGEM.md`.
