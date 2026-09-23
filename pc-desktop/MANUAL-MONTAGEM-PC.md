# Manual de Montagem do PC

> Manual comum aos três orçamentos (Brasil, EUA e Paraguai). A montagem é a
> mesma independente de onde as peças foram compradas.
>
> Leia o manual **inteiro** antes de abrir a primeira caixa. Tempo estimado:
> **3 a 4 horas** na primeira montagem, sem pressa.


## 1. Antes de começar

### 1.1 Ferramentas

| Item | Observação |
|---|---|
| Chave Philips **PH2** com ponta imantada | A única chave realmente necessária |
| Abraçadeiras de velcro | Melhor que plástico — dá para refazer |
| Álcool isopropílico 99% + pano de microfibra | Limpar o IHS antes da pasta |
| Lanterna | O interior do gabinete é escuro |
| Pote pequeno | Para os parafusos não sumirem |
| Pen drive de 8 GB+ | Para o instalador do Windows |

### 1.2 Eletricidade estática

Não é folclore: um estalo que você nem sente já mata um chip.

1. Monte sobre mesa de **madeira ou bancada**, nunca sobre carpete.
2. **Toque a parte metálica do gabinete** (com a fonte desligada e o cabo fora)
   antes de pegar qualquer peça.
3. Segure placas **pelas bordas**, nunca pelos contatos dourados.
4. Se for pausar a montagem, toque o metal de novo ao voltar.

### 1.3 Ordem de montagem

Monte **fora do gabinete** primeiro. É mais fácil, tem mais luz, e você
descobre defeito antes de ter cabeado tudo.

```
   1. CPU na placa-mãe          ┐
   2. Memória                   │  fora do gabinete,
   3. SSDs M.2                  │  sobre a caixa da placa
   4. Backplate do cooler       ┘
   ─────────────────────────────────────
   5. Fonte no gabinete
   6. Placa-mãe no gabinete
   7. Radiador e ventoinhas
   8. Bloco da CPU + pasta térmica
   9. GPU (SEMPRE por último)
  10. Cabeamento
  11. Primeiro boot FORA do gabinete fechado
```

---

## 2. Passo a passo

### Passo 1 — CPU na placa-mãe

1. Apoie a placa-mãe **sobre a própria caixa**, com a espuma antiestática.
2. Levante a alavanca do socket AM5 e abra a tampa metálica.
3. O Ryzen tem um **triângulo dourado em um canto**. Alinhe com o triângulo
   marcado no socket.
4. **Solte o processador na vertical, sem força nenhuma.** Ele assenta pelo
   próprio peso.
5. Feche a tampa e baixe a alavanca. **Vai exigir força e fazer um barulho de
   metal — é normal.**

> ⚠️ **AM5 é LGA: os pinos estão no socket, não no processador.** Um pino
> entortado na placa é conserto de R$ 400 ou placa nova. Nunca encoste no
> socket aberto.
>
> ⚠️ A tampa plástica de proteção sai sozinha ao fechar a alavanca.
> **Guarde-a** — é exigida em qualquer RMA da placa.

### Passo 2 — Memória

1. Consulte o manual da placa: com **2 módulos**, eles vão nos slots
   **A2 e B2** (normalmente o 2º e o 4º contando do processador).
2. Abra as travas das pontas.
3. O pente tem um **entalhe fora do centro** — só entra de um jeito.
4. Pressione as **duas pontas ao mesmo tempo**, com firmeza, até as travas
   fecharem sozinhas com um clique.

> ⚠️ **Slot errado = o PC não liga.** Instalar em A1/B1 com 2 módulos é o erro
> mais comum de todos. Confira no manual impresso da placa.

### Passo 3 — SSDs M.2

1. Remova o dissipador do slot M.2 (1 ou 2 parafusos).
2. Insira o SSD no slot **em ângulo de ~30°** e empurre até o fim.
3. Baixe até ficar reto e prenda com o parafuso ou a trava.
4. **Retire o filme plástico** do pad térmico do dissipador antes de recolocar.

> O **SSD 1 (sistema)** vai no slot M.2 mais próximo do processador — é o que
> tem as linhas PCIe diretas.

### Passo 4 — Backplate do water cooler

O AM5 já vem com backplate de fábrica. O **Kraken Elite usa a original** — não
remova.

1. Retire apenas os dois suportes plásticos laterais do socket.
2. Rosqueie os **standoffs** do kit AM5 do Kraken na backplate existente.

### Passo 5 — Fonte no gabinete

1. Instale com a **ventoinha para baixo** se o gabinete tiver entrada de ar
   embaixo (o normal). Se não tiver, vire para cima.
2. Prenda com os 4 parafusos.
3. **Conecte os cabos modulares na fonte agora**, antes de ela ficar no fundo
   do gabinete: 24 pinos, EPS 8+8 (CPU), e os PCIe da GPU.

> ⚠️ **Nunca use cabos modulares de outra fonte**, mesmo que o conector encaixe.
> A pinagem muda entre fabricantes e **queima o que estiver ligado.**

### Passo 6 — Placa-mãe no gabinete

1. **Conte os standoffs.** Devem existir exatamente onde a placa tem furos —
   nem mais, nem menos. Um standoff sobrando embaixo da placa causa
   curto-circuito.
2. Encaixe o **espelho I/O** (se não for integrado) pressionando de dentro
   para fora até clicar nos 4 cantos.
3. Baixe a placa em ângulo, encostando primeiro no espelho.
4. Parafuse em **cruz**, começando pelo centro. **Firme, sem apertar demais.**

### Passo 7 — Radiador de 360 mm

**Posição recomendada: topo, ventoinhas soprando para fora.**

```
        ┌──────────────────────────────┐
        │  ↑↑↑  RADIADOR 360  ↑↑↑      │  ← ar quente sai
        │                              │
        │   ┌────┐                     │
   →    │   │CPU │        ┌─────┐      │
   ar   │   └────┘        │ RAM │      │
   entra│                 └─────┘      │
   →    │  ▓▓▓▓▓ GPU ▓▓▓▓▓             │
        │                              │
        └──────────────────────────────┘
             ↑↑ entrada frontal ↑↑
```

1. Monte as ventoinhas no radiador **antes** de instalar no gabinete.
2. **Confira a seta de fluxo** na lateral de cada ventoinha.
3. Parafuse o radiador no topo. **Use os parafusos curtos** que vêm com o kit —
   parafuso longo demais **fura o radiador** e vaza líquido.

> ⚠️ Se o radiador no topo conflitar com a memória (acontece com RAM alta),
> mova-o para a frente do gabinete, soprando para dentro.

### Passo 8 — Pasta térmica e bloco da CPU

1. Limpe o IHS do processador com álcool isopropílico e microfibra.
2. **Quantidade:** o IHS do AM5 é retangular e grande. Aplique **5 pontos**
   (4 cantos + centro) ou uma linha em cada diagonal. Cada ponto do tamanho de
   um grão de arroz.
3. **Não espalhe com o dedo** — a pressão do bloco faz isso melhor.
4. Retire o **filme plástico do fundo do bloco** (erro clássico).
5. Assente o bloco reto, sem deslizar.
6. Aperte os 4 parafusos **em X, meia volta por vez**, alternando, até parar.
   Não force além disso.

> ⚠️ **Pasta demais é pior que pasta de menos.** Excesso escorre para o socket.

### Passo 9 — GPU

**Sempre a última peça**, porque ela bloqueia o acesso a tudo.

1. Remova as tampas dos slots traseiros correspondentes.
2. Abra a trava do slot **PCIe x16 superior**.
3. Encaixe com firmeza até a trava fechar com clique.
4. Parafuse no gabinete.
5. Se a placa for pesada, use o **suporte anti-sag** (vem na caixa).

### Passo 10 — Cabeamento

Lista de verificação — **nenhum pode faltar**:

| # | Cabo | Onde | Se esquecer |
|:-:|---|---|---|
| 1 | **24 pinos** ATX | Lateral da placa | Não liga |
| 2 | **EPS 8+8** (CPU) | Topo da placa | Não liga ou desliga sob carga |
| 3 | **PCIe 12V-2x6** ou 3× 8 pinos | GPU | Não liga / desliga em jogo |
| 4 | **SATA/molex da bomba** | Kraken → fonte | Superaquece |
| 5 | ⚠️ **USB 2.0 interno do Kraken** | Header USB da placa | **A tela não acende** |
| 6 | Ventoinhas | Headers CHA_FAN | Superaquece |
| 7 | Painel frontal (Power SW, LED, USB, áudio) | F_PANEL | Botão não funciona |

> ⚠️ **O conector 12V-2x6 da GPU precisa entrar até o fim.** Se ficar 1 mm para
> fora, o contato parcial esquenta e **derrete o conector**. Empurre até o clique
> e confira olhando de lado.
>
> ⚠️ **O item 5 é o mais esquecido de todos.** Sem o cabo USB interno, o Kraken
> funciona normalmente mas a tela fica preta e o NZXT CAM não detecta o cooler.

**Organização:** passe tudo pela traseira, use velcro a cada 10 cm. Cabo solto
na frente atrapalha o fluxo de ar e prende ventoinha.

---

## 3. Primeiro boot — com o gabinete ainda aberto

1. Confira visualmente **todos os 7 cabos** da tabela acima.
2. Ligue **só um monitor** na **saída da GPU** (não na placa-mãe).
3. Ligue a fonte na tomada, chave em **I**.
4. Aperte o power.

### O que deve acontecer

| Sinal | Significado |
|---|---|
| Ventoinhas giram | Fonte e placa ok |
| Tela do Kraken acende | Cabo USB conectado ✅ |
| LED de diagnóstico passa por CPU → DRAM → VGA → BOOT | POST normal |
| Imagem na tela | ✅ Sucesso |

### Se não ligar — diagnóstico na ordem

| Sintoma | Causa mais provável |
|---|---|
| Nada acontece | Chave da fonte, cabo 24 pinos ou botão do painel frontal |
| Liga e desliga em 2 s | EPS 8+8 do processador |
| LED **DRAM** aceso | Memória no slot errado (Passo 2) ou mal encaixada |
| LED **VGA** aceso | GPU mal encaixada ou sem PCIe |
| Liga mas tela preta | Monitor na saída da placa-mãe em vez da GPU |

> ⚠️ **No primeiro boot com EXPO ativo, o AM5 faz "memory training".**
> A tela pode ficar preta por **2 a 5 minutos**. **NÃO DESLIGUE.** É normal e
> acontece só na primeira vez.

---

## 4. BIOS — o que configurar

Entre com **DEL** durante o POST.

| Configuração | Valor | Por quê |
|---|---|---|
| **EXPO / DOCP** | **Ativar (perfil 1)** | Sem isso a RAM roda a 4800 em vez de 6000 |
| **fTPM / TPM 2.0** | Ativar | Exigido pelo Windows 11 |
| **Resizable BAR** | Ativar | +5 a 10% de FPS |
| **Above 4G Decoding** | Ativar | Necessário para o ReBAR |
| **PBO** | Auto | Deixe o padrão no início |
| Ordem de boot | Pen drive primeiro | Para instalar o SO |
| **Curva das ventoinhas** | Silencioso até 60 °C | Ver §5 |

**Salve com F10.** Se algo der errado, o botão **Clear CMOS** na traseira
devolve tudo ao padrão.

---

## 5. Instalação e validação

### 5.1 Ordem de instalação

1. **Windows 11 Pro** (ou Linux) no SSD 1 — desconecte o SSD 2 durante a
   instalação para não errar o destino.
2. **Chipset AMD** — sempre o primeiro driver, do site da AMD.
3. **GPU NVIDIA** — use o instalador limpo.
4. Rede, áudio, Bluetooth (do site da placa-mãe).
5. **NZXT CAM** — para configurar a tela do cooler.
6. Windows Update até não sobrar nada.

### 5.2 Validação obrigatória — antes de considerar pronto

| Teste | Ferramenta | Critério de aprovação |
|---|---|---|
| **Memória** | MemTest86 (boot USB) | **4 passagens sem erro** — deixe a noite |
| CPU sustentada | Cinebench R24, 10 min | Sem travar, < 90 °C |
| GPU | FurMark 15 min | Sem artefato, < 83 °C |
| SSD | CrystalDiskMark | Próximo do anunciado |
| Temperatura ociosa | HWiNFO64 | CPU < 45 °C |

> **Se o MemTest86 acusar 1 erro que seja, a memória está defeituosa.**
> Acione a garantia — não tente conviver com isso.

### 5.3 Configurar a tela do Kraken

1. Abra o **NZXT CAM** → *Lighting & Cooling* → *Kraken*.
2. Modos disponíveis: **imagem**, **GIF animado**, **sensores** (temperatura,
   frequência, uso de CPU/GPU) ou **relógio**.
3. Para GIF: resolução ideal **640 × 640**, arquivo abaixo de 5 MB.
4. Em *Cooling*, crie a curva da bomba e das ventoinhas.

---

## 6. Configurar o que você vai usar

### 6.1 Servidor do ATM10 com 200 mods

**Alocação de memória** (Java 21+, com as flags do Aikar):

```
-Xms12G -Xmx12G -XX:+UseG1GC -XX:+ParallelRefProcEnabled
-XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions
-XX:+DisableExplicitGC -XX:G1NewSizePercent=30
-XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M
```

> **Não dê mais heap do que precisa.** Coletar lixo em 24 GB pausa mais que em
> 12 GB. Para 200 mods, **12 GB é o ponto certo**.

### 6.2 ⚠️ Fixar o servidor no CCD com V-Cache

Este passo vale mais que qualquer overclock.

O 9950X3D tem **2 CCDs**: o **CCD 0** tem o 3D V-Cache (núcleos 0–7), o
**CCD 1** não tem (núcleos 8–15). O Windows pode colocar o servidor no CCD
errado e você perde o benefício inteiro.

**Como fixar** (PowerShell como administrador, com o servidor rodando):

```powershell
$p = Get-Process java
$p.ProcessorAffinity = 0x0000FFFF   # núcleos 0-7 (CCD com V-Cache), com SMT
```

Ou, mais simples: no **Gerenciador de Tarefas** → *Detalhes* → clique direito
no `java.exe` → *Definir afinidade* → marque só as CPUs 0 a 15.

> **Ganho típico: 20 a 40% de TPS** num servidor com 200 mods. É a diferença
> entre um mundo fluido e um mundo que engasga.

Deixe o **cliente** do Minecraft livre no CCD 1 — assim os dois não disputam.

### 6.3 Hermes local

| Passo | O quê |
|---|---|
| 1 | Baixe o [Hermes Desktop](https://nousresearch.com/) — MIT, gratuito, não exige conta para modelos locais |
| 2 | Com 16 GB de VRAM, comece pelo **Hermes 4.3 14B** em Q4/Q6 — cabe inteiro e roda rápido |
| 3 | O **36B** em Q4 ocupa ~21,8 GB — vai usar offload para a RAM, a ~6–10 tok/s |
| 4 | Guarde os modelos no **SSD 2**, não no do sistema |

### 6.4 Claude Code

Requisito: **Node.js 18+**. O Claude Code é leve — o peso do seu projeto está
no Hermes, não nele. Com 16 núcleos, compilação e testes rodam em paralelo com
o servidor de Minecraft sem disputa.

---

## 7. Manutenção

| Quando | O quê |
|---|---|
| Mensal | Ar comprimido nos filtros de poeira |
| Trimestral | Conferir temperaturas no HWiNFO; comparar com a linha de base |
| Semestral | Limpeza interna completa |
| Anual | Verificar se a bomba do AIO ainda faz ruído normal |
| 4–6 anos | Trocar o AIO (a vida útil típica) |
| **Sempre** | **Backup dos mundos do Minecraft** — automatize |

---

## 8. Fontes de preço

| Componente | Preço | Fonte |
|---|---|---|
| Ryzen 9 9950X3D | R$ 4.705,87 (melhor) · R$ 5.410,90 · R$ 6.399,99 (06/09) | [Amazon BR](https://www.amazon.com.br/Processador-AMD-Ryzen-9950X3D-Graphics/dp/B0DVZSG8D5) · [Mercado Livre](https://www.mercadolivre.com.br/processador-amd-ryzen-9-9950x3d-am5-43ghz-57ghz-turbo/p/MLB47079231) · [Hardware Barato](https://www.hardwarebarato.com/produtos/processadores/ryzen-9-9950x3d) |
| RTX 5070 Ti 16 GB | R$ 7.124,05 · mín. R$ 4.999 (04/05/2026) | [Hardware Barato](https://www.hardwarebarato.com/produtos/placas-de-video/rtx-5070-ti) |
| RTX 5060 Ti 16 GB | R$ 3.959 (ML) · R$ 5.129–5.412 (pré-venda) | [Hardware Barato](https://www.hardwarebarato.com/produtos/placas-de-video/rtx-5060-ti-16gb) · [NVIDIA Marketplace BR](https://marketplace.nvidia.com/pt-br/consumer/graphics-cards/) |
| RTX 5070 12 GB | R$ 6.499,90 | [Hardware Barato](https://www.hardwarebarato.com/produtos/placas-de-video/rtx-5070) |
| DDR5 32 GB | R$ 2.450 – R$ 8.139,99 | [KaBuM](https://www.kabum.com.br/hardware/memoria-ram/ddr-5) · [Terabyte](https://www.terabyteshop.com.br/hardware/memorias/ddr5) |
| DDR5 64 GB (2×32) | R$ 8.882,50 à vista | [Pichau](https://www.pichau.com.br/hardware/memorias) |
| NZXT Kraken Elite 360 LCD | R$ 1.999,99 – R$ 2.672,07 | [KaBuM](https://www.kabum.com.br/produto/486417/water-cooler-nzxt-kraken-elite-360-argb-com-display-lcd-360mm-preto) · [Terabyte](https://www.terabyteshop.com.br/produto/35072/water-cooler-nzxt-kraken-elite-360-360mm-display-lcd-intelamd-preto-rl-kn36e-b2) |
| Placa-mãe X870 | R$ 1.919 – R$ 2.543 | [GK Infostore](https://www.gkinfostore.com.br/amd-x870) · [MSI Brasil](https://br.msi.com/Landing/amd-am5-x870e-x870-b850-b840-ryzen-9000-x3d-best-ai-gaming-motherboard) |
| Fonte 850 W | R$ 401 – R$ 810 | [KaBuM](https://www.kabum.com.br/hardware/fontes/fonte-850w) · [Pichau](https://www.pichau.com.br/hardware/fonte) |
| SSD NVMe | "todos dobraram de preço nos últimos meses" | [Fórum Adrenaline](https://forum.adrenaline.com.br/threads/que-tipo-de-ssd-custo-beneficio-e-recomendado-em-2026.720263/) · [KaBuM](https://www.kabum.com.br/hardware/ssd-2-5/ssd-pcie-nvme) |

---

## 9. Premissas e limites

1. **Preços de 16/09/2026.** Componente no Brasil muda de preço toda semana.
   Confira no [Hardware Barato](https://www.hardwarebarato.com/),
   [Buscapé](https://www.buscape.com.br/) e [Zoom](https://www.zoom.com.br/)
   antes de fechar.
2. Valores de **placa-mãe, SSD, fonte e gabinete são estimativas** de faixa —
   não cotação de listagem específica.
3. **Preço à vista no PIX costuma dar 8–15%** de desconto sobre o parcelado.
   Os valores aqui são de tabela.
4. **Não incluído:** monitor, teclado, mouse, headset, Windows (≈ R$ 300 OEM)
   e nobreak.
5. **Nenhum link é de afiliado**, conforme a regra do repositório.
6. Este manual cobre a montagem padrão. **O manual impresso da sua placa-mãe
   tem precedência** sobre qualquer coisa escrita aqui — especialmente sobre
   quais slots de memória usar.
