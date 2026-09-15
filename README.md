# Projetos-Futuros — Catálogo de Notebooks

Branch: `base-movel/notebooks`

Esta branch concentra **orçamento, seleção, documentação técnica, upgrades, autonomia, durabilidade, manutenção e custo de propriedade** dos notebooks da Base Móvel.

## Objetivo

Não escolher notebook apenas pelo preço. Cada candidato deve possuir uma ficha que responda:

- O que ele consegue rodar?
- O que ele não consegue rodar bem?
- O que ele aguenta em carga prolongada?
- Quanto tempo a bateria pode durar em uso leve, moderado e pesado?
- Quanto custa comprar?
- Quanto custa fazer upgrade?
- Quanto custa trocar cada peça importante?
- Quão fácil é reparar?
- Quais peças são substituíveis e quais normalmente exigem placa-mãe?
- Quanto custa manter o equipamento por 3, 5 e 7 anos?

## Regra Hermes

`Hermes = SIM` significa **adequação planejada para cargas locais de IA/Hermes**, não certificação do fabricante. O Hermes pode mudar e cargas futuras podem exigir mais VRAM/RAM.

A classificação prioriza 32 GB+ de RAM, RTX com 8 GB+ de VRAM, boa refrigeração e CPU Ryzen 7/9 ou Ryzen AI equivalente.

## Matriz de compra

| ID | Modelo / configuração-alvo | Hermes | RAM-alvo | GPU | VRAM | SSD inicial | Upgrade prioritário | Orçamento-alvo |
|---|---|---:|---:|---|---:|---:|---|---:|
| N01 | Razer Blade 14 — Ryzen 9 8945HS + RTX 4070 | SIM | 32 GB | RTX 4070 Laptop | 8 GB | 1 TB | 2 TB SSD / confirmar RAM do part number | R$ 13–18 mil* |
| N02 | ASUS ROG Zephyrus G14 GA403UI — Ryzen 9 8945HS + RTX 4070 | SIM | 32 GB | RTX 4070 Laptop | 8 GB | 1 TB | 2 TB SSD | R$ 12–17 mil* |
| N03 | ASUS TUF A16 — Ryzen AI 9 HX 370 + RTX 4070 | SIM | 32 GB | RTX 4070 Laptop | 8 GB | 1 TB | 2 TB SSD | R$ 11–15 mil* |
| N04 | Lenovo Legion Pro 5 — Ryzen 9 7945HX + RTX 4070 | SIM | 32 GB | RTX 4070 Laptop | 8 GB | 1 TB | RAM/SSD conforme part number | R$ 12–17 mil* |
| N05 | Acer Nitro 16 — Ryzen 7 8845HS + RTX 4060 | SIM | 32 GB | RTX 4060 Laptop | 8 GB | 1 TB | 64 GB RAM se oficialmente suportado / 2 TB SSD | R$ 8–12 mil* |
| N06 | ASUS ROG Strix G17 — Ryzen 9 7945HX + RTX 4060 | NÃO* | 32 GB | RTX 4060 Laptop | 8 GB | 1 TB | 32 GB RAM / 2 TB SSD | R$ 9–13 mil* |
| N07 | HP Victus 16 — Ryzen 7 8845HS + RTX 4070 | NÃO* | 32 GB | RTX 4070 Laptop | 8 GB | 512 GB–1 TB | 32 GB + 2 TB quando suportado | R$ 8–12 mil* |
| N08 | Acer Nitro V15 — Ryzen 7 7735HS + RTX 4050 | NÃO | 32 GB | RTX 4050 Laptop | 6 GB | 512 GB | segundo M.2 + 32 GB | R$ 6–9 mil |
| N09 | ASUS ROG Zephyrus G14 — Ryzen 7 8845HS + RTX 4060 | NÃO* | 32 GB | RTX 4060 Laptop | 8 GB | 1 TB | 2 TB SSD | R$ 9–13 mil* |
| N10 | Lenovo Legion Pro 5 — Ryzen 7 7745HX + RTX 4060 | NÃO* | 32 GB | RTX 4060 Laptop | 8 GB | 1 TB | RAM/SSD conforme part number | R$ 9–13 mil* |

`*` Faixas de planejamento, não cotação garantida. Preço muda por estoque, promoção, importação, câmbio e vendedor.

## Fichas individuais

- [N01 — Razer Blade 14](docs/N01-razer-blade-14.md)
- [N02 — ROG Zephyrus G14 RTX 4070](docs/N02-zephyrus-g14-4070.md)
- [N03 — ASUS TUF A16 RTX 4070](docs/N03-tuf-a16-4070.md)
- [N04 — Lenovo Legion Pro 5 RTX 4070](docs/N04-legion-pro-5-4070.md)
- [N05 — Acer Nitro 16 RTX 4060](docs/N05-nitro-16-4060.md)
- [N06 — ROG Strix G17 RTX 4060](docs/N06-strix-g17-4060.md)
- [N07 — HP Victus 16 RTX 4070](docs/N07-victus-16-4070.md)
- [N08 — Acer Nitro V15 RTX 4050](docs/N08-nitro-v15-4050.md)
- [N09 — ROG Zephyrus G14 RTX 4060](docs/N09-zephyrus-g14-4060.md)
- [N10 — Lenovo Legion Pro 5 RTX 4060](docs/N10-legion-pro-5-4060.md)

## Documentos gerais

- [Orçamento, reparo e TCO](docs/ORCAMENTO-E-MANUTENCAO.md)
- [Critérios de avaliação e testes](docs/CRITERIOS-DE-AVALIACAO.md)

## Estrutura obrigatória de cada ficha

Cada documento deve conter:

1. **Identificação:** marca, modelo, part number e geração.
2. **Hardware:** CPU, GPU, VRAM, RAM, SSD, tela, bateria, Wi-Fi, portas e peso.
3. **Upgrades:** RAM máxima oficial, número de slots, M.2 disponíveis e limites conhecidos.
4. **Uso real:** programação, game dev, 3D, edição, IA local, jogos e Minecraft modded.
5. **Limites:** cargas que não são recomendadas ou que podem saturar VRAM/RAM/CPU.
6. **Autonomia:** capacidade em Wh e estimativa por perfil; nunca confundir autonomia publicitária com garantia de tempo.
7. **Térmica:** ventilação, comportamento em carga e manutenção recomendada.
8. **Durabilidade:** nota de 1 a 5 e fatores de desgaste.
9. **Peças:** preços de orçamento para bateria, tela, teclado, ventoinhas, dissipador, carregador, RAM, SSD, Wi-Fi, tampa e placa-mãe.
10. **Reparo:** cenário simples, médio e crítico; peças + mão de obra.
11. **TCO:** compra + upgrades + manutenção + reserva de reparo.
12. **Fontes:** fabricante, manual/service guide e referências de mercado.

## Como tratar CPU/GPU

CPU e GPU de notebook frequentemente são soldadas à placa-mãe. Portanto, a ficha não deve inventar um “preço de RTX removível”. Para falha desses componentes, registrar:

- reparo de placa, quando tecnicamente viável;
- troca da placa-mãe;
- ou substituição do notebook.

## Escala de durabilidade

**5/5:** excelente para uso intenso com manutenção correta.

**4/5:** boa resistência para uso diário pesado.

**3/5:** adequada, mas exige atenção maior a temperatura/bateria/estrutura.

**2/5:** sensível a uso extremo ou cara de reparar.

**1/5:** inadequada para a Base Móvel.

Essa escala é uma avaliação de projeto, não promessa de vida útil.

## Política de preços

Todos os preços de peças devem ter uma das etiquetas:

- **VERIFICADO:** preço encontrado para a peça/serviço específico.
- **ESTIMATIVA:** faixa de orçamento baseada em peças equivalentes/mercado.
- **A CONFIRMAR:** depende do part number ou assistência autorizada.

Data-base desta revisão: **14/09/2026**.

## Fontes iniciais de referência

- NVIDIA — comparação de GPUs GeForce RTX para notebooks: https://www.nvidia.com/pt-br/geforce/laptops/compare/
- AMD — Ryzen 9 7945HX: https://www.amd.com/en/products/processors/laptop/ryzen/7000-series/amd-ryzen-9-7945hx.html
- ASUS — ROG Zephyrus G14 2024: https://rog.asus.com/pt/laptops/rog-zephyrus/rog-zephyrus-g14-2024/spec/
- ASUS — guia de serviço GA403UI: https://www.asus.com/us/supportonly/ga403ui/helpdesk_service_guide/
- Lenovo — Legion Pro 5 16ARX8 PSREF: https://psrefstuff.lenovo.com/syspool/Sys/PDF/Legion/Legion_Pro_5_16ARX8/Legion_Pro_5_16ARX8_Spec.html
- HP — Victus 16 série s1000: https://support.hp.com/br-pt/document/ish_9927306-9944473-16
- Acer — Nitro V15: https://br-store.acer.com/notebook-acer-anv15-41-r6j0--r77735hs--8gb--512gb-ssd--rtx4050--agpos--black--fhd-15-6-nh-u20al-004/p
- KaBuM — Razer Blade 14: https://www.kabum.com.br/produto/571236/
- KaBuM — ROG Strix G17: https://www.kabum.com.br/produto/453041/
- KaBuM — HP Victus 16: https://www.kabum.com.br/produto/904125/
- KaBuM — Acer Nitro 16: https://www.kabum.com.br/produto/710820/
- KaBuM — ASUS TUF A16: https://www.kabum.com.br/produto/650326/
