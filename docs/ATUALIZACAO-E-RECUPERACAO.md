# Atualização e Recuperação

## Objetivo
Atualizações previsíveis, reversíveis e resistentes a falhas de energia, disco ou pacote.

## Modelo desejado
- Sistema base com imagem/árvore versionada.
- Atualização atômica quando possível.
- Partição/slots de recuperação ou mecanismo equivalente.
- Snapshot antes de grandes mudanças.
- Rollback simples pelo menu de boot.

## Fluxo
1. Verificar assinatura e integridade.
2. Baixar nova versão.
3. Validar espaço e compatibilidade.
4. Aplicar mudança sem destruir o sistema atual.
5. Reiniciar.
6. Confirmar saúde do boot e serviços.
7. Marcar nova versão como estável somente após sucesso.

## Falha
Se a nova versão não inicializar corretamente, retornar automaticamente para a última versão saudável.

## Recovery
- Modo gráfico de recuperação.
- Terminal avançado.
- Reparo de boot.
- Restauração de snapshot.
- Reinstalação sem apagar `/home` quando possível.
- Diagnóstico exportável.

## Offline
Uma ISO de recuperação deve conseguir restaurar o sistema sem internet.
