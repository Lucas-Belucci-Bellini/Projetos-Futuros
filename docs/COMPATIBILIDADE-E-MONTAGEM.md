# PC-02 — Compatibilidade e montagem

## Checklist crítico
- Placa-mãe WRX90 compatível com o Threadripper PRO escolhido.
- Confirmar quantidade e distribuição das pistas PCIe para 4 GPUs.
- Confirmar suporte a 128/256 GB ECC e módulos exatos.
- Medir largura/espessura de cada GPU.
- Confirmar conectores e capacidade total da fonte.
- Planejar entrada e saída de ar antes da montagem.
- Reservar NVMe separado para sistema e para dados temporários.

## Ordem recomendada
1. Escolher plataforma CPU + placa-mãe + memória ECC.
2. Definir o modelo exato das quatro GPUs.
3. Dimensionar gabinete e refrigeração pelas medidas reais.
4. Dimensionar fonte pelo consumo máximo especificado pelos fabricantes.
5. Instalar CPU, RAM e NVMe.
6. Instalar GPUs deixando espaço adequado ao fluxo de ar.
7. Instalar Ubuntu e stack NVIDIA/CUDA.
8. Testar cada GPU individualmente e depois as quatro em conjunto.

## Teste de aceitação
- Todas as GPUs aparecem no sistema.
- Memória ECC reconhecida.
- CUDA funcionando nas quatro GPUs.
- Teste de carga individual e conjunto.
- Monitoramento de temperatura, potência e estabilidade.
- Rede 25 GbE validada.
- Acesso ao STORAGE-01 validado.

## Arquitetura de software
```text
Ubuntu
├── NVIDIA Driver
├── CUDA
├── Docker
├── PyTorch
├── Hermes
├── APIs internas
└── datasets -> STORAGE-01
```
