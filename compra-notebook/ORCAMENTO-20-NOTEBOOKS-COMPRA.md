# Dossiê de Compra — 20 Orçamentos de Notebook

> **Objetivo:** comprar **1 notebook até 31/12/2026**.
> Este documento traz **20 orçamentos completos** para escolher entre eles —
> não é uma frota de 20 máquinas.
>
> **Cotação: 16 de setembro de 2026.** Preços em USD, mercado dos EUA.
> Cada orçamento traz **custo final** (preço + imposto do Texas de 8,25% +
> frete), garantia, onde comprar e veredito técnico.
>
> Complementa [`base-movel/notebooks/ORCAMENTO-20-NOTEBOOKS.md`](../base-movel/notebooks/ORCAMENTO-20-NOTEBOOKS.md),
> que é o documento de **seleção técnica** (roda Hermes ou não). Este aqui é o
> de **decisão de compra**.

---

## 1. Resumo executivo

| | Modelo | Custo final | Por que |
|---|---|---:|---|
| 🥇 **Escolha recomendada** | **Lenovo Legion Pro 7 Gen 10 AMD** (9955HX3D + RTX 5080 + 32 GB + 1 TB) | **$2.627** | Único ponto da lista que fecha AMD + expansível + desempenho sem pagar o prêmio da RTX 5090 |
| 🥈 Se o Hermes 36B for inegociável | **MSI Raider A18 HX** (9955HX3D + RTX 5090 + 64 GB + 2 TB) | **$6.494** | Único do mercado com Ryzen 9 9955HX3D + RTX 5090; 3 slots M.2 |
| 🥉 Se o orçamento mandar | **ASUS TUF A16** (Ryzen 9 270 + RTX 5070 + 32 GB + 1 TB) | **$1.840** | Cumpre tudo menos o Hermes grande, por 28% do preço do topo |

**Faixa de custo final dos 20 orçamentos: $1.298 a $8.009.**

---

## 2. ⚠️ O calendário até 31/12/2026 — leia antes de tudo

Sua janela de compra contém a **melhor data do ano**:

| Data | Evento | O que esperar |
|---|---|---|
| 23–29/11/2026 | **Black Week** | Ofertas antecipadas começam |
| **27/11/2026** | **🎯 BLACK FRIDAY** | O pico de desconto |
| 30/11/2026 | Cyber Monday | Foco em online, sobras de estoque |
| Dez/2026 | Fim de ano | Estoque baixo, desconto menor |

### Mas há uma armadilha, e ela é específica de 2026

Os dois movimentos estão em direções opostas:

| Força | Efeito | Número |
|---|---|---|
| ⬇️ Desconto de Black Friday em **RTX 50** | Reduz | **8% a 13%** |
| ⬆️ Aumento de preço confirmado pelos fabricantes | Aumenta | **15% a 20%** (até 30% no topo) |

**Lenovo, Dell, HP, Acer e ASUS confirmaram aumentos de 15–20% para 2026.**
Isso significa que um desconto de 10% na Black Friday pode apenas devolver o
preço de hoje — ou nem isso, se o reajuste entrar antes.

### O que fazer com essa informação

| Cenário | Recomendação |
|---|---|
| **Você pode comprar hoje** | Registre o preço de hoje como sua **linha de base**. Se na Black Friday o preço final estiver **abaixo** dela, compre; se estiver igual ou acima, compre agora e pare de esperar. |
| **Precisa esperar o dinheiro** | Mire a **Black Week (23–29/11)**, não o dia 27. Ofertas antecipadas frequentemente igualam a Black Friday, com estoque melhor. |
| **Quer um RTX 40** | Aí o desconto é maior: **15–20%**. Mas 12 GB de VRAM não roda o Hermes grande. |

> **Regra prática:** anote hoje o preço final do modelo escolhido, com data e
> loja. Na Black Friday, compare com esse número — não com o "de/por" da
> vitrine, que costuma inflar o preço original.

---

## 3. Suas exigências → critérios objetivos

| # | O que você pediu | Critério | Verificado em cada orçamento |
|---|---|---|:---:|
| 1 | 32 GB DDR5 (DDR4 aceita) | ≥32 GB, **SO-DIMM** (não soldada) | ✓ |
| 2 | 1 ou 2 TB, com espaço para expandir até 8 TB | ≥1 TB + **slots M.2 livres** | ✓ |
| 3 | AMD Ryzen 7 ou 9 | AMD preferencial; Intel só onde AMD não existe | ✓ |
| 4 | RTX (ferramentas NVIDIA) | GeForce RTX série 50 | ✓ |
| 5 | Roda o Hermes | ≥24 GB VRAM (nativo) ou 16 GB + 64 GB RAM (offload) | ✓ |
| 6 | Roda ATM10 com 200 mods | ≥32 GB RAM + CPU forte em thread única | ✓ |

### O corte técnico que organiza a lista

O **Hermes 4.3 36B** em Q4_K_M ocupa **~21,8 GB**. Só a **RTX 5090 de notebook
(24 GB GDDR7)** roda o modelo inteiro na placa. Com 16 GB (RTX 5080) roda por
*offload* — funciona, mas cai de ~35 tok/s para ~6–10 tok/s.

| Tier | VRAM | Hermes 4.3 36B | Orçamentos |
|---|---|---|---|
| **A** | 24 GB (RTX 5090) | ✅ Nativo, fluido | 1–5 |
| **B** | 16 GB (RTX 5080) | ⚠️ Offload, usável | 6–11 |
| **C** | 8–12 GB (RTX 5070/Ti/5060) | ❌ Só modelos até 14B | 12–20 |

---

## 4. Como ler cada orçamento

```
  Preço de tabela              o que a loja anuncia
+ Imposto do Texas (8,25%)     6,25% estadual + até 2% municipal
+ Frete                        $0 nas grandes lojas acima de $99
= CUSTO FINAL                  o que sai do seu bolso
```

**Marcação de confiança do preço:**

| Marca | Significa |
|:-:|---|
| 🔗 | **Preço de listagem específica**, verificado. Confiável. |
| ≈ | **Faixa do segmento**, não cotação. Trate como estimativa. |

---

# 5. Os 20 orçamentos

## TIER A — Roda o Hermes 4.3 36B nativo (24 GB VRAM)

### Orçamento 01 ⭐ — MSI Raider A18 HX · A9WJG-052US

| Item | Especificação |
|---|---|
| CPU | **AMD Ryzen 9 9955HX3D** (16C/32T, 3D V-Cache) |
| GPU | **RTX 5090 · 24 GB** GDDR7 |
| RAM | 64 GB DDR5-5600 · **SO-DIMM, 2 slots** |
| Armazenamento | 2 TB · **3 slots M.2** (18") |
| Tela | 18" UHD+ Mini LED 120 Hz |
| Garantia | 1 ano MSI · extensão ~$249/2 anos |

| Custo | Valor |
|---|---:|
| Preço de tabela ≈ | $5.999 |
| Imposto TX 8,25% | $494 |
| Frete | $0 |
| **CUSTO FINAL** | **$6.494** |

**Veredito:** Hermes 36B ✅ nativo · ATM10 ✅ (o melhor CPU da lista) ·
Expansão ✅ (3 slots M.2 = caminho limpo para 8 TB)

> **É o único notebook do mercado com Ryzen 9 9955HX3D + RTX 5090.** Se os três
> requisitos (AMD + Hermes grande + RAM expansível) forem inegociáveis, a lista
> tem um item só, e é este.

Onde: [Newegg](https://www.newegg.com/msi-18-geforce-rtx-5090-laptop-gpu-amd-ryzen-9-9955hx3d-64gb-ddr5-5600mhz-memory-2-tb-ssd/p/N82E16834156732) ·
[Best Buy](https://www.bestbuy.com/site/6621615.p?skuId=6621615) ·
[Adorama](https://www.adorama.com/msi-raider-a18-hx-gaming-laptop-ryzen-9-rtx-5090-64gb-ram-2tb-ssd/p/msi18hx052) ·
[Excaliber PC](https://www.excaliberpc.com/810967/msi-raider-a18-hx-a9wjg-052us.html)

---

### Orçamento 02 — Razer Blade 16 (2025) 64 GB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 (12C/24T, **28 W**) |
| GPU | RTX 5090 · 24 GB |
| RAM | 64 GB LPDDR5X-8000 — ❌ **SOLDADA** |
| Armazenamento | 4 TB · 1 slot M.2 |
| Garantia | 1 ano Razer |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $4.899,99 |
| Imposto TX | $404 |
| **CUSTO FINAL** | **$5.304** |

**Veredito:** Hermes ✅ · ATM10 ⚠️ (CPU de 28 W) · Expansão ❌ **reprova o
requisito 1**

> O mais bonito e mais fino da lista. **Reprova no seu requisito de RAM
> expansível** — você escolhe 64 GB na compra e nunca mais muda.

Onde: [Amazon](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLX9VQQ) ·
[Razer](https://www.razer.com/gaming-laptops/razer-blade-16)

---

### Orçamento 03 — Razer Blade 16 (2025) 32 GB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 · GPU RTX 5090 24 GB |
| RAM | 32 GB LPDDR5X — ❌ soldada |
| Armazenamento | 2 TB · 1 slot M.2 |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $4.499,99 |
| Imposto TX | $371 |
| **CUSTO FINAL** | **$4.871** |

**Veredito:** Hermes ✅ · Expansão ❌

Onde: [Amazon](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLFFLK8) ·
[Notebookcheck (histórico)](https://www.notebookcheck.net/Razer-Blade-16-2025-pre-orders-open-starting-at-2-799-with-up-to-AMD-Ryzen-AI-9-HX-370-RTX-5090-and-64-GB-of-RAM.967573.0.html)

---

### Orçamento 04 — ASUS ROG Strix SCAR 18 (2026)

| Item | Especificação |
|---|---|
| CPU | Intel Core Ultra 9 275HX — ⚠️ **não é AMD** |
| GPU | RTX 5090 · 24 GB |
| RAM | 32 GB DDR5 · SO-DIMM, 2 slots |
| Armazenamento | 2 TB · 2 slots M.2 |
| Garantia | 1 ano ASUS |

| Custo | Valor |
|---|---:|
| Preço ≈ | $4.899 |
| Imposto TX | $404 |
| **CUSTO FINAL** | **$5.303** |

**Veredito:** Hermes ✅ · ATM10 ✅ · Expansão ✅ · AMD ❌

Onde: [PC Guide — melhores RTX 5090](https://www.pcguide.com/laptop/guide/best-rtx-5090/) ·
[B&H](https://www.bhphotovideo.com/c/buy/rtx-5090-laptops/ci/60263)

---

### Orçamento 05 — Alienware 18 Area-51 / Acer Predator Helios 18P

| Item | Especificação |
|---|---|
| CPU | Intel Core Ultra 9 275HX · GPU RTX 5090 24 GB |
| RAM | 32 GB DDR5 SO-DIMM · 2 TB · **3 slots M.2** (18") |
| Garantia | Dell: 1 ano · **Premium Support +$199** |

| Custo | Valor |
|---|---:|
| Preço ≈ | $5.299 |
| Imposto TX | $437 |
| **CUSTO FINAL** | **$5.735** |

**Veredito:** Hermes ✅ · Expansão ✅ · AMD ❌

Onde: [Dell](https://www.dell.com/en-us/shop/dell-laptops/scr/laptops/appref=nvidia-geforce-rtx-5090-video) ·
[Best Buy — RTX 5090](https://www.bestbuy.com/site/searchpage.jsp?browsedCategory=pcmcat287600050003&id=pcat17071&qp=graphicscardsv_facet%3DVideo+Card%7ENVIDIA+GeForce+RTX+5090&st=categoryid%24pcmcat287600050003)

---

## TIER B — Hermes 36B por offload (16 GB VRAM)

### Orçamento 06 ⭐⭐ — Lenovo Legion Pro 7 Gen 10 AMD · 32 GB / 1 TB

> **A escolha recomendada deste dossiê.**

| Item | Especificação |
|---|---|
| CPU | **AMD Ryzen 9 9955HX3D** (3D V-Cache) |
| GPU | RTX 5080 · 16 GB |
| RAM | 32 GB DDR5 · **SO-DIMM, expansível a 96 GB** |
| Armazenamento | 1 TB Gen5 · **2 slots M.2** |
| Tela | 16" WQXGA OLED 240 Hz |
| Garantia | 1 ano Lenovo · **Premium Care +$129/2 anos** |

| Custo | Valor |
|---|---:|
| Preço 🔗 (oferta rastreada) | $2.427,24 |
| Imposto TX 8,25% | $200 |
| Frete | $0 |
| **CUSTO FINAL** | **$2.627** |

**Veredito:** Hermes 36B ⚠️ offload (~6–10 tok/s) · Hermes 8B/14B ✅ instantâneo ·
ATM10 ✅ **excelente** · Expansão ✅ total

> **Por que esta é a recomendação:** mesmo CPU do orçamento 01 — o 9955HX3D com
> 3D V-Cache, que é o melhor processador de notebook que existe para Minecraft
> modded — por **40% do preço**. O que você perde é VRAM: 16 GB em vez de 24 GB,
> o que empurra o Hermes 36B para offload. Se o modelo de 36B não for uso
> diário, **os $3.867 de diferença não se justificam.**

Onde: [Slickdeals ($2.427,24)](https://slickdeals.net/f/18887428-legion-pro-7-gen-10-16-qhd-240hz-oled-ryzen-9-9955hx3d-rtx-5080-32gb-ddr5-1tb-ssd-2427-24) ·
[Micro Center](https://www.microcenter.com/product/703054/lenovo-legion-7-pro-oled-16-gaming-laptop-computer-eclipse-black) ·
[Lenovo](https://www.lenovo.com/us/en/)

---

### Orçamento 07 — Lenovo Legion Pro 7 Gen 10 AMD · 64 GB / 1 TB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · GPU RTX 5080 16 GB |
| RAM | **64 GB** DDR5 SO-DIMM · 1 TB Gen5 · 2 slots M.2 |

| Custo | Valor |
|---|---:|
| Preço ≈ | $3.199 |
| Imposto TX | $264 |
| **CUSTO FINAL** | **$3.463** |

**Veredito:** Hermes 36B ⚠️ offload **melhor** (64 GB de RAM ajudam muito) ·
ATM10 ✅ · Expansão ✅

> **Compare com o 06:** +$836 por 32 GB extras. Comprar a RAM depois custa
> ~$628 (ver §7) e você perde a garantia sobre o módulo. **Se pretende chegar a
> 64 GB, comprar já com 64 é mais barato e mais simples.**

Onde: [Amazon](https://www.amazon.com/clp/B0GNDNY4CM) ·
[Notebookcheck](https://www.notebookcheck.net/New-Lenovo-Legion-Pro-7-gaming-laptop-debuts-with-up-to-Ryzen-9-9955HX3D-and-RTX-5080.1104908.0.html)

---

### Orçamento 08 — Lenovo Legion Pro 7 Gen 10 AMD · 32 GB / 2 TB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · RTX 5080 16 GB · 32 GB · **2 TB** |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.799 |
| Imposto TX | $231 |
| **CUSTO FINAL** | **$3.030** |

**Veredito:** igual ao 06, com o dobro do SSD.

> **+$403 por 1 TB a mais.** Um SSD de 1 TB avulso custa ~$190. Aqui você paga
> o dobro — mas ganha garantia de fábrica e não abre a máquina. Decida pelo
> quanto você valoriza não mexer no equipamento novo.

Onde: [Amazon](https://www.amazon.com/Legion-9955HX3D-NVIDIA-GeForce-Graphics/dp/B0FTVPVVFW) ·
[eBay](https://www.ebay.com/itm/326818335306)

---

### Orçamento 09 — MSI Raider A18 HX · A9WIG-082US (RTX 5080)

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · GPU RTX 5080 16 GB |
| RAM | 64 GB · 2 TB · **3 slots M.2** · 18" |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $7.399 |
| Imposto TX | $610 |
| **CUSTO FINAL** | **$8.009** |

**Veredito:** ⚠️ **O pior custo-benefício da lista.** Custa $1.515 mais que o
orçamento 01, que tem a GPU **superior**. Só entra na lista pelos 3 slots M.2.

> Evite, a menos que caia 40% na Black Friday.

Onde: [MSI Store](https://us-store.msi.com/msi-Raider-A18-HX-A9WIG-082US) ·
[NVIDIA Marketplace](https://marketplace.nvidia.com/en-au/consumer/gaming-laptops/msi-raider-a18-hx-a9w-18-120hz-uhd-gaming-laptop-ryzen-9-9955hx3d-64gb-2tb-rtx5080-w11p)

---

### Orçamento 10 — Lenovo Legion Pro 7i Gen 10 (Intel)

| Item | Especificação |
|---|---|
| CPU | Intel Core Ultra 9 275HX — ❌ não é AMD |
| GPU | RTX 5080 16 GB · 32 GB DDR5 · 1 TB · 16" OLED 240 Hz |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.699 |
| Imposto TX | $223 |
| **CUSTO FINAL** | **$2.922** |

**Veredito:** $295 **mais caro** que o orçamento 06 e sem o 3D V-Cache.
Sem motivo para escolher, dado o seu requisito de AMD.

Onde: [Newegg Insider](https://www.newegg.com/insider/best-rtx-50-series-gaming-laptops-2026-which-one-is-right-for-you/)

---

### Orçamento 11 — HP Omen Max 16

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen AI 9 HX 475 · GPU RTX 5080 16 GB |
| RAM | 32 GB SO-DIMM · 1 TB · 16" 2.5K OLED 240 Hz |
| Garantia | 1 ano HP · **HP Care Pack +$159** |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.899 |
| Imposto TX | $239 |
| **CUSTO FINAL** | **$3.138** |

**Veredito:** Hermes ⚠️ offload · ATM10 ✅ · Expansão ✅

Onde: [Best Buy — AMD Ryzen 9 + NVIDIA](https://www.bestbuy.com/site/searchpage.jsp?browsedCategory=abcat0500000&id=pcat17071&qp=gpubrand_facet%3DGPU+Brand%7ENVIDIA%5Eparent_processormodelsv_facet%3DAMD+Ryzen+9%7EAMD+Ryzen+9&st=categoryid%24abcat0500000) ·
[HP](https://www.hp.com/us-en/shop/cv/gaminglaptops)

---

## TIER C — Não roda o Hermes grande (mas roda tudo o mais)

> Todos daqui rodam **Hermes 3 8B** sem esforço e **ATM10 com 200 mods** bem.

### Orçamento 12 ⭐ — ASUS TUF Gaming A16 · Ryzen 9 270 + RTX 5070

> **Melhor custo-benefício absoluto da lista.**

| Item | Especificação |
|---|---|
| CPU | **AMD Ryzen 9 270** |
| GPU | RTX 5070 · 8 GB |
| RAM | 32 GB DDR5 · **SO-DIMM, 2 slots, até 64 GB** |
| Armazenamento | 1 TB · **expansível até 4 TB** |
| Garantia | 1 ano ASUS |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $1.699,99 |
| Imposto TX | $140 |
| **CUSTO FINAL** | **$1.840** |

**Veredito:** Hermes 36B ❌ · Hermes 8B ✅ · ATM10 ✅ · Expansão ✅

> **Cumpre 5 dos seus 6 requisitos por 28% do preço do orçamento 01.** O que
> falta é só o Hermes grande — e há uma saída para isso na §10.

Onde: [Slickdeals / Best Buy](https://slickdeals.net/f/18354607-asus-tuf-a16-2025-16-fhd-165hz-ryzen-9-270-rtx-5070-32gb-ddr5-1tb-ssd-1699-99) ·
[ASUS](https://www.asus.com/us/laptops/for-gaming/tuf-gaming/asus-tuf-gaming-a16-2025/)

---

### Orçamento 13 — ASUS ROG Strix G16 · Ryzen 9 8940HX + RTX 5070 Ti · 2 TB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 9 8940HX · GPU RTX 5070 Ti **12 GB** |
| RAM | 32 GB DDR5 SO-DIMM · **2 TB** · 2 slots M.2 |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.149 |
| Imposto TX | $177 |
| **CUSTO FINAL** | **$2.326** |

**Veredito:** 12 GB de VRAM rodam **Hermes até 14B** confortável — o melhor do
Tier C para IA local.

Onde: [Walmart](https://www.walmart.com/ip/17871308188) ·
[Amazon](https://www.amazon.com/ASUS-1900x1200-Display-Keyboard-Accessories/dp/B0FPF28L6X)

---

### Orçamento 14 — ASUS ROG Strix G16 · Ryzen 9 8940HX + RTX 5070 Ti · 1 TB

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 9 8940HX · RTX 5070 Ti 12 GB · 32 GB · 1 TB |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.049 |
| Imposto TX | $169 |
| **CUSTO FINAL** | **$2.218** |

Onde: [Newegg / HIDevolution](https://www.newegg.com/asus-rog-strix-g16-16-0-geforce-rtx-5070ti-laptop-gpu-amd-ryzen-9-8940hx-fhd-32gb-memory-1-tb-pcie-ssd/p/2WC-000N-0G0J8) ·
[ROG](https://rog.asus.com/laptops/rog-strix/rog-strix-g16-2025-g614/)

---

### Orçamento 15 — ASUS ROG Strix G16 · Ryzen 9 9955HX + RTX 5070

| Item | Especificação |
|---|---|
| CPU | **AMD Ryzen 9 9955HX** (16 núcleos) · GPU RTX 5070 8 GB |
| RAM | 32 GB DDR5-5600 · 1 TB · 16" 240 Hz |

| Custo | Valor |
|---|---:|
| Preço ≈ | $1.649 |
| Imposto TX | $136 |
| **CUSTO FINAL** | **$1.785** |

**Veredito:** CPU de 16 núcleos por menos de $1.800 — melhor CPU do Tier C.

Onde: [Newegg](https://www.newegg.com/asus-rog-strix-g16-16-geforce-rtx-5070-laptop-gpu-amd-ryzen-9-9955hx-32gb-memory-1-tb-pcie-ssd/p/N82E16834236630)

---

### Orçamento 16 — ASUS ROG Zephyrus G16

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 · GPU RTX 5070/5080 |
| RAM | 32 GB LPDDR5X — ❌ **soldada** · 1 TB · **1,95 kg** |

| Custo | Valor |
|---|---:|
| Preço ≈ | $2.399 |
| Imposto TX | $198 |
| **CUSTO FINAL** | **$2.596** |

**Veredito:** O mais leve da lista — e o que **menos expande**. Reprova no
requisito 1.

Onde: [PC Guide](https://www.pcguide.com/laptop/guide/best-rtx-5090/)

---

### Orçamento 17 — ASUS TUF Gaming 18

| Item | Especificação |
|---|---|
| CPU | Conforme SKU · GPU RTX 5060/5070 · 32 GB · 1 TB |
| Tela | **18"** — refrigeração melhor, mais slots M.2 |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $1.499,99 |
| Imposto TX | $124 |
| **CUSTO FINAL** | **$1.624** |

**Veredito:** chassi de 18" pelo preço de um 16". Melhor opção se você pretende
sessões longas de ATM10 (carga térmica sustentada).

Onde: [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/)

---

### Orçamento 18 — Acer Nitro V16

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 7 · GPU RTX 5060 8 GB · 32 GB · 1 TB |

| Custo | Valor |
|---|---:|
| Preço 🔗 | $1.487,99 |
| Imposto TX | $123 |
| **CUSTO FINAL** | **$1.611** |

Onde: [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/)

---

### Orçamento 19 — ASUS TUF Gaming A16 · Ryzen 7 260 + RTX 5060

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 7 260 · GPU RTX 5060 8 GB · 32 GB DDR5 · 1 TB |

| Custo | Valor |
|---|---:|
| Preço ≈ | $1.349 |
| Imposto TX | $111 |
| **CUSTO FINAL** | **$1.460** |

**Veredito:** a GPU não é o gargalo do Minecraft — este roda ATM10 igual aos de
$2.000.

Onde: [Best Buy](https://www.bestbuy.com/product/asus-tuf-a16-rtx-5060-ryzen-7-32gb-1tb-ssd-165hz-gaming-laptop/JJGGLH8TZY) ·
[Newegg](https://www.newegg.com/asus-tuf-gaming-a16-16-geforce-rtx-5060-laptop-gpu-amd-ryzen-7-260-wqxga-32gb-memory-1-tb-pcie-ssd/p/N82E16834236636)

---

### Orçamento 20 — Lenovo LOQ 15/16 Gen 10

| Item | Especificação |
|---|---|
| CPU | AMD Ryzen 7 · GPU RTX 5060 8 GB · **16 GB de fábrica** · 1 TB |

| Custo | Valor |
|---|---:|
| Preço ≈ | $1.199 |
| Imposto TX | $99 |
| **CUSTO FINAL** | **$1.298** |
| ⚠️ Upgrade obrigatório para 32 GB | **+$314** |
| **CUSTO FINAL REAL** | **$1.612** |

**Veredito:** o mais barato da tabela **deixa de ser** o mais barato depois do
upgrade de RAM obrigatório. Empata com o Acer Nitro V16, que já vem com 32 GB.

Onde: [Comparativo Newegg](https://www.newegg.com/insider/best-rtx-50-series-gaming-laptops-2026-which-one-is-right-for-you/)

---

# 6. Tabela mestre — os 20, por custo final

| # | Modelo | CPU | GPU / VRAM | RAM | SSD | RAM expansível | Custo final |
|:-:|---|---|---|---|---|:-:|---:|
| 20 | Lenovo LOQ Gen 10 | Ryzen 7 | 5060 / 8 GB | 16 GB* | 1 TB | ✅ | **$1.298** |
| 19 | ASUS TUF A16 | R7 260 | 5060 / 8 GB | 32 GB | 1 TB | ✅ | **$1.460** |
| 18 | Acer Nitro V16 | Ryzen 7 | 5060 / 8 GB | 32 GB | 1 TB | ✅ | **$1.611** |
| 17 | ASUS TUF 18 | conforme SKU | 5060/5070 | 32 GB | 1 TB | ✅ | **$1.624** |
| 15 | ROG Strix G16 | **R9 9955HX** | 5070 / 8 GB | 32 GB | 1 TB | ✅ | **$1.785** |
| **12** ⭐ | **ASUS TUF A16** | **R9 270** | 5070 / 8 GB | 32 GB | 1 TB | ✅ | **$1.840** |
| 14 | ROG Strix G16 | R9 8940HX | 5070 Ti / 12 GB | 32 GB | 1 TB | ✅ | **$2.218** |
| 13 | ROG Strix G16 | R9 8940HX | 5070 Ti / 12 GB | 32 GB | 2 TB | ✅ | **$2.326** |
| 16 | ROG Zephyrus G16 | AI 9 HX 370 | 5070/5080 | 32 GB | 1 TB | ❌ | **$2.596** |
| **06** 🥇 | **Legion Pro 7 G10 AMD** | **R9 9955HX3D** | 5080 / 16 GB | 32 GB | 1 TB | ✅ | **$2.627** |
| 10 | Legion Pro 7i G10 | Ultra 9 275HX | 5080 / 16 GB | 32 GB | 1 TB | ✅ | **$2.922** |
| 08 | Legion Pro 7 G10 AMD | R9 9955HX3D | 5080 / 16 GB | 32 GB | 2 TB | ✅ | **$3.030** |
| 11 | HP Omen Max 16 | AI 9 HX 475 | 5080 / 16 GB | 32 GB | 1 TB | ✅ | **$3.138** |
| 07 | Legion Pro 7 G10 AMD | R9 9955HX3D | 5080 / 16 GB | **64 GB** | 1 TB | ✅ | **$3.463** |
| 03 | Razer Blade 16 | AI 9 HX 370 | **5090 / 24 GB** | 32 GB | 2 TB | ❌ | **$4.871** |
| 05 | Alienware 18 / Helios 18P | Ultra 9 275HX | **5090 / 24 GB** | 32 GB | 2 TB | ✅ | **$5.303** |
| 04 | ROG Strix SCAR 18 | Ultra 9 275HX | **5090 / 24 GB** | 32 GB | 2 TB | ✅ | **$5.303** |
| 02 | Razer Blade 16 | AI 9 HX 370 | **5090 / 24 GB** | 64 GB | 4 TB | ❌ | **$5.304** |
| **01** 🥈 | **MSI Raider A18 HX** | **R9 9955HX3D** | **5090 / 24 GB** | 64 GB | 2 TB | ✅ | **$6.494** |
| 09 | MSI Raider A18 HX | R9 9955HX3D | 5080 / 16 GB | 64 GB | 2 TB | ✅ | **$8.009** |

\* O LOQ precisa de +$314 de RAM para chegar a 32 GB → custo real **$1.612**.

---

# 7. ⚠️ O custo de "aumentar depois" — a conta que surpreende

Você mencionou querer chegar a **8 TB** e talvez mais RAM. Em 2026, isso custa
mais do que parece:

| Upgrade | Peça | Custo |
|---|---|---:|
| 32 GB → 64 GB | 2× SO-DIMM DDR5 32 GB (~$314 cada) | **$628** |
| +4 TB (1 slot) | 1× M.2 4 TB | **$400–600** |
| **1 TB → 8 TB** | 2× M.2 4 TB (exige 2 slots livres) | **$800–1.200** |
| **Pacote completo** | 64 GB + 8 TB | **$1.428–1.828** |

> **O upgrade completo custa quase o mesmo que o orçamento 12 inteiro.**
>
> Causa: a crise de NAND. O preço do SSD **quintuplicou** até janeiro de 2026,
> e a DDR5 SO-DIMM está na mediana de **$9,81/GB**. A capacidade de NAND de 2026
> já está vendida para clientes de IA.

### Três conclusões práticas

1. **Compare o upgrade de fábrica com o aftermarket.** O orçamento 08 cobra
   **+$403** por 1 TB extra; um SSD avulso de 1 TB custa ~$190 — mas a
   diferença cobre garantia e não abrir a máquina. Para 4 TB, a conta inverte:
   de fábrica costuma ser mais caro.
2. **Priorize chassi de 18"** se 8 TB for meta firme — eles têm **3 slots M.2**
   (orçamentos 01, 05, 09, 17), os de 16" têm 2.
3. **Repense os 8 TB.** Um HDD externo USB de 8 TB custa **~$180** e resolve
   biblioteca de jogos e arquivo. Reserve o NVMe interno para o sistema, os
   mundos de Minecraft e os modelos de IA — que é onde a velocidade importa.
   **Economia: ~$800.**

---

# 8. Custo total de propriedade — 4 anos

Comparando os três finalistas:

| Item | 12 · TUF A16 | **06 · Legion Pro 7** | 01 · MSI Raider |
|---|---:|---:|---:|
| Custo final | $1.840 | **$2.627** | $6.494 |
| Garantia estendida (2 anos) | $129 | **$129** | $249 |
| Upgrade realista (RAM ou SSD) | $314 | **$400** | $0 (já vem) |
| Energia (4 anos, uso médio) | $88 | **$120** | $160 |
| Bateria de reposição (ano 3) | $130 | **$150** | $180 |
| **TCO 4 anos** | **$2.501** | **$3.426** | **$7.083** |
| **Por mês** | **$52** | **$71** | **$148** |

---

# 9. Matriz de decisão — os 5 finalistas

Pesos: Hermes 36B ×3 · AMD ×2 · Expansível ×3 · ATM10 ×2 · Preço ×3

| Critério | 12 · TUF A16 | **06 · Legion** | 07 · Legion 64 GB | 01 · MSI Raider | 04 · SCAR 18 |
|---|:-:|:-:|:-:|:-:|:-:|
| Hermes 36B (×3) | 0 | 6 | 7 | **10** | **10** |
| CPU AMD (×2) | **10** | **10** | **10** | **10** | 0 |
| RAM expansível (×3) | **10** | **10** | 8 | **10** | **10** |
| ATM10 (×2) | 8 | **10** | **10** | **10** | 9 |
| Preço (×3) | **10** | 8 | 6 | 2 | 3 |
| **PONTUAÇÃO** | **101** | **📊 116** | 107 | 102 | 81 |

**O Legion Pro 7 Gen 10 AMD (orçamento 06) vence** por equilibrar os cinco
critérios sem zerar nenhum. O TUF A16 perde só no Hermes; o MSI Raider ganha em
tudo menos preço, e o preço tem peso 3.

---

# 10. 💡 A jogada que muda a conta

O seu [orçamento da base móvel](../base-movel/ORCAMENTO-2026.md) prevê um
**SERVER-02 de IA** com GPUs de verdade. Se ele existir:

| | Notebook roda o Hermes local | Notebook é terminal do servidor |
|---|---|---|
| Modelo acessível | Hermes 4.3 **36B** | Hermes 4 **70B** |
| Notebook necessário | Orçamento 01 — **$6.494** | Orçamento 12 — **$1.840** |
| Diferença | | **$4.654 no bolso** |

Um notebook barato conectado por [Tailscale](https://tailscale.com/) ao nó de
IA te dá acesso a um modelo que **nenhum notebook do mundo roda**. A GPU tem
energia e refrigeração de verdade no rack, não no seu colo.

**Só compre um notebook de $6.494 se a IA precisar rodar offline, longe do
servidor.** Caso contrário, o orçamento 12 mais uma RTX 5090 no rack custa
menos que o orçamento 01 sozinho.

---

# 11. Checklist de compra

### Antes de comprar

- [ ] Registrar **preço final de hoje** do modelo escolhido, com loja e data
- [ ] Conferir preço em **pelo menos 3 lojas** (Best Buy, Newegg, Amazon, B&H, Micro Center)
- [ ] Checar [camelcamelcamel](https://camelcamelcamel.com/) e [Slickdeals](https://slickdeals.net/) para histórico
- [ ] Confirmar **número de slots M.2 e de RAM** na ficha técnica oficial, não no anúncio
- [ ] Confirmar se a RAM é **SO-DIMM ou soldada** — os orçamentos 02, 03 e 16 são soldados
- [ ] Verificar a **política de devolução** (Best Buy 15 dias · Amazon 30 · Micro Center 15)
- [ ] Decidir sobre garantia estendida (custo na §8)
- [ ] Se for esperar: marcar **23–29 de novembro** no calendário

### No ato da compra

- [ ] Pagar com cartão que **dobra a garantia** (muitos Visa/Mastercard fazem)
- [ ] Guardar a nota fiscal em PDF **no NAS e fora dele**
- [ ] Registrar o número de série
- [ ] Recusar seguros de loja que dupliquem a garantia do cartão

### Na chegada — nas primeiras 48 h

- [ ] Rodar `memtest86` a noite inteira — memória com defeito é o problema nº 1
- [ ] Testar **todas** as portas USB, HDMI e o leitor de cartão
- [ ] Teste de pixel morto na tela (fundo branco, preto, RGB)
- [ ] Teste de estresse 30 min (FurMark + Prime95) medindo temperatura
- [ ] Confirmar VRAM, RAM e SSD reais no sistema (não confie na caixa)
- [ ] Instalar o ATM10 com os 200 mods e rodar **1 h** — é o seu teste real
- [ ] Baixar o [Hermes Desktop](https://nousresearch.com/) e medir tok/s
- [ ] **Se algo falhar, devolver dentro da janela.** Não tente consertar.

---

# 12. Riscos

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| Preço subir antes de dezembro | **Alta** (fabricantes confirmaram +15–20%) | Alto | Comprar contra a linha de base registrada hoje |
| Desconto de Black Friday decepcionar | **Alta** (só 8–13% em RTX 50) | Médio | Não contar com ele; tratar como bônus |
| Estoque esgotar do modelo escolhido | Média | Médio | Ter 2ª e 3ª opções definidas (matriz da §9) |
| RAM vir soldada sem estar no anúncio | Média | **Alto** | Conferir ficha oficial antes de pagar |
| Upgrade de 8 TB custar $1.200 | **Certa** | Alto | HDD externo de 8 TB por $180 (§7) |
| Hermes 36B ficar lento por offload | Alta no Tier B | Médio | Usar o servidor de IA (§10) ou modelo menor |

---

# 13. Fontes

| Dado | Fonte |
|---|---|
| Black Friday 2026 = 27/11/2026 · Black Week 23–29/11 | [AwarenessDays](https://www.awarenessdays.com/awareness-days-calendar/black-friday/) · [ExpertSender](https://expertsender.com/blog/when-is-black-friday/) |
| Desconto esperado: RTX 50 em 8–13%, RTX 40/30 em 15–20% | [PriceDropDeals](https://www.pricedropdeals.com/2026/04/24/prime-day-gaming-laptop-deals-2026/) |
| Fabricantes confirmaram aumento de 15–20% (até 30% no topo) | [Newegg Insider](https://www.newegg.com/insider/why-gaming-laptop-prices-are-rising-in-2026-and-why-waiting-will-cost-you/) |
| SO-DIMM DDR5: mediana $9,81/GB | [RAM Prices USA](https://rampricesusa.com/32gb-ram-prices) |
| NAND quintuplicou até jan/2026; SSD $100–150/TB | [Tom's Hardware — rastreador de SSD](https://www.tomshardware.com/pc-components/ssds/ssd-price-tracking-2026-lowest-price-on-every-m-2-ssd) · [Storage Disk Prices](https://storagediskprices.com/ssd-price-history/) |
| Kit DDR5 32 GB desktop a $392 (referência de mercado) | [Tom's Hardware — rastreador de RAM](https://www.tomshardware.com/pc-components/ram/ram-price-index-2026-lowest-price-on-ddr5-and-ddr4-memory-of-all-capacities) |
| Hermes 4.3 36B: Q4_K_M ~21,8 GB, ideal ≥33 GB | [WillItRunAI](https://willitrunai.com/models/hf-nousresearch--hermes-4-3-36b-gguf) · [LocalAIMaster](https://localaimaster.com/blog/hermes-agent-ollama) |
| Raider A18 é o único com 9955HX3D + RTX 5090 | [UltrabookReview](https://www.ultrabookreview.com/70461-amd-fire-range-laptops/) |
| Razer Blade 16: $4.499,99 / $4.899,99, LPDDR5X soldada | [Amazon](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLFFLK8) · [Tom's Hardware](https://www.tomshardware.com/laptops/gaming-laptops/razer-blade-16-review) |
| Legion Pro 7 Gen 10 AMD a partir de $2.399 | [Notebookcheck](https://www.notebookcheck.net/New-Lenovo-Legion-Pro-7-gaming-laptop-debuts-with-up-to-Ryzen-9-9955HX3D-and-RTX-5080.1104908.0.html) |
| ASUS TUF A16 a $1.699,99 | [Slickdeals](https://slickdeals.net/f/18354607-asus-tuf-a16-2025-16-fhd-165hz-ryzen-9-270-rtx-5070-32gb-ddr5-1tb-ssd-1699-99) |
| Acer Nitro V16 $1.487,99 · ASUS TUF 18 $1.499,99 | [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/) |

---

## 14. Premissas e limites

1. **Imposto do Texas a 8,25%** (6,25% estadual + 2% municipal máximo). Sua
   cidade pode cobrar menos — confirme.
2. **Frete $0** assumido; a maioria das grandes lojas não cobra acima de $99.
   Micro Center exige retirada na loja para alguns preços.
3. Preços marcados **≈ são faixa de segmento**, não cotação de listagem.
   **Reconfira antes de pagar** — hardware em 2026 muda de preço em dias.
4. **Nenhum link é de afiliado**, conforme a regra do repositório.
5. **Não cobre:** mochila, mouse, monitor externo, dock, software, ou
   importação para fora dos EUA.
