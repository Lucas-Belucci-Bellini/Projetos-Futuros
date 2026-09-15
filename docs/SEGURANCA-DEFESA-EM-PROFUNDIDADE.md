# Segurança — Defesa em Profundidade

## Meta
Construir um sistema com camadas independentes, de modo que a falha de uma camada não entregue automaticamente o sistema inteiro.

## Cadeia de confiança
UEFI → Secure Boot → bootloader assinado → kernel assinado → initramfs verificado → serviços mínimos → sessão gráfica.

## Hardware e boot
- Secure Boot como configuração padrão quando o hardware suporta.
- TPM 2.0 para proteção de chaves e measured boot quando disponível.
- Proteção contra alteração não autorizada do boot.
- Recovery independente do sistema instalado.

## Identidade
- Contas de usuário sem privilégios administrativos por padrão.
- Elevação explícita para tarefas administrativas.
- MFA opcional/forte para contas sensíveis.
- Chaves de recuperação separadas e documentadas.

## Aplicações
- Sandboxing por padrão para apps não confiáveis.
- Permissões para câmera, microfone, arquivos, rede, USB e localização.
- Pacotes assinados e repositórios confiáveis.
- Verificação de integridade antes da execução quando tecnicamente viável.

## Kernel e serviços
- Minimizar serviços ativos.
- Hardening configurável.
- AppArmor ou mecanismo equivalente como referência inicial.
- Seccomp, namespaces, cgroups e capabilities para isolamento.
- Superfície de ataque documentada.

## Gaming e integridade
O objetivo inspirado pelo rigor do VAC é proteger a integridade do ambiente do usuário e dos jogos sem copiar implementação proprietária. O sistema poderá oferecer um serviço de integridade opcional para software compatível, usando assinaturas, atestação e verificação de arquivos. Não deve funcionar como ferramenta de vigilância oculta.

## Atualizações
- Artefatos assinados.
- Atualização transacional/atômica.
- Rollback automático quando o boot de uma versão falhar.
- Proteção contra downgrade inseguro.

## Detecção e resposta
- Auditoria local.
- Alertas claros.
- Histórico de eventos de segurança.
- Isolamento de aplicação suspeita quando possível.
- Recovery offline.

## Princípios de privacidade
- Telemetria desligada por padrão quando possível.
- Dados locais minimizados.
- Nenhum envio secreto de conteúdo pessoal.
- Logs sensíveis com retenção configurável.

## Limites
Segurança deve ser tratada como processo contínuo. O projeto precisa de threat modeling, testes, fuzzing, revisão de dependências, atualizações de terceiros e auditoria antes de alegar um nível elevado de segurança.
