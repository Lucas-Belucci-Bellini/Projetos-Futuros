# N01 — Razer Blade 14 / Ryzen 9 8945HS + RTX 4070

## Papel na Base
Notebook premium compacto para programação, game dev, 3D e IA local leve/média. É o perfil mais móvel do catálogo.

## Configuração de referência

| Item | Referência |
|---|---|
| CPU | AMD Ryzen 9 8945HS, 8c/16t, até 5,2 GHz |
| GPU | NVIDIA GeForce RTX 4070 Laptop |
| VRAM | 8 GB GDDR6 |
| RAM | 32 GB DDR5/LPDDR5X conforme configuração; confirmar part number |
| SSD | 1 TB NVMe Gen4 |
| Tela | 14", QHD, 240 Hz na configuração consultada |
| Bateria | 68 Wh na configuração consultada |
| Peso | ~1,78 kg |
| Hermes | SIM, como candidato de IA local |

## O que aguenta

- IDEs pesadas, Docker, WSL e máquinas virtuais moderadas.
- Desenvolvimento Unreal/Unity e Blender em escala profissional móvel.
- Minecraft muito modificado, desde que o perfil de renderização não estoure os 8 GB de VRAM.
- Modelos de IA locais pequenos/médios e quantizados. 8 GB de VRAM é o principal limite.

## O que não deve ser considerado

Não tratar como substituto do servidor de IA da Base. Treino/inferência de modelos muito grandes ou múltiplos processos de IA concorrentes devem ir para o servidor HERMES-01.

## Autonomia

Fabricante/varejo consultado informa 68 Wh e até ~10 h em uso leve/médio. Para planejamento da Base, usar uma faixa conservadora de **4–8 h em uso leve** e **~1–3 h em carga pesada**, porque jogos/IA usam muito mais energia. Não tratar número publicitário como garantia.

## Durabilidade — 4/5

Pontos positivos: formato premium e compacto. Pontos de atenção: alta densidade térmica, tela delicada e custo de placa-mãe elevado. Manter entradas/saídas de ar limpas e evitar operação contínua sobre superfícies macias.

## Peças e orçamento de reparo

Valores abaixo são **estimativas de planejamento**, não cotação OEM do part number exato.

| Peça | Faixa estimada (R$) | Status | Observação |
|---|---:|---|---|
| Bateria 68 Wh | 600–1.300 | ESTIMATIVA | confirmar código |
| Tela QHD 240 Hz | 1.500–3.000 | ESTIMATIVA | painel específico |
| Carregador original | 500–1.000 | ESTIMATIVA | verificar potência/conector |
| Teclado/top case | 700–1.800 | ESTIMATIVA | pode vir como conjunto |
| Ventoinha/conjunto térmico | 300–800 | ESTIMATIVA | depende do módulo |
| SSD 1 TB NVMe | 350–800 | ESTIMATIVA | peça de mercado |
| RAM | 300–1.000 | A CONFIRMAR | depende de ser soldada ou SODIMM |
| Wi-Fi | 120–350 | ESTIMATIVA | part number |
| Tampa inferior | 400–900 | ESTIMATIVA | acabamento premium |
| Placa-mãe | 4.000–8.000+ | A CONFIRMAR | CPU/GPU normalmente integradas/soldadas |

### Mão de obra

- Limpeza + inspeção: R$ 150–300.
- Troca de bateria/SSD/RAM quando acessível: R$ 100–250.
- Tela/teclado: R$ 200–500.
- Diagnóstico/reparo de placa: R$ 250–700+, fora componentes.

## TCO de planejamento — 5 anos

Reserva inicial recomendada: **R$ 1.500–3.000** para manutenção, bateria e pequenos reparos, além de SSD/RAM planejados. Falha de placa-mãe pode tornar economicamente melhor trocar o notebook.

## Fontes

- KaBuM — configuração consultada: https://www.kabum.com.br/produto/571236/
- AMD — Ryzen 9 8945HS: https://www.amd.com/en/products/processors/laptop/ryzen/8000-series/amd-ryzen-9-8945hs.html
- NVIDIA — GPUs RTX para notebooks: https://www.nvidia.com/pt-br/geforce/laptops/compare/

Atualizado em 14/09/2026.
