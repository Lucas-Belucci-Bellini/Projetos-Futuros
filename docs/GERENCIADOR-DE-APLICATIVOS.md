# Gerenciador de Aplicativos

## Objetivo
Criar uma experiência única para instalar, atualizar, remover e reparar aplicativos sem obrigar o usuário a entender vários formatos de pacote.

## Modelo
- Pacotes nativos assinados.
- Flatpak como formato de sandbox de referência.
- Repositórios por níveis: Core, Verified, Community.
- Metadados de permissões antes da instalação.
- Rollback de atualização quando suportado.

## UX
Cada aplicativo mostra: origem, desenvolvedor, versão, permissões, espaço usado, atualizações disponíveis e grau de confiança do repositório.

## Desenvolvedor
Oferecer SDK/CLI para empacotamento e validação de aplicativos.

## Segurança
- Verificação criptográfica.
- Evitar scripts de instalação arbitrários como método principal.
- Repositórios com assinatura e cadeia de confiança.
- Política de atualização de dependências.
