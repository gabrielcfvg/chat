const { app, BrowserWindow, ipcMain, globalShortcut } = require("electron");
const { savePreferences, backupMessages, viewed } = require('./functions');

const storage = require('electron-json-storage')
var win;
app.whenReady().then(() => {
    win = new BrowserWindow({
        width: 800, // Largura da tela
        height: 600, // Altura da tela
        webPreferences: {
            nodeIntegration: true
        },
        backgroundColor: "#0000",
        minHeight: 300,
        minWidth: 300
    })
    win.loadFile("pages/main.html") // Carrega a página
    // win.removeMenu() // Tira o menu superior

    globalShortcut.register('CommandOrControl+B', backupMessages) // Faz backup com Ctrl+B
}) // Promessa de criação da tela

app.on('window-all-closed', () => {
    // MacOS é estranho
    if (process.platform !== 'darwin') {
        app.quit()
    }
})

app.on('activate', () => {
    // Muito estranho
    if (BrowserWindow.getAllWindows().length === 0) {
        createWindow()
    }
})



// IPC 

ipcMain.on('save-preferences', savePreferences);

ipcMain.on('try-connect', (event, arg) => {
    event.returnValue = login_err
})

ipcMain.on('viewed', viewed)

// ######################################################################
// #                                                                    #
// #                              REDES                                 #
// #                                                                    #
// ######################################################################


const net = require("net");
const crypto = require("crypto");
const { Socket } = require("dgram")

const encoder = new TextEncoder();

let login_status = false;
let socket;

const PORTA = 1234;
const IP = "127.0.0.1";
var dados = {
    name: "teste",
    password: "senha",
    operation: 0
}

let login_ready, login_res, login_err;
async function MessageFunction(message) {
    while (!win) await new Promise(r => setTimeout(r, 100));
    win.webContents.on('did-finish-load', function() {
        win.webContents.send("MessageFunction", message);
    });
};

ipcMain.on('login', login);
ipcMain.on("send_message", send_message);
ipcMain.on("MessageFunction", (event, arg) => MessageFunction = arg);
ipcMain.on("reconnect", (event, args) => reconnect())


function connect() {
    let new_socket = new net.Socket;
    new_socket.connect({ host: IP, port: PORTA }, () => {
        console.log("conectado com sucesso!!!");
        //socket.write('{"type": 2}');
    });
    
    new_socket.on('error', () => {
        login_err = true;
    });
    
    new_socket.on('data', data => {
    
        if (!login_status) {
            socket_login(data);
        }
        else {
            socket_client(data);
        }
    });
    return new_socket;
}


var sleep = ms => new Promise(resolve => setTimeout(() => resolve(true), ms));

async function login(event, arg) {

    if (login_status) { event.returnValue = 666; return; }

    dados = {...dados, ...arg};

    login_ready = false;
    socket.write('{"type": 2}');

    while ( !login_ready && await sleep(100) ) console.log("ciclo de sleep") // debug

    event.returnValue = login_res;
}

function socket_login(data) {
    if (!data) return;

    let pacote = JSON.parse(data.toString());

    if (pacote.type === 0) {

        let chave = new Buffer.from(pacote.content, "base64").toString(); // decodificação da chave pública de base64 pra bytes

        let saida = encoder.encode(JSON.stringify({
            type: 0,
            content: dados
        })); // pacote a ser enviado ao servidor

        socket.write(crypto.publicEncrypt({
            key: chave,
            padding: crypto.constants.RSA_PKCS1_PADDING
        }, saida)); // envio do pacote para o servidor

    } else if (pacote.type === 1) {

        if (pacote.content === 0) {
            login_status = true;
            socket.write('{"type": 20, "content": 1}')
        }

        login_ready = true;
        login_res = pacote.content;

        console.log(">>>" + pacote.content.toString());

    }
}

function socket_client(data) {

    console.log("pacote recebido");
    if (!data) return; 

    for(raw_package of data.toString().split("|")) {

        if (!raw_package) continue;

        let package = JSON.parse(raw_package);

        switch (package.type) {
            case 10: {
                MessageFunction(package.content);

                console.log(package.content.autor);
                console.log(package.content.message);
                console.log("-----------------------");
            }
        }
    }
}

function send_message(event, arg) {

    if (!login_status) return;

    socket.write(JSON.stringify({
        type: 10,
        content: arg
    }));
}

function reconnect() {
    socket = connect();
}


socket = connect();