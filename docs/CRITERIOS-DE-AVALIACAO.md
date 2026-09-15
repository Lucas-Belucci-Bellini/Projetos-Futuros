# Critérios de avaliação dos notebooks

## 1. Desempenho

| Critério | Peso |
|---|---:|
| CPU / compilação | 15% |
| GPU / jogos / 3D | 20% |
| VRAM / IA local | 20% |
| RAM / multitarefa | 10% |
| SSD / expansão | 5% |
| Refrigeração / carga sustentada | 10% |
| Tela | 5% |
| Mobilidade / peso | 5% |
| Bateria | 5% |
| Manutenção / reparabilidade | 5% |

## 2. Classificação de uso

### Nível A — leve
Navegação, documentos, programação simples, reuniões e estudo.

### Nível B — desenvolvimento
IDEs, compilação, Docker/WSL, bancos locais, ferramentas web e game dev.

### Nível C — pesado
Unreal/Unity, Blender, edição, VMs, Minecraft extremamente modificado e compilação grande.

### Nível D — IA local
Modelos quantizados, embeddings, visão, automações e agentes locais dentro da memória/VRAM disponíveis.

### Nível E — servidor
Treinamento grande, inferência pesada concorrente, workloads 24/7 e modelos que excedem a VRAM/RAM do notebook. **Não é o objetivo de nenhum dos 10 notebooks.**

## 3. Minecraft / ATM10

Para o projeto, o notebook deve ser avaliado com:

- 32 GB de RAM;
- Java com memória configurada corretamente;
- distância de renderização e shaders controlados;
- monitoramento de temperatura;
- armazenamento NVMe com espaço livre suficiente.

“200 mods extras” é um cenário de teste, não uma garantia de FPS. O resultado depende da versão dos mods, shaders, resolução, CPU, GPU, RAM e otimizações.

## 4. Game Dev

Avaliar separadamente:

- editor Unreal/Unity;
- shader compilation;
- importação de assets;
- Blender;
- geração de lightmaps;
- builds Windows;
- multitarefa com navegador, IDE e ferramentas de arte.

## 5. IA / Hermes

Registrar:

- VRAM total;
- RAM disponível;
- tamanho de contexto suportado pela ferramenta;
- quantização utilizada;
- número de modelos/processos concorrentes;
- velocidade observada no teste;
- temperatura durante 30/60/120 minutos;
- consumo aproximado.

Nunca declarar que o notebook “aguenta qualquer modelo”. O documento deve informar o **limite medido/testado**.

## 6. Bateria

Registrar quatro valores:

1. Wh da bateria;
2. autonomia declarada pelo fabricante;
3. autonomia medida em teste leve;
4. autonomia medida em teste pesado.

Fazer teste com brilho, rede e perfil de energia documentados para tornar as medições comparáveis.

## 7. Temperatura

Teste padrão futuro da Base:

- 15 min idle;
- 30 min desenvolvimento;
- 60 min jogo;
- 60 min carga CPU/GPU;
- 120 min carga combinada quando seguro e autorizado pelo fabricante.

Registrar CPU/GPU máxima e média, ruído quando houver instrumento, throttling e estabilidade.

## 8. Durabilidade

A nota de 1–5 deve considerar:

- qualidade estrutural;
- dobradiças;
- teclado/touchpad;
- refrigeração;
- facilidade de limpeza;
- acessibilidade de RAM/SSD;
- disponibilidade de peças;
- custo de placa-mãe;
- histórico de manutenção, quando houver evidência.

Não usar “vai durar X anos” como fato. Utilizar cenários de 3, 5 e 7 anos.

## 9. Reparabilidade

A nota de reparabilidade deve considerar:

**5/5:** peças modulares, documentação/service guide e baixo custo relativo.

**4/5:** boa manutenção, algumas peças específicas.

**3/5:** reparo possível, mas vários componentes dependem de conjunto proprietário.

**2/5:** desmontagem ou peças caras/difíceis.

**1/5:** grande parte do equipamento depende de placa-mãe proprietária e reparo caro.

## 10. Checklist antes da compra

- Part number exato confirmado.
- RAM máxima confirmada no manual.
- SSD e número de slots confirmados.
- Tipo de tela confirmado.
- Wh da bateria confirmado.
- TGP da GPU confirmado quando disponível.
- Fonte e potência confirmadas.
- Garantia local confirmada.
- Assistência autorizada identificada.
- Preço novo e preço de peças registrados.
- TCO de 5 anos calculado.

## 11. Regra de evidência

Cada dado deve ser classificado como:

- **FABRICANTE** — ficha oficial/manual.
- **VAREJO** — página de venda.
- **MEDIDO** — teste realizado pela equipe.
- **ESTIMADO** — cálculo/faixa para planejamento.
- **A CONFIRMAR** — ainda não há evidência suficiente.

Atualizado em 14/09/2026.
