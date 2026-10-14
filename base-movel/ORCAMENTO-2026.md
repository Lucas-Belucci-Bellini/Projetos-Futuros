# Orçamento Mestre — Base Móvel de Computação

> **Data da cotação: 15 de setembro de 2026.** Todos os preços em USD, mercado
> dos EUA. Preço de hardware em 2026 muda de semana para semana — confira o link
> da fonte antes de comprar. As fontes estão em
> [`docs/FONTES-DE-PRECO-2026.md`](../docs/FONTES-DE-PRECO-2026.md).

Este documento põe números no que
[`PLANEJAMENTO-BASE-MOVEL.md`](PLANEJAMENTO-BASE-MOVEL.md) descreve em
conceito. A arquitetura ali continua valendo; aqui está o que ela custa.

---

## ⚠️ Leia isto antes de qualquer coisa: 2026 é o pior ano da década para comprar

Não é opinião, é o que as cotações mostram. Os dois componentes que mais pesam
neste projeto — GPU e memória — estão em crise de preço simultânea, causada
pela demanda de datacenter de IA:

| Componente | Preço normal / MSRP | Preço em set/2026 | Variação |
|---|---|---|---|
| RTX 5090 (32 GB) | $1.999 (MSRP de lançamento) | **~$4.525** (média) / $5.199 (melhor preço real) | **+126%** |
| Kit DDR5 64 GB | ~$200 (out/2025) | **~$849** | **+325%** |
| Kit DDR5 32 GB | ~$85 (histórico) | **~$392** | **+360%** |
| RTX PRO 6000 96 GB | $8.565 (lançamento) | **$16.000 – $19.999** | **+87% a +133%** |

O que isso significa em dinheiro, **neste projeto específico**:

| Item | Quantidade no projeto | Custo hoje | Custo a preço normal | Sobrepreço |
|---|---|---|---|---|
| RTX 5090 | 6 unidades | $27.150 | $11.994 | **+$15.156** |
| Memória DDR5/ECC | 464 GB no total | ~$7.400 | ~$1.900 | **+$5.500** |
| | | | **Imposto de 2026** | **≈ $20.700** |

**Vinte mil dólares** é o pedágio de comprar tudo agora. E a análise de mercado
aponta que o alívio não chega antes de **meados de 2027** — a Samsung e a SK
Hynix migraram linhas de produção para memória de IA, e a IDC projeta
crescimento de oferta de DRAM de apenas 16% em 2026.

**Recomendação:** não é "desista do projeto". É **fasear a compra**. O veículo,
a estrutura, a energia, a climatização e a armaria **não estão em crise de
preço** — é exatamente isso que dá para construir em 2026 enquanto GPU e RAM
esfriam. Veja o [Cenário C](#cenário-c--faseado-recomendado) e o
[cronograma](#8-cronograma-sugerido).

---

## 1. Os três cenários

| | **A — Completo** | **B — Consolidado** | **C — Faseado** |
|---|---|---|---|
| Nós físicos | 4 PCs + NAS | 2 PCs (virtualização) | 2 PCs, GPUs depois |
| GPUs | 6× RTX 5090 | 2× RTX 5090 (+2 depois) | 1× RTX 5090 |
| RAM total | 464 GB | 256 GB | 192 GB |
| Armazenamento | 360 TB brutos | 360 TB brutos | 180 TB brutos |
| Veículo | Peterbilt 379 + trailer 40 ft | Box truck 24 ft | Box truck 24 ft |
| Consumo de pico | ~6,8 kW | ~3,9 kW | ~2,4 kW |
| **Custo TI** | **$63.747** | **$34.509** | **$21.800** |
| **Custo infraestrutura** | **$58.157** | **$43.317** | **$38.000** |
| **Custo veículo** | **$185k – $490k** | **$95k – $180k** | **$95k – $130k** |
| **TOTAL** | **$307k – $612k** | **$173k – $258k** | **$155k – $190k** |

### Cenário A — Completo
Exatamente o que foi pedido: 4 máquinas independentes, 6 GPUs, cavalo Peterbilt
379 com trailer. É o mais fiel ao pedido e o mais caro. Detalhado abaixo.

### Cenário B — Consolidado
Junta os 4 nós em 2 máquinas físicas com **Proxmox + passthrough de GPU**:
uma workstation (gamer + IA) e um servidor (NAS + Minecraft + serviços em VMs).
Mesma capacidade funcional, **–43% no consumo**, –35% no peso, –46% no custo de
TI. A perda real: menos isolamento entre funções, e o gamer divide a máquina
com a IA.

### Cenário C — Faseado (recomendado)
Constrói o **veículo, a energia, a climatização e a armaria em 2026** — que não
estão inflacionados — e compra GPU/RAM em 2027, quando o mercado normalizar.
Você fica com a base móvel funcional o ano inteiro e economiza ~$20 mil.

---

## 2. Nó 1 — SERVER-01 · Gaming / Workstation

**Alvo:** Ryzen 9, 64 GB, 8 TB, 2× RTX, water cooler.

| Item | Modelo | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| CPU | AMD Ryzen 9 9950X3D (16C/32T) | 1 | $569 | $569 |
| Placa-mãe | X870E ATX, 2× PCIe 5.0 x16, 10 GbE | 1 | $450 | $450 |
| RAM | 64 GB DDR5-6000 (2×32 GB) | 1 | $849 | $849 |
| GPU | RTX 5090 32 GB | 2 | $4.525 | $9.050 |
| SSD sistema | 2 TB NVMe Gen5 | 1 | $280 | $280 |
| SSD jogos | 4 TB NVMe Gen4 | 1 | $420 | $420 |
| SSD projetos | 2 TB NVMe Gen4 | 1 | $280 | $280 |
| Fonte | 1600 W Platinum ATX 3.1 | 1 | $480 | $480 |
| Water cooler | AIO 360 mm (anti-vibração) | 1 | $230 | $230 |
| Gabinete | Rack 4U com trava de GPU | 1 | $320 | $320 |
| | | | **Subtotal** | **$12.648** |

**Notas técnicas que mudam a decisão:**

- **8 TB entregues como 3 NVMe, não HDD.** Biblioteca de jogos em disco
  mecânico dentro de veículo em movimento é pedir para perder o disco. O AM5
  tem 4 slots M.2 na maioria das X870E — sobra um para expansão.
- **O 9950X3D tem 24 linhas PCIe.** Com 2 GPUs, elas rodam em **x8/x8**. Para
  jogo é irrelevante (perda <2%); para IA multi-GPU, importa — e é por isso que
  o nó de IA usa Threadripper.
- **2× RTX 5090 = 1.150 W só de GPU.** Some CPU e periféricos: ~1.470 W de pico
  nesta máquina sozinha. Isso é um forno de 1,5 kW dentro de um baú. Ver
  [climatização](climatizacao/ORCAMENTO-CLIMATIZACAO.md).
- **SLI/NVLink não existe mais.** Duas 5090 **não somam** para jogar — a segunda
  só serve para renderização, IA, ou uma segunda carga simultânea. Se a intenção
  era "jogo mais rápido", **uma 5090 e $4.525 no bolso** é a decisão correta.

→ Detalhamento: [`servidores/ORCAMENTO-SERVIDORES.md`](servidores/ORCAMENTO-SERVIDORES.md)

---

## 3. Nó 2 — SERVER-02 · IA / Hermes

**Alvo:** 256 GB RAM, 4× RTX, plataforma AMD.

| Item | Modelo | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| CPU | Threadripper PRO 9975WX (32C/64T) | 1 | $4.099 | $4.099 |
| Placa-mãe | WRX90E-SAGE (7× PCIe 5.0 x16) | 1 | $1.250 | $1.250 |
| RAM | 256 GB DDR5 RDIMM ECC (8×32 GB) | 1 | $4.800 | $4.800 |
| GPU | RTX 5090 32 GB | 4 | $4.525 | $18.100 |
| SSD | 4 TB NVMe Gen5 (sistema + datasets) | 2 | $520 | $1.040 |
| Fonte | 2000 W Titanium | 2 | $600 | $1.200 |
| Refrigeração | AIO sTR5 + ventilação dirigida | 1 | $450 | $450 |
| Chassi | 4U GPU server, 4 slots dual-width | 1 | $1.300 | $1.300 |
| | | | **Subtotal** | **$32.239** |

**Por que Threadripper PRO e não Ryzen 9:** 4 GPUs exigem 64 linhas PCIe só de
GPU. O Ryzen 9 tem 24 no total. O 9975WX entrega **128 linhas PCIe 5.0** e
suporta RDIMM ECC de 8 canais — que é o que permite os 256 GB. Não há atalho
aqui; é plataforma diferente, não CPU melhor.

**A conta de energia deste nó é o problema central do projeto:**
4× RTX 5090 × 575 W = **2.300 W só de GPU**. Com CPU (350 W) e o resto,
**~2.900 W sustentados** sob carga de inferência. Isso é mais que um chuveiro
elétrico, rodando por horas, dentro de um baú, no Texas.

**Alternativa que vale considerar seriamente:**

| Opção | VRAM total | Custo | Consumo | Roda Hermes 4 70B? |
|---|---|---|---|---|
| 4× RTX 5090 | 128 GB (fragmentada) | $18.100 | 2.300 W | Sim, com tensor-parallel |
| 1× RTX PRO 6000 96 GB | 96 GB (unificada) | $16.000–19.999 | **600 W** | Sim, nativo |

A RTX PRO 6000 custa quase o mesmo, consome **um quarto** da energia e dá VRAM
unificada — sem a complexidade de particionar modelo entre 4 placas sem NVLink.
Num veículo, onde cada watt vira calor que você precisa bombear para fora,
**esta é provavelmente a escolha certa** apesar de contrariar o pedido de
"4 placas". Registro a recomendação; a decisão é sua.

---

## 4. Nó 3 — SERVER-03 · Minecraft + Nuvem 256 TB

**Alvo:** 128 GB RAM, 32 TB, servidor de Minecraft + nuvem privada.

| Item | Modelo | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| CPU | Ryzen 9 9950X (alto clock por núcleo) | 1 | $470 | $470 |
| Placa-mãe | ASRock Rack X870 (IPMI, 2× 10 GbE) | 1 | $600 | $600 |
| RAM | 128 GB DDR5 ECC UDIMM (4×32 GB) | 1 | $1.750 | $1.750 |
| NVMe mundos | 2 TB Gen5 (mundos ativos, espelhado) | 2 | $280 | $560 |
| HDD nuvem | Seagate Exos M 30 TB | 12 | $570 | $6.840 |
| HBA | Broadcom 9500-16i | 1 | $450 | $450 |
| Chassi | 4U 16 baias, backplane SAS3 | 1 | $850 | $850 |
| Fonte | 1000 W Gold | 1 | $200 | $200 |
| | | | **Subtotal** | **$11.720** |

**Decisão de projeto: o mundo do Minecraft não fica em HDD.**
Um servidor de ATM10 com 200 mods extras faz I/O aleatório pesado de chunk.
Em HDD, o servidor engasga. Os **32 TB pedidos para o nó Minecraft saem como
dataset do pool ZFS**, e os **mundos ativos vivem em NVMe espelhado**. Você fica
com o mesmo número, e com um servidor que não trava.

**Como os 256 TB se fecham:**

| | |
|---|---|
| Discos | 12 × 30 TB = **360 TB brutos** |
| Layout | ZFS RAIDZ2 (10 dados + 2 paridade) |
| Tolerância | **2 discos podem morrer** sem perda |
| Capacidade útil | ~300 TB = **~273 TiB** |
| Meta pedida | 256 TB ✅ com folga |

Custo por TB útil: **$22,80/TB**. Usando Exos recondicionados de 28 TB
(~$12,50/TB) o mesmo pool sai por ~$4.200 — economia de $2.600, com garantia
menor. Para dado que já tem paridade e backup, é um risco razoável.

→ Detalhamento: [`storage/ORCAMENTO-STORAGE-NUVEM.md`](storage/ORCAMENTO-STORAGE-NUVEM.md)

---

## 5. Nó 4 — SERVER-04 · Serviços

**Alvo:** 16 GB RAM, 8 TB.

| Item | Modelo | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| CPU | Ryzen 7 9700X (65 W TDP) | 1 | $310 | $310 |
| Placa-mãe | B850 mATX, 2.5 GbE | 1 | $180 | $180 |
| RAM | 16 GB DDR5-5600 (2×8 GB) | 1 | $210 | $210 |
| SSD sistema | 2 TB NVMe Gen4 | 1 | $280 | $280 |
| Dados | 8 TB SATA (Exos 8 TB) | 1 | $190 | $190 |
| Fonte | 550 W Gold | 1 | $110 | $110 |
| Chassi | 2U rack curto | 1 | $220 | $220 |
| | | | **Subtotal** | **$1.500** |

Nó de DNS interno, Home Assistant, runners de Git, monitoramento (Prometheus +
Grafana), Uptime Kuma e automações. 16 GB é pouco para tudo isso com folga —
mas o slot livre permite ir a 64 GB quando a RAM baratear.

---

## 6. Infraestrutura

| Bloco | Detalhe | Total |
|---|---|---:|
| **Rack e rede** | Rack 24U com amortecedores, switch 10/25 GbE 24p, firewall pfSense, KVM-over-IP, 2 PDUs gerenciáveis, cabeamento | **$5.650** |
| **Energia** | 3× UPS SRT 3000VA, banco LiFePO4 40 kWh, inversor 12 kW, gerador diesel 12 kW, solar 3 kWp, quadro/DPS/aterramento | **$40.540** |
| **Climatização** | 2× mini-split 12k BTU, dutos corredor quente/frio, isolamento, filtragem, sensores | **$8.800** |
| **Conectividade** | Starlink Flat High Performance + Standard reserva, 5G failover, antenas | **$2.298** |
| **Armaria** | Cofre 30 longas, cofre 15 curtas, racks modulares, reforço estrutural, munição | **$6.519** |
| | **Subtotal infraestrutura** | **$63.807** |

→ [`energia/`](energia/ORCAMENTO-ENERGIA.md) · [`climatizacao/`](climatizacao/ORCAMENTO-CLIMATIZACAO.md) · [`rede/`](rede/ORCAMENTO-REDE-SATELITE.md) · [`armaria/`](armaria/ORCAMENTO-ARMARIA.md)

---

## 7. Veículo

| Opção | Componentes | Faixa |
|---|---|---|
| **A — Peterbilt 379 + trailer** | Cavalo 379 usado ($45k–90k) + trailer expedição 40 ft ($60k–150k) + conversão ($80k–250k) | **$185k – $490k** |
| **B — Box truck 24 ft** | Chassi F-550/Ram 5500 usado ($35k–60k) + conversão ($60k–120k) | **$95k – $180k** |
| **C — Só o trailer** | Trailer 40 ft convertido, rebocado sob demanda | **$60k – $150k** |

Referência de mercado para builds profissionais: a
[Field Van FAV](https://www.sportsmobileforum.com/threads/field-van-fav-box-truck-conversion-on-e350-chassis.2199009/)
(box truck sobre E350) sai ~$300.000; a
[Wanderbox Outpost](https://wanderbox.com/) parte de $395.000 e a Fortress
chega perto de $1M. Conversão DIY de box truck fica entre $10k e $50k — mas
**nenhuma dessas referências carrega 4 kW de eletrônica**, que é o que encarece
a sua.

→ Planta baixa, distribuição de peso e memorial:
[`veiculo/ORCAMENTO-VEICULO-E-PLANTA.md`](veiculo/ORCAMENTO-VEICULO-E-PLANTA.md)

---

## 8. Cronograma sugerido

| Fase | Quando | O que | Custo |
|---|---|---|---:|
| **0** | Agora | Notebook (ferramenta de trabalho imediata) | $2.400 – $5.500 |
| **1** | Q4 2026 | Veículo + estrutura + isolamento | $95k – $180k |
| **2** | Q1 2027 | Energia + climatização + rede + armaria | $58k |
| **3** | Q2 2027 | Nó 3 (NAS/nuvem) e nó 4 — **não dependem de GPU** | $13k |
| **4** | Q3 2027 | GPU e RAM, **após o alívio de mercado** | $30k – $46k |
| **5** | Q4 2027 | Integração, testes de campo, shakedown | $5k |

A fase 4 é a única que espera. Todo o resto pode andar em 2026 — e é
exatamente por isso que fasear economiza ~$20 mil sem atrasar o projeto.

---

## 9. Consolidado

| Bloco | Cenário A | Cenário B | Cenário C |
|---|---:|---:|---:|
| SERVER-01 Gaming | $12.648 | — | — |
| SERVER-02 IA | $32.239 | — | — |
| SERVER-03 Minecraft/NAS | $11.720 | $11.160 | $8.900 |
| SERVER-04 Serviços | $1.500 | — | — |
| Workstation consolidada | — | $17.699 | $7.250 |
| Rack e rede | $5.650 | $5.650 | $5.650 |
| **TI — subtotal** | **$63.757** | **$34.509** | **$21.800** |
| Energia | $40.540 | $28.000 | $24.000 |
| Climatização | $8.800 | $6.500 | $6.500 |
| Conectividade | $2.298 | $2.298 | $2.298 |
| Armaria | $6.519 | $6.519 | $5.200 |
| **Infra — subtotal** | **$58.157** | **$43.317** | **$37.998** |
| **Equipamento total** | **$121.914** | **$77.826** | **$59.798** |
| Veículo | $185k – $490k | $95k – $180k | $95k – $130k |
| **TOTAL** | **$307k – $612k** | **$173k – $258k** | **$155k – $190k** |

**Custo recorrente mensal** (qualquer cenário):

| Item | Mensal |
|---|---:|
| Starlink Roam Ilimitado | $175 |
| 5G failover | $60 |
| Diesel (gerador, ~6 L/dia em uso pesado) | ~$130 |
| Backup off-site (ver storage) | $80 – $1.536 |
| Manutenção/reserva (2% a.a. do equipamento) | ~$200 |
| **Total** | **$645 – $2.100** |

---

## 10. Referências visuais

- [Peterbilt 379 "Optimus Prime" — ficha do veículo do filme de 2007 (IMCDb)](https://www.imcdb.org/v110958.html)
- [Peterbilt 379 stunt truck original, leiloado na Barrett-Jackson](https://www.barrett-jackson.com/scottsdale-2016/docket/vehicle/1992-peterbilt-379-transformers-optimus-prime-stunt-truck-190024)
- [Wanderbox — campers de expedição (referência de acabamento)](https://wanderbox.com/)
- [Field Van FAV — conversão de box truck](https://www.sportsmobileforum.com/threads/field-van-fav-box-truck-conversion-on-e350-chassis.2199009/)
- [Seagate Exos M 30 TB — página do produto (B&H)](https://www.bhphotovideo.com/c/product/1905376-REG/seagate_st30000nm004k_exos_m_internal_hard.html)
- [Liberty USA 30 — cofre de 30 armas longas](https://libertysafe.com/products/usa)
- [APC Smart-UPS SRT 3000VA rack 2U](https://www.cdw.com/product/apc-smart-ups-srt-3000va-sinewave-2u-rackmount-120v/4246855)
- [Starlink — planos e preços](https://www.starlink.com/us/service-plans)

---

## 11. O que este orçamento não cobre

Honestidade sobre os buracos:

1. **Licenciamento e registro do veículo.** Um trailer com instalação elétrica
   fixa pode exigir certificação como RV (RVIA) para seguro e para entrar em
   certos parques. Varia por estado.
2. **Seguro.** Um veículo com $120 mil de eletrônica dentro não é apólice de
   RV comum. Orçar com corretor especializado.
3. **Mão de obra elétrica certificada.** O sistema de 12 kW com gerador,
   inversor e transfer switch deve ser executado e assinado por eletricista
   licenciado — não é item onde economizar.
4. **Impostos e frete.** Sales tax do Texas (6,25% estadual + até 2% local) e
   frete de itens pesados (rack, cofre, baterias) podem somar **8–12%** ao
   total de equipamento.
5. **Contingência.** Reserve **15–20%** sobre o total. Projetos de conversão
   veicular estouram o orçamento como regra, não como exceção.

Somando 4 e 5 ao Cenário B: **$173k–258k vira ~$205k–310k realistas.**
