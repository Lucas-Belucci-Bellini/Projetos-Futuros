# Pacote de Hardware

## Objetivo

Definir uma matriz de compatibilidade para que o SO seja testado em hardware real antes de cada release.

## Requisitos mínimos de referência

- CPU x86-64 ou arquitetura oficialmente suportada;
- 8 GB RAM;
- 64 GB de armazenamento para protótipo, com espaço recomendado maior;
- UEFI;
- GPU compatível com a stack gráfica escolhida.

## Recomendado

- 16–32 GB RAM;
- SSD NVMe;
- TPM 2.0;
- Secure Boot;
- GPU AMD/NVIDIA/Intel suportada.

## Workstation / Base Móvel

- 64 GB+ RAM;
- múltiplas GPUs quando suportadas pelo kernel/driver;
- 10/25 GbE;
- vários NVMe;
- múltiplos monitores;
- virtualização.

## Classes de hardware

```text
S0 = protótipo
S1 = desktop comum
S2 = gamer
S3 = workstation
S4 = servidor/laboratório
```

Cada release deve registrar:

- hardware validado;
- driver usado;
- firmware/UEFI testado;
- regressões conhecidas.

## GPU NVIDIA

Não prender o sistema a uma versão específica de driver. A matriz de teste deve acompanhar versões estáveis compatíveis com o kernel e com CUDA quando IA local for requisito.

## GPU AMD

Priorizar Mesa/DRM e componentes upstream sempre que possível.

## Wi-Fi/Bluetooth

Manter uma lista de chipsets testados e um modo diagnóstico para identificar dispositivos sem driver.

## Impressoras/periféricos

Suporte deve priorizar padrões abertos e drivers do kernel/CUPS quando disponíveis.

## Firmware

Nunca atualizar firmware silenciosamente. Exibir:

- versão atual;
- versão nova;
- origem;
- motivo;
- risco/consequência;
- possibilidade de recuperação.
