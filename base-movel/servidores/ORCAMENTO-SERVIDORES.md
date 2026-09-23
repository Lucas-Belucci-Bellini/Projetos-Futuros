# Servidores — Detalhamento Técnico

> Complementa o [orçamento mestre](../ORCAMENTO-2026.md), que já traz as
> listas de peças e os preços. Aqui está o que **não** cabe numa tabela:
> por que cada escolha, o que muda no Cenário B, e a pilha de software.

---

## 1. O Cenário B em detalhe — consolidar 4 nós em 2

A arquitetura de 4 máquinas independentes está certa em princípio: isolamento
de falha é uma virtude. Mas num veículo, cada chassi extra custa **watt, peso
e calor** — os três recursos mais escassos do projeto.

### Proposta

```text
 CENÁRIO A (pedido)                   CENÁRIO B (consolidado)
 ┌──────────────┐                     ┌────────────────────────────┐
 │ SERVER-01    │ Gaming              │  WORKSTATION               │
 │ 1.470 W      │                     │  Threadripper PRO 9955WX   │
 ├──────────────┤                     │  2× RTX 5090               │
 │ SERVER-02    │ IA                  │  128 GB ECC                │
 │ 2.900 W      │                     │  ├ Windows (jogo) bare-mtl │
 ├──────────────┤                     │  └ Linux (IA) dual-boot    │
 │ SERVER-03    │ Minecraft/NAS       │  1.750 W                   │
 │   450 W      │                     ├────────────────────────────┤
 ├──────────────┤                     │  SERVIDOR (Proxmox)        │
 │ SERVER-04    │ Serviços            │  Ryzen 9 9950X · 128 GB    │
 │   120 W      │                     │  ├ VM: TrueNAS (HBA PT)    │
 └──────────────┘                     │  ├ VM: Minecraft           │
   4.940 W · 4 chassis                │  └ LXC: serviços           │
                                      │  480 W                     │
                                      └────────────────────────────┘
                                        2.230 W · 2 chassis
```

| | Cenário A | Cenário B | Diferença |
|---|---:|---:|---:|
| Consumo (nós) | 4.940 W | 2.230 W | **–55%** |
| Custo TI | $63.757 | $34.509 | **–$29.248** |
| Peso | 210 kg | 135 kg | –36% |
| Calor a remover | 16.860 BTU/h | 7.610 BTU/h | **–55%** |
| Espaço no rack | 14U | 8U | –43% |

### O que você perde

Seja claro sobre o custo real da consolidação:

1. **Jogo e IA na mesma máquina.** Dual-boot resolve (Windows para jogar,
   Linux para IA), mas não dá para fazer os dois ao mesmo tempo. Se rodar IA
   enquanto joga é requisito, o Cenário A ganha.
2. **Single point of failure no servidor.** Se o host Proxmox cai, caem NAS,
   Minecraft e serviços juntos. Mitiga-se com backup de configuração e um
   plano de recuperação testado — mas o risco existe.
3. **2 GPUs em vez de 6.** Menos capacidade de inferência paralela.

### O que você ganha

Metade do consumo. Num veículo, isso não é economia — é **viabilidade**. Com
2,2 kW em vez de 4,9 kW, o banco de baterias dura o dobro, o gerador roda
metade do tempo, e a climatização cai de 29.000 para 22.000 BTU/h.

---

## 2. Por que Threadripper PRO no nó de IA

Não é "CPU melhor". É **plataforma diferente**, e a diferença é decisiva:

| | Ryzen 9 9950X3D | Threadripper PRO 9975WX |
|---|---:|---:|
| Linhas PCIe | 24 | **128** |
| GPUs a x16 | 1 (ou 2 a x8) | **4 a x16** |
| Canais de memória | 2 | **8** |
| RAM máxima | 192 GB (UDIMM) | **2 TB (RDIMM ECC)** |
| ECC | Limitado | ✅ Nativo |
| Preço | $569 | $4.099 |

Os **256 GB pedidos são impossíveis no Ryzen 9** com RDIMM ECC de verdade. E
4 GPUs em 24 linhas PCIe significaria x4 por placa — o que estrangula o
tensor-parallel. Não há atalho: é Threadripper PRO, ou são menos GPUs.

**Escolha dentro da linha:** o 9975WX (32 núcleos, $4.099) é o ponto ótimo.
O 9985WX (64 núcleos, $7.999) custa $3.900 a mais para núcleos que a carga de
inferência **não usa** — o trabalho está na GPU. O 9955WX (16 núcleos, $1.649)
já entrega as 128 linhas PCIe e é a escolha do Cenário B.

---

## 3. A pergunta das 4 GPUs

4× RTX 5090 = 128 GB de VRAM por $18.100 e 2.300 W.

**O problema:** as RTX 5090 **não têm NVLink**. As 4 placas não formam uma
memória única — elas conversam por PCIe, que é ~10× mais lento. Para inferência
com tensor-parallel (vLLM, llama.cpp) funciona bem; para treino, a penalidade
de comunicação é severa.

| Opção | VRAM | Custo | Consumo | Calor |
|---|---:|---:|---:|---:|
| 4× RTX 5090 | 128 GB fragmentada | $18.100 | 2.300 W | 7.850 BTU/h |
| 2× RTX 5090 | 64 GB fragmentada | $9.050 | 1.150 W | 3.920 BTU/h |
| 1× RTX PRO 6000 | **96 GB unificada** | $16.000–19.999 | **600 W** | **2.050 BTU/h** |

Para um veículo, a RTX PRO 6000 é tecnicamente superior por uma margem grande:
**quase 4× menos consumo** pela mesma classe de VRAM, e sem a complexidade de
particionar modelo. Custa o mesmo. A objeção legítima é que ela **não joga tão
bem** quanto a 5090 e que "4 placas" era o pedido.

**Recomendação honesta:** 1× RTX PRO 6000 (IA) + 1× RTX 5090 (jogo) resolve
os dois casos de uso melhor que 6 placas, por **$20.500 em vez de $27.150**,
com **1.175 W em vez de 3.450 W**. A decisão é sua; o registro fica aqui.

---

## 4. Pilha de software

### SERVER-02 — IA

| Camada | Escolha |
|---|---|
| SO | Ubuntu Server 24.04 LTS |
| Driver | NVIDIA 5xx + CUDA 13 |
| Inferência | [vLLM](https://docs.vllm.ai/) (throughput) + [llama.cpp](https://github.com/ggml-org/llama.cpp) (GGUF, offload) |
| Modelos | Hermes 4.3 36B · Hermes 4 70B (tensor-parallel em 4 GPUs) |
| Interface | [Open WebUI](https://openwebui.com/) + API compatível com OpenAI |
| Orquestração | Docker Compose; GPU via `nvidia-container-toolkit` |

**Hermes 4 70B nas 4 GPUs:** em Q8 ocupa ~75 GB — cabe nas 4 placas com
`--tensor-parallel-size 4`. Em FP16 (140 GB) **não cabe**; seria preciso
quantizar ou usar offload para os 256 GB de RAM.

### SERVER-03 — Minecraft + NAS

| Camada | Escolha |
|---|---|
| Hipervisor | [Proxmox VE](https://www.proxmox.com/) |
| NAS | VM TrueNAS SCALE com **HBA em passthrough** (obrigatório para ZFS) |
| Minecraft | LXC por instância; [Paper](https://papermc.io/) ou Fabric conforme modpack |
| Backup de mundo | Snapshot ZFS a cada hora + `zfs send` para o pool principal |

**Regra crítica:** ZFS precisa de **acesso direto ao disco**. Passar o HBA
inteiro em PCIe passthrough para a VM TrueNAS não é otimização, é requisito —
ZFS sobre disco virtualizado perde a capacidade de detectar e corrigir
corrupção silenciosa, que é a razão inteira de usar ZFS.

### SERVER-04 — Serviços

Proxmox com containers LXC: Pi-hole (DNS), Home Assistant, Gitea + runners,
Prometheus + Grafana, Uptime Kuma, WireGuard.

---

## 5. Ordem de montagem

Monte e valide **fora do veículo**, na bancada, antes de instalar:

1. **Bancada:** monte cada nó, rode 24 h de estresse (Prime95 + FurMark +
   `memtest86`), meça o consumo real com wattímetro.
2. **Confirme os números deste orçamento.** Se o consumo medido divergir do
   estimado, **a climatização e a energia precisam ser redimensionadas** antes
   de qualquer coisa ser parafusada.
3. **Rack fora do veículo:** monte, cabeie, etiquete, teste a rede completa.
4. **Teste térmico:** rode o rack fechado, com a climatização definitiva, por
   8 h em carga máxima. É aqui que se descobre se o dimensionamento está certo.
5. **Só então instale no veículo.**
6. **Teste de estrada:** 500 km carregado, depois reinspeção completa de
   parafusos, conectores e montagem.

O passo 4 é o que ninguém faz e o que separa um projeto que funciona de um que
desliga por superaquecimento na primeira tarde de verão.
