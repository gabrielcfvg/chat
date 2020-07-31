const { ipcRenderer } = require('electron')
const label = [document.getElementById("nicknameLabel"), document.getElementById("passwordLabel"), document.getElementById("passwordConfirmLabel")]
const input = [document.getElementById("nickname"),      document.getElementById("password"),      document.getElementById("passwordConfirm")     ]
const error = [document.getElementById("nicknameError"), document.getElementById("passwordError"), document.getElementById("passwordConfirmError")]

var passwordMatch = true;

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
    if (!passwordMatch) {
        error[2].innerHTML = "Senhas não coincidem";
        error[2].classList.add("visible");
        return 1;
    }
    returnValue = ipcRenderer.sendSync("login", {
        name: input[0].value,
        password: input[1].value,
        operation: 1
    });
    switch (returnValue) {
        case 3:
            location.href="login.html";
            break;
        case 4:
            error[0].innerHTML = "O usuário já existe";
            error[0].classList.add("visible");
            break;
    }
}

function noVisible(id) {
    error[id].classList.remove("visible");
}

function passwordLiveValidation() {
    var password = input[1].value;
    if (password.length < 8) {
        error[1].innerHTML = "Senha fraca, menor que 8 caracteres";
        error[1].classList.add("visible");
    } else {
        error[1].classList.remove("visible")
    }
}

function passwordConfirmLiveValidation() {
    var password = input[1].value;
    var passwordConfirm = input[2].value;

    if (password != passwordConfirm) {
        console.log(password, passwordConfirm)
        error[2].innerHTML = "As senhas não coincidem";
        error[2].classList.add("visible");
        passwordMatch = false;
    } else {
        error[2].classList.remove("visible");
        passwordMatch = true;
    }
}

writeText();
