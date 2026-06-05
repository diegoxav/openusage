# OpenUsage KDE Plasma Widget

Este é um widget (Plasmoid) para o **KDE Plasma** que mostra informações de limites e cotas de uso do OpenUsage (focado no **Antigravity CLI** e outros provedores) diretamente no seu painel (menu bar).

## Como Funciona
1. O widget realiza uma requisição HTTP a cada 15 segundos para `http://127.0.0.1:6736/v1/usage` para obter as estatísticas de uso em tempo real.
2. Ele exibe um ícone e o percentual de uso da ferramenta ativa (ex: `Agy: 74%`).
3. Ao clicar no widget, ele faz uma requisição POST para `http://127.0.0.1:6736/v1/toggle-window`, que sinaliza o aplicativo Tauri para abrir/exibir a interface gráfica rica logo abaixo do widget.

## Instalação

Abra o terminal na pasta deste widget (`kde-plasma-widget/`) e execute o comando correspondente à sua versão do Plasma:

### No Plasma 6 (CachyOS, Fedora 40+, Arch recente):
```bash
kpackagetool6 -t Plasma/Applet -i .
```

### No Plasma 5:
```bash
kpackagetool5 -t Plasma/Applet -i .
```

### Como Adicionar ao Painel:
1. Clique com o botão direito no painel (barra superior/menu bar) do seu KDE Plasma e selecione **"Entrar no Modo de Edição"** (Enter Edit Mode).
2. Clique em **"Adicionar Widgets..."** (Add Widgets).
3. Busque por **"OpenUsage"**.
4. Arraste e solte o widget no canto superior direito do seu painel, próximo aos outros ícones de bandeja.
5. Saia do modo de edição.
