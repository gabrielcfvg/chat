const { app, BrowserWindow } = require("electron")

function createWindow() {
    let win = new BrowserWindow({
        width: 800, // Largura da tela
        height: 600, // Altura da tela
        webPreferences: {
            nodeIntegration: false 
        }
    })
    win.loadFile("pages/main.html") // Carrega a página
    win.removeMenu() // Tira o menu superior
}
app.whenReady().then(createWindow) // Promessa de criação da tela

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
