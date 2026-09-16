# PC Desktop — Orçamento BRASIL 🇧🇷

> **Para:** rodar o Hermes local, usar o Claude Code, jogar **e hospedar** o
> ATM10 com 200 mods, com 4 TB de armazenamento, Ryzen 9, RTX e water cooler
> com tela customizável.
>
> 📄 **Este é 1 de 3 orçamentos.** Os outros:
> [🇺🇸 EUA](ORCAMENTO-PC-EUA.md) · [🇵🇾 Paraguai](ORCAMENTO-PC-PARAGUAI.md).
> O [manual de montagem](MANUAL-MONTAGEM-PC.md) é comum aos três.
>
> **Cotação: 16 de setembro de 2026 · mercado brasileiro · valores em R$.**
> Preço de hardware no Brasil muda toda semana e depende do câmbio.
> **Reconfira antes de comprar.**

---

## 1. Resumo executivo

| | Config A — Essencial | **Config B — Equilibrada** ⭐ | Config C — Teto |
|---|---|---|---|
| CPU | Ryzen 9 9950X3D | **Ryzen 9 9950X3D** | Ryzen 9 9950X3D |
| RAM | 32 GB | **64 GB** | 128 GB |
| GPU | RTX 5060 Ti **16 GB** | **RTX 5070 Ti 16 GB** | RTX 5070 Ti 16 GB |
| Armazenamento | 4 TB (2×2 TB) | **4 TB (2×2 TB)** | 6 TB |
| **TOTAL** | **R$ 22.714** | **R$ 33.062** | **R$ 43.944** |

**Recomendação: Config B.** O motivo está na §3 — com 32 GB você **não
consegue** hospedar o ATM10 e rodar o Hermes ao mesmo tempo.

---

## 2. ⚠️ Antes de tudo: a memória é o problema deste projeto

Você disse "32 GB, mas planejo 128 se possível". Em setembro de 2026, no
Brasil, isso custa isto:

| Quantidade | Configuração | Preço |
|---|---|---:|
| 32 GB | 2 × 16 GB | **R$ 2.450** |
| 64 GB | 2 × 32 GB | **R$ 8.882** |
| 128 GB | 4 × 32 GB | **≈ R$ 17.765** |

**Os 128 GB custam quase 4× o processador.** Na Config C, a memória sozinha é
**44% do orçamento** — mais que CPU e GPU somados.

Causa: a crise global de DRAM. Um kit DDR5 de 32 GB que custava ~R$ 600 hoje
sai por R$ 2.450, e no Brasil o câmbio e o imposto entram por cima.
Os SSDs dobraram de preço pelo mesmo motivo.

### ⚠️ A armadilha técnica: não compre 32 GB "para somar depois"

O instinto é comprar 2 × 16 GB agora e adicionar mais 2 × 16 GB no futuro.
**Não faça isso.** Duas razões:

1. **O AM5 perde velocidade com 4 módulos.** O controlador de memória do Ryzen
   9000 é projetado para 2 módulos. Com os 4 slots preenchidos, a velocidade
   estável cai de DDR5-6000 para algo entre **DDR5-3600 e 4400**. Você ganha
   capacidade e perde desempenho — justamente no Minecraft, que é sensível a
   latência de memória.
2. **Misturar kits é a causa nº 1 de instabilidade em AM5.** Dois kits comprados
   em datas diferentes podem ter chips de fabricantes diferentes, mesmo com o
   mesmo código de produto. O resultado é tela azul aleatória que ninguém
   consegue diagnosticar.

### As duas estratégias honestas

| Estratégia | Como | Custo hoje | Custo do upgrade |
|---|---|---:|---|
| **A — Começar em 32 GB** | 2 × 16 GB, 2 slots livres | R$ 2.450 | Para 64 GB de verdade: **jogar fora** os 2×16 e comprar 2×32 (R$ 8.882) |
| **B — Começar em 64 GB** ⭐ | 2 × 32 GB, 2 slots livres | R$ 8.882 | Para 128 GB: +2×32 do **mesmo lote** (R$ 8.882), aceitando a queda de velocidade |

> **A recomendação é a B**, e não por perfeccionismo: com 32 GB você não
> executa o que pediu. A conta está na §3.

---

## 3. A conta de memória do seu caso de uso

Você quer, **ao mesmo tempo**: hospedar o servidor do ATM10, jogar nele, rodar
o Hermes e usar o Claude Code.

| Processo | RAM necessária |
|---|---:|
| Windows 11 + navegador + Discord | 6–8 GB |
| **Servidor** ATM10 com 200 mods (heap da JVM) | **12–14 GB** |
| **Cliente** Minecraft com os mesmos 200 mods | **8–10 GB** |
| Claude Code + VS Code + Node | 3–4 GB |
| Hermes em execução (parte que não cabe na VRAM) | 4–8 GB |
| **TOTAL** | **33–44 GB** |

**Com 32 GB você já estoura antes de abrir o Hermes.** O sistema começa a usar
arquivo de paginação, o servidor de Minecraft engasga e o tick despenca.

**Com 64 GB sobra folga confortável** para os quatro ao mesmo tempo.
**Com 128 GB** você poderia rodar modelos de IA maiores em CPU — mas paga
R$ 8.882 a mais por um ganho que a GPU já entrega melhor.

---

## 4. Config A — Essencial · R$ 22.714

Para quem quer o servidor e a IA funcionando, com jogo bom, gastando o mínimo.

| Componente | Especificação | Preço |
|---|---|---:|
| **CPU** | AMD Ryzen 9 9950X3D · 16C/32T · 144 MB cache | **R$ 4.706** |
| **Placa-mãe** | B850 Gaming Plus WiFi · AM5 · DDR5 · 2× M.2 | R$ 1.500 |
| **Memória** | 32 GB DDR5-6000 CL30 (2 × 16 GB) | R$ 2.450 |
| **GPU** | RTX 5060 Ti **16 GB** GDDR7 | **R$ 3.959** |
| **SSD 1** | 2 TB NVMe Gen4 (sistema + jogos) | R$ 1.700 |
| **SSD 2** | 2 TB NVMe Gen4 (mundos + modelos de IA) | R$ 1.700 |
| **Water cooler** | NZXT Kraken Elite 360 · LCD 2,72" | **R$ 2.000** |
| **Fonte** | 850 W 80+ Gold modular | R$ 700 |
| **Gabinete** | **ASUS ROG Hyperion GR701** · Full Tower E-ATX · 4× 140 mm | **R$ 3.999** |
| | **TOTAL** | **R$ 22.714** |

> **Por que a RTX 5060 Ti de 16 GB e não a RTX 5070 de 12 GB:** a 5070 custa
> **R$ 6.499** e tem **menos VRAM**. Para IA local, quantidade de VRAM vale mais
> que velocidade — um modelo que não cabe na placa não roda, por mais rápida que
> ela seja. **Você economiza R$ 2.540 e ganha 4 GB de VRAM.**

---

## 5. Config B — Equilibrada ⭐ · R$ 33.062

**A recomendada.** É a menor configuração que executa tudo o que você pediu ao
mesmo tempo.

| Componente | Especificação | Preço |
|---|---|---:|
| **CPU** | AMD Ryzen 9 9950X3D · 16C/32T | **R$ 4.706** |
| **Placa-mãe** | X870 ATX · AM5 · PCIe 5.0 · 3–4× M.2 | R$ 2.251 |
| **Memória** | **64 GB** DDR5-6000 CL30 (2 × 32 GB) · 2 slots livres | **R$ 8.882** |
| **GPU** | RTX 5070 Ti **16 GB** GDDR7 | **R$ 7.124** |
| **SSD 1** | 2 TB NVMe Gen4 (sistema + jogos) | R$ 1.700 |
| **SSD 2** | 2 TB NVMe Gen4 (mundos + modelos) | R$ 1.700 |
| **Water cooler** | NZXT Kraken Elite 360 · LCD 2,72" | **R$ 2.000** |
| **Fonte** | 850 W 80+ Gold modular ATX 3.1 | R$ 700 |
| **Gabinete** | **ASUS ROG Hyperion GR701** · Full Tower E-ATX | **R$ 3.999** |
| | **TOTAL** | **R$ 33.062** |

### O que esta configuração entrega

| Tarefa | Resultado |
|---|---|
| Servidor ATM10 + cliente simultâneos | ✅ Com folga |
| Hermes 4.3 **14B** na VRAM | ✅ Rápido (~30 tok/s) |
| Hermes 4.3 **36B** com offload | ⚠️ Funciona, ~6–10 tok/s |
| Claude Code + compilação | ✅ 16 núcleos dão conta |
| Jogos AAA em 1440p | ✅ Alto/Ultra |
| Caminho para 128 GB | ✅ 2 slots livres |

---

## 6. Config C — Teto · R$ 43.944

Só faz sentido se você **realmente** for usar os 128 GB.

| Componente | Especificação | Preço |
|---|---|---:|
| CPU | AMD Ryzen 9 9950X3D | R$ 4.706 |
| Placa-mãe | X870E ATX · 4 DIMM · 4× M.2 | R$ 2.550 |
| **Memória** | **128 GB** DDR5 (4 × 32 GB) ⚠️ | **R$ 17.765** |
| GPU | RTX 5070 Ti 16 GB | R$ 7.124 |
| SSD 1 | 2 TB NVMe Gen5 | R$ 2.100 |
| SSD 2 | 4 TB NVMe Gen4 | R$ 2.800 |
| Water cooler | NZXT Kraken Elite 360 LCD | R$ 2.000 |
| Fonte | 1000 W 80+ Gold | R$ 900 |
| Gabinete | **ASUS ROG Hyperion GR701** · Full Tower E-ATX | **R$ 3.999** |
| | **TOTAL** | **R$ 43.944** |

> ⚠️ **A memória é 44% do orçamento** e vem com a queda de velocidade dos 4
> módulos (§2). **Recomendo a Config B e guardar os R$ 10.882** — quando a
> crise de DRAM passar (previsão: meados de 2027), os mesmos 128 GB devem custar
> uma fração disso.

---

## 7. O water cooler com tela — as opções reais

Você pediu tela customizável. Três produtos disponíveis no Brasil:

| Modelo | Tela | Resolução | Preço |
|---|---|---|---:|
| **NZXT Kraken Elite 360** ⭐ | **2,72" IPS** | 640 × 640 | **R$ 1.999 – 2.672** |
| NZXT Kraken Elite 240 | 2,36" LCD | 640 × 640 | ≈ R$ 1.600 |
| Lian Li Hydroshift LCD 360S | 2,88" | 480 × 480 | ≈ R$ 1.900 |

**Escolhido: NZXT Kraken Elite 360.** A tela de 2,72" é a maior da categoria,
aceita **imagem estática, GIF animado e leitura de sensores** (temperatura,
frequência, uso), configurada pelo software **NZXT CAM**. O radiador de 360 mm
é o correto para os 170 W do 9950X3D.

> ⚠️ **A tela só funciona com o cabo USB 2.0 interno conectado.** É o erro mais
> comum na montagem — o cooler funciona, a bomba gira, mas a tela fica preta.
> Está no passo 6.4 do manual.

[KaBuM](https://www.kabum.com.br/produto/486417/water-cooler-nzxt-kraken-elite-360-argb-com-display-lcd-360mm-preto) ·
[Terabyte](https://www.terabyteshop.com.br/produto/35072/water-cooler-nzxt-kraken-elite-360-360mm-display-lcd-intelamd-preto-rl-kn36e-b2) ·
[Amazon](https://www.amazon.com.br/kraken-elite/s?k=kraken+elite)

---

## 8. Por que cada peça

### CPU — Ryzen 9 9950X3D

**A peça mais acertada do projeto.** Dois motivos que se somam no seu caso:

- **3D V-Cache (144 MB)** — o Minecraft é limitado por latência de memória no
  laço principal do servidor. Cache grande resolve isso melhor que clock alto.
  É o melhor processador de consumo que existe para servidor de Minecraft.
- **16 núcleos / 32 threads** — o servidor, o cliente, o Hermes e a compilação
  do Claude Code rodam ao mesmo tempo sem disputar núcleo.

> ⚠️ **Detalhe que quase ninguém sabe:** o 9950X3D tem **dois CCDs**, e só
> **um deles** tem o 3D V-Cache. Se o servidor de Minecraft cair no CCD errado,
> você perde o benefício inteiro. O passo 10.3 do manual ensina a fixar.

### GPU — a lógica da VRAM

| GPU | VRAM | Preço | Para IA local |
|---|---|---:|---|
| RTX 5060 Ti | **16 GB** | R$ 3.959 | ✅ Melhor custo por GB |
| RTX 5070 | 12 GB | R$ 6.499 | ❌ **Menos VRAM, mais cara** |
| RTX 5070 Ti | **16 GB** | R$ 7.124 | ✅ Mesma VRAM, muito mais rápida em jogo |

**A RTX 5070 de 12 GB é a pior compra da lista para você.** Para IA, o que
manda é caber o modelo na placa.

### Armazenamento — dois SSDs, não um

Pedir 4 TB e comprar um único SSD de 4 TB funciona, mas **dois de 2 TB é
melhor** e costuma custar menos:

| Disco | Conteúdo | Por quê |
|---|---|---|
| **SSD 1 (2 TB)** | Windows, jogos, programas | Isola o sistema |
| **SSD 2 (2 TB)** | **Mundos do Minecraft**, modelos de IA, projetos | Se o sistema corromper, o mundo sobrevive |

O mundo do ATM10 faz escrita constante. Mantê-lo fora do disco do sistema
reduz risco e facilita backup.

---

## 9. Consumo e fonte

| Componente | Pico |
|---|---:|
| Ryzen 9 9950X3D (PPT) | 200 W |
| RTX 5070 Ti | 300 W |
| Placa, RAM, 2 SSD, ventoinhas, bomba | 100 W |
| **Total de pico** | **600 W** |
| **Fonte recomendada** | **850 W** (margem de 40%) |

850 W não é exagero: a folga mantém a fonte na faixa de melhor eficiência e
absorve os picos transientes da RTX, que chegam ao dobro do nominal por
milissegundos.

---

