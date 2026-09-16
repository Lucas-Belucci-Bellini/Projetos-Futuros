# Energia — Memorial de Cálculo e Orçamento

> **Este é o documento que decide se o projeto é viável.** Tudo o mais é
> questão de dinheiro; energia é questão de física. Cotação de 15/09/2026.

---

## 1. Por que 3 nobreaks não resolvem (e o que resolve)

Você pediu 3 nobreaks. Eles estão no orçamento — mas é importante que fique
claro o que eles fazem e o que não fazem:

**Um nobreak não é fonte de energia. É uma ponte.** Ele segura a carga por
minutos, o suficiente para o gerador partir ou para os servidores desligarem
de forma controlada. Um APC SRT 3000VA carregado com 2.700 W dura **~5 minutos**.

Para uma base móvel que precisa funcionar **estacionada, sem tomada, por dias**,
a estrutura correta tem quatro camadas:

```text
┌─ CAMADA 1 ── Shore power 50 A / 240 V  ── quando disponível (camping, casa)
├─ CAMADA 2 ── Solar 3 kWp no teto       ── 15–18 kWh/dia no Texas
├─ CAMADA 3 ── Banco LiFePO4 40 kWh      ── autonomia real, silenciosa
├─ CAMADA 4 ── Gerador diesel 12 kW      ── quando o resto não dá conta
└─ CAMADA 5 ── 3× UPS (a sua)            ── ponte de 5 min entre as trocas
```

Os nobreaks são a **camada 5**, e são indispensáveis — mas sozinhos não
sustentam nada. O que sustenta é o banco de baterias com o inversor.

---

## 2. Levantamento de carga

### 2.1 Cenário A (4 nós, 6 GPUs)

| Equipamento | Ocioso | Típico | Pico |
|---|---:|---:|---:|
| SERVER-01 Gaming (9950X3D + 2× RTX 5090) | 120 W | 800 W | **1.470 W** |
| SERVER-02 IA (9975WX + 4× RTX 5090) | 250 W | 2.400 W | **2.900 W** |
| SERVER-03 Minecraft/NAS (12 HDD) | 180 W | 380 W | **450 W** |
| SERVER-04 Serviços | 35 W | 90 W | **120 W** |
| Rede (switch 10G, firewall, KVM, PDU) | 90 W | 110 W | **140 W** |
| Starlink Flat High Performance | 75 W | 110 W | **150 W** |
| Climatização (2× mini-split 12k BTU) | 0 W | 1.100 W | **1.400 W** |
| Iluminação, tomadas, geladeira, bomba | 80 W | 160 W | **250 W** |
| **TOTAL** | **830 W** | **5.150 W** | **6.880 W** |

### 2.2 Cenário B (2 nós consolidados)

| | Ocioso | Típico | Pico |
|---|---:|---:|---:|
| Workstation (9955WX + 2× RTX 5090) | 180 W | 1.400 W | 1.750 W |
| Servidor (NAS + VMs) | 190 W | 400 W | 480 W |
| Rede + Starlink | 165 W | 220 W | 290 W |
| Climatização | 0 W | 750 W | 1.000 W |
| Casa | 80 W | 160 W | 250 W |
| **TOTAL** | **615 W** | **2.930 W** | **3.770 W** |

**A consolidação corta 45% do pico.** Esse é o argumento técnico mais forte a
favor do Cenário B — não é economia de dinheiro, é economia de watt, e watt
dentro de um veículo é o recurso mais escasso que existe.

---

## 3. Energia diária (kWh/dia)

Perfil de uso assumido — ajuste se o seu for diferente:

| Carga | Potência | Horas/dia | kWh/dia |
|---|---:|---:|---:|
| Base 24 h (NAS, serviços, rede, Starlink) | 500 W | 24 | **12,0** |
| IA sob carga | 2.400 W | 6 | **14,4** |
| Gaming | 800 W | 3 | **2,4** |
| Climatização (verão texano) | 900 W | 14 | **12,6** |
| Casa (luz, cozinha, bomba) | 160 W | 8 | **1,3** |
| | | **Total** | **42,7 kWh/dia** |

Para comparação: **uma casa americana média consome 29 kWh/dia.** Esta base
móvel consome mais que uma casa. É o número mais importante deste documento.

### Balanço

| Fonte | Produção | Observação |
|---|---:|---|
| Solar 3 kWp | +15,0 kWh/dia | Média anual no Texas (~5 h de sol pleno) |
| **Déficit** | **–27,7 kWh/dia** | Precisa vir de gerador ou shore power |

**Gerador diesel:** ~0,30 L/kWh → **8,3 L/dia** (~2,2 galões) → a $3,80/gal
= **$8,40/dia** = **~$250/mês** rodando 100% off-grid.

Com shore power disponível metade do tempo, cai para ~$130/mês (valor usado
no orçamento mestre).

---

## 4. Dimensionamento do banco de baterias

| Critério | Cálculo | Resultado |
|---|---|---|
| Autonomia alvo (noite sem gerador) | 12 h × carga base+clima (1,4 kW) | 16,8 kWh |
| Margem de profundidade (LiFePO4, 80% DoD) | 16,8 / 0,8 | 21 kWh |
| Margem para pico de IA (2 h) | + 4,8 kWh | 25,8 kWh |
| Envelhecimento (10 anos, 80% capacidade) | / 0,8 | **32,3 kWh** |
| **Especificado** | | **40 kWh** |

40 kWh a 48 V = **833 Ah**. Em módulos de 48 V/100 Ah (5,12 kWh), são
**8 módulos**. Peso: **~280 kg**. Esse peso vai **sobre o eixo**, nunca em
balanço traseiro.

---

## 5. Orçamento

| Item | Especificação | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| UPS | APC Smart-UPS SRT 3000VA / 2700 W, rack 2U | 3 | $4.280 | **$12.840** |
| Banco de baterias | LiFePO4 48 V, 8× 5,12 kWh (40 kWh), BMS + aquecedor | 1 | $11.000 | **$11.000** |
| Inversor/carregador | 12 kW 48 V split-phase 120/240 V, onda senoidal pura | 1 | $3.600 | **$3.600** |
| Gerador | Diesel 12 kW, refrigerado a líquido, cofre acústico | 1 | $6.500 | **$6.500** |
| Solar | 3 kWp (6× 500 W) + 2 MPPT + estrutura de teto | 1 | $3.800 | **$3.800** |
| Quadro e proteção | Transfer switch automático, DPS tipo 2, disjuntores, barramento, aterramento, shunt de monitoramento | 1 | $2.400 | **$2.400** |
| Entrada externa | Shore power 50 A, cabo 30 m, adaptadores | 1 | $400 | **$400** |
| | | | **Total** | **$40.540** |

### Divisão dos 3 UPS por grupo de carga

| UPS | Protege | Carga | Autonomia |
|---|---|---:|---:|
| **UPS-01** | SERVER-02 (IA) | 2.400 W | ~5 min |
| **UPS-02** | SERVER-01 (Gaming) + estação de trabalho | 1.100 W | ~12 min |
| **UPS-03** | SERVER-03 + SERVER-04 + rede + Starlink | 700 W | ~20 min |

**UPS-03 é o crítico** — é o que mantém NAS e rede vivos durante a troca de
fonte, e o que tem mais autonomia justamente porque é onde o desligamento
descontrolado causa mais estrago (pool ZFS em escrita).

---

## 6. Desligamento controlado (NUT)

Sem isto, os nobreaks servem para metade do que deveriam. Configure
[NUT (Network UPS Tools)](https://networkupstools.org/) com esta escalada:

| Gatilho | Ação |
|---|---|
| Falha de rede detectada | Log + notificação; gerador recebe comando de partida |
| Bateria < 50% | Suspende jobs de IA e backup; encerra SERVER-02 |
| Bateria < 30% | Desliga SERVER-01; NAS entra em modo somente-leitura |
| Bateria < 20% | `zpool sync` + desligamento de SERVER-03 e SERVER-04 |
| Bateria < 10% | Corta tudo exceto rede e sensores |

Um pool ZFS de 360 TB interrompido no meio de uma escrita **não corrompe**
(ZFS é transacional), mas um `resilver` de 30 TB leva 12–20 horas. Evitar isso
é o trabalho do UPS-03.

---

## 7. Riscos

| Risco | Consequência | Mitigação |
|---|---|---|
| **Carga excede a fiação** | Incêndio | Dimensionar cabos para 125% da carga contínua (NEC 210.19); execução por eletricista licenciado |
| **Baterias LiFePO4 no frio** | Não carregam abaixo de 0 °C | Módulos com aquecedor integrado (já especificado) |
| **Gerador em espaço fechado** | Monóxido de carbono | Compartimento estanque em relação à área habitável, exaustão externa, **2 detectores de CO** |
| **Vibração nos terminais** | Arco elétrico, falha intermitente | Terminais crimpados + travarrosca + inspeção trimestral |
| **Sobrecarga simultânea** | Inversor desarma | Automação que impede IA + clima + gaming no pico juntos |

---

## 8. Fontes

| Dado | Fonte |
|---|---|
| APC Smart-UPS SRT 3000VA, preço e especificação | [CDW](https://www.cdw.com/product/apc-smart-ups-srt-3000va-sinewave-2u-rackmount-120v/4246855) · [Schneider Electric](https://www.se.com/us/en/product/SURTA3000RMXL3U/) |
| Consumo RTX 5090 (575 W TGP) | [NVIDIA — especificações](https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/) |
| Threadripper PRO 9975WX (350 W TDP) | [Tom's Hardware](https://www.tomshardware.com/pc-components/cpus/amd-launches-threadripper-pro-9000-wx-series-cpus-with-up-to-96-zen-5-cores-for-usd11-699-shimada-peak-and-radeon-ai-pro-r9700-arrive-on-july-23) |
| Sistemas off-grid para contêiner/veículo | [DIY Solar Power Forum](https://diysolarforum.com/threads/off-grid-box-truck-rv-conversion.68767/) |
| Consumo médio residencial EUA | [U.S. EIA](https://www.eia.gov/tools/faqs/faq.php?id=97) |
