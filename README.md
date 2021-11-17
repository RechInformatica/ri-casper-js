<h1><center>ri-casper-js - Wrapper para executar o Casper JS</h1>

O CasperJS é um aplicativo open source para execução de Web Scraping.

Esse projeto foi criado com o objetivo de permitir disparar a execução do CasperJS em ambientes de clientes com servidor Linux. Dessa forma, esse projeto serve somente para fazer a ponte para executar o CasperJS.

O projeto do CasperJS possui um disparador para sistemas operacionais Linux, porém é um script Python.
Como não queremos depender de ter o Python instalado nos clientes, reescrevemos o wrapper disponibilizado pela comunidade na linguagem Rust. Dessa forma, podemos compilar o projeto gerando um binário para execução em Linux sem necessitar de nenhuma outra dependência.

Atualmente a nossa aplicação só funciona para ambiente com sistema operacional Linux, pois a única biblioteca que encontramos para executar o comando execvp está preparada somente para funcionar em Linux.

No script Python disponibilizado pela comunidade, é utilizado um método 'execvp' da biblioteca 'os' do Python.
Esse método serve para substituir o processo atual por um novo processo. Nesse caso, simplesmente criar um novo processo filho do atual não iria resolver o nosso problema. Devido a isso, estamos utilizando a biblioteca 'exec' que permite mantermos o mesmo comportamento do script Python.

<h2>Como buildar o projeto?</h2>

Caso seja necessáro buildar o projeto, pode ser feito através do seguinte comando:

```
cargo build --release --target x86_64-unknown-linux-musl
```

<h2>Alterações</h2>

Caso seja necessário fazer alguma alteração no projeto, é necessário buildar o projeto e atualizar o binário 'ri-casper-js' distribuído no zip CasperJS.7z. O zip se encontra 'F:\SIGER\\<versao_siger>\lib\CasperJs.7z'.

