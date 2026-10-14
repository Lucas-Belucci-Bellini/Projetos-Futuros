# Climatização — Carga Térmica e Orçamento

> Cotação de 15/09/2026.

---

## 1. A conta que define o projeto

**Toda a energia elétrica consumida pela eletrônica vira calor.** Não parte —
toda. Um rack que consome 4.000 W dissipa 4.000 W de calor no ambiente, o
equivalente a **13.650 BTU/h**, ou a três secadores de cabelo ligados
permanentemente dentro de um contêiner.

| Cenário | Carga TI típica | Calor gerado |
|---|---:|---:|
| A — 4 nós, 6 GPUs | 3.790 W | **12.935 BTU/h** |
| B — 2 nós consolidados | 2.020 W | **6.893 BTU/h** |

### Ganho térmico externo (verão texano, 40 °C)

| Superfície | Área | R-value | Ganho a ΔT=15 °C |
|---|---:|---:|---:|
| Teto | 28 m² | R-30 | 1.510 BTU/h |
| Paredes laterais | 60 m² | R-20 | 4.860 BTU/h |
| Piso | 28 m² | R-13 | 3.490 BTU/h |
| Portas, janelas, pontes térmicas | — | — | 1.800 BTU/h |
| Ocupação (1 pessoa) | — | — | 600 BTU/h |
| **Subtotal envoltória** | | | **12.260 BTU/h** |

### Carga total

| | Cenário A | Cenário B |
|---|---:|---:|
| Eletrônica | 12.935 | 6.893 |
| Envoltória | 12.260 | 12.260 |
| **TOTAL** | **25.195 BTU/h** | **19.153 BTU/h** |
| **Especificado (margem 15%)** | **29.000 BTU/h** | **22.000 BTU/h** |

**Conclusão:** o Cenário A precisa de ~29.000 BTU/h — praticamente o
ar-condicionado de uma casa de 3 quartos, dentro de um trailer. É o argumento
térmico a favor da consolidação: **o Cenário B corta 24% da carga de
refrigeração** e permite usar equipamento menor e mais leve.

---

## 2. Solução: duas zonas independentes

Um único sistema para tudo é errado: a sala técnica quer 18–24 °C e ar seco
filtrado; a habitação quer 22–25 °C e conforto acústico. Requisitos diferentes,
equipamentos diferentes.

```text
  ZONA TÉCNICA (18–24 °C)            ZONA HABITÁVEL (22–25 °C)
  ┌─────────────────────┐            ┌─────────────────────┐
  │  2× mini-split      │            │  1× mini-split      │
  │  12.000 BTU         │            │  9.000 BTU          │
  │  ↓ insuflamento     │            │  conforto + silêncio│
  │  CORREDOR FRIO      │            └─────────────────────┘
  │  ▓▓▓ RACK ▓▓▓       │
  │  CORREDOR QUENTE    │            Separadas por parede
  │  ↑ exaustão externa │            isolada. O ar quente
  └─────────────────────┘            NUNCA cruza.
```

### Por que corredor quente/frio

Sem separação, o ar quente que sai do rack é aspirado de volta pela frente —
o equipamento cozinha o próprio ar de admissão e a temperatura sobe em espiral.
Com a contenção (painéis cegos nos U vazios, vedação das laterais), a mesma
capacidade de refrigeração rende **20–30% mais**. Custa painel de espuma e fita;
é a melhoria de melhor retorno do projeto inteiro.

---

## 3. Orçamento

| Item | Especificação | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| Mini-split técnico | 12.000 BTU inverter 240 V, SEER 20+ | 2 | $1.300 | $2.600 |
| Mini-split habitação | 9.000 BTU inverter | 1 | $950 | $950 |
| Dutos e contenção | Plenum, painéis cegos, vedação, grelhas | 1 | $1.400 | $1.400 |
| Isolamento | Spray foam célula fechada + barreira radiante | 1 | $3.200 | $3.200 |
| Filtragem | Pré-filtro + MERV13, pressurização positiva | 1 | $900 | $900 |
| Sensores | Temp (×6), umidade, fumaça, água, CO + shutdown | 1 | $700 | $700 |
| **Total** | | | | **$9.750** |

> No orçamento mestre aparece **$8.800** para o Cenário A sem o split da
> habitação separado; este documento detalha a versão completa de 3 unidades.
> Adote **$9.750** como número de projeto.

---

## 4. Filtragem — o item invisível

Uma base móvel para em estrada de terra, acampamento, deserto. **Poeira é o que
mata eletrônica em veículo**, muito antes do calor.

- **Pressão positiva:** insufle mais ar do que exaure na zona técnica. O ar
  entra só pelo filtro; frestas expelem em vez de admitir.
- **Pré-filtro lavável** + **MERV13** na entrada.
- **Troca a cada 500 h** de operação, ou ao mudar de bioma.

Custo anual de filtro: ~$120. Custo de uma GPU entupida de poeira: $4.525.

---

## 5. Monitoramento e desligamento por temperatura

| Gatilho | Ação |
|---|---|
| Zona técnica > 30 °C | Alerta; ventilação de emergência ao máximo |
| Zona técnica > 35 °C | Suspende jobs de IA (maior fonte de calor) |
| Zona técnica > 40 °C | Desliga SERVER-02 e SERVER-01 |
| Zona técnica > 45 °C | Desligamento geral, exceto rede e sensores |
| Detector de fumaça | Corte total de energia + alarme |
| Sensor de água no piso | Alerta + corte da zona afetada |

Integrado ao mesmo sistema do [desligamento por energia](../energia/ORCAMENTO-ENERGIA.md#6-desligamento-controlado-nut).

---

## 6. Referência de consumo

As 3 unidades somam **~2.050 W** em operação plena — 30% de toda a carga
elétrica da base. **A climatização é o segundo maior consumidor, atrás só do
nó de IA.** Isso está contabilizado no
[balanço energético](../energia/ORCAMENTO-ENERGIA.md#3-energia-diária-kwhdia).
