# PC-03 — Compatibilidade e montagem

## Checklist
- Confirmar CPU e BIOS compatíveis com a placa-mãe.
- Confirmar 128 GB de RAM na lista QVL do fabricante quando possível.
- Separar NVMe de sistema do armazenamento de mundos ativos.
- Planejar 10 GbE.
- Definir se os 32 TB serão totalmente locais ou parte deles ficarão no STORAGE-01.

## Montagem
1. Instalar CPU, RAM e NVMe de sistema.
2. Instalar o cooler e verificar pressão/contato.
3. Instalar SSD/NVMe para mundos ativos.
4. Montar a placa no gabinete e ligar alimentação.
5. Instalar Ubuntu Server.
6. Instalar Java conforme o servidor/modpack escolhido.
7. Configurar Docker ou serviço dedicado.
8. Configurar backup automático no STORAGE-01.

## Teste de aceitação
- Memória completa reconhecida.
- Mundo inicia sem erros.
- Teste de jogadores simultâneos definido pelo projeto.
- Latência e TPS monitorados.
- SSD sem erros.
- Backup e restauração testados.
- Reinicialização do servidor não afeta os outros nós.
