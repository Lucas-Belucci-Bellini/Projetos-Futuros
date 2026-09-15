# Arquitetura de Implementação

## Camadas
1. Firmware/UEFI e Secure Boot.
2. Bootloader e mecanismo de recuperação.
3. Kernel Linux + drivers.
4. Init e serviços base.
5. Rede, áudio, Bluetooth e dispositivos.
6. Display server/protocolo Wayland.
7. Desktop Shell.
8. Daemons do sistema.
9. Package/App Manager.
10. APIs e SDK.
11. Aplicações padrão.

## Linguagens
- Rust para novos componentes privilegiados quando apropriado.
- C/C++ somente quando exigido por interfaces existentes, kernel ou bibliotecas.
- TypeScript/JavaScript para partes de UI quando tecnicamente adequado.
- Python somente para automação/desenvolvimento auxiliar, evitando dependências críticas no boot.

## Modularidade
Componentes críticos devem possuir interfaces estáveis e testes próprios. Evitar um monólito de desktop.

## IPC
Priorizar mecanismos Linux maduros e APIs bem definidas. Processos privilegiados devem ter superfícies mínimas.

## Observabilidade
- logs estruturados;
- journal;
- métricas de CPU/GPU/RAM/disco/rede;
- diagnóstico exportável;
- coleta local com consentimento.

## API do sistema
Projetar uma API local documentada para gerenciamento de energia, rede, apps, atualizações, segurança e hardware.
