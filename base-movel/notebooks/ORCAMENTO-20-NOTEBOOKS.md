# 20 Orçamentos de Notebook — Hermes, IA local e ATM10

> 🛒 **Vai comprar de verdade?** Use o [Dossiê de Compra](../../compra-notebook/ORCAMENTO-20-NOTEBOOKS-COMPRA.md) — os mesmos 20 modelos com custo final (imposto + frete), garantia, timing de
> Black Friday e matriz de decisão. Este documento aqui é a **seleção técnica**.
>
> **Cotação de 15 de setembro de 2026.** Preços em USD, mercado dos EUA.
> Preço de notebook muda todo dia e varia por loja — **o link é mais importante
> que o número**. Onde o preço veio de uma listagem específica, está marcado
> com 🔗; onde é faixa observada no segmento, está marcado com ≈.

---

## 1. O que significa "rodar o Hermes" — a conta que decide tudo

O [Hermes](https://nousresearch.com/) da Nous Research não é um modelo só; é
uma família, e a versão muda o requisito de hardware por um fator de quatro.

| Modelo | Quantização | VRAM necessária | Cabe em notebook? |
|---|---|---:|---|
| Hermes 3 8B | Q4_K_M (~4,9 GB) | 8 GB | ✅ Qualquer RTX moderna |
| Hermes 3 8B | BF16 (full) | 24 GB | ✅ Só RTX 5090 |
| **Hermes 4.3 36B** | **Q4_K_M (~21,8 GB)** | **24 GB** | ⚠️ **Só RTX 5090 laptop** |
| Hermes 4.3 36B | ideal (folga) | 33 GB | ❌ Nenhum notebook |
| Hermes 4 70B | Q4 | ~40 GB | ❌ Nenhum notebook |
| Hermes 4 70B | full | ~80 GB (A100/H100) | ❌ |

**O corte é em 24 GB de VRAM, e só a RTX 5090 mobile tem isso.** A RTX 5090 de
notebook traz **24 GB GDDR7** — o maior frame buffer de qualquer GPU móvel. É
literalmente a única porta de entrada para o Hermes 4.3 36B rodando inteiro na
placa.

### E com 16 GB (RTX 5080)?

Roda, mas por **offload**: parte do modelo fica na VRAM, parte na RAM do
sistema. Funciona, e é usável — mas a velocidade cai de forma acentuada.

| Configuração | Velocidade aproximada | Experiência |
|---|---|---|
| 24 GB VRAM (5090), modelo inteiro na placa | 30–45 tok/s | Conversa fluida |
| 16 GB VRAM (5080) + 64 GB RAM, offload parcial | 6–10 tok/s | Usável, com paciência |
| 16 GB VRAM + 32 GB RAM | 4–7 tok/s | Limítrofe |
| 12 GB ou menos | — | ❌ Só Hermes 3 8B |

Por isso o **Bloco A** inclui tanto RTX 5090 (nativo) quanto RTX 5080 com
64 GB de RAM (offload viável). O **Bloco B** é tudo que fica abaixo disso.

> **Setup local:** o [Hermes Desktop](https://www.marktechpost.com/2026/09/05/nous-research-hermes-desktop-one-click-local-model-setup/)
> é MIT, gratuito, roda em Windows/Linux/macOS e desde setembro/2026 tem
> instalação de modelo local em um clique. Não precisa de conta para modelos
> locais.

---

## 2. Seus requisitos, traduzidos em checklist

| # | O que você pediu | Critério objetivo |
|---|---|---|
| 1 | 32 GB DDR5 (DDR4 aceitável) | ≥32 GB, **em slots SO-DIMM** (não soldada) |
| 2 | 1 TB + espaço para expandir até ~8 TB | ≥1 TB instalado + **2 slots M.2 livres ou 1 livre** |
| 3 | AMD Ryzen 7 ou 9 | AMD preferencial; Intel só onde AMD não existe |
| 4 | RTX (ferramentas NVIDIA + IA + jogos) | GeForce RTX série 50 |
| 5 | Roda ATM10 com 200 mods | ≥32 GB RAM, CPU forte em thread única |
| 6 | Roda Hermes | ≥24 GB VRAM (ideal) ou 16 GB + 64 GB RAM |

### ⚠️ A armadilha nº 1: memória soldada

**O Razer Blade 16 usa LPDDR5X-8000 soldada na placa.** Você escolhe 32 ou
64 GB **na compra e nunca mais muda**. É o notebook mais bonito da lista e
falha o seu requisito nº 1. Está incluído porque a combinação AMD + RTX 5090
é rara — mas com o aviso em destaque.

### A armadilha nº 2: chegar aos 8 TB

M.2 2280 de 8 TB num único pente existe, mas é caro e raro. O caminho prático
é **2× 4 TB**. Isso exige **dois slots M.2** — e a maioria dos notebooks de
16" tem exatamente dois (um ocupado de fábrica). Modelos de 18" costumam ter
**três**. Se 8 TB interno é meta firme, **priorize chassi de 18"**.

---

## 3. BLOCO A — Os 10 que rodam o Hermes

### A1 ⭐ MSI Raider A18 HX (A9WJG-052US) — a combinação que quase não existe

| | |
|---|---|
| CPU | **AMD Ryzen 9 9955HX3D** (16C/32T, 3D V-Cache) |
| GPU | **RTX 5090 24 GB** GDDR7 |
| RAM | 64 GB DDR5-5600 (SO-DIMM, expansível) |
| Armazenamento | 2 TB · chassi 18" com 3 slots M.2 |
| Tela | 18" UHD+ Mini LED 120 Hz |
| **Preço** | ≈ **$5.500 – $7.400** |
| Hermes | ✅ **4.3 36B nativo na VRAM** |
| ATM10 | ✅ O 3D V-Cache é o melhor CPU de notebook para Minecraft |

**É o único notebook do mercado com Ryzen 9 9955HX3D + RTX 5090.** Se o
requisito "AMD + Hermes grande + RAM expansível" é inegociável, a lista tem
um item só, e é este. O 3D V-Cache do 9955HX3D é exatamente o que o Minecraft
modded aproveita — cache grande resolve o gargalo de chunk tick melhor que
clock alto.

🔗 [Newegg](https://www.newegg.com/msi-18-geforce-rtx-5090-laptop-gpu-amd-ryzen-9-9955hx3d-64gb-ddr5-5600mhz-memory-2-tb-ssd/p/N82E16834156732) ·
[Best Buy](https://www.bestbuy.com/site/6621615.p?skuId=6621615) ·
[Target](https://www.target.com/p/-/A-1007161544) ·
[Adorama](https://www.adorama.com/msi-raider-a18-hx-gaming-laptop-ryzen-9-rtx-5090-64gb-ram-2tb-ssd/p/msi18hx052) ·
[Excaliber PC](https://www.excaliberpc.com/810967/msi-raider-a18-hx-a9wjg-052us.html)

---

### A2 Razer Blade 16 (2025) — AMD + 5090, mas RAM soldada

| | |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 (12C/24T, **28 W**) |
| GPU | RTX 5090 24 GB |
| RAM | 64 GB LPDDR5X-8000 — ⚠️ **SOLDADA** |
| Armazenamento | 4 TB · 16" |
| **Preço** | 🔗 **$4.899,99** (64 GB/4 TB) |
| Hermes | ✅ 4.3 36B nativo |
| ATM10 | ⚠️ CPU de 28 W — bom, não ótimo, para servidor local |

🔗 [Amazon (64 GB/4 TB)](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLX9VQQ) ·
[Razer oficial](https://www.razer.com/gaming-laptops/razer-blade-16) ·
[review Tom's Hardware](https://www.tomshardware.com/laptops/gaming-laptops/razer-blade-16-review)

---

### A3 Razer Blade 16 (2025) — versão 32 GB

| | |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 · GPU RTX 5090 24 GB |
| RAM | 32 GB LPDDR5X-8000 — ⚠️ soldada, **sem upgrade futuro** |
| Armazenamento | 2 TB · 16" |
| **Preço** | 🔗 **$4.499,99** |
| Hermes | ✅ 4.3 36B nativo (a VRAM é a mesma) |

🔗 [Amazon](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLFFLK8) ·
[Notebookcheck (preços de lançamento a partir de $2.799)](https://www.notebookcheck.net/Razer-Blade-16-2025-pre-orders-open-starting-at-2-799-with-up-to-AMD-Ryzen-AI-9-HX-370-RTX-5090-and-64-GB-of-RAM.967573.0.html)

---

### A4 Lenovo Legion Pro 7 Gen 10 AMD (16ARX10) — 64 GB

| | |
|---|---|
| CPU | **AMD Ryzen 9 9955HX3D** · GPU RTX 5080 **16 GB** |
| RAM | **64 GB DDR5 SO-DIMM** (expansível) |
| Armazenamento | 1 TB Gen5 · 2 slots M.2 |
| Tela | 16" WQXGA OLED 240 Hz |
| **Preço** | ≈ **$2.900 – $3.400** |
| Hermes | ⚠️ 4.3 36B **com offload** (6–10 tok/s) · 8B nativo folgado |
| ATM10 | ✅ Excelente (3D V-Cache + 64 GB) |

**Melhor custo-benefício do Bloco A.** Metade do preço do A1, com o mesmo CPU.
O que você perde é a VRAM: 16 GB em vez de 24 GB, o que empurra o Hermes 36B
para offload.

🔗 [Amazon](https://www.amazon.com/clp/B0GNDNY4CM) ·
[Notebookcheck](https://www.notebookcheck.net/New-Lenovo-Legion-Pro-7-gaming-laptop-debuts-with-up-to-Ryzen-9-9955HX3D-and-RTX-5080.1104908.0.html)

---

### A5 Lenovo Legion Pro 7 Gen 10 AMD — 32 GB / 1 TB

| | |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · GPU RTX 5080 16 GB |
| RAM | 32 GB DDR5 (expansível a 64/96 GB) · 1 TB |
| **Preço** | 🔗 **$2.427,24** (oferta rastreada) · a partir de **$2.399** |
| Hermes | ⚠️ 36B com offload lento · 8B ✅ |
| ATM10 | ✅ |

🔗 [Slickdeals ($2.427,24)](https://slickdeals.net/f/18887428-legion-pro-7-gen-10-16-qhd-240hz-oled-ryzen-9-9955hx3d-rtx-5080-32gb-ddr5-1tb-ssd-2427-24) ·
[Micro Center](https://www.microcenter.com/product/703054/lenovo-legion-7-pro-oled-16-gaming-laptop-computer-eclipse-black)

---

### A6 Lenovo Legion Pro 7 Gen 10 AMD — 32 GB / 2 TB

| | |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · GPU RTX 5080 16 GB · 32 GB · **2 TB** |
| **Preço** | ≈ **$2.700 – $2.950** |
| Hermes | ⚠️ 36B com offload · 8B ✅ |

🔗 [Amazon](https://www.amazon.com/Legion-9955HX3D-NVIDIA-GeForce-Graphics/dp/B0FTVPVVFW) ·
[eBay](https://www.ebay.com/itm/326818335306)

---

### A7 MSI Raider A18 HX (A9WIG-082US) — 18" AMD com 5080

| | |
|---|---|
| CPU | AMD Ryzen 9 9955HX3D · GPU RTX 5080 16 GB |
| RAM | 64 GB DDR5 · 2 TB · **3 slots M.2** (caminho fácil para 8 TB) |
| **Preço** | 🔗 **$7.399** (era $7.999) |
| Hermes | ⚠️ 36B com offload (64 GB de RAM ajudam) |
| ATM10 | ✅ |

Caro para o que entrega — está na lista porque o chassi de 18" com **3 slots
M.2** é o caminho mais limpo para os 8 TB que você quer no futuro.

🔗 [MSI Store oficial](https://us-store.msi.com/msi-Raider-A18-HX-A9WIG-082US) ·
[NVIDIA Marketplace](https://marketplace.nvidia.com/en-au/consumer/gaming-laptops/msi-raider-a18-hx-a9w-18-120hz-uhd-gaming-laptop-ryzen-9-9955hx3d-64gb-2tb-rtx5080-w11p)

---

### A8 ASUS ROG Strix SCAR 18 (2026) — Intel

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX (**não é AMD**) |
| GPU | RTX 5090 24 GB · RAM 32 GB DDR5 (expansível) · 2 TB |
| **Preço** | ≈ **$4.500 – $5.200** |
| Hermes | ✅ 4.3 36B nativo |
| ATM10 | ✅ |

🔗 [PC Guide — melhores RTX 5090](https://www.pcguide.com/laptop/guide/best-rtx-5090/) ·
[B&H](https://www.bhphotovideo.com/c/buy/rtx-5090-laptops/ci/60263)

---

### A9 Lenovo Legion Pro 7i Gen 10 — Intel, 5080

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX · GPU RTX 5080 16 GB |
| RAM | 32 GB DDR5 · 1 TB · 16" OLED 240 Hz |
| **Preço** | ≈ **$2.400 – $3.000** |
| Hermes | ⚠️ 36B com offload |

🔗 [Newegg Insider — melhores RTX 50](https://www.newegg.com/insider/best-rtx-50-series-gaming-laptops-2026-which-one-is-right-for-you/)

---

### A10 Alienware 18 Area-51 / Acer Predator Helios 18P — 18" com 5090

| | |
|---|---|
| CPU | Intel Core Ultra 9 275HX · GPU RTX 5090 24 GB |
| RAM | 32–64 GB DDR5 · 2 TB · 18", 3 slots M.2 |
| **Preço** | ≈ **$4.800 – $6.000** |
| Hermes | ✅ 4.3 36B nativo |

🔗 [Dell](https://www.dell.com/en-us/shop/dell-laptops/scr/laptops/appref=nvidia-geforce-rtx-5090-video) ·
[Best Buy — RTX 5090](https://www.bestbuy.com/site/searchpage.jsp?browsedCategory=pcmcat287600050003&id=pcat17071&qp=graphicscardsv_facet%3DVideo+Card%7ENVIDIA+GeForce+RTX+5090&st=categoryid%24pcmcat287600050003)

---

## 4. BLOCO B — Os 10 que NÃO rodam o Hermes grande

Todos aqui rodam **Hermes 3 8B** sem dificuldade (4,9 GB em Q4_K_M) e rodam
**ATM10 com 200 mods** bem. O que não fazem é o Hermes 4.3 36B.

### B1 ASUS ROG Strix G16 — Ryzen 9 9955HX + RTX 5070

| | |
|---|---|
| CPU | **AMD Ryzen 9 9955HX** · GPU RTX 5070 **8 GB** |
| RAM | 32 GB DDR5-5600 (2 slots) · 1 TB · 16" 240 Hz |
| **Preço** | ≈ **$1.500 – $1.800** |
| Hermes | ❌ 36B · ✅ 8B em Q4 |
| ATM10 | ✅ |

🔗 [Newegg](https://www.newegg.com/asus-rog-strix-g16-16-geforce-rtx-5070-laptop-gpu-amd-ryzen-9-9955hx-32gb-memory-1-tb-pcie-ssd/p/N82E16834236630)

---

### B2 ASUS ROG Strix G16 G614PR — Ryzen 9 8940HX + RTX 5070 Ti

| | |
|---|---|
| CPU | AMD Ryzen 9 8940HX · GPU RTX 5070 Ti **12 GB** |
| RAM | 32 GB DDR5 · 1 TB |
| **Preço** | ≈ **$1.900 – $2.200** |
| Hermes | ❌ 36B · ✅ 8B folgado, 14B em Q4 |
| ATM10 | ✅ |

🔗 [Newegg / HIDevolution](https://www.newegg.com/asus-rog-strix-g16-16-0-geforce-rtx-5070ti-laptop-gpu-amd-ryzen-9-8940hx-fhd-32gb-memory-1-tb-pcie-ssd/p/2WC-000N-0G0J8) ·
[ROG oficial](https://rog.asus.com/laptops/rog-strix/rog-strix-g16-2025-g614/)

---

### B3 ASUS ROG Strix G16 — Ryzen 9 8940HX + 5070 Ti + 2 TB

| | |
|---|---|
| CPU | AMD Ryzen 9 8940HX · RTX 5070 Ti 12 GB · 32 GB · **2 TB** |
| **Preço** | ≈ **$2.000 – $2.300** |
| Hermes | ❌ 36B · ✅ 8B |

🔗 [Walmart](https://www.walmart.com/ip/17871308188) ·
[Amazon](https://www.amazon.com/ASUS-1900x1200-Display-Keyboard-Accessories/dp/B0FPF28L6X)

---

### B4 ASUS TUF Gaming A16 — Ryzen 9 270 + RTX 5070 ⭐ melhor custo

| | |
|---|---|
| CPU | **AMD Ryzen 9 270** · GPU RTX 5070 8 GB |
| RAM | 32 GB DDR5 (2 slots, até 64 GB) · 1 TB · **até 4 TB expansível** |
| **Preço** | 🔗 **$1.699,99** |
| Hermes | ❌ 36B · ✅ 8B |
| ATM10 | ✅ |

**O melhor custo-benefício da lista inteira.** Se o Hermes grande sair do
requisito, é aqui que o dinheiro rende mais.

🔗 [Slickdeals / Best Buy ($1.699,99)](https://slickdeals.net/f/18354607-asus-tuf-a16-2025-16-fhd-165hz-ryzen-9-270-rtx-5070-32gb-ddr5-1tb-ssd-1699-99) ·
[ASUS oficial](https://www.asus.com/us/laptops/for-gaming/tuf-gaming/asus-tuf-gaming-a16-2025/)

---

### B5 ASUS TUF Gaming A16 — Ryzen 7 260 + RTX 5060

| | |
|---|---|
| CPU | AMD Ryzen 7 260 · GPU RTX 5060 8 GB · 32 GB DDR5 · 1 TB |
| **Preço** | ≈ **$1.250 – $1.450** |
| Hermes | ❌ 36B · ✅ 8B |
| ATM10 | ✅ (a GPU não é o gargalo do Minecraft) |

🔗 [Best Buy](https://www.bestbuy.com/product/asus-tuf-a16-rtx-5060-ryzen-7-32gb-1tb-ssd-165hz-gaming-laptop/JJGGLH8TZY) ·
[Newegg](https://www.newegg.com/asus-tuf-gaming-a16-16-geforce-rtx-5060-laptop-gpu-amd-ryzen-7-260-wqxga-32gb-memory-1-tb-pcie-ssd/p/N82E16834236636)

---

### B6 ASUS TUF Gaming 18 — chassi grande, preço de entrada

| | |
|---|---|
| CPU | Intel/AMD conforme SKU · GPU RTX 5060/5070 · 32 GB · 1 TB |
| Tela | **18"** — mais espaço interno, melhor refrigeração, mais slots M.2 |
| **Preço** | 🔗 **$1.499,99** |
| Hermes | ❌ 36B · ✅ 8B |

🔗 [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/)

---

### B7 Acer Nitro V16 — Ryzen 7 + RTX 5060

| | |
|---|---|
| CPU | AMD Ryzen 7 · GPU RTX 5060 8 GB · 32 GB · 1 TB |
| **Preço** | 🔗 **$1.487,99** |
| Hermes | ❌ 36B · ✅ 8B |

🔗 [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/)

---

### B8 Lenovo LOQ 15/16 Gen 10 — AMD, o mais barato que ainda serve

| | |
|---|---|
| CPU | AMD Ryzen 7 · GPU RTX 5060 8 GB · 32 GB (upgrade) · 1 TB |
| **Preço** | ≈ **$1.100 – $1.350** |
| Hermes | ❌ 36B · ✅ 8B |
| ATM10 | ✅ com 32 GB (vem com 16 GB — **faça o upgrade**) |

🔗 [ASUS TUF vs LOQ — comparativo](https://www.newegg.com/insider/best-rtx-50-series-gaming-laptops-2026-which-one-is-right-for-you/)

---

### B9 ASUS ROG Zephyrus G16 — Ryzen AI 9 HX 370, portátil

| | |
|---|---|
| CPU | AMD Ryzen AI 9 HX 370 · GPU RTX 5070/5080 · 32 GB LPDDR5X ⚠️ soldada |
| **Preço** | ≈ **$2.000 – $2.800** |
| Hermes | ⚠️ Só na versão 5080, com offload |
| Nota | O mais leve da lista (1,95 kg) — **e o que menos expande** |

🔗 [PC Guide](https://www.pcguide.com/laptop/guide/best-rtx-5090/)

---

### B10 HP Omen Max 16 — Ryzen AI 9 HX 475 + RTX 5080

| | |
|---|---|
| CPU | AMD Ryzen AI 9 HX 475 · GPU RTX 5080 16 GB · 32 GB · 1 TB |
| Tela | 16" 2.5K OLED 240 Hz |
| **Preço** | ≈ **$2.600 – $3.200** |
| Hermes | ⚠️ 36B com offload (RAM 32 GB limita) · ✅ 8B/14B |

🔗 [Best Buy — AMD Ryzen 9 + NVIDIA](https://www.bestbuy.com/site/searchpage.jsp?browsedCategory=abcat0500000&id=pcat17071&qp=gpubrand_facet%3DGPU+Brand%7ENVIDIA%5Eparent_processormodelsv_facet%3DAMD+Ryzen+9%7EAMD+Ryzen+9&st=categoryid%24abcat0500000)

---

## 5. Tabela comparativa — os 20

| # | Modelo | CPU | GPU / VRAM | RAM | SSD | Preço | Hermes 36B | RAM expansível |
|---|---|---|---|---|---|---:|:---:|:---:|
| **A1** ⭐ | MSI Raider A18 HX (5090) | R9 9955HX3D | 5090 / 24 GB | 64 GB | 2 TB | $5.500–7.400 | ✅ nativo | ✅ |
| A2 | Razer Blade 16 64 GB | AI 9 HX 370 | 5090 / 24 GB | 64 GB | 4 TB | $4.899 | ✅ nativo | ❌ soldada |
| A3 | Razer Blade 16 32 GB | AI 9 HX 370 | 5090 / 24 GB | 32 GB | 2 TB | $4.499 | ✅ nativo | ❌ soldada |
| **A4** ⭐ | Legion Pro 7 G10 AMD 64 GB | R9 9955HX3D | 5080 / 16 GB | 64 GB | 1 TB | $2.900–3.400 | ⚠️ offload | ✅ |
| A5 | Legion Pro 7 G10 AMD 32 GB | R9 9955HX3D | 5080 / 16 GB | 32 GB | 1 TB | $2.427 | ⚠️ offload | ✅ |
| A6 | Legion Pro 7 G10 AMD 2 TB | R9 9955HX3D | 5080 / 16 GB | 32 GB | 2 TB | $2.700–2.950 | ⚠️ offload | ✅ |
| A7 | MSI Raider A18 HX (5080) | R9 9955HX3D | 5080 / 16 GB | 64 GB | 2 TB | $7.399 | ⚠️ offload | ✅ 3× M.2 |
| A8 | ROG Strix SCAR 18 | Ultra 9 275HX | 5090 / 24 GB | 32 GB | 2 TB | $4.500–5.200 | ✅ nativo | ✅ |
| A9 | Legion Pro 7i G10 | Ultra 9 275HX | 5080 / 16 GB | 32 GB | 1 TB | $2.400–3.000 | ⚠️ offload | ✅ |
| A10 | Alienware 18 / Helios 18P | Ultra 9 275HX | 5090 / 24 GB | 32–64 GB | 2 TB | $4.800–6.000 | ✅ nativo | ✅ 3× M.2 |
| B1 | ROG Strix G16 (5070) | R9 9955HX | 5070 / 8 GB | 32 GB | 1 TB | $1.500–1.800 | ❌ | ✅ |
| B2 | ROG Strix G16 (5070 Ti) | R9 8940HX | 5070 Ti / 12 GB | 32 GB | 1 TB | $1.900–2.200 | ❌ | ✅ |
| B3 | ROG Strix G16 (5070 Ti 2 TB) | R9 8940HX | 5070 Ti / 12 GB | 32 GB | 2 TB | $2.000–2.300 | ❌ | ✅ |
| **B4** ⭐ | ASUS TUF A16 (5070) | R9 270 | 5070 / 8 GB | 32 GB | 1 TB | **$1.699** | ❌ | ✅ |
| B5 | ASUS TUF A16 (5060) | R7 260 | 5060 / 8 GB | 32 GB | 1 TB | $1.250–1.450 | ❌ | ✅ |
| B6 | ASUS TUF 18 | conforme SKU | 5060/5070 | 32 GB | 1 TB | $1.499 | ❌ | ✅ |
| B7 | Acer Nitro V16 | Ryzen 7 | 5060 / 8 GB | 32 GB | 1 TB | $1.487 | ❌ | ✅ |
| B8 | Lenovo LOQ Gen 10 | Ryzen 7 | 5060 / 8 GB | 32 GB* | 1 TB | $1.100–1.350 | ❌ | ✅ |
| B9 | ROG Zephyrus G16 | AI 9 HX 370 | 5070/5080 | 32 GB | 1 TB | $2.000–2.800 | ⚠️ | ❌ soldada |
| B10 | HP Omen Max 16 | AI 9 HX 475 | 5080 / 16 GB | 32 GB | 1 TB | $2.600–3.200 | ⚠️ offload | ✅ |

\* LOQ costuma vir com 16 GB — orçar upgrade para 32 GB (+$390 aos preços de 2026).

---

## 6. Recomendação

### Se "roda Hermes 36B + AMD + expansível" é inegociável
**A1 — MSI Raider A18 HX com RTX 5090 e Ryzen 9 9955HX3D.**
É o único do mercado que fecha os três requisitos ao mesmo tempo. Caro, pesado
(18"), e sem substituto.

### Se o melhor equilíbrio importa mais que o modelo grande
**A4 — Legion Pro 7 Gen 10 AMD com 64 GB.**
Mesmo CPU do A1, metade do preço. Roda o Hermes 36B com offload (lento mas
usável), roda o 8B/14B instantâneo, destrói o ATM10, e tem RAM e SSD
expansíveis. **É a que eu compraria.**

### Se o orçamento manda
**B4 — ASUS TUF A16 com Ryzen 9 270 e RTX 5070, por $1.699.**
Não roda o Hermes grande. Roda tudo o mais que você listou, por menos de um
terço do A1. Com o dinheiro economizado dá para comprar **três** e ainda sobra.

> 💡 **A jogada mais inteligente:** o notebook **não precisa** rodar o Hermes
> grande, porque o SERVER-02 da base móvel roda. Um B4 de $1.699 conectado por
> [Tailscale](https://tailscale.com/) ao nó de IA te dá o Hermes 4 **70B** —
> que nenhum notebook do mundo roda — pela rede. O notebook vira terminal, e
> os $3.800 de diferença viram outra RTX 5090 no rack, onde ela tem energia e
> refrigeração de verdade.

---

## 7. Notas sobre ATM10 com 200 mods

| Fator | Importa? | Detalhe |
|---|---|---|
| RAM | 🔴 **Crítico** | ATM10 base pede 8–10 GB de heap; +200 mods empurra para **12–16 GB**. Com o SO e o resto, **32 GB é o piso**, 64 GB é confortável. |
| CPU thread única | 🔴 **Crítico** | Minecraft é single-thread no tick principal. Cache grande vence clock — por isso o **9955HX3D (3D V-Cache)** é o melhor da lista. |
| GPU | 🟡 Médio | Só importa com shaders. Sem shaders, uma RTX 5060 roda igual a uma 5090. |
| SSD | 🟡 Médio | Carregamento de mundo e chunk. NVMe Gen4 já resolve. |
| Refrigeração | 🟠 Alto | Sessão de 6 h com 200 mods é carga sustentada. Chassi de 18" sofre menos. |

**Ajuste de JVM recomendado** (Java 21+, com [Aikar's flags](https://docs.papermc.io/paper/aikars-flags)):
```
-Xms12G -Xmx12G -XX:+UseG1GC -XX:+ParallelRefProcEnabled
-XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions
-XX:+DisableExplicitGC -XX:G1NewSizePercent=30
```
Não dê mais heap do que precisa: GC de 24 GB pausa mais que GC de 12 GB.

---

## 8. Fontes

| Dado | Fonte |
|---|---|
| Hermes 4.3 36B — Q4_K_M ~21,8 GB, ideal ≥33 GB | [WillItRunAI](https://willitrunai.com/models/hf-nousresearch--hermes-4-3-36b-gguf) · [LocalAIMaster](https://localaimaster.com/blog/hermes-agent-ollama) |
| Hermes 3 8B — 16 GB @ Q4, 24 GB @ BF16; 70B ~40 GB @ Q4 | [Fast.io — guia Hermes 3](https://fast.io/resources/hermes-3-model-guide/) · [Hardware Corner](https://www.hardware-corner.net/llm-database/Nous-Hermes/) |
| Hermes 4 70B — ~80 GB (A100/H100) | [Daniel Norin](https://danielnorin.com/en/ai-en/ai-models/hermes-4-ai-model-without-direct-censorship) |
| Hermes Desktop — MIT, setup local em 1 clique (set/2026) | [MarkTechPost](https://www.marktechpost.com/2026/09/05/nous-research-hermes-desktop-one-click-local-model-setup/) |
| RTX 5090 laptop — 24 GB GDDR7, $4.788–5.588 | [Smartprix](https://us.smartprix.com/laptops/nvidia-geforce-rtx-5090-laptops-list) · [PC Guide](https://www.pcguide.com/laptop/guide/best-rtx-5090/) |
| Faixas por GPU: 5090 $3.000+, 5080 $2.000–3.400, 5070 Ti $1.800–2.500, 5070 $1.500–1.800 | [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/) |
| Raider A18 é o único com 9955HX3D + RTX 5090 | [UltrabookReview — lista Fire Range](https://www.ultrabookreview.com/70461-amd-fire-range-laptops/) |
| Razer Blade 16 — $4.499,99 (32 GB) / $4.899,99 (64 GB), LPDDR5X soldada | [Amazon](https://www.amazon.com/Razer-Blade-16-Gaming-Laptop/dp/B0DYLFFLK8) · [Tom's Hardware](https://www.tomshardware.com/laptops/gaming-laptops/razer-blade-16-review) |
| Legion Pro 7 Gen 10 AMD — a partir de $2.399 | [Notebookcheck](https://www.notebookcheck.net/New-Lenovo-Legion-Pro-7-gaming-laptop-debuts-with-up-to-Ryzen-9-9955HX3D-and-RTX-5080.1104908.0.html) |
| ASUS TUF A16 — $1.699,99 (R9 270 + 5070 + 32 GB) | [Slickdeals](https://slickdeals.net/f/18354607-asus-tuf-a16-2025-16-fhd-165hz-ryzen-9-270-rtx-5070-32gb-ddr5-1tb-ssd-1699-99) |
| Acer Nitro V16 $1.487,99 · ASUS TUF 18 $1.499,99 | [Newegg Insider](https://www.newegg.com/insider/best-gaming-laptops-in-2026-rtx-50-series-showdown-across-every-budget/) |
| Crise de RAM 2026 — kit 32 GB a ~$392 | [Tom's Hardware — rastreador de preço de RAM](https://www.tomshardware.com/pc-components/ram/ram-price-index-2026-lowest-price-on-ddr5-and-ddr4-memory-of-all-capacities) |
