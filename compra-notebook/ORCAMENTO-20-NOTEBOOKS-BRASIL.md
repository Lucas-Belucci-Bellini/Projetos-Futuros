# Dossiê de Compra Brasil — 20 Orçamentos de Notebook

> **Objetivo:** comprar **1 notebook até 31/12/2026**, no **mercado brasileiro**.
> São **20 orçamentos** para escolher entre eles — não é uma frota.
>
> **Cotação: 16 de setembro de 2026.** Valores em **R$**, varejo nacional,
> **com todos os impostos já embutidos** (é assim que o preço aparece na loja).
>
> **Câmbio de referência: US$ 1 = R$ 5,15** (banda da semana: 5,08–5,19).
>
> A versão americana deste documento está em
> [`ORCAMENTO-20-NOTEBOOKS-COMPRA.md`](ORCAMENTO-20-NOTEBOOKS-COMPRA.md).
> **Não traduza os números de lá** — a estrutura de preço é outra, como a §2
> explica.

---

## 1. Resumo executivo

| | Modelo | À vista (PIX) | Por que |
|---|---|---:|---|
| 🥇 **Recomendado** | **Avell Storm 570 Ti** (RTX 5070 Ti 12 GB, 32 GB, 1 TB) | **R$ 13.949** | Melhor desempenho por real no Brasil, configurável, suporte nacional |
| 🥈 **Se quiser AMD** | **ASUS TUF Gaming A16** (Ryzen 9, RTX 5070, 32 GB) | **≈ R$ 12.400** | A melhor combinação AMD + RTX que o Brasil realmente vende |
| 🥉 **Orçamento apertado** | **Notebook RTX 5060 de entrada** (32 GB) | **R$ 9.699** | Roda ATM10 e Hermes 8B; não roda o Hermes grande |
| ✈️ **Se você viajar aos EUA** | **ASUS TUF A16** trazido na bagagem | **≈ R$ 10.558** | Mais barato que o varejo nacional — ver §3 |

**Faixa dos 20 orçamentos: R$ 9.699 a R$ 30.000.**

---

## 2. ⚠️ O que muda do mercado americano para o brasileiro

Três diferenças estruturais. Elas mudam a resposta, não só o número.

### 2.1 O preço brasileiro é ~2× o americano

Não é margem de loja — é carga tributária (II, IPI, PIS/COFINS, ICMS) embutida
no preço de prateleira.

| Máquina | EUA (custo final TX) | Equivalente no Brasil | Multiplicador |
|---|---:|---:|---:|
| RTX 5060 + 32 GB | US$ 1.460 ≈ R$ 7.519 | **R$ 9.699** | **1,29×** |
| RTX 5070 + 32 GB | US$ 1.840 ≈ R$ 9.476 | **≈ R$ 14.000** | **1,48×** |
| RTX 5070 Ti + 32 GB | US$ 2.326 ≈ R$ 11.979 | **R$ 15.499** | **1,29×** |
| RTX 5080 + 32 GB | US$ 2.627 ≈ R$ 13.529 | **R$ 23.999** | **1,77×** |

> **Quanto mais caro o equipamento, pior a diferença.** No topo da linha, o
> Brasil cobra quase o dobro.

### 2.2 A RTX 5090 de notebook está fora de alcance no Brasil

A placa **de desktop** sozinha está a **R$ 25.999** no KaBuM (pico de
R$ 26.999 em agosto/2026). Um notebook com RTX 5090 passa de **R$ 35.000**, e
mesmo assim é item raro de importadora, sem garantia nacional.

**Consequência direta para você:** o tier "roda o Hermes 4.3 36B nativo na
VRAM" — que exige 24 GB de VRAM — **não existe de forma viável no varejo
brasileiro**. Quem quiser isso precisa importar (§3) ou usar o servidor (§10).

### 2.3 AMD Ryzen + RTX de topo é escasso aqui

O estoque brasileiro de notebook gamer é majoritariamente **Intel**. Os modelos
Avell Storm 570 Ti e 580, por exemplo, são **Intel Core Ultra 9 275HX**.

A faixa AMD real no Brasil:

| Linha | CPU | GPU máxima no Brasil |
|---|---|---|
| ASUS TUF Gaming A15 / A16 | Ryzen 7 / Ryzen 9 | RTX 5070 |
| Acer Nitro V15 / V16 | Ryzen 5 / 7 | RTX 5070 |
| Lenovo LOQ (versões AMD) | Ryzen 5 / 7 | RTX 5060 |
| Lenovo Legion 5 AMD | Ryzen 7 / 9 | RTX 5070 Ti |

> **Seu requisito de AMD custa desempenho no Brasil.** Se ele for
> inegociável, seu teto prático é **RTX 5070 / 5070 Ti**. Se aceitar Intel,
> abre a faixa até RTX 5080.

---

## 3. As três rotas de compra — a conta de cada uma

Usando o **ASUS TUF A16** (Ryzen 9 270, RTX 5070, 32 GB, 1 TB), que nos EUA
custa **US$ 1.699,99**:

### Rota A — Varejo nacional

```
  Preço de prateleira          ≈ R$ 14.000
  Imposto                      já embutido
  ─────────────────────────────────────────
  TOTAL                        ≈ R$ 14.000
  À vista no PIX (−10%)        ≈ R$ 12.600
```
✅ Garantia nacional · ✅ CDC integral · ✅ Parcelamento sem juros
❌ Mais caro que a bagagem

### Rota B — Importação por site (comprar dos EUA e receber em casa)

```
  Mercadoria                   US$ 1.700
+ Frete internacional          US$    90
= Valor aduaneiro              US$ 1.790
+ Imposto de Importação 60%    US$ 1.074
= Subtotal                     US$ 2.864
+ ICMS 18% "por dentro"        US$   629
─────────────────────────────────────────
  TOTAL                        US$ 3.493  ≈  R$ 17.989
```
❌ **A rota mais cara de todas** · ❌ Sem garantia nacional
❌ Risco de retenção na alfândega · ❌ Suporte só no exterior

### Rota C — Bagagem acompanhada (você viaja e traz) ⭐

```
  Preço nos EUA                US$ 1.700
  Cota de isenção aérea        US$ 1.000  (isento)
  Excedente tributável         US$   700
+ Imposto 50% sobre excedente  US$   350
─────────────────────────────────────────
  TOTAL                        US$ 2.050  ≈  R$ 10.558
```
✅ **25% mais barato que o varejo nacional**
⚠️ Exige declaração via **e-DBV** e pagamento do excedente
⚠️ Garantia internacional depende do fabricante
⚠️ Só vale se a viagem já estiver acontecendo por outro motivo

> **Observação relevante:** em fevereiro de 2026 a Comissão de Desenvolvimento
> Econômico da Câmara **aprovou um projeto de lei** isentando do Imposto de
> Importação o notebook de uso pessoal trazido por viajante. **Ainda é projeto,
> não lei.** Não planeje a compra contando com isso — mas acompanhe, porque se
> virar lei a Rota C fica ainda melhor.

### Comparação final

| Rota | Custo | Garantia | Risco |
|---|---:|---|---|
| **C — Bagagem** | **R$ 10.558** | Internacional | Médio (alfândega) |
| A — Varejo BR | R$ 12.600–14.000 | **Nacional** | **Nenhum** |
| B — Importação online | R$ 17.989 | Nenhuma | Alto |

**Se você não tem viagem marcada, a Rota A é a resposta.** A diferença de
~R$ 2.000 não compensa comprar passagem, e a Rota B nunca compensa.

---

## 4. Seus requisitos, no contexto brasileiro

| # | Você pediu | Alcançável no Brasil? |
|---|---|---|
| 1 | 32 GB DDR5, expansível | ✅ Sim, comum |
| 2 | 1–2 TB, expansível a 8 TB | ⚠️ Sim — **mas 7 dos 20 vêm com 512 GB** (§6.1) |
| 3 | AMD Ryzen 7 ou 9 | ⚠️ Sim, **com teto de RTX 5070/5070 Ti** |
| 4 | RTX | ✅ Sim |
| 5 | Roda o Hermes 4.3 36B | ❌ **Não no varejo** — exige 24 GB de VRAM |
| 6 | Roda ATM10 com 200 mods | ✅ Sim, desde o tier de entrada |

### O que dá para rodar de IA local em cada faixa

| VRAM | GPU | Modelos que rodam bem | Preço BR |
|---|---|---|---|
| 8 GB | RTX 5060 / 5070 | Hermes 3 **8B** em Q4 | R$ 9.700–14.000 |
| 12 GB | RTX 5070 Ti | Hermes até **14B** confortável | R$ 15.500–17.000 |
| 16 GB | RTX 5080 | Hermes **36B** por offload (lento) | R$ 24.000–30.000 |
| 24 GB | RTX 5090 | Hermes **36B** nativo | ❌ indisponível |

---

# 5. Os 20 orçamentos

**Legenda de confiança:** 🔗 preço de listagem verificada · ≈ estimativa de
mercado. **Reconfira antes de pagar** — preço de eletrônico no Brasil muda toda
semana.

**Condição de pagamento assumida:** preço cheio parcelado em 10× sem juros;
**PIX/boleto com 8–12% de desconto** (padrão do varejo brasileiro).

---

## TIER 1 — RTX 5080 (Hermes 36B por offload)

### 01 — Avell Storm 580 🔗

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX — ❌ não é AMD |
| GPU | **RTX 5080 · 16 GB** GDDR7 |
| RAM | Configurável, **até 64 GB DDR5** |
| Armazenamento | **Configurável** — base 1 TB NVMe · 2 slots M.2 |
| Garantia | **12 meses Avell, suporte nacional** |

| | |
|---|---:|
| Preço de tabela | **R$ 23.999** |
| À vista PIX (−10%) | **≈ R$ 21.599** |
| Parcelado 10× | R$ 2.400/mês |

**Veredito:** Hermes 36B ⚠️ offload · ATM10 ✅ · Expansão ✅ · AMD ❌

> Melhor RTX 5080 do Brasil em custo-benefício, e o único da lista com
> **suporte técnico nacional de fábrica** — a Avell é brasileira.

[avell.com.br/storm-580](https://avell.com.br/storm-580)

### 02 — ASUS ROG Strix SCAR 16 · RTX 5080 ≈

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX · GPU RTX 5080 16 GB · 32 GB · 1 TB |
| Preço | **≈ R$ 27.000** · PIX ≈ R$ 24.300 |

[Loja ASUS Brasil](https://br.store.asus.com/gaming.html)

### 03 — Lenovo Legion Pro 7i · RTX 5080 ≈

| | |
|---|---|
| CPU | Intel Core Ultra 9 · RTX 5080 16 GB · 32 GB · 1 TB |
| Preço | **≈ R$ 25.500** · PIX ≈ R$ 22.950 |

### 04 — MSI Raider / Vector · RTX 5080 ≈

| | |
|---|---|
| CPU | Intel Ultra 9 · RTX 5080 16 GB · 32 GB · 2 TB |
| Preço | **≈ R$ 28.000** · PIX ≈ R$ 25.200 |

### 05 — Acer Predator Helios 16 · RTX 5080 ≈

| | |
|---|---|
| CPU | Intel Ultra 9 · RTX 5080 16 GB · 32 GB · 1 TB |
| Preço | **≈ R$ 24.500** · PIX ≈ R$ 22.050 |

[Acer Brasil](https://www.acer.com/br-pt/predator/laptops/helios/helios-neo-16)

---

## TIER 2 — RTX 5070 Ti (o ponto de equilíbrio brasileiro)

### 06 ⭐⭐ — Avell Storm 570 Ti 🔗

> **A recomendação deste dossiê.**

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX — ❌ não é AMD |
| GPU | **RTX 5070 Ti · 12 GB** GDDR7 |
| RAM | Configurável, até 64 GB · SO-DIMM |
| Armazenamento | **Configurável** — base 1 TB NVMe · 2 slots M.2 |
| Tela | **16" QHD+ 300 Hz** |
| Garantia | 12 meses Avell · **assistência no Brasil** |

| | |
|---|---:|
| Preço de tabela | **R$ 15.499** |
| **À vista PIX (−10%)** | **R$ 13.949** |
| Parcelado 10× | R$ 1.550/mês |

**Veredito:** Hermes até **14B** ✅ · ATM10 ✅ · Expansão ✅ · AMD ❌

> **Por que vence:** 12 GB de VRAM por menos de R$ 14.000 é o melhor
> desempenho por real do mercado nacional. Configurável na compra (você escolhe
> RAM e SSD, evitando o custo de upgrade da §7), com suporte de uma empresa
> brasileira — o que importa muito quando algo quebra.
>
> **O que você abre mão:** o processador é Intel. Se AMD for inegociável, vá
> para o orçamento 09.

[avell.com.br/storm-570-ti](https://avell.com.br/storm-570-ti)

### 07 — Acer Predator Helios Neo 16 · RTX 5070 Ti 🔗

| | |
|---|---|
| CPU | Intel Core Ultra 7/9 · GPU RTX 5070 Ti 12 GB · 16–32 GB · 1 TB |
| Preço | **R$ 16.845** · PIX ≈ R$ 15.160 |
| Garantia | 12 meses Acer Brasil |

[Acer Brasil](https://www.acer.com/br-pt/predator/laptops/helios/helios-neo-16-ai) ·
[KaBuM](https://www.kabum.com.br/produto/989341/notebook-gamer-predator-helios-neo-16-ai-intel-core-ultra-7-255hx-16gb-ram-rtx-5070-8gb-ssd-512gb-tela-16-windows-11-home)

### 08 — Lenovo Legion 5 · RTX 5070 Ti ≈

| | |
|---|---|
| CPU | Intel Ultra 7 · RTX 5070 Ti 12 GB · 32 GB · 1 TB |
| Preço | **≈ R$ 17.500** · PIX ≈ R$ 15.750 |

### 09 ⭐ — Lenovo Legion 5 **AMD** · RTX 5070 Ti ≈

> **A melhor máquina AMD que o Brasil vende.**

| | |
|---|---|
| CPU | **AMD Ryzen 9** (série HX) ✅ |
| GPU | RTX 5070 Ti 12 GB · 32 GB DDR5 SO-DIMM · 1 TB |
| Preço | **≈ R$ 18.000** · PIX ≈ R$ 16.200 |

**Veredito:** ✅ **Cumpre o requisito de AMD no teto do que é possível aqui.**

> Disponibilidade irregular — confirme estoque antes de contar com ele.

---

## TIER 3 — RTX 5070 (a faixa AMD do Brasil)

### 10 🔗 — ASUS ROG Strix G16 · Ultra 9 + RTX 5070 + 32 GB + 1 TB

| | |
|---|---|
| CPU | Intel Core Ultra 9 · GPU RTX 5070 8 GB · 32 GB · 1 TB · 16" |
| Preço | **R$ 17.999** 🔗 (loja oficial) · PIX ≈ R$ 16.200 |

**Veredito:** ⚠️ **Caro pelo que entrega.** Custa mais que o orçamento 06, que
tem GPU superior. Só se houver desconto agressivo.

[Loja ASUS Brasil](https://br.store.asus.com/gaming.html)

### 11 ⭐ — ASUS TUF Gaming A16 · Ryzen 9 + RTX 5070 ≈

> **Melhor custo-benefício AMD do Brasil.**

| | |
|---|---|
| CPU | **AMD Ryzen 9** ✅ · GPU RTX 5070 8 GB |
| RAM | 32 GB DDR5 · **SO-DIMM, 2 slots** |
| Armazenamento | 1 TB · 2 slots M.2 |
| Garantia | 12 meses ASUS Brasil |

| | |
|---|---:|
| Preço | **≈ R$ 14.000** |
| **À vista PIX** | **≈ R$ 12.400** |
| Parcelado 10× | R$ 1.400/mês |

**Veredito:** AMD ✅ · ATM10 ✅ · Expansão ✅ · Hermes 8B ✅ / 36B ❌

> **Se o requisito de AMD for inegociável e o orçamento importar, é este.**
> R$ 1.500 mais barato que o orçamento 06, com CPU AMD, mas 8 GB de VRAM em
> vez de 12 GB.

[Loja ASUS Brasil](https://br.store.asus.com/gaming.html) ·
[KaBuM](https://www.kabum.com.br/computadores/notebooks/notebook-gamer) ·
[Buscapé](https://www.buscape.com.br/busca/notebook+gamer+rtx+5070)

### 12 — Acer Predator Helios Neo 16 AI · RTX 5070 · 16 GB / 512 GB 🔗

| | |
|---|---|
| CPU | Intel Core Ultra 7 255HX · RTX 5070 · **16 GB** · 512 GB |
| Preço | **≈ R$ 13.500** · PIX ≈ R$ 12.150 |
| ⚠️ Upgrade obrigatório | +16 GB RAM (≈ R$ 700) e SSD 1 TB (≈ R$ 600) |
| **Custo real** | **≈ R$ 13.450** |

> O preço baixo esconde 16 GB e 512 GB. Depois do upgrade, empata com o
> orçamento 11 — que já vem pronto.

[KaBuM](https://www.kabum.com.br/produto/989341/notebook-gamer-predator-helios-neo-16-ai-intel-core-ultra-7-255hx-16gb-ram-rtx-5070-8gb-ssd-512gb-tela-16-windows-11-home) ·
[Acer Store BR](https://br-store.acer.com/)

### 13 — Acer Predator Helios Neo 16 AI · RTX 5070 · 16 GB / 1 TB ≈

| | |
|---|---|
| Preço | **≈ R$ 14.500** · PIX ≈ R$ 13.050 |

[Horizon Play](https://www.horizonplay.com.br/produto/notebook-gamer-acer-predator-helios-neo-16-ai-intel-core-ultra-7-16gb-1tb-ssd-rtx-5070-16-phn16-73-7166-preto-233897)

### 14 — Acer Nitro V16 · **Ryzen 7** + RTX 5070 ≈

| | |
|---|---|
| CPU | **AMD Ryzen 7** ✅ · RTX 5070 8 GB · 16–32 GB · 512 GB–1 TB |
| Preço | **≈ R$ 12.500** · PIX ≈ R$ 11.250 |

**Veredito:** o AMD + RTX 5070 mais barato do Brasil. Confirme a RAM da versão.

### 15 — Avell Storm 570 ≈

| | |
|---|---|
| CPU | Intel Core i9 · RTX 5070 · 16" QHD+ 180 Hz |
| RAM / SSD | **Configuráveis** — base 1 TB NVMe |
| Preço | **≈ R$ 13.500** · PIX ≈ R$ 12.150 |

[Linha Storm — Avell](https://avell.com.br/notebooks/notebooks-linhas/gamers-linha-storm)

---

## TIER 4 — RTX 5060 (entrada que ainda cumpre)

> Todos rodam **ATM10 com 200 mods** e **Hermes 3 8B**. A GPU não é o gargalo
> do Minecraft.

### 16 — Notebook gamer RTX 5060 mais barato do mercado 🔗

| | |
|---|---|
| GPU | RTX 5060 · 8 GB GDDR7 · 32 GB RAM |
| Armazenamento | ⚠️ **512 GB** típico do tier — confirmar no anúncio |
| Preço | **R$ 9.699** 🔗 · PIX ≈ R$ 8.730 |

**Veredito:** o piso de entrada que ainda atende 5 dos 6 requisitos.

[KaBuM — busca RTX 5060](https://www.kabum.com.br/busca/rtx-5060) ·
[Buscapé](https://www.buscape.com.br/busca/rtx+5060+notebook)

### 17 — ASUS ROG Strix G16 · RTX 5060 🔗

| | |
|---|---|
| Armazenamento | ⚠️ **512 GB** típico — confirmar no anúncio |
| Preço | **R$ 11.899** 🔗 (13/09/2026) · PIX ≈ R$ 10.709 |

### 18 — Acer Predator Helios Neo 16 · RTX 5060 🔗

| | |
|---|---|
| Armazenamento | ⚠️ **512 GB** típico — confirmar no anúncio |
| Preço | **R$ 13.090** 🔗 · PIX ≈ R$ 11.781 |

**Veredito:** ⚠️ caro para um RTX 5060 — o orçamento 14 dá RTX 5070 **e** AMD
por menos.

### 19 — Acer Nitro V16 · **Ryzen 7** + RTX 5060 ≈

| | |
|---|---|
| CPU | **AMD Ryzen 7** ✅ · RTX 5060 8 GB · 16–32 GB · ⚠️ 512 GB típico |
| Preço | **≈ R$ 10.500** · PIX ≈ R$ 9.450 |

### 20 — Lenovo LOQ · **Ryzen** + RTX 5060 ≈

| | |
|---|---|
| CPU | **AMD Ryzen 5/7** ✅ · RTX 5060 · **16 GB** · ⚠️ **512 GB** típico |
| Preço | **≈ R$ 9.900** · PIX ≈ R$ 8.910 |
| ⚠️ Upgrade para 32 GB | **+ R$ 700** |
| **Custo real** | **≈ R$ 9.610** |

> A linha LOQ começa em **R$ 4.649**, mas essas versões têm GPU menor e
> **não atendem** seus requisitos. Confira a GPU antes de se animar com o preço.

---

# 6. Tabela mestre — os 20, por preço à vista

| # | Modelo | CPU | AMD? | GPU / VRAM | RAM | **SSD** | **M.2** | PIX | Tabela |
|:-:|---|---|:-:|---|---|:-:|:-:|---:|---:|
| 16 | RTX 5060 entrada 🔗 | — | ? | 5060 / 8 GB | 32 GB | ⚠️ **512 GB** ᶜ | 2 | **R$ 8.730** | R$ 9.699 |
| 20 | Lenovo LOQ | Ryzen 5/7 | ✅ | 5060 / 8 GB | 16 GB* | ⚠️ **512 GB** ᶜ | 2 | **R$ 8.910** | R$ 9.900 |
| 19 | Acer Nitro V16 | Ryzen 7 | ✅ | 5060 / 8 GB | 16–32 GB | ⚠️ **512 GB** ᶜ | 2 | **R$ 9.450** | R$ 10.500 |
| 17 | ROG Strix G16 🔗 | Intel | ❌ | 5060 / 8 GB | 16 GB | ⚠️ **512 GB** ᶜ | 2 | **R$ 10.709** | R$ 11.899 |
| 14 | Acer Nitro V16 | **Ryzen 7** | ✅ | **5070** / 8 GB | 16–32 GB | ⚠️ 512 GB–1 TB ᶜ | 2 | **R$ 11.250** | R$ 12.500 |
| 18 | Helios Neo 16 🔗 | Intel | ❌ | 5060 / 8 GB | 16 GB | ⚠️ **512 GB** ᶜ | 2 | **R$ 11.781** | R$ 13.090 |
| 12 | Helios Neo 16 AI 🔗 | Ultra 7 | ❌ | 5070 / 8 GB | 16 GB* | ⚠️ **512 GB** 🔗 | 2 | **R$ 12.150** | R$ 13.500 |
| 15 | Avell Storm 570 | i9 | ❌ | 5070 / 8 GB | config. | **config.** (base 1 TB) | 2 | **R$ 12.150** | R$ 13.500 |
| **11** ⭐ | **ASUS TUF A16** | **Ryzen 9** | **✅** | 5070 / 8 GB | 32 GB | **1 TB** ✅ | **2** | **R$ 12.400** | R$ 14.000 |
| 13 | Helios Neo 16 AI | Ultra 7 | ❌ | 5070 / 8 GB | 16 GB | **1 TB** ✅ | 2 | **R$ 13.050** | R$ 14.500 |
| **06** 🥇 | **Avell Storm 570 Ti** 🔗 | Ultra 9 | ❌ | **5070 Ti / 12 GB** | config. | **config.** (base 1 TB) | **2** | **R$ 13.949** | R$ 15.499 |
| 07 | Helios Neo 16 🔗 | Ultra 7/9 | ❌ | 5070 Ti / 12 GB | 16–32 GB | **1 TB** ✅ | 2 | **R$ 15.160** | R$ 16.845 |
| 08 | Lenovo Legion 5 | Ultra 7 | ❌ | 5070 Ti / 12 GB | 32 GB | **1 TB** ✅ | 2 | **R$ 15.750** | R$ 17.500 |
| 10 | ROG Strix G16 🔗 | Ultra 9 | ❌ | 5070 / 8 GB | 32 GB | **1 TB** ✅ | 2 | **R$ 16.200** | R$ 17.999 |
| **09** ⭐ | **Legion 5 AMD** | **Ryzen 9** | **✅** | **5070 Ti / 12 GB** | 32 GB | **1 TB** ✅ | **2** | **R$ 16.200** | R$ 18.000 |
| **01** 🥈 | **Avell Storm 580** 🔗 | Ultra 9 | ❌ | **5080 / 16 GB** | até 64 GB | **config.** (base 1 TB) | 2 | **R$ 21.599** | R$ 23.999 |
| 05 | Predator Helios 16 | Ultra 9 | ❌ | 5080 / 16 GB | 32 GB | **1 TB** ✅ | 2 | **R$ 22.050** | R$ 24.500 |
| 03 | Legion Pro 7i | Ultra 9 | ❌ | 5080 / 16 GB | 32 GB | **1 TB** ✅ | 2 | **R$ 22.950** | R$ 25.500 |
| 02 | ROG Strix SCAR 16 | Ultra 9 | ❌ | 5080 / 16 GB | 32 GB | **1 TB** ✅ | 2 | **R$ 24.300** | R$ 27.000 |
| 04 | MSI Raider/Vector | Ultra 9 | ❌ | 5080 / 16 GB | 32 GB | **2 TB** ✅✅ | 2 | **R$ 25.200** | R$ 28.000 |

\* Vem com 16 GB — somar **R$ 700** do upgrade obrigatório para 32 GB.

ᶜ **Confirmar na ficha do anúncio.** O tier de entrada brasileiro costuma sair
com 512 GB, mas varia por lote e por loja. Estes valores são o padrão do
segmento, não listagem verificada.

---

## 6.1 ⚠️ Atenção ao armazenamento — seu requisito nº 2

Você pediu **1 ou 2 TB**. A lista se divide assim:

| Armazenamento | Quantos | Quais | Situação |
|---|:-:|---|---|
| **2 TB** | 1 | 04 | ✅ Acima do pedido |
| **1 TB** | 8 | 07, 08, 09, 10, 11, 13, 02, 03, 05 | ✅ Atende |
| **Configurável** | 3 | 01, 06, 15 (Avell) | ✅ **Você escolhe na compra** |
| ⚠️ **512 GB** | 7 | 12, 14, 16, 17, 18, 19, 20 | ❌ **Abaixo do pedido** |

### O que isso significa na prática

**Os sete de 512 GB não atendem o seu requisito de fábrica.** Para chegar a
1 TB, cada um precisa de **+R$ 600** (SSD M.2 de 1 TB). Isso muda o ranking de
preço:

| # | Modelo | PIX anunciado | +SSD 1 TB | **Custo real** |
|:-:|---|---:|---:|---:|
| 16 | RTX 5060 entrada | R$ 8.730 | +R$ 600 | **R$ 9.330** |
| 20 | Lenovo LOQ | R$ 8.910 | +R$ 600 +R$ 700 (RAM) | **R$ 10.210** |
| 19 | Acer Nitro V16 | R$ 9.450 | +R$ 600 | **R$ 10.050** |
| 17 | ROG Strix G16 | R$ 10.709 | +R$ 600 | **R$ 11.309** |
| 12 | Helios Neo 16 AI | R$ 12.150 | +R$ 600 +R$ 700 (RAM) | **R$ 13.450** |

> **O orçamento 11 (ASUS TUF A16, R$ 12.400) já vem com 32 GB e 1 TB
> prontos.** Depois de somar os upgrades, o 12 custa R$ 1.050 a mais e ainda é
> Intel. O 17 fica R$ 1.100 mais barato, mas com 16 GB e GPU inferior.

### Os três configuráveis são a jogada mais inteligente

Os **Avell (01, 06, 15)** deixam você escolher RAM e SSD no pedido. Isso importa
por três motivos:

1. **Sai mais barato** que comprar a peça depois no varejo brasileiro.
2. **Mantém a garantia de fábrica** sobre o conjunto — você não abre a máquina.
3. **Evita o risco** de descobrir na chegada que o slot livre não existe.

**No Storm 570 Ti (orçamento 06), subir de 1 TB para 2 TB na configuração custa
bem menos que os R$ 600 do SSD avulso.** Peça o orçamento configurado no site
antes de decidir.


---

# 7. Custo de upgrade no Brasil

Você queria chegar a 8 TB. No Brasil, a conta é pior que nos EUA:

| Upgrade | Peça | Preço BR |
|---|---|---:|
| +16 GB (chegar a 32 GB) | 1× SO-DIMM DDR5 16 GB | **≈ R$ 700** |
| 32 GB → 64 GB | 2× SO-DIMM DDR5 32 GB | **≈ R$ 3.400** |
| +1 TB NVMe | 1× M.2 1 TB | **≈ R$ 600** |
| +4 TB NVMe | 1× M.2 4 TB | **≈ R$ 3.200** |
| **1 TB → 8 TB** | 2× M.2 4 TB | **≈ R$ 6.400** |

> **Os 8 TB internos custam metade de um notebook de entrada.** A crise de NAND
> (o preço quintuplicou até janeiro/2026) chega ao Brasil com o câmbio e o
> imposto por cima.

### Três saídas práticas

1. **Configure na compra.** A Avell deixa você escolher RAM e SSD no pedido —
   quase sempre mais barato que comprar a peça depois, e com garantia de
   fábrica sobre o conjunto.
2. **HDD externo USB de 8 TB: ≈ R$ 1.200.** Resolve biblioteca de jogos,
   arquivo e backup. **Economia de ~R$ 5.200** contra os 8 TB internos.
3. **Reserve o NVMe interno** para sistema, mundos de Minecraft e modelos de
   IA — onde a velocidade realmente muda a experiência.

---

# 8. Seus direitos na compra — CDC

Isto não existe no dossiê americano e vale dinheiro:

| Direito | Prazo | Base legal |
|---|---|---|
| **Arrependimento** (compra online/telefone) | **7 dias corridos** do recebimento, **sem justificativa**, com frete de devolução por conta do vendedor | [CDC art. 49](https://www.planalto.gov.br/ccivil_03/leis/l8078compilado.htm) |
| **Garantia legal** (bem durável) | **90 dias** | CDC art. 26, II |
| Garantia contratual | Normalmente **12 meses** (soma-se à legal) | Contrato do fabricante |
| Prazo de reparo | **30 dias**; depois disso você escolhe troca, dinheiro de volta ou abatimento | CDC art. 18, §1º |
| Vício oculto | Conta a partir da **descoberta**, não da compra | CDC art. 26, §3º |

> **Use os 7 dias.** Faça o teste de 48 h da §11 dentro dessa janela. Se
> aparecer pixel morto, memória com defeito ou superaquecimento,
> **devolva — não tente consertar.**

---

# 9. Black Friday no Brasil — 27/11/2026

Cai dentro do seu prazo. Mas o comportamento brasileiro tem particularidades:

| Fato | O que fazer |
|---|---|
| "Metade do dobro" é real no varejo BR | **Registre o preço de hoje**, com print, data e loja |
| O desconto verdadeiro costuma ser **10–20%** em notebook | Não espere 50% |
| **PIX/boleto já dá 8–12%** o ano inteiro | Compare o preço PIX de hoje com o de Black Friday |
| Estoque das melhores configurações acaba antes | Decida o modelo **antes** de novembro |
| Sites de histórico mostram a manipulação | [Buscapé](https://www.buscape.com.br/) · [Zoom](https://www.zoom.com.br/) · [Hardware Barato](https://www.hardwarebarato.com/) |

**Conselho:** escolha o modelo agora, registre o preço PIX, e em 27/11 compre
**só se estiver abaixo desse número.**

---

# 10. 💡 A jogada que resolve o Hermes

O tier de 24 GB de VRAM **não existe no Brasil** — mas você não precisa dele
no notebook.

| | Notebook roda o Hermes | Notebook é terminal do servidor |
|---|---|---|
| Modelo acessível | Hermes 36B (só importando, R$ 35.000+) | Hermes **70B** |
| Notebook necessário | Importado, sem garantia nacional | Orçamento 11 — **R$ 12.400** |

Um notebook nacional com garantia, conectado por
[Tailscale](https://tailscale.com/) ao **SERVER-02** do seu
[orçamento da base móvel](../base-movel/ORCAMENTO-2026.md), te dá acesso a um
modelo que **nenhum notebook do mundo roda** — e a GPU fica onde tem energia e
refrigeração de verdade.

**Conclusão:** compre no Brasil, com garantia nacional, na faixa de
R$ 12.000–14.000. Deixe a IA pesada no rack.

---

# 11. Checklist de compra

### Antes

- [ ] Registrar **preço PIX de hoje** com print, data e loja
- [ ] Comparar em **KaBuM, Pichau, Terabyte, Amazon.com.br, Mercado Livre, Magazine Luiza** e loja oficial
- [ ] Conferir histórico no [Buscapé](https://www.buscape.com.br/) e [Zoom](https://www.zoom.com.br/)
- [ ] Confirmar **RAM: SO-DIMM ou soldada** na ficha do fabricante
- [ ] Confirmar **nº de slots M.2 livres**
- [ ] Verificar se a garantia é **nacional** (fuja de "produto importado, garantia do vendedor")
- [ ] Checar reputação do vendedor (Mercado Livre: só **MercadoLíder Platinum**)
- [ ] Comparar PIX à vista × 10× sem juros — se você tem o dinheiro rendendo acima do CDI, parcelar pode valer

### No ato

- [ ] Guardar **nota fiscal eletrônica** em PDF, no NAS e fora dele
- [ ] Registrar número de série e data de recebimento (inicia os 7 dias)
- [ ] Recusar garantia estendida que duplique a do fabricante

### Na chegada — dentro dos 7 dias do CDC

- [ ] Gravar vídeo da abertura da caixa
- [ ] `memtest86` a noite inteira
- [ ] Teste de pixel morto (branco, preto, RGB)
- [ ] Estresse 30 min (FurMark + Prime95) medindo temperatura
- [ ] Conferir RAM, VRAM e SSD reais no sistema
- [ ] **1 h de ATM10 com os 200 mods** — o seu teste real
- [ ] Testar todas as portas
- [ ] **Qualquer defeito: acione o arrependimento e devolva**

---

# 12. Riscos

| Risco | Probabilidade | Mitigação |
|---|---|---|
| Preço subir com o câmbio | **Alta** | Comprar contra o preço PIX registrado |
| "Desconto" de Black Friday ser inflado | **Alta** | Histórico no Buscapé/Zoom |
| Modelo AMD sem estoque | **Alta** | Ter 2ª opção Intel definida |
| Produto "importado" sem garantia nacional | Média | Comprar em loja oficial ou grande varejista |
| Upgrade de 8 TB custar R$ 6.400 | **Certa** | HDD externo por R$ 1.200 (§7) |
| Hermes 36B não rodar | **Certa no varejo BR** | Servidor de IA (§10) |

---

# 13. Fontes

| Dado | Fonte |
|---|---|
| Câmbio USD/BRL ≈ 5,15 (banda 5,08–5,19) | [Investing.com](https://br.investing.com/currencies/usd-brl) · [Wise](https://wise.com/us/currency-converter/usd-to-brl-rate/history) · [Banco Central](https://www.bcb.gov.br/) |
| Notebook RTX 5060 a partir de R$ 9.699 · ROG Strix G16 a R$ 11.899 (13/09/2026) | [Como Comprar](https://comocomprar.com.br/notebook-gamer-rtx-5060/) · [KaBuM](https://www.kabum.com.br/busca/rtx-5060) |
| Avell Storm 570 Ti — R$ 15.499 com 10% no PIX | [Avell](https://avell.com.br/storm-570-ti) |
| Avell Storm 580 — R$ 23.999, até 64 GB DDR5 | [Avell](https://avell.com.br/storm-580) |
| ASUS ROG Strix G16 (Ultra 9 + RTX 5070 + 32 GB) — R$ 17.999 | [Loja ASUS Brasil](https://br.store.asus.com/gaming.html) |
| Predator Helios Neo: R$ 13.090 (5060) a R$ 16.845 (5070 Ti) | [Acer Brasil](https://www.acer.com/br-pt/predator/laptops/helios/helios-neo-16-ai) · [KaBuM](https://www.kabum.com.br/produto/989341/notebook-gamer-predator-helios-neo-16-ai-intel-core-ultra-7-255hx-16gb-ram-rtx-5070-8gb-ssd-512gb-tela-16-windows-11-home) |
| Faixa do mercado BR: R$ 4.649 (LOQ) a R$ 34.267 | [CPG — guia de notebooks gamer 2026](https://clickpetroleoegas.com.br/guia-atualizado-revela-os-melhores-notebooks-gamers-a-venda-no-brasil-em-2026-com-opcoes-que-vao-de-r-4-600-ate-mais-de-r-34-mil-btl96/) |
| Acer Nitro V até RTX 5070; LOQ até RTX 5060 | [Zoom — comparativo](https://www.zoom.com.br/notebook/deumzoom/acer-nitro-v-vs-lenovo-loq-e) |
| RTX 5090 desktop a R$ 25.999 no KaBuM (pico R$ 26.999 em ago/2026) | [Hardware Barato](https://www.hardwarebarato.com/produtos/placas-de-video/rtx-5090) · [TecMundo](https://www.tecmundo.com.br/voxel/500406-rtx-5090-chega-por-ate-r-20-mil-no-brasil-veja-preco-das-rtx-50-no-pais.htm) |
| Importação online: 60% II + ICMS 17–20% | [CalculaCentro](https://calculacentro.com/blog/imposto-importacao-compra-exterior-como-calcular-2026) · [Guelcos](https://guelcos.com.br/conteudo/importacao/imposto-de-importacao-guia-completo/) |
| Bagagem aérea: cota US$ 1.000, excedente 50% via e-DBV | [Nomad Global](https://www.nomadglobal.com/conteudos/o-que-pode-trazer-do-exterior) · [Receita Federal](https://www.gov.br/receitafederal/) |
| PL isentando notebook de viajante — aprovado em comissão (fev/2026), **ainda não é lei** | [Brasilturis](https://brasilturis.com.br/2026/02/05/comissao-aprova-isencao-de-imposto-para-notebook-de-uso-pessoal-trazido-do-exterior/) · [Adrenaline](https://www.adrenaline.com.br/off-topic/adeus-taxas-novo-projeto-isenta-imposto-de-importacao-de-notebook-trazido-do-exterior/) |
| Direitos do consumidor (7 dias, 90 dias, 30 dias de reparo) | [CDC — Lei 8.078/1990](https://www.planalto.gov.br/ccivil_03/leis/l8078compilado.htm) |

---

## 14. Premissas e limites

1. **Preços coletados em 16/09/2026.** Eletrônico no Brasil muda de preço toda
   semana e depende do câmbio. **Reconfira antes de pagar.**
2. Marcados **🔗 são listagens verificadas**; marcados **≈ são estimativas**
   derivadas da estrutura de preço do mercado nacional. Trate os ≈ como faixa,
   não cotação.
3. **Desconto PIX de 8–12%** é prática padrão, mas **não é garantido** — varia
   por loja e por campanha.
4. Impostos brasileiros **já estão no preço de prateleira**. O frete costuma
   ser grátis acima de R$ 300 nas grandes lojas.
5. **Nenhum link é de afiliado**, conforme a regra do repositório.
6. **Não cobre:** mochila, mouse, monitor, dock, software, nem seguro.
7. Este documento **não é consultoria tributária**. As regras de importação e
   bagagem mudam; confirme na
   [Receita Federal](https://www.gov.br/receitafederal/) antes de viajar.
