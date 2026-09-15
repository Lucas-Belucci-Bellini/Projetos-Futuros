# Rede — arquitetura e checklist

## Objetivo

Oferecer rede interna estável, segmentada e independente da internet externa.

## Topologia

SAT/5G -> Firewall -> Core Switch -> VLANs de servidores, storage, notebooks, casa, convidados e administração.

## Requisitos

- 2.5/10 GbE para usuários e serviços
- 25 GbE quando o fluxo IA/storage justificar
- VLAN e firewall entre zonas
- VPN para acesso remoto
- DNS interno
- monitoramento
- failover de internet

## Checklist

[ ] mapa de portas
[ ] endereçamento IP
[ ] VLANs
[ ] DHCP/DNS
[ ] firewall
[ ] Wi-Fi
[ ] VPN
[ ] QoS
[ ] failover
[ ] documentação de credenciais e recuperação
