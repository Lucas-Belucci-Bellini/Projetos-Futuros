# Storage — arquitetura e checklist

## Objetivo

Manter uma nuvem local com redundância, snapshots, backups e expansão previsível.

## Estrutura

- pool principal ZFS
- datasets por projeto
- SSD/NVMe para sistema/metadados quando necessário
- SMB/NFS/S3
- snapshots
- backup externo
- monitoramento SMART e temperatura

## Meta

Projetar a capacidade como **bruta, utilizável e reserva de expansão**. A meta atual do projeto é 258 TB úteis ou mais, sem confundir capacidade bruta com capacidade disponível após redundância.

## Checklist

[ ] modelo dos HDDs
[ ] quantidade
[ ] redundância
[ ] capacidade útil
[ ] spare
[ ] SSD/NVMe
[ ] backup
[ ] snapshots
[ ] criptografia
[ ] 10/25 GbE
[ ] teste de restauração
