# Orçamento Corporativo — Frota de 20 Estações de Trabalho

> **Proposta técnico-comercial.** Cotação de 15 de setembro de 2026, mercado
> dos EUA, preços em USD, **sem impostos e sem frete**.
>
> **Especificação pedida:** 20 unidades · AMD Ryzen 7 ou 9 · GPU NVIDIA RTX ·
> 32 GB de RAM · 1 ou 2 TB de armazenamento.

---

## 1. Resumo executivo

| | Tier 1 — Essencial | Tier 2 — Padrão | Tier 3 — Desempenho |
|---|---|---|---|
| CPU | Ryzen 7 9700X | Ryzen 7 9800X3D | Ryzen 9 9950X3D |
| GPU | RTX 5060 Ti 16 GB | RTX 5070 12 GB | RTX 5070 Ti 16 GB |
| RAM | 32 GB DDR5 | 32 GB DDR5 | 32 GB DDR5 |
| SSD | 1 TB NVMe Gen4 | 2 TB NVMe Gen4 | 2 TB NVMe Gen5 |
| **Por unidade** | **$2.028** | **$2.650** | **$3.365** |
| **20 unidades** | **$40.560** | **$53.000** | **$67.300** |
| Com periféricos e reserva | $53.916 | $68.396 | $84.836 |
| **Investimento total (CapEx)** | **$58.716** | **$73.196** | **$89.636** |
| TCO 3 anos | $77.436 | $91.916 | $108.356 |

**Recomendação: Tier 2.** O 9800X3D e a RTX 5070 cobrem desenvolvimento,
CAD, edição, containers e carga de IA leve sem folga excessiva. O Tier 3 só
se justifica se houver renderização 3D ou treino de modelo na rotina — o
salto de $14.300 na frota compra ~25% de desempenho.

---

## 2. ⚠️ Condição de mercado — leia antes de aprovar a compra

Setembro de 2026 é um momento atipicamente ruim para comprar hardware. Os
três componentes que mais pesam nesta frota subiram ao mesmo tempo, puxados
pela demanda de datacenter de IA:

| Componente | Preço de referência | Preço hoje | Variação |
|---|---|---|---|
| RTX 5070 | $549 (MSRP de lançamento) | **$899,99** | **+64%** |
| RTX 5060 Ti 16 GB | $569,99 (jun/2026) | **$804,99** | **+39% em 2 meses** |
| Kit 32 GB DDR5 | ~$85 (histórico) | **$392** | **+360%** |
| SSD NVMe 2 TB | $110–130 (2024) | **$350–480** | **+200% a +270%** |

### Quanto isso custa nesta compra específica

| | Tier 2 a preço histórico | Tier 2 hoje | Diferença |
|---|---:|---:|---:|
| Por unidade | $1.797 | $2.650 | **+$853** |
| **20 unidades** | **$35.940** | **$53.000** | **+$17.060** |

**Dezessete mil dólares** é o que a conjuntura adiciona a esta frota. Três
observações honestas para a decisão:

1. **Não há previsão de alívio antes de meados de 2027.** Os fabricantes de
   NAND confirmaram que a capacidade de produção de 2026 está *vendida*,
   majoritariamente para clientes de IA. Esperar é uma aposta com prazo longo.
2. **Esperar tem custo também.** Se as 20 estações destravam trabalho hoje,
   o custo de não tê-las supera $17 mil rapidamente. A conta é de produtividade,
   não de hardware.
3. **O meio-termo:** comprar agora as unidades que destravam trabalho imediato
   (8–12), e escalonar o restante para 2027. Reduz exposição sem parar a
   operação. Ver [seção 9](#9-estratégias-de-aquisição).

---

## 3. Composição por tier

### Tier 1 — Essencial · $2.028/unidade

Escritório, desenvolvimento web, BI, tarefas de produtividade com aceleração
gráfica. 16 GB de VRAM permitem carga de IA local leve (modelos até ~13B).

| Item | Especificação | Preço |
|---|---|---:|
| CPU | AMD Ryzen 7 9700X (8C/16T, 65 W) | $271 |
| Placa-mãe | B850M mATX, 2.5 GbE, 2× M.2 | $150 |
| Memória | 32 GB DDR5-5600 (2×16 GB) | $392 |
| GPU | NVIDIA RTX 5060 Ti **16 GB** | $680 |
| Armazenamento | 1 TB NVMe Gen4 | $190 |
| Fonte | 650 W 80+ Gold | $90 |
| Gabinete | mATX corporativo, painel frontal USB-C | $70 |
| Refrigeração | Cooler torre 120 mm | $40 |
| SO | Windows 11 Pro (OEM) | $145 |
| | **Total por unidade** | **$2.028** |
| | **× 20 unidades** | **$40.560** |

> **Por que a 5060 Ti de 16 GB e não a de 8 GB:** a diferença de preço é de
> ~$90 e a de 8 GB já estrangula cargas de CAD, edição em 4K e qualquer
> modelo de IA local. Numa frota que precisa durar 4 anos, é a economia que
> sai mais cara.

### Tier 2 — Padrão · $2.650/unidade ⭐ recomendado

Desenvolvimento pesado, CAD/BIM, edição de vídeo, virtualização, containers,
IA local até ~30B parâmetros.

| Item | Especificação | Preço |
|---|---|---:|
| CPU | AMD Ryzen 7 9800X3D (8C/16T, 3D V-Cache) | $433 |
| Placa-mãe | B850 ATX, 2.5 GbE, 3× M.2 | $180 |
| Memória | 32 GB DDR5-6000 (2×16 GB), expansível a 96 GB | $392 |
| GPU | NVIDIA RTX 5070 12 GB | $900 |
| Armazenamento | 2 TB NVMe Gen4 | $350 |
| Fonte | 750 W 80+ Gold, ATX 3.1 | $110 |
| Gabinete | ATX, filtro de poeira, fluxo dirigido | $90 |
| Refrigeração | Cooler torre duplo 120 mm | $50 |
| SO | Windows 11 Pro (OEM) | $145 |
| | **Total por unidade** | **$2.650** |
| | **× 20 unidades** | **$53.000** |

### Tier 3 — Desempenho · $3.365/unidade

Renderização 3D, simulação, treino de modelos, compilação pesada.

| Item | Especificação | Preço |
|---|---|---:|
| CPU | AMD Ryzen 9 9950X3D (16C/32T, 3D V-Cache) | $569 |
| Placa-mãe | X870 ATX, 5 GbE, 4× M.2 | $260 |
| Memória | 32 GB DDR5-6000 (2×16 GB), expansível a 192 GB | $392 |
| GPU | NVIDIA RTX 5070 Ti **16 GB** | $1.219 |
| Armazenamento | 2 TB NVMe Gen5 | $420 |
| Fonte | 850 W 80+ Gold, ATX 3.1 | $140 |
| Gabinete | ATX full, suporte a GPU | $110 |
| Refrigeração | Water cooler AIO 240 mm | $110 |
| SO | Windows 11 Pro (OEM) | $145 |
| | **Total por unidade** | **$3.365** |
| | **× 20 unidades** | **$67.300** |

---

## 4. ⚠️ O conflito entre "Ryzen" e "nível de empresa"

Este é o ponto que muda a decisão de compra e precisa estar explícito.

**Os fabricantes tier-1 praticamente não vendem workstation AMD Ryzen com
slot para RTX.** Dell Precision, HP Z e Lenovo ThinkStation, na faixa que
aceita uma placa RTX dedicada, são linhas **Intel Core Ultra ou Xeon**. A
oferta AMD desses fabricantes ou é ThinkStation P8 com Threadripper PRO
(muito acima desta especificação e desta faixa de preço), ou são desktops
corporativos com Ryzen PRO que **não comportam uma GPU dedicada de porte**.

Isso deixa quatro caminhos reais:

| Caminho | AMD? | Suporte | Custo/unidade (Tier 2) | Observação |
|---|:---:|---|---:|---|
| **A — Montagem própria** | ✅ | Só garantia de peça | **$2.650** | Mais barato; TI interna assume o suporte |
| **B — Integrador especializado** | ✅ | 3 anos, peças e mão de obra | **$3.200–3.800** | Puget, Velocity Micro; recebe testado e com imagem |
| **C — OEM tier-1** | ❌ Intel | ProSupport NBD onsite | **$2.904** | HP Z2 G1i com RTX 2000 Ada 16 GB |
| **D — Híbrido** | ✅ | Misto | — | 16 montadas + 4 OEM para funções críticas |

### A comparação que importa: A contra C

| | A — Montada (Tier 2) | C — HP Z2 G1i |
|---|---|---|
| CPU | Ryzen 7 9800X3D | Intel Core Ultra 7 265 |
| GPU | RTX 5070 12 GB (classe consumo alto) | RTX 2000 Ada 16 GB (classe pro, ~RTX 4060) |
| RAM / SSD | 32 GB / 2 TB | 32 GB / 1 TB |
| Preço unitário | **$2.650** | **$2.904** |
| **20 unidades** | **$53.000** | **$58.080** |
| Suporte | Garantia de peça (RMA), 1–3 anos por fabricante | **Onsite próximo dia útil, 3 anos** |
| Driver certificado ISV | Não | **Sim** (SolidWorks, AutoCAD, Revit) |
| Gestão de frota | Manual | **HP Sure / imagem corporativa** |
| Tempo de parada estimado | 3–7 dias por incidente | **1 dia útil** |

**Interpretação:** o OEM custa **$5.080 a mais na frota** e entrega GPU mais
fraca, mas compra tempo de parada. Se uma estação parada custa $400/dia à
operação, os 20 PCs sofrendo dois incidentes por ano cada, com 4 dias de
diferença, já superam esses $5.080. **Se o trabalho é faturado por hora, o
caminho C se paga. Se há TI interna com peças de reserva, o caminho A ganha.**

O caminho **D (híbrido)** costuma ser a resposta correta: máquinas montadas
para a maioria, mais 3–4 unidades OEM com suporte onsite para as funções que
não podem parar.

---

## 5. Periféricos e itens de frota

| Item | Especificação | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| Monitor | 27" 1440p IPS, altura ajustável, USB-C | 40 | $220 | **$8.800** |
| Teclado + mouse | Conjunto corporativo sem fio | 20 | $45 | $900 |
| Headset | USB com cancelamento de ruído | 20 | $60 | $1.200 |
| Nobreak individual | 900 VA linha-interativa | 20 | $120 | $2.400 |
| Suporte de monitor | Braço duplo articulado | 20 | $75 | $1.500 |
| **Subtotal periféricos** | | | | **$14.800** |

> **Dois monitores por estação** é a configuração padrão assumida. Se for um
> monitor por posto, subtraia **$4.400**.

### Unidades de reserva (spares)

| | |
|---|---|
| Política | **10% da frota = 2 unidades completas** |
| Custo (Tier 2) | **$5.300** |
| Justificativa | Substituição imediata em falha; sem elas, cada incidente vira 3–7 dias de parada (caminho A) |

---

## 6. Licenciamento e software

| Item | Modelo | Qtd | Unit. | Anual |
|---|---|---:|---:|---:|
| Windows 11 Pro | OEM (já incluso no preço unitário) | 20 | $145 | — |
| Microsoft 365 Business Standard | Assinatura | 20 | $12,50/mês | **$3.000** |
| Antivírus / EDR corporativo | Assinatura | 20 | $40/ano | $800 |
| Backup de endpoint | Assinatura | 20 | $60/ano | $1.200 |
| **Subtotal anual de software** | | | | **$5.000** |

> **Windows 11 Pro é obrigatório**, não opcional: a edição Home **não entra
> em domínio**, não tem BitLocker gerenciado, nem Política de Grupo, nem
> Área de Trabalho Remota. Numa frota de 20, isso não é preferência, é
> requisito de administração.

---

## 7. Infraestrutura para 20 estações

Uma frota de 20 PCs com GPU não é neutra para o escritório. O que precisa ser
verificado **antes** da compra:

### Energia

| | Tier 1 | Tier 2 | Tier 3 |
|---|---:|---:|---:|
| Consumo típico por estação | 180 W | 250 W | 340 W |
| **20 estações + monitores** | **4,6 kW** | **6,0 kW** | **7,8 kW** |
| Corrente em 120 V | 38 A | 50 A | 65 A |
| Consumo mensal (8 h × 22 dias) | 810 kWh | 1.056 kWh | 1.373 kWh |
| Custo mensal a $0,14/kWh | $113 | **$148** | $192 |

**Verifique a capacidade do quadro antes de comprar.** Seis quilowatts
distribuídos em circuitos de 20 A significam **no mínimo 4 circuitos
dedicados** só para as estações. Não é item onde improvisar.

### Carga térmica

6,0 kW de eletrônica = **20.500 BTU/h** de calor adicional no ambiente. Some
ao ganho pela envoltória e à ocupação: um escritório de 20 pessoas com esta
frota precisa de **ar-condicionado dimensionado para ~36.000 BTU/h**. Se o
sistema atual já opera no limite, este é um custo escondido de $3.000–8.000.

### Rede

| Item | Especificação | Custo |
|---|---|---:|
| Switch | 48 portas Gigabit PoE gerenciável | $900 |
| Cabeamento | Cat6 + patch panel + certificação, 20 pontos | $2.200 |
| Uplink | 10 GbE para servidor/NAS | $400 |
| **Subtotal rede** | | **$3.500** |

### Implantação

| Item | Custo |
|---|---:|
| Imagem corporativa (criação + validação) | $800 |
| Implantação por estação (20 × $45) | $900 |
| Etiquetagem, inventário e cadastro em MDM | $400 |
| **Subtotal implantação** | **$2.100** |

---

## 8. Custo total

### CapEx — investimento inicial

| Bloco | Tier 1 | Tier 2 | Tier 3 |
|---|---:|---:|---:|
| 20 estações | $40.560 | $53.000 | $67.300 |
| Periféricos | $14.800 | $14.800 | $14.800 |
| 2 unidades de reserva | $4.056 | $5.300 | $6.730 |
| Rede | $3.500 | $3.500 | $3.500 |
| Implantação | $2.100 | $2.100 | $2.100 |
| **Subtotal** | **$65.016** | **$78.700** | **$94.430** |
| Impostos (Texas, 8,25%) | $5.364 | $6.493 | $7.791 |
| Frete e seguro | $900 | $900 | $900 |
| Contingência 8% | $5.201 | $6.296 | $7.554 |
| **TOTAL CapEx** | **$76.481** | **$92.389** | **$110.675** |

### OpEx — anual

| Item | Anual |
|---|---:|
| Software e assinaturas | $5.000 |
| Energia elétrica (Tier 2) | $1.776 |
| Manutenção e peças (3% do CapEx) | $2.361 |
| **Total anual** | **$9.137** |

### TCO em 3 anos (Tier 2)

| | |
|---|---:|
| CapEx | $92.389 |
| OpEx × 3 anos | $27.411 |
| **TCO 3 anos** | **$119.800** |
| **Por estação, por mês** | **$166** |

Para comparação: um serviço de Desktop-as-a-Service equivalente custa
**$180–260 por estação/mês**. A compra se paga em relação ao aluguel em
aproximadamente **22 meses**.

---

## 9. Estratégias de aquisição

| Estratégia | Como funciona | Quando faz sentido |
|---|---|---|
| **Compra à vista** | CapEx integral agora | Caixa disponível; quer o menor custo total |
| **Escalonada** ⭐ | 10 unidades agora, 10 em Q2/2027 | Reduz exposição ao pico de preço; mantém operação |
| **Leasing 36 meses** | ~$2.900/mês, equipamento retorna ou é comprado ao fim | Preserva capital; previsibilidade contábil |
| **DaaS** | $180–260/estação/mês, tudo incluso | Frota volátil; sem equipe de TI |

**A estratégia escalonada, em números:** se GPU e RAM recuarem 25% até
Q2/2027, as 10 unidades adiadas custam ~$19.875 em vez de $26.500 —
**economia de $6.625**, sem atrasar quem precisa da máquina hoje.

---

## 10. Onde comprar

### Canais B2B (conta corporativa, faturamento a prazo, cotação em volume)

| Canal | Para quê | Link |
|---|---|---|
| **CDW** | Revenda corporativa completa, gerente de conta dedicado | [cdw.com](https://www.cdw.com/) |
| **Insight** | Volume, licenciamento e serviços | [insight.com](https://www.insight.com/) |
| **SHI** | Licenciamento e hardware corporativo | [shi.com](https://www.shi.com/) |
| **Connection** | Hardware e serviços de implantação | [connection.com](https://www.connection.com/) |
| **Newegg Business** | Componentes em volume, faturamento Net-30 | [business.newegg.com](https://business.newegg.com/) |
| **Amazon Business** | Preço B2B, isenção fiscal, aprovação por alçada | [business.amazon.com](https://business.amazon.com/) |
| **B&H Corporate** | Componentes e monitores, conta corporativa | [bhphotovideo.com/c/corporate](https://www.bhphotovideo.com/c/corporate) |

### Fabricantes — vendas corporativas

| Fabricante | Linha relevante | Link |
|---|---|---|
| **HP for Business** | Z2 G1i Tower (workstation com RTX) | [hp.com — desktops corporativos](https://www.hp.com/us-en/shop/cv/businessdesktops) |
| **Dell for Business** | Precision Tower | [dell.com/work](https://www.dell.com/en-us/work) |
| **Lenovo Pro** | ThinkStation P3 / P5 | [lenovo.com — negócios](https://www.lenovo.com/us/en/business/) |

### Integradores (a via AMD com suporte)

| Integrador | Diferencial | Link |
|---|---|---|
| **Puget Systems** | Especializado em workstation AMD; testa e publica benchmark por carga de trabalho | [pugetsystems.com](https://www.pugetsystems.com/) |
| **Velocity Micro** | Montagem corporativa sob medida, garantia própria | [velocitymicro.com](https://www.velocitymicro.com/) |

### Rastreadores de preço (confira antes de fechar)

| Ferramenta | Para quê | Link |
|---|---|---|
| videocardprices.com | Preço real de GPU, atualizado | [videocardprices.com](https://videocardprices.com/) |
| Tom's Hardware RAM tracker | Melhor preço de memória | [tomshardware.com](https://www.tomshardware.com/pc-components/ram/ram-price-index-2026-lowest-price-on-ddr5-and-ddr4-memory-of-all-capacities) |
| cheapestssd.com | $/TB de SSD | [cheapestssd.com](https://cheapestssd.com/) |
| camelcamelcamel | Histórico de preço na Amazon | [camelcamelcamel.com](https://camelcamelcamel.com/) |
| Pangoly | Histórico de preço de componente | [pangoly.com](https://pangoly.com/) |

> **Peça cotação formal em volume.** A partir de 10 unidades, todos os canais
> B2B acima negociam. Desconto típico de **5–12%** sobre preço de varejo — na
> frota Tier 2, isso é **$2.650 a $6.360**. Uma ligação ao gerente de conta
> vale o tempo.

---

## 11. Checklist de procurement

**Antes de emitir a ordem de compra:**

- [ ] Capacidade do quadro elétrico confirmada (6 kW / 4 circuitos dedicados)
- [ ] Capacidade de refrigeração confirmada (+20.500 BTU/h)
- [ ] Cotação formal em volume solicitada a **pelo menos 3 canais**
- [ ] Definido o caminho: montagem própria, integrador, OEM ou híbrido
- [ ] Prazo de entrega confirmado por escrito (GPU e RAM estão com prazo irregular)
- [ ] Política de garantia e SLA de substituição definida
- [ ] Certificado de isenção fiscal apresentado, se aplicável
- [ ] Imagem corporativa preparada e validada em 1 unidade piloto
- [ ] Inventário e etiquetagem definidos antes da chegada
- [ ] Destinação do parque antigo decidida (revenda, doação, descarte certificado)

**Compre 1 unidade piloto antes das 20.** Valide a imagem, o desempenho na
carga real e a compatibilidade dos periféricos. Custa $2.650 e evita
descobrir um problema multiplicado por vinte.

---

## 12. Fontes

| Dado | Preço | Fonte |
|---|---|---|
| RTX 5070 | $899,99 (mediana Newegg) · $959,56 em 12/09 | [videocardprices.com](https://videocardprices.com/card/nvidia-rtx-5070/) · [BestValueGPU](https://bestvaluegpu.com/history/new-and-used-rtx-5070-price-history-and-specs/) |
| RTX 5070 Ti | $1.219 | [gpupricehistory.com](https://gpupricehistory.com/us/rtx-5070-ti) · [videocardprices.com](https://videocardprices.com/card/nvidia-rtx-5070-ti/) |
| RTX 5060 Ti 16 GB | $679,99 (Amazon) · $804,99 (Newegg) · era $569,99 em jun/2026 | [videocardprices.com](https://videocardprices.com/card/nvidia-rtx-5060-ti/) · [Tech Insider](https://tech-insider.org/nvidia-gpu-price-hike-rtx-50-series-2026/) |
| Alta de 36–64% da série RTX 50 em 2026 | — | [Tech Insider](https://tech-insider.org/nvidia-gpu-price-hike-rtx-50-series-2026/) |
| Ryzen 7 9700X | $270,53 | [Pangoly](https://pangoly.com/en/price-history/amd-ryzen-7-9700x) |
| Ryzen 7 9800X3D | $433 (mínimo $419,99) | [VideoCardz](https://videocardz.com/newz/amd-ryzen-7-9800x3d-drops-to-449-in-the-us) · [Newegg](https://www.newegg.com/amd-ryzen-7-9000-series-ryzen-7-9800x3d-granite-ridge-zen-5-socket-am5-desktop-cpu-processor/p/N82E16819113877) |
| Ryzen 9 9950X3D | $569 | [TechPowerUp](https://www.techpowerup.com/351096/amd-ryzen-9-9950x3d-drops-to-an-all-time-low-price-of-usd-569) |
| Kit 32 GB DDR5 | $392 | [Tom's Hardware](https://www.tomshardware.com/pc-components/ram/ram-price-index-2026-lowest-price-on-ddr5-and-ddr4-memory-of-all-capacities) |
| SSD NVMe 2 TB | $350–480 (era $110–130 em 2024) | [GamersNexus](https://gamersnexus.net/features/ssds-wtf) · [cheapestssd.com](https://cheapestssd.com/) |
| Capacidade NAND de 2026 esgotada | — | [Tech Insider](https://tech-insider.org/ssd-prices-nand-shortage-2026/) |
| HP Z2 G1i (Ultra 7 265, 32 GB, 1 TB, RTX 2000 Ada) | $2.903,99 | [Amazon](https://www.amazon.com/clp/B0FH6H4QJ4) · [comparativo de workstations](https://pcserverandparts.com/blog/pc-prices-in-2026-why-buying-now-beats-waiting/) |
| Lenovo ThinkStation P5 (32 GB, 1 TB, RTX A2000) | $1.999,99 | [comparativo de workstations](https://pcserverandparts.com/blog/pc-prices-in-2026-why-buying-now-beats-waiting/) |

---

## 13. Premissas e limites desta cotação

1. **Preços de varejo dos EUA em 15/09/2026**, sem imposto e sem frete.
   Preço de componente em 2026 muda em dias — **reconfirme antes de fechar**.
2. **Desconto de volume não está aplicado.** Os totais são conservadores;
   a negociação B2B tende a reduzi-los em 5–12%.
3. **Mão de obra de montagem não incluída** no caminho A. Estime 1,5 h por
   máquina; 20 unidades = 30 h de técnico.
4. **Nenhum link é de afiliado** — são links normais e oficiais, conforme a
   regra do repositório.
5. **Não cobre:** mobiliário, cabeamento elétrico novo, obra civil,
   seguro do parque, ou treinamento.
6. **Câmbio:** valores em USD. Para compra no Brasil, some importação,
   frete internacional e variação cambial — a conta muda completamente.
