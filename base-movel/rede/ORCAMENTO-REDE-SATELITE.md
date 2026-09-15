# Rede e Internet via Satélite

> Cotação de 15/09/2026.

---

## 1. Conectividade — Starlink

### Hardware

| Kit | Preço | Uso neste projeto |
|---|---:|---|
| **Flat High Performance** | **$599** | ✅ **Principal.** Antena plana, projetada para uso **em movimento**, sem partes móveis |
| **Standard (Gen 3)** | **$349** | ✅ **Reserva.** Para uso estacionado prolongado |
| High Performance | $1.999 | Maior ganho; excesso para este caso |
| Mini | $249 | Portátil, para sair do veículo a pé |

**Escolhido: Flat High Performance ($599) + Standard ($349) = $948.**

Duas antenas não é exagero: é a diferença entre "a internet caiu" e "a internet
mudou de antena". Com duas, você tem redundância física e pode deixar uma
montada no teto (em movimento) e outra em tripé (estacionado, com céu melhor).

### Planos

| Plano | Mensal | Dados | Em movimento |
|---|---:|---|---|
| Roam 100 GB | $55 | 100 GB | ✅ até 100 mph |
| Roam 300 GB | $80 | 300 GB | Em áreas selecionadas |
| **Roam Ilimitado** | **$175** | Ilimitado | ✅ até 100 mph |

**Escolhido: Roam Ilimitado — $175/mês.** Com backup off-site, replicação ZFS,
atualização de modelos de IA e um servidor de Minecraft público, 300 GB acabam
na primeira semana.

### Redundância celular

| Item | Custo |
|---|---:|
| Roteador 5G com dual SIM + antenas MIMO externas | $900 |
| Plano de dados 5G (failover) | $60/mês |

Failover automático no firewall: Starlink cai → 5G assume em segundos. Em
cidade, o 5G frequentemente tem **latência menor** que o satélite — vale
priorizar 5G para jogo e SSH, e Starlink para volume.

---

## 2. Rede interna

| Item | Especificação | Qtd | Unit. | Total |
|---|---|---:|---:|---:|
| Switch core | 24 portas 10 GbE + 4× 25 GbE SFP28, gerenciável | 1 | $1.100 | $1.100 |
| Firewall | Netgate/pfSense, 4× 2.5 GbE | 1 | $700 | $700 |
| KVM over IP | 4 portas, acesso remoto aos nós | 1 | $450 | $450 |
| PDU gerenciável | 2 unidades, medição por tomada | 2 | $350 | $700 |
| Access point Wi-Fi 7 | Cobertura interna + externa | 1 | $300 | $300 |
| Cabeamento | Cat6A blindado, patch panel, etiquetagem | 1 | $500 | $500 |
| Rack | 24U com amortecedores anti-vibração | 1 | $2.200 | $2.200 |
| **Subtotal rede + rack** | | | | **$5.950** |
| Starlink Flat HP + Standard | | | | **$948** |
| Roteador 5G + antenas | | | | **$900** |
| Passagem de teto, mastro, cabos | | | | **$450** |
| **TOTAL** | | | | **$8.248** |

---

## 3. Topologia e VLANs

```text
            STARLINK (principal)      5G (failover)
                   │                       │
                   └───────┬───────────────┘
                           │
                    ┌──────────────┐
                    │   FIREWALL   │  pfSense · failover · VPN · IDS
                    └──────┬───────┘
                           │
                    ┌──────────────┐
                    │ SWITCH CORE  │  10/25 GbE
                    └──────┬───────┘
      ┌────────┬───────────┼───────────┬────────┬────────┐
      │        │           │           │        │        │
   VLAN 10  VLAN 20    VLAN 30     VLAN 40  VLAN 50  VLAN 99
   GERÊNCIA SERVIDORES ARMAZENAM.  PESSOAL  VISITA   IoT
   KVM,IPMI SRV 01-04  25GbE p/IA  notebook wi-fi   sensores
   PDU,UPS                                   isolada  câmeras
```

| VLAN | Regra |
|---|---|
| **10 — Gerência** | Sem saída para internet. Acesso só por VPN. |
| **20 — Servidores** | Saída controlada. Sem acesso à VLAN 10. |
| **30 — Armazenamento** | 25 GbE dedicado IA ↔ NAS. **Sem rota para internet.** |
| **40 — Pessoal** | Notebook e celular. Acesso normal. |
| **50 — Visitantes** | Isolada de tudo. Só internet. |
| **99 — IoT** | Sensores e câmeras. Sem saída, sem acesso lateral. |

**A regra que não se quebra:** a rede interna **funciona sem internet**.
DNS interno, NTP local (com GPS como fonte de tempo), Nextcloud, Minecraft,
inferência de IA — nada disso pode depender do satélite.

---

## 4. Acesso remoto

- **WireGuard** no firewall — leve, rápido, sobrevive a troca de IP (que
  acontece o tempo todo em Starlink).
- **CGNAT:** Starlink usa CGNAT por padrão. Você **não recebe conexão de fora**.
  Duas saídas: IP público fixo pela Starlink (custo adicional) ou um relay
  ([Tailscale](https://tailscale.com/) ou VPS barata com túnel reverso).
  **Recomendado: Tailscale** — resolve CGNAT sem custo e sem configurar nada.
- **Servidor de Minecraft acessível externamente** exige a mesma solução:
  túnel reverso por VPS ($5/mês) ou [Playit.gg](https://playit.gg/).

---

## 5. Fontes

| Dado | Fonte |
|---|---|
| Starlink — kits e preços (Standard $349, Flat HP $599, HP $1.999, Mini $249) | [US Mobile — guia 2026](https://www.usmobile.com/blog/starlink-cost/) · [starlinkprice.com](https://starlinkprice.com/plans.html) |
| Planos Roam ($55 / $80 / $175) e uso em movimento até 100 mph | [Starlink Insider](https://starlinkinsider.com/starlink-roam/) · [Starlink — planos oficiais](https://www.starlink.com/us/service-plans) |
