# Projetos-Futuros

Central de planejamento para projetos futuros.

## Estrutura de branches

- `main` — base estável / índice geral.
- `projeto/base-movel` — arquitetura geral da Base Móvel.
- `base-movel/servidores` — os 4 servidores e virtualização/orquestração.
- `base-movel/casa` — área habitacional, móveis e organização interna.
- `base-movel/energia` — UPS, baterias, inversor, gerador e distribuição elétrica.
- `base-movel/rede` — switches, firewall, VLANs, Wi-Fi, VPN e internet satelital.
- `base-movel/storage` — NAS, nuvem privada, backups e expansão de armazenamento.
- `base-movel/notebooks` — notebooks de trabalho, desenvolvimento, jogos e IA.
- `base-movel/veiculo` — caminhão/baú, estrutura, peso, isolamento e integração.
- `base-movel/climatizacao` — refrigeração, fluxo de ar e sala técnica.
- `base-movel/documentacao` — requisitos, ADRs, diagramas, inventário e decisões.

## Regra

Cada branch deve conter somente o planejamento e os arquivos diretamente relacionados ao seu subsistema. Integrações que afetem mais de uma área devem ser documentadas em `projeto/base-movel` antes de serem incorporadas.
