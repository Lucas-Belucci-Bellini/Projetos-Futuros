# Armazenamento e Nuvem Privada — 256 TB

> Cotação de 15/09/2026.

---

## 1. Como os 256 TB se fecham

Você pediu 256 TB **úteis**. Disco bruto não é disco útil — paridade,
metadados e margem operacional consomem uma fatia grande.

| | |
|---|---|
| Discos | **12 × Seagate Exos M 30 TB** = 360 TB brutos |
| Layout | ZFS **RAIDZ2** (10 dados + 2 paridade) |
| Tolerância a falha | **2 discos simultâneos** |
| Capacidade bruta do pool | 300 TB |
| Menos overhead ZFS (~3%) | 291 TB |
| Menos margem de 15% (ZFS degrada cheio) | **~247 TB de trabalho confortável** |
| **Capacidade formatada** | **~273 TiB / 300 TB** ✅ |

**Por que RAIDZ2 e não RAIDZ1:** com discos de 30 TB, um `resilver` leva
**12–20 horas**. Em RAIDZ1, durante essas horas você está sem redundância
nenhuma — e é exatamente quando o segundo disco morre, porque os discos do lote
envelheceram juntos. RAIDZ2 custa 60 TB de capacidade e compra tranquilidade.

**Por que 12 discos e não 16:** 12 baias cabem em chassi 4U padrão, consomem
menos energia (cada disco = ~7 W ocioso, 10 W ativo) e pesam menos. Para chegar
a 256 TB úteis, 12 × 30 TB já basta.

---

## 2. Orçamento

| Item | Especificação | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| HDD | Seagate Exos M 30 TB SATA (HAMR) | 12 | $570 | **$6.840** |
| NVMe mundos/cache | 2 TB Gen5, espelhado | 2 | $280 | $560 |
| NVMe metadados (special vdev) | 1 TB, espelhado | 2 | $140 | $280 |
| HBA | Broadcom 9500-16i (IT mode) | 1 | $450 | $450 |
| Chassi | 4U 16 baias, backplane SAS3, hot-swap | 1 | $850 | $850 |
| **Total do subsistema de disco** | | | | **$8.980** |

### Opção econômica: discos recondicionados

| | Novo 30 TB | Recondicionado 28 TB |
|---|---:|---:|
| Preço unitário | $570 | **$350** |
| Custo por TB | $19,00 | **$12,50** |
| 12 unidades | $6.840 | **$4.200** |
| Bruto | 360 TB | 336 TB |
| Útil (RAIDZ2) | ~300 TB | ~280 TB |
| Garantia | 5 anos | Tipicamente 1–2 anos |

**Economia: $2.640.** Para dados que já têm paridade dupla **e** backup, é um
risco defensável — [ServerPartDeals](https://serverpartdeals.com/collections/20tb-to-30tb-hard-drives)
é a referência de mercado nesse segmento. Se adotar, compre **13 discos** e
mantenha um de reserva fria.

---

## 3. O problema do backup — leia antes de comemorar

Um pool com RAIDZ2 **não é backup**. Ele protege contra falha de disco. Não
protege contra: apagar por engano, ransomware, incêndio, furto do veículo,
ou o próprio pool corromper.

E aqui está a conta que assusta:

| Estratégia | Custo mensal | Custo anual |
|---|---:|---:|
| Backblaze B2 para 256 TB ($6/TB/mês) | **$1.536** | **$18.432** |
| AWS S3 Glacier Deep Archive (~$1/TB/mês) | $256 | $3.072 |
| **Segundo NAS em endereço fixo** | **$0** | **$0** (após $5.500 iniciais) |

**Fazer backup em nuvem de 256 TB custa mais por ano do que o storage inteiro.**

### A resposta correta: não faça backup de 256 TB

Classifique o dado. Na prática, a divisão fica assim:

| Classe | O que é | Volume típico | Estratégia |
|---|---|---:|---|
| 🔴 **Insubstituível** | Código, projetos, fotos, documentos, configs, inventário da armaria | **~2 TB** | 3-2-1 completo: local + NAS remoto + nuvem cifrada |
| 🟡 **Caro de refazer** | Mundos de Minecraft, datasets de IA tratados, modelos ajustados | **~15 TB** | Local + NAS remoto |
| 🟢 **Recuperável** | Bibliotecas de jogos, modpacks, mídia, modelos base | **~230 TB** | Só snapshot local. Se perder, baixa de novo. |

**Custo real desse esquema:**

| Item | Custo |
|---|---:|
| Nuvem cifrada para 2 TB (Backblaze B2) | **$12/mês** |
| Segundo NAS (4× 30 TB, RAIDZ1, casa de familiar) | **$3.200** uma vez |
| Replicação `zfs send` incremental por VPN | $0 |
| **Total recorrente** | **$12/mês** em vez de $1.536 |

É a diferença entre **$144/ano e $18.432/ano**, com proteção melhor para o que
importa de verdade.

---

## 4. Serviços da nuvem privada

| Serviço | Software | Função |
|---|---|---|
| Nuvem pessoal | [Nextcloud](https://nextcloud.com/) | Arquivos, calendário, contatos, sincronização |
| Objeto S3 | [MinIO](https://min.io/) | Datasets de IA, artefatos de build, backup de apps |
| Compartilhamento local | SMB + NFS | Acesso direto dos nós e do notebook |
| Snapshots | ZFS `zfs-auto-snapshot` | Horário × 24, diário × 30, mensal × 12 |
| Replicação | `zfs send` sobre WireGuard | Para o NAS remoto, incremental |
| Cifragem | ZFS native encryption | Pool cifrado em repouso — **crítico**: o veículo pode ser furtado |
| Monitoramento | `smartd` + Prometheus + Grafana | SMART, temperatura, erros de leitura |

**Sistema base: [TrueNAS SCALE](https://www.truenas.com/truenas-scale/)** —
ZFS nativo, containers para Nextcloud/MinIO, replicação pronta, e interface web
que funciona bem em conexão de satélite com latência alta.

---

## 5. Cifragem é obrigatória, não opcional

Um NAS numa casa, se roubado, é um crime raro. Um NAS **num veículo** viaja
por estacionamentos, acampamentos e estradas. A probabilidade de furto é ordens
de grandeza maior.

- **ZFS native encryption** no pool inteiro.
- **Chave em token físico** (YubiKey) ou frase que **não** fica no veículo.
- Consequência aceita: depois de queda de energia total, o pool **não monta
  sozinho** — alguém precisa destravar. É o preço, e vale.

---

## 6. Expansão futura

O chassi tem 16 baias e o pool usa 12. As 4 restantes permitem:

| Caminho | Resultado |
|---|---|
| Adicionar vdev de 4× 30 TB em RAIDZ1 | +90 TB úteis, ~$2.280 |
| Trocar os 12 discos por 50 TB (quando existirem) | ~500 TB úteis, sem mexer no chassi |
| 2 baias para spare quente + 2 para expansão | Resilver automático sem intervenção |

**Recomendação:** deixe **1 disco de spare quente** configurado. Num veículo,
onde você pode estar a 200 km da loja mais próxima, o pool se reconstruir
sozinho vale mais que 30 TB extras.

---

## 7. Fontes

| Dado | Fonte |
|---|---|
| Seagate Exos M 30 TB — $570–600, ~$19–20/TB | [Tom's Hardware (review)](https://www.tomshardware.com/pc-components/hdds/seagate-exos-m-30tb-hdd-review) · [B&H](https://www.bhphotovideo.com/c/product/1905376-REG/seagate_st30000nm004k_exos_m_internal_hard.html) · [TechRadar ($18,80/TB)](https://www.techradar.com/pro/seagates-massive-30tb-hard-drive-has-massive-price-cut-at-just-usd18-80-per-tb-exos-m-hdd-in-on-sale-at-provantage-for-usd564-has-a-5-year-warranty) |
| Exos 28 TB recondicionado — $349,99 (~$12,50/TB) | [Slickdeals](https://slickdeals.net/f/18767644-seagate-exos-hdd-28tb-st28000nm000c-recertified-349-99) · [ServerPartDeals](https://serverpartdeals.com/collections/20tb-to-30tb-hard-drives) |
| Preço de nuvem B2 | [Backblaze B2 pricing](https://www.backblaze.com/cloud-storage/pricing) |
