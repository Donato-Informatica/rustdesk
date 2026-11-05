# Donato - Suporte: GitHub Actions

Workflow manual: `.github/workflows/donato-windows.yml`, nome **Donato - Windows x64**.
Reutiliza a preparação e as versões do job Windows original, a geração da bridge
no CI e o helper de elevação. Compila somente o cliente Windows x64, executa os
testes Rust do job original, gera EXE portátil e MSI instalável. Não usa secrets
de assinatura nem publica Releases. A bridge compartilhada também gera a variante
ARM usada pelo upstream; isso não gera outro cliente Donato.

## Enviar o código pelo Ubuntu

O remoto local já é https://github.com/Donato-Informatica/rustdesk.git.
Confirme que você tem permissão de escrita e está autenticado no GitHub.
Antes do push, desabilite no fork os workflows upstream que executam por push,
pull request ou agendamento, caso não queira compilar as outras plataformas.
Mantenha habilitado o workflow Donato; os workflows bridge e third-party são
chamados por ele. Não altere os workflows do repositório original RustDesk.

No terminal:

```bash
cd ~/git/rustdesk
git status --short
git diff --check
git add .gitignore .github/workflows/donato-windows.yml Cargo.toml libs/portable/Cargo.toml src/donato.rs src/lib.rs src/common.rs src/platform/mod.rs flutter/lib/desktop/pages/desktop_setting_page.dart flutter/windows/runner/Runner.rc flutter/windows/runner/resources/app_icon.ico res/icon.ico res/icon.png res/tray-icon.ico res/msi/preprocess.py flutter/assets/icon.ico flutter/assets/icon.png flutter/assets/logo.png flutter/assets/logo_light.png flutter/assets/logo_dark.png docs/DONATO-WINDOWS.md
git diff --cached --stat
git commit -m "Personaliza cliente Windows Donato - Suporte"
git push origin master
```

Confira as alterações preparadas antes de confirmar o commit. Os comandos acima
são instruções; o assistente não fez commit nem push. Se master for protegida,
use uma branch e faça um pull request para master no SEU fork. O workflow precisa
estar na branch padrão para aparecer com o botão de execução manual.

## Executar e baixar

1. Abra https://github.com/Donato-Informatica/rustdesk/actions.
2. Habilite Actions no fork se o GitHub solicitar. Selecione **Donato - Windows x64**.
3. Clique **Run workflow**, selecione master e confirme **Run workflow**.
4. Aguarde bridge, helper e Windows. A primeira compilação pode demorar bastante
   devido às dependências nativas. Verifique limites de minutos da sua conta.
5. Na execução concluída, abra **Artifacts** e baixe **Donato-Suporte-Windows-x64**.
6. Extraia o ZIP: `Donato-Suporte-portatil.exe`,
   `Donato-Suporte-instalador.msi` e `SHA256SUMS.txt`.
7. Se falhar, abra o primeiro passo vermelho e compartilhe o trecho do erro.
   Não há resultado Windows validado até uma execução bem-sucedida.

## Configuração e validação

Nome: Donato - Suporte. Site: https://www.donato.inf.br.
Servidor ID e relay: rustdesk.apps.donato.inf.br. A chave PÚBLICA fornecida está
em src/donato.rs. Modo nativo incoming: interface reduzida e bloqueio de saída.
Não foi definida senha permanente; o comportamento de autorização original é
preservado. Em instalações existentes, confira as preferências de segurança.

Teste ambos numa VM Windows limpa: nome, ícone, logos nos dois temas, ID online,
conexão pelo técnico direta e via relay, solicitação de autorização, elevação/UAC,
instalação MSI, serviço após reiniciar e desinstalação. Os serviços ID/relay precisam
estar acessíveis nas portas RustDesk; HTTPS sozinho não comprova funcionamento.

Os arquivos sairão sem assinatura digital nesta etapa. Preserve LICENCE e os
avisos upstream. Nenhuma compilação foi iniciada pelo assistente.

## Superfície das alterações

- src/donato.rs, src/lib.rs e src/common.rs: inicialização Windows do nome,
  servidores e modo incoming, usando os mapas nativos de configuração.
- src/platform/mod.rs: permite somente o nome fixo Donato com espaços no
  instalador Windows; os demais nomes mantêm a validação original. A verificação
  foi adicionada ao teste de nomes existente para execução no CI Windows.
- flutter/lib/desktop/pages/desktop_setting_page.dart: site no Sobre do Windows.
- res/msi/preprocess.py: consulta versão e data do executável por argumentos
  separados, permitindo o caminho com espaços exigido pelo nome comercial.
- Runner.rc e metadados Cargo do app/portable: nome comercial nos executáveis.
- Ícones e assets: identidade visual fornecida, com logos claro/escuro.
- .gitignore: inclui os assets de marca no envio; sem isso eram ignorados.
- donato-windows.yml: fluxo manual independente para gerar os dois pacotes.

Nenhum submódulo foi alterado. A exceção de nome e as configurações em Rust são
exclusivas do Windows; o site mantém o comportamento anterior em outras plataformas.
Os assets e metadados de marca pertencem a este fork personalizado.
