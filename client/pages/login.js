const { ipcRenderer } = require('electron')
const label = [document.getElementById("nicknameLabel"), document.getElementById("passwordLabel")]
const input = [document.getElementById("nickname"), document.getElementById("password")]
const error = [document.getElementById("nicknameError"), document.getElementById("passwordError")]

function labelFocus(id) {
    let obj = label[id]
    obj.style.transform = "translate(5vh, 0)"
}

function labelBlur(id) {
    if (input[id].value.length <= 0) {
        let obj = label[id]
        obj.style.transform = "translate(5vh, 5.5vh)"
    }
}

function logIn() {
    returnValue = ipcRenderer.sendSync("login", {
        name: input[0].value,
        password: input[1].value,
        operation: 0
    });
    switch (returnValue) {
        case 0:
            location.href="main.html";
            break;
        case 1:
            error[1].innerHTML = "Senha incorreta";
            error[0].classList.remove("visible")
            error[1].classList.add("visible");
            break;
        case 2:
            error[0].innerHTML = "O usuário não existe";
            error[0].classList.add("visible");
            error[1].classList.remove("visible");
            break;
    }
}

function noVisible(id) {
    error[id].classList.remove("visible");
}


