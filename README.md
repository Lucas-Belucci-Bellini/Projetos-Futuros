# Projetos-Futuros

Central de planejamento para projetos futuros.

## Estrutura de branches

- `main` — base estável / índice geral.
- `projeto/base-movel` — arquitetura geral da Base Móvel.
- `base-movel/servidores` — os 4 servidores e virtualização/orquestração.
- `pc-01-gaming` — projeto completo do PC de Gaming/workstation.
- `pc-02-ia-hermes` — projeto completo do PC de IA/Hermes.
- `pc-03-minecraft` — projeto completo do PC de Minecraft.
- `pc-04-servicos` — projeto completo do PC de serviços.
- `pc-05-storage-nas` — projeto do storage/NAS e nuvem privada.
- `base-movel/casa` — área habitacional, móveis e organização interna.
- `base-movel/energia` — UPS, baterias, inversor, gerador e distribuição elétrica.
- `base-movel/rede` — switches, firewall, VLANs, Wi-Fi, VPN e internet satelital.
- `base-movel/storage` — NAS, nuvem privada, backups e expansão de armazenamento.
- `base-movel/notebooks` — notebooks de trabalho, desenvolvimento, jogos e IA.
- `base-movel/veiculo` — caminhão/baú, estrutura, peso, isolamento e integração.
- `base-movel/climatizacao` — refrigeração, fluxo de ar e sala técnica.
- `base-movel/documentacao` — requisitos, ADRs, diagramas, inventário e decisões.
- `site-promocoes-pc-faculdade` — futuro site de promoções e orçamentos de PCs/notebooks para estudantes e usuários gerais.
- `sistema-operacional-linux` — projeto de sistema operacional desktop baseado em Linux, com UX inspirada em desktops modernos, segurança forte e identidade visual própria.

## Orçamentos — cotação de 15/09/2026

Os documentos abaixo põem preço real (com fonte e link) no que o planejamento
descreve em conceito. Estão organizados nas pastas que correspondem à convenção
de branches acima.

| Documento | Cobre | Branch correspondente |
|---|---|---|
| [`base-movel/ORCAMENTO-2026.md`](base-movel/ORCAMENTO-2026.md) | **Orçamento mestre** — 3 cenários, consolidado, cronograma | `projeto/base-movel` |
| [`base-movel/servidores/`](base-movel/servidores/ORCAMENTO-SERVIDORES.md) | Os 4 nós, consolidação em 2, pilha de software | `base-movel/servidores` |
| [`base-movel/storage/`](base-movel/storage/ORCAMENTO-STORAGE-NUVEM.md) | Nuvem de 256 TB, ZFS, backup 3-2-1 | `base-movel/storage` |
| [`base-movel/energia/`](base-movel/energia/ORCAMENTO-ENERGIA.md) | Carga, baterias, gerador, os 3 nobreaks | `base-movel/energia` |
| [`base-movel/climatizacao/`](base-movel/climatizacao/ORCAMENTO-CLIMATIZACAO.md) | Carga térmica, corredor quente/frio | `base-movel/climatizacao` |
| [`base-movel/rede/`](base-movel/rede/ORCAMENTO-REDE-SATELITE.md) | Starlink, VLANs, failover 5G | `base-movel/rede` |
| [`base-movel/veiculo/`](base-movel/veiculo/ORCAMENTO-VEICULO-E-PLANTA.md) | Peterbilt 379, **planta baixa**, peso, acabamento | `base-movel/veiculo` |
| [`base-movel/armaria/`](base-movel/armaria/ORCAMENTO-ARMARIA.md) | 30 armas longas + 15 curtas, referências legais | `base-movel/casa` |
| [`base-movel/notebooks/`](base-movel/notebooks/ORCAMENTO-20-NOTEBOOKS.md) | **20 orçamentos de notebook** (10 rodam Hermes, 10 não) | `base-movel/notebooks` |
| [`pc-desktop/`](pc-desktop/) | 🖥️ **PC desktop — 3 orçamentos** (Brasil, EUA, Paraguai) para Ryzen 9 + RTX + 4 TB + water cooler com tela, em full tower ASUS, **com manual de montagem** | `pc-desktop` |
| [`pc-desktop/GUIA-DE-LOJAS.md`](pc-desktop/GUIA-DE-LOJAS.md) | 🛒 **Onde comprar** — qual loja para qual peça nos três países, com as táticas de cada mercado | `pc-desktop` |
| [`compra-notebook/ORCAMENTO-20-NOTEBOOKS-BRASIL.md`](compra-notebook/ORCAMENTO-20-NOTEBOOKS-BRASIL.md) | 🇧🇷 **Dossiê de compra Brasil — 20 orçamentos em reais**: varejo nacional, as três rotas (loja / importação / bagagem), direitos do CDC e Black Friday brasileira | `base-movel/notebooks` |
| [`compra-notebook/ORCAMENTO-20-NOTEBOOKS-COMPRA.md`](compra-notebook/ORCAMENTO-20-NOTEBOOKS-COMPRA.md) | 🇺🇸 **Dossiê de compra EUA — 20 orçamentos em dólar** para escolher 1 até 31/12/2026: custo final com imposto, garantia, timing de Black Friday, custo de upgrade e matriz de decisão | `base-movel/notebooks` |
| [`frota-corporativa/ORCAMENTO-20-PCS.md`](frota-corporativa/ORCAMENTO-20-PCS.md) | **Frota corporativa de 20 PCs** — 3 tiers, montar vs. OEM, TCO 3 anos, canais B2B | `frota-corporativa` |
| [`docs/FONTES-DE-PRECO-2026.md`](docs/FONTES-DE-PRECO-2026.md) | Toda fonte de preço, com data e link | `base-movel/documentacao` |

## Estudos

Material de revisão para prova, com gabarito conferido questão a questão.

| Documento | Cobre | PDF |
|---|---|---|
| [`estudos/REVISAO-LPOO.md`](estudos/REVISAO-LPOO.md) | **LPOO / Java** — os 4 pilares ancorados no código real do [FanVerse](https://github.com/Lucas-Belucci-Bellini/FanVerse) e do [Java-activities](https://github.com/Lucas-Belucci-Bellini/Java-activities), com inventário medido dos 103 arquivos `.java` e as lacunas que sobraram | [`pdf/revisao-lpoo.pdf`](pdf/revisao-lpoo.pdf) |
| [`estudos/REVISAO-SISTEMAS-DIGITAIS.md`](estudos/REVISAO-SISTEMAS-DIGITAIS.md) | **Sistemas Digitais** — Álgebra Booleana, DeMorgan e Mapas de Karnaugh, com as 15 questões resolvidas e gabarito comentado | [`pdf/revisao-sistemas-digitais.pdf`](pdf/revisao-sistemas-digitais.pdf) |
| [`estudos/LISTA-SISTEMAS-DIGITAIS-2.md`](estudos/LISTA-SISTEMAS-DIGITAIS-2.md) | **Sistemas Digitais — caderno 2** — revisão, as 19 questões da lista para resolver em branco, gabarito comentado com os mapas desenhados e 7 exercícios extras | [`pdf/lista-sistemas-digitais-2.pdf`](pdf/lista-sistemas-digitais-2.pdf) |
| [`estudos/REVISAO-SD-BOOLEANA-KARNAUGH.md`](estudos/REVISAO-SD-BOOLEANA-KARNAUGH.md) | **Sistemas Digitais — os slides da disciplina** (Álgebra Booleana, 79 lâminas + Mapa de Karnaugh, 78 lâminas) reorganizados em ordem de estudo, com os 9 exercícios propostos resolvidos, o caso do incinerador fechado ponta a ponta, as 10 pegadinhas e um simulado de 15 questões | [`pdf/revisao-sd-booleana-karnaugh.pdf`](pdf/revisao-sd-booleana-karnaugh.pdf) |
| [`estudos/REVISAO-ALGEBRA-LINEAR.md`](estudos/REVISAO-ALGEBRA-LINEAR.md) | **Álgebra Linear** — os 4 conjuntos de slides (Matrizes I e II, Determinantes, Sistemas Lineares) com **todos os exercícios resolvidos e reconferidos por cálculo independente**, errata dos 3 resultados que não fecham nos slides, e simulado de 20 questões | [`pdf/revisao-algebra-linear.pdf`](pdf/revisao-algebra-linear.pdf) |
| [`estudos/LISTA-ALGEBRA-LINEAR.md`](estudos/LISTA-ALGEBRA-LINEAR.md) | **Álgebra Linear — 40 exercícios novos** (não são os dos slides) com a conta inteira passo a passo: operações elementares linha a linha, triangulação, Laplace, cofatores e discussão de sistema com parâmetro | [`pdf/lista-algebra-linear.pdf`](pdf/lista-algebra-linear.pdf) |
| [`estudos/LISTA-SD-BOOLEANA-KARNAUGH.md`](estudos/LISTA-SD-BOOLEANA-KARNAUGH.md) | **Booleana e Karnaugh — 45 exercícios novos** com resolução passo a passo, mapas K desenhados célula a célula, e os casos em que há **mais de uma solução de custo mínimo** | [`pdf/lista-sd-booleana-karnaugh.pdf`](pdf/lista-sd-booleana-karnaugh.pdf) |

### PDFs

- [`pdf/orcamento-base-movel.pdf`](pdf/orcamento-base-movel.pdf) — 37 páginas, tudo da base móvel
- [`pdf/orcamento-notebooks.pdf`](pdf/orcamento-notebooks.pdf) — 10 páginas, os 20 notebooks
- [`pdf/orcamento-frota-20-pcs.pdf`](pdf/orcamento-frota-20-pcs.pdf) — 10 páginas, a frota corporativa de 20 estações
- [`pdf/pc-orcamento-brasil.pdf`](pdf/pc-orcamento-brasil.pdf) · [`-eua`](pdf/pc-orcamento-eua.pdf) · [`-paraguai`](pdf/pc-orcamento-paraguai.pdf) — PC desktop, 3 orçamentos com manual de montagem
- [`pdf/pc-guia-de-lojas.pdf`](pdf/pc-guia-de-lojas.pdf) — onde comprar cada peça nos três países
- [`pdf/compra-20-notebooks-brasil.pdf`](pdf/compra-20-notebooks-brasil.pdf) — 17 páginas, o dossiê de compra no **Brasil**
- [`pdf/compra-20-notebooks.pdf`](pdf/compra-20-notebooks.pdf) — 19 páginas, o dossiê de compra do notebook nos **EUA**
- [`pdf/revisao-algebra-linear.pdf`](pdf/revisao-algebra-linear.pdf) — 30 páginas, Álgebra Linear com errata e simulado
- [`pdf/revisao-sd-booleana-karnaugh.pdf`](pdf/revisao-sd-booleana-karnaugh.pdf) — 16 páginas, Álgebra Booleana + Mapa de Karnaugh
- [`pdf/lista-algebra-linear.pdf`](pdf/lista-algebra-linear.pdf) — 25 páginas, 40 exercícios de Álgebra Linear resolvidos
- [`pdf/lista-sd-booleana-karnaugh.pdf`](pdf/lista-sd-booleana-karnaugh.pdf) — 20 páginas, 45 exercícios de Booleana/Karnaugh resolvidos

Gerados por [`scripts/md2pdf.py`](scripts/md2pdf.py) (fpdf2, sem pandoc/LaTeX):

```bash
python3 scripts/md2pdf.py pdf/saida.pdf "Título" "Subtítulo" arquivo.md [...]

# a nota de rodapé da capa é configurável (o padrão é a de orçamento, em USD/EUA):
python3 scripts/md2pdf.py --nota "Material de estudo." pdf/saida.pdf "Título" "Sub" arquivo.md
```

### ⚠️ Contexto de mercado: 2026 é um ano ruim para comprar

GPU e memória estão em crise de preço simultânea por causa da demanda de IA:
a RTX 5090 está **+126%** sobre o MSRP e o kit DDR5 de 64 GB **+325%**. No
escopo deste projeto isso representa **~$20.700 de sobrepreço**, e a previsão
de alívio é **meados de 2027**. Detalhes e a recomendação de faseamento estão
no [orçamento mestre](base-movel/ORCAMENTO-2026.md#-leia-isto-antes-de-qualquer-coisa-2026-é-o-pior-ano-da-década-para-comprar).

## Compra e afiliados

- Comparadores de preço e histórico devem ser usados para encontrar o menor preço real antes da compra.
- Programas de afiliados devem ser usados apenas de acordo com os termos de cada plataforma.
- O projeto deve registrar preço, loja, data/hora da verificação e link normal/oficial.
- Um link de afiliado nunca deve ser apresentado como se fosse um preço melhor por si só.

## Regra

Cada branch deve conter somente o planejamento e os arquivos diretamente relacionados ao seu subsistema. Integrações que afetem mais de uma área devem ser documentadas em `projeto/base-movel` antes de serem incorporadas.
