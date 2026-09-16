# Projeto Base Móvel — Arquitetura e Orçamento

> Planejamento conceitual de uma base móvel de computação para viagens, desenvolvimento, jogos, IA, servidores e armazenamento local.
>
> 💰 **Preços, fontes e planta baixa estão em [`ORCAMENTO-2026.md`](ORCAMENTO-2026.md)** — este documento é a arquitetura; aquele é quanto custa.

## 1. Visão geral

A base será dividida em **4 computadores independentes**, um **sistema de armazenamento/nuvem**, infraestrutura de rede, energia, climatização, conectividade e área habitacional.

A ideia é evitar tratar tudo como um único computador. Cada função terá seu próprio nó, permitindo desligar ou reiniciar um sistema sem derrubar os demais.

---

# 2. CLASSIFICAÇÃO GERAL

| Grupo | Função | Onde fica |
|---|---|---|
| SERVER-01 | Gaming / workstation | Rack técnico |
| SERVER-02 | IA / Hermes / GPU compute | Rack técnico |
| SERVER-03 | Minecraft / serviços | Rack técnico |
| SERVER-04 | Serviços gerais / automações | Rack técnico |
| STORAGE-01 | Nuvem local / NAS | Rack de armazenamento |
| NETWORK-01 | Firewall / switch / roteador | Rack de rede |
| POWER-01 | UPS e distribuição | Compartimento técnico |
| COOLING-01 | Climatização da sala técnica | Compartimento técnico |
| SAT-01 | Internet via satélite | Teto do veículo |
| NOTEBOOK | Máquina móvel pessoal | Área habitacional |
| CASA | Cama, bancada, armários, cozinha e banheiro | Área habitacional |

---

# 3. SERVER-01 — GAMING

## Objetivo

Máquina principal para jogos, desenvolvimento de jogos, testes gráficos, VR e tarefas pesadas de workstation.

### Configuração-alvo

- CPU: AMD Ryzen 9 classe topo de linha
- RAM: 64 GB DDR5
- GPU: 2x NVIDIA RTX
- Armazenamento: 8 TB
- Water cooler de alta capacidade
- Placa-mãe com múltiplos slots PCIe
- Ethernet 10 GbE
- Wi-Fi não é prioridade, pois o sistema ficará conectado ao switch
- Gabinete/rack com fluxo de ar adequado

### Observação

Duas GPUs grandes em um mesmo sistema exigem planejamento de espaçamento, alimentação e refrigeração. A escolha final da GPU deve considerar comprimento, espessura e consumo da placa.

---

# 4. SERVER-02 — IA / HERMES

## Objetivo

Nó dedicado para IA, inferência, treinamento, processamento de modelos, Hermes, automações e experimentos locais.

### Configuração-alvo

- Plataforma AMD workstation/server
- Preferência: AMD Threadripper PRO ou equivalente profissional
- RAM: **256 GB ECC** como alvo; 128 GB como configuração inicial
- GPU: **4x NVIDIA RTX**
- SSD NVMe para sistema e cache
- Armazenamento separado no STORAGE-01
- Rede: 25 GbE preferencialmente
- Refrigeração: dedicada
- Fonte(s): dimensionadas para múltiplas GPUs

### Por que não usar Ryzen 9 comum?

O requisito de 4 GPUs e centenas de GB de RAM torna uma plataforma workstation muito mais apropriada. O Ryzen 9 pode funcionar em projetos menores, mas o nó principal de IA deve priorizar linhas com mais pistas PCIe, memória ECC e capacidade de expansão.

### Hermes

O notebook poderá executar uma versão local/leve do Hermes, enquanto o SERVER-02 será o principal ambiente de computação pesada.

---

# 5. SERVER-03 — MINECRAFT / SERVIÇOS

## Objetivo

Hospedar servidores Minecraft e outros serviços persistentes.

### Configuração-alvo

- CPU: AMD Ryzen 9 de alto desempenho por núcleo
- RAM: 128 GB DDR5
- Armazenamento local: **32 TB**
- SSD/NVMe dedicado para sistema, mundo ativo e cache
- HDD/SSD adicional para armazenamento de dados
- Ethernet 10/25 GbE
- Backup automático no STORAGE-01

### Organização

```text
SERVER-03
├── Minecraft
│   ├── Survival
│   ├── Modpacks
│   ├── NEXORA testing
│   └── Worlds
├── Containers
├── Game servers
└── Backup local
```

---

# 6. SERVER-04 — SERVIÇOS GERAIS

## Objetivo

Nó mais leve, mas ainda profissional, para serviços que não precisam competir com IA ou Minecraft.

### Configuração-alvo

- CPU: AMD Ryzen 7/9
- RAM: 16 GB inicialmente
- Armazenamento: 8 TB
- SSD NVMe para sistema
- Ethernet 10 GbE
- Docker/containers
- Monitoramento
- DNS interno
- Home Assistant/domótica, se desejado
- Automação
- Git runners
- Serviços auxiliares

### Expansão futura

RAM pode subir para 32/64 GB se os serviços crescerem.

---

# 7. STORAGE-01 — NUVEM LOCAL

## Objetivo

Criar uma nuvem privada da base móvel.

### Meta

**258 TB úteis ou mais**.

### Proposta

Usar vários HDDs de alta capacidade em ZFS/RAID apropriado, em vez de depender de um único disco gigantesco.

Exemplo conceitual:

- 16x HDD de 24 TB = 384 TB brutos
- Pool com redundância
- SSD/NVMe para cache/metadados conforme necessidade
- Snapshots
- Backups
- Criptografia
- Compartilhamento SMB/NFS
- Interface web de nuvem

### Serviços

- Nextcloud ou equivalente
- S3 compatível
- SMB
- NFS
- Backup dos 4 servidores
- Arquivo de projetos
- Biblioteca de jogos/mods
- Dataset de IA
- Vídeos/imagens
- Repositórios locais

### Importante

258 TB **úteis** não significa 258 TB de discos instalados. RAID/ZFS, margem operacional e redundância reduzem a capacidade disponível.

---

# 8. NETWORK-01 — REDE INTERNA

## Topologia

```text
                    SATÉLITE
                       │
                  FIREWALL
                       │
                 CORE SWITCH
             ┌─────────┼─────────┐
             │         │         │
          SERVER-01 SERVER-02 SERVER-03
                       │
                   SERVER-04
                       │
                    STORAGE
                       │
                NOTEBOOK / CASA
```

### Recomendação

- Core switch 10 GbE mínimo
- 25 GbE entre IA e storage, se o orçamento permitir
- VLAN para cada grupo
- VLAN de administração
- VLAN de servidores
- VLAN de armazenamento
- VLAN de convidados
- Firewall dedicado
- VPN para acesso remoto

---

# 9. INTERNET

## Principal

Internet via satélite, instalada no teto do veículo.

## Redundância

- 4G/5G
- Segundo modem/roteador
- Failover automático

## Regra

A internet externa nunca deve ser necessária para a rede interna funcionar.

Os servidores devem continuar operacionais mesmo sem internet.

---

# 10. POWER-01 — ENERGIA

## UPS

Serão utilizados **3 UPS/nobreaks**, separados por grupos de carga.

### UPS-01

SERVER-01 + equipamentos críticos.

### UPS-02

SERVER-02 + GPU/IA.

### UPS-03

SERVER-03 + SERVER-04 + rede + storage crítico.

### Energia complementar

- Banco de baterias LiFePO4
- Inversor profissional
- Gerador
- Entrada de energia externa
- Proteção contra surtos
- DPS
- Aterramento adequado
- Monitoramento de consumo

### Objetivo

O sistema deve conseguir desligar os computadores de forma controlada quando a energia disponível estiver terminando.

---

# 11. COOLING-01 — CLIMATIZAÇÃO

A sala dos computadores deve ser fisicamente separada da área habitacional.

### Princípio

```text
AR FRIO → RACKS → AR QUENTE → EXAUSTÃO
```

Não deixar o calor dos servidores entrar diretamente na área de dormir.

### Necessário

- Ar-condicionado dimensionado por carga térmica
- Exaustão
- Sensores de temperatura
- Sensores de umidade
- Detecção de fumaça
- Detecção de água
- Desligamento emergencial

---

# 12. NOTEBOOK

O notebook é uma categoria separada dos servidores.

## Requisitos

- AMD Ryzen 7 ou Ryzen 9
- 32 GB DDR5 preferencialmente
- 1–2 TB SSD interno
- Pelo menos um caminho de expansão de armazenamento
- Preferência por dois slots de RAM
- NVIDIA RTX
- Boa refrigeração
- Capacidade de jogar Minecraft modded pesado
- Capacidade de desenvolvimento
- Capacidade de executar Hermes local/leve quando suportado

## 10 opções a pesquisar

### Grupo A — Hermes / IA local

1. Ryzen 9 + RTX 5090 + 32/64 GB
2. Ryzen 9 + RTX 5080 + 32/64 GB
3. Ryzen 9 + RTX 5070 Ti + 32/64 GB
4. Ryzen 9 + RTX 4070/5070 + 32 GB
5. Ryzen 9 + RTX 4070 + 32 GB

### Grupo B — sem Hermes como requisito

6. Ryzen 7 + RTX 5070
7. Ryzen 7 + RTX 4070
8. Ryzen 7 + RTX 4060 Ti/4060
9. Ryzen 7 + RTX 4050
10. Ryzen 7 + RTX equivalente disponível com 32 GB

A escolha final deve verificar: slots de RAM, limite oficial de RAM, quantidade de slots M.2, possibilidade de segundo SSD e disponibilidade no Brasil.

---

# 13. CASA — ÁREA HABITACIONAL

A casa não deve competir fisicamente com o rack.

## Divisão sugerida

```text
┌───────────────────────────────────────────────┐
│                  CABINE                        │
├───────────────────────────────────────────────┤
│                                               │
│       ÁREA HABITACIONAL / ESCRITÓRIO          │
│                                               │
│  CAMA        BANCADA        ARMÁRIOS          │
│                                               │
│  COZINHA     BANHEIRO       NOTEBOOK          │
│                                               │
├───────────────────────────────────────────────┤
│        PAREDE TÉCNICA / ISOLAMENTO             │
├───────────────────────────────────────────────┤
│                                               │
│       SALA TÉCNICA / RACKS / STORAGE          │
│                                               │
│ SERVER 01 │ SERVER 02 │ SERVER 03 │ SERVER 04│
│                                               │
│ STORAGE │ NETWORK │ UPS │ BATERIAS │ COOLING │
│                                               │
└───────────────────────────────────────────────┘
```

---

# 14. ÁREA HABITACIONAL

## Mobiliário

- Cama dobrável ou fixa
- Bancada de trabalho
- Cadeira ergonômica
- Armários
- Gavetas
- Pequena cozinha
- Geladeira compacta
- Pia
- Banheiro compacto
- Reservatório de água
- Sistema de água limpa/cinza

## Estética

- Industrial
- Militar/função técnica
- Preto/cinza/metal
- Poucos elementos decorativos
- Iluminação LED
- Painéis metálicos
- Piso resistente
- Fixação de equipamentos para movimento

---

# 15. EQUIPAMENTOS DE SUPORTE

Além dos PCs:

- Monitor(es)
- Teclado/mouse
- KVM
- Rack
- Patch panels
- Cabos Cat6A/Cat7
- Fibra óptica interna, se necessário
- Switches
- Firewall
- Access points Wi-Fi
- Câmeras de segurança
- NVR
- Sensores
- Sistema de alarme
- Ferramentas
- Peças de reposição
- SSDs/HDDs reserva
- Ventoinhas reserva
- Cabos de energia
- Fonte(s) reserva

---

# 16. SEGURANÇA DA BASE

A infraestrutura deve priorizar segurança física e digital.

## Digital

- MFA
- VPN
- Firewall
- VLANs
- Backups offline
- Criptografia
- Logs
- Monitoramento
- Conta administrativa separada

## Física

- Compartimento técnico trancado
- Rack fixado ao veículo
- Extintor apropriado para equipamentos elétricos
- Detector de fumaça
- Detector de temperatura
- Detector de vazamento
- Corte de energia de emergência

Qualquer compartimento destinado a armas de fogo deve ser projetado e instalado somente conforme as leis e requisitos de segurança aplicáveis à jurisdição em que o veículo estiver sendo usado. Este documento não especifica métodos de armazenamento de armas.

---

# 17. ORÇAMENTO — CATEGORIAS

| Categoria | Prioridade |
|---|---|
| Server-01 Gaming | Alta |
| Server-02 IA | Muito alta |
| Server-03 Minecraft | Alta |
| Server-04 Serviços | Média |
| Storage 258 TB+ | Muito alta |
| Rede 10/25 GbE | Muito alta |
| UPS x3 | Muito alta |
| Baterias | Muito alta |
| Gerador | Alta |
| Climatização | Muito alta |
| Satélite | Alta |
| Veículo | Muito alta |
| Conversão do veículo | Muito alta |
| Notebook | Alta |
| Área habitacional | Alta |

---

# 18. PRINCÍPIO DE CONSTRUÇÃO

A construção deve acontecer em fases.

### Fase 1 — Infraestrutura

Veículo, isolamento, elétrica, aterramento, climatização e rack.

### Fase 2 — Rede

Firewall, switch, Wi-Fi, satélite e cabeamento.

### Fase 3 — Storage

NAS, pool de dados, backups e monitoramento.

### Fase 4 — Servidores

SERVER-01 → SERVER-02 → SERVER-03 → SERVER-04.

### Fase 5 — Energia redundante

UPS, baterias e gerador.

### Fase 6 — Casa

Cama, cozinha, banheiro, bancada e armários.

### Fase 7 — Testes móveis

Testar a base estacionada, depois pequenos deslocamentos e finalmente viagens longas.

---

# 19. PRINCIPAL REGRA DE ARQUITETURA

**A casa não é o servidor.**

**O notebook não é o servidor.**

**O storage não é um servidor de aplicação.**

Cada função deve ter isolamento suficiente para permitir manutenção independente.

Isso permite que:

- o SERVER-02 seja reiniciado sem desligar Minecraft;
- o Minecraft continue funcionando enquanto o PC Gaming está desligado;
- a nuvem continue disponível enquanto um servidor está em manutenção;
- o notebook seja usado fora do veículo;
- a área habitacional permaneça confortável mesmo com os servidores sob carga.

---

# 20. NOME DOS SISTEMAS

Sugestão:

- **ATLAS-01** — Gaming
- **HERMES-01** — IA
- **AEGIS-01** — Minecraft
- **ORACLE-01** — Serviços
- **ARK-01** — Storage/Nuvem
- **CITADEL-01** — Firewall/Rede
- **CORE-POWER** — Energia
- **SKYLINK** — Satélite
- **MOBILE-HQ** — veículo completo

---

## Status

Documento inicial de arquitetura. Preços e modelos comerciais devem ser atualizados imediatamente antes da compra, pois GPUs, CPUs, discos, notebooks e veículos têm grande variação de preço e disponibilidade.
