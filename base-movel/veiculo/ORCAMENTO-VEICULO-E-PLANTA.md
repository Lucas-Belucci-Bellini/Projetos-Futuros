# Veículo e Planta Baixa — Base Móvel

> Cotação de 15/09/2026. Dimensões em metros. Pesos em kg.

---

## 1. Escolha do veículo

### Por que o Peterbilt 379 não é só estética

O caminhão do Optimus Prime de 2007 é um **Peterbilt 379 long-nose 1992**
([ficha do veículo do filme](https://www.imcdb.org/v110958.html); o
[stunt truck original foi a leilão na Barrett-Jackson](https://www.barrett-jackson.com/scottsdale-2016/docket/vehicle/1992-peterbilt-379-transformers-optimus-prime-stunt-truck-190024)).
Além de ser o visual que você quer, ele resolve o problema que mata este
projeto em veículos menores: **capacidade de carga**.

| Plataforma | Payload útil | Comporta este projeto? |
|---|---:|---|
| Van Sprinter 3500 | ~1.400 kg | ❌ Não — falta ~700 kg |
| Box truck F-550 / Ram 5500 | ~4.500 kg | ✅ Sim, apertado |
| **Peterbilt 379 + trailer 40 ft** | **~20.000 kg** | ✅ Sim, com folga enorme |

### Estimativa de peso do projeto

| Bloco | Peso |
|---|---:|
| Rack + 4 nós + storage | 210 kg |
| Banco LiFePO4 40 kWh | 280 kg |
| Gerador diesel 12 kW + cofre acústico | 195 kg |
| Solar 3 kWp + estrutura de teto | 120 kg |
| 2× mini-split + dutos | 85 kg |
| Cofre 30 armas longas (Liberty USA 30) | 320 kg |
| Cofre de armas curtas + racks modulares | 95 kg |
| Água potável 200 L + cinza 100 L | 300 kg |
| Diesel 150 L | 125 kg |
| Isolamento, revestimento, estrutura interna | 480 kg |
| Mobiliário, cozinha, banheiro | 260 kg |
| Pertences, munição, equipamento de caça | 350 kg |
| **TOTAL** | **≈ 2.820 kg** |

Num F-550 isso consome 63% do payload — viável, mas sem margem para erro.
Num Peterbilt com trailer, é **14%**. A folga permite o que realmente importa:
**estrutura mais pesada e melhor isolada**, que é o que faz a diferença entre
um baú que esquenta e um que se mantém a 25 °C no verão texano.

### Orçamento do veículo

| Opção | Itens | Faixa |
|---|---|---|
| **A — Peterbilt 379 + trailer 40 ft** | Cavalo usado ($45k–90k) + trailer expedição ($60k–150k) + conversão ($80k–250k) | **$185k – $490k** |
| **B — Box truck 24 ft** | Chassi F-550/Ram 5500 usado ($35k–60k) + conversão ($60k–120k) | **$95k – $180k** |
| **C — Trailer 40 ft só** | Trailer convertido, cavalo alugado/contratado quando mover | **$60k – $150k** |

Referências de mercado: box truck usado pré-conversão custa
[$10k–15k](https://www.mortonsonthemove.com/box-truck-conversions/); conversão
DIY fica entre $10k e $50k; builds profissionais como a
[Field Van FAV](https://www.sportsmobileforum.com/threads/field-van-fav-box-truck-conversion-on-e350-chassis.2199009/)
chegam a ~$300k e a [Wanderbox](https://wanderbox.com/) parte de $395k.
**Nenhum desses carrega 4 kW de eletrônica** — a sua conversão custa mais que
a média porque o sistema elétrico e térmico é de outra categoria.

---

## 2. Planta baixa — trailer 40 ft

**Interior útil: 11,90 m × 2,35 m = 27,9 m².** Pé-direito 2,55 m.

Princípio de organização, do mais quente/barulhento ao mais silencioso:

```text
        FRENTE (engate / cabine)                                    TRASEIRA
        ├────────────────────────────────────────────────────────────────┤

  ┌─────────────┬──────────────┬─────────────────┬──────────┬────────────┐
  │   ZONA 5    │    ZONA 4    │     ZONA 3      │  ZONA 2  │   ZONA 1   │
  │  HABITAÇÃO  │   ARMARIA    │ POSTO COMANDO   │ TÉCNICA  │  MÁQUINAS  │
  │   2,10 m    │    2,20 m    │     2,80 m      │  2,60 m  │   2,20 m   │
  │   4,9 m²    │    5,2 m²    │     6,6 m²      │  6,1 m²  │   5,2 m²   │
  └─────────────┴──────────────┴─────────────────┴──────────┴────────────┘
     silencioso                                              ruído + calor
                                   ▲                    ▲
                              eixos do trailer (peso concentrado aqui)
```

### Planta detalhada

```text
 ╔═══════════════════════════════════════════════════════════════════════════╗
 ║ ZONA 5 — HABITAÇÃO (2,10 m)                                               ║
 ║ ┌──────────────────┐  ┌──────────────┐                                    ║
 ║ │ CAMA 1,40×2,00   │  │ COZINHA      │   • Beliche superior rebatível     ║
 ║ │ (sob ela: gavetas│  │ indução 2 bc │   • Geladeira 12 V 90 L            ║
 ║ │  de equipamento) │  │ pia + filtro │   • Blackout + luz âmbar noturna   ║
 ║ └──────────────────┘  └──────────────┘                                    ║
 ║ ┌──────────────────────────────────┐   • Banheiro seco (sanitário         ║
 ║ │ BANHEIRO 0,90×1,20 + chuveiro    │     compostagem) + chuveiro          ║
 ║ └──────────────────────────────────┘                                      ║
 ╠═══════════════════════════════════════════════════════════════════════════╣
 ║ ZONA 4 — ARMARIA + OFICINA (2,20 m)   🔒 porta com fechadura independente  ║
 ║ ┌────────────────────────┐ ┌────────────────────────┐                     ║
 ║ │ COFRE 30 ARMAS LONGAS  │ │ BANCADA DE MANUTENÇÃO  │                     ║
 ║ │ Liberty USA 30         │ │ 1,60×0,60 + morsa      │                     ║
 ║ │ 1h fogo · 320 kg       │ │ iluminação 5000 K      │                     ║
 ║ │ ancorado ao chassi     │ └────────────────────────┘                     ║
 ║ └────────────────────────┘ ┌────────────────────────┐                     ║
 ║ ┌────────────────────────┐ │ MUNIÇÃO — armário      │                     ║
 ║ │ COFRE 15 ARMAS CURTAS  │ │ ventilado, SEPARADO    │                     ║
 ║ │ gaveta blindada        │ │ do cofre de armas      │                     ║
 ║ └────────────────────────┘ └────────────────────────┘                     ║
 ╠═══════════════════════════════════════════════════════════════════════════╣
 ║ ZONA 3 — POSTO DE COMANDO (2,80 m)                                        ║
 ║ ┌─────────────────────────────────────────────────┐                       ║
 ║ │ MESA 2,40×0,75 · 3 monitores em braço articulado│                       ║
 ║ │ KVM 4 portas → alterna entre os 4 nós           │                       ║
 ║ │ cadeira com trava de deslocamento               │                       ║
 ║ └─────────────────────────────────────────────────┘                       ║
 ║ ┌──────────────┐  ┌──────────────────────────────┐                        ║
 ║ │ ARMÁRIOS     │  │ PAINEL DE MONITORAMENTO      │                        ║
 ║ │ superiores   │  │ energia · temp · rede · água │                        ║
 ║ └──────────────┘  └──────────────────────────────┘                        ║
 ╠══════════════ PAREDE TÉCNICA — isolada térmica e acusticamente ═══════════╣
 ║ ZONA 2 — SALA TÉCNICA (2,60 m)   ❄️ 18–24 °C · pressão positiva           ║
 ║ ┌────────────────────────┐                                                ║
 ║ │ RACK 24U               │  ┌─────────┐  CORREDOR FRIO ← insuflamento     ║
 ║ │ ├ 4U SERVER-01 Gaming  │  │ MINI-   │  ═══════════════════════          ║
 ║ │ ├ 4U SERVER-02 IA      │  │ SPLIT   │  RACK (frente → trás)             ║
 ║ │ ├ 4U SERVER-03 NAS     │  │ 2× 12k  │  ═══════════════════════          ║
 ║ │ ├ 2U SERVER-04         │  │ BTU     │  CORREDOR QUENTE → exaustão       ║
 ║ │ ├ 2U switch + firewall │  └─────────┘                                   ║
 ║ │ ├ 2U KVM + PDU         │  Filtro MERV13 na entrada de ar                ║
 ║ │ └ 6U 3× UPS SRT        │  Sensores: temp · umidade · fumaça · água      ║
 ║ └────────────────────────┘                                                ║
 ╠══════════════ PAREDE CORTA-FOGO — estanque à área habitável ══════════════╣
 ║ ZONA 1 — SALA DE MÁQUINAS (2,20 m)   🚪 acesso SÓ pelo exterior           ║
 ║ ┌──────────────────┐ ┌──────────────────┐ ┌────────────────────┐          ║
 ║ │ GERADOR 12 kW    │ │ BANCO LiFePO4    │ │ INVERSOR 12 kW     │          ║
 ║ │ diesel · cofre   │ │ 40 kWh · 280 kg  │ │ + quadro + DPS     │          ║
 ║ │ acústico         │ │ sobre o eixo     │ │ + transfer switch  │          ║
 ║ │ exaustão externa │ └──────────────────┘ └────────────────────┘          ║
 ║ └──────────────────┘ ┌──────────────────┐ ┌────────────────────┐          ║
 ║  2× detector de CO   │ ÁGUA 200 L       │ │ DIESEL 150 L       │          ║
 ║                      └──────────────────┘ └────────────────────┘          ║
 ╚═══════════════════════════════════════════════════════════════════════════╝
                          TETO: 3 kWp solar + 2× Starlink + exaustores
```

### A separação que você pediu: o que é PC e o que não é

| | Zona | Área | Conteúdo |
|---|---|---:|---|
| 🖥️ **PC** | Zona 2 — Sala Técnica | 6,1 m² | Os 4 nós, storage, rede, nobreaks. **Ninguém dorme, come ou trabalha aqui.** |
| 🖥️ **PC** | Zona 3 — Posto de Comando | 6,6 m² | Onde você *usa* os PCs. Monitores e periféricos, zero máquina. |
| 🏠 Casa | Zona 5 — Habitação | 4,9 m² | Dormir, cozinhar, banho. |
| 🔫 Armaria | Zona 4 | 5,2 m² | Cofres, bancada, munição. |
| ⚡ Serviço | Zona 1 — Máquinas | 5,2 m² | Gerador, baterias, tanques. Sem acesso interno. |

**Total PC: 12,7 m² (46%). Total não-PC: 15,2 m² (54%).**

A regra que organiza tudo: **calor e ruído nunca atravessam para onde se
dorme.** O gerador fica atrás de parede corta-fogo, com acesso apenas externo;
a sala técnica fica atrás de parede isolada; a habitação fica na ponta oposta
do trailer, a 9 metros do gerador.

---

## 3. Distribuição de peso

```text
        FRENTE                                                    TRASEIRA
        ├──────────────────────────────────────────────────────────────┤
  kg/m  │                                          ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │
        │                            ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │
        │  ▓▓▓▓▓▓▓▓  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │
        └──────────────────────────────────────────────────────────────┘
          Z5: 380   Z4: 480    Z3: 240      Z2: 420       Z1: 730
                        ▲                        ▲     ▲
                   kingpin                    eixos do trailer
```

Regras seguidas:
1. **Baterias e gerador sobre os eixos** — as duas maiores massas, no ponto de
   menor efeito de alavanca.
2. **Cofre de armas ancorado ao chassi**, não ao piso do baú. 320 kg soltos em
   uma freada de emergência são um projétil.
3. **Nada pesado em balanço traseiro** — carga atrás do eixo alivia o pino-rei
   e induz rabeamento.
4. **Lateral equilibrada:** rack a bombordo, cofres a boreste.

---

## 4. Estilo militar — especificação de acabamento

Você disse que não gosta de luxo. Esta é a paleta funcional, não decorativa:

| Elemento | Especificação | Razão |
|---|---|---|
| Piso | Alumínio xadrez 3 mm ou vinil industrial antiderrapante | Lava com mangueira, não retém umidade |
| Paredes | Painel FRP branco na técnica, compensado marítimo + tinta OD green na casa | Branco reflete luz na técnica; OD não mostra sujeira |
| Estrutura | Perfil de alumínio 40×40 (extrusão tipo 8020) | Reconfigurável sem solda |
| Fixação | Painéis MOLLE nas paredes livres | Reorganiza sem furar |
| Armazenamento | Caixas Pelican + caixas de munição M2A1 | Empilháveis, estanques, identificáveis |
| Iluminação | LED 4000 K branco + circuito **âmbar** separado para visão noturna | Âmbar preserva adaptação escotópica |
| Instalação elétrica | Eletroduto aparente, tudo etiquetado, trilho DIN no quadro | Manutenção no escuro, na estrada |
| Janelas | Mínimas, com blackout e grade interna | Segurança e carga térmica |
| Cores | OD green / coyote / preto fosco | Sem verniz, sem madeira aparente |

**Custo do acabamento militar vs. RV convencional:** aproximadamente o mesmo.
Alumínio e FRP custam mais que compensado; MOLLE e Pelican custam menos que
marcenaria sob medida. Fica empatado, e **pesa 15–20% menos**.

---

## 5. Isolamento térmico — não economize aqui

| Camada | Material | Espessura | R-value |
|---|---|---:|---:|
| Paredes | Spray foam de célula fechada | 50 mm | R-20 |
| Teto | Spray foam + barreira radiante | 75 mm | R-30 |
| Piso | Placa de poliiso + contrapiso | 40 mm | R-13 |
| Ponte térmica | Fita de quebra térmica nas longarinas | — | — |

Custo: **~$3.200** (incluso na climatização). Um baú de metal sem isolamento
chega a **60 °C** no sol do Texas. Com R-20/R-30, o ar-condicionado dimensionado
no orçamento dá conta; sem isolamento, **nenhum ar-condicionado dá**.

---

## 6. Pendências que exigem profissional

1. **Engenheiro estrutural** para os pontos de ancoragem do cofre e do banco de
   baterias. Cálculo de 20 g de desaceleração frontal.
2. **Eletricista licenciado** para o sistema de 12 kW (obrigatório para seguro).
3. **Pesagem em balança certificada (CAT scale)** depois da conversão, por eixo.
4. **Registro/licenciamento:** trailer com instalação fixa pode precisar de
   certificação RVIA. Varia por estado — confirmar no Texas DMV.
5. **Seguro especializado** — $120k de eletrônica não entra em apólice de RV
   padrão.
