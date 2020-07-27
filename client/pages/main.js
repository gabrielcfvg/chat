var activeUser = -100;

var activeType = "none"
var active = -1;

var rightSide = document.getElementById("rightSide")
var configIcon = document.getElementById("configIcon")
var alternateIcon = document.getElementById("alternateIcon")

var leftMenuDiv = document.getElementById("leftMenuDiv")

var time1 = getComputedStyle(document.documentElement).getPropertyValue('--t1');

function selectActive(type, id) {
    if (activeType === "config")
        configIcon.classList.remove("active")
    else if (activeType === "user")
        document.getElementById("user" + active).classList.remove("active")

    if (active != id || activeType != type) {
        active = id
        activeType = type
    } else {
        active = -1
        activeType = "none"
    }

    if (activeType === "config")
        configIcon.classList.add("active")
    else if (activeType === "user")
        document.getElementById("user" + active).classList.add("active")


    if (activeType === "none")
        rightSide.innerHTML = `<h1 class="ns">Você não selecionou nenhum chat</h1>
        <h2 class="ns">Selecione um canal ou uma conversa</h2>`
    else
        rightSide.innerHTML = ""
}

function selectActiveUser(id) { // DEPRECATED, remove later
    document.getElementById("rightSide").innerHTML = ""
    if (activeUser === -1)
        document.getElementById("configIcon").classList.remove("active")
    else if (activeUser === -100)
        document.getElementById("rightSide").innerHTML = `<h1 class="ns">Você não selecionou nenhum chat</h1><h2 class="ns">Selecione um canal ou uma conversa</h2>`
    else
        document.getElementById("user" + activeUser).classList.remove("active")

    if (id === activeUser)
        activeUser = -100
    else
        activeUser = id;

    if (activeUser === -1)
        document.getElementById("configIcon").classList.add("active")
    else if (activeUser === -100)
        document.getElementById("rightSide").innerHTML = `<h1 class="ns">Você não selecionou nenhum chat</h1><h2 class="ns">Selecione um canal ou uma conversa</h2>`
    else
        document.getElementById("user" + activeUser).classList.add("active")
}


function listMessages() {
    listing = 1;
    leftMenuDiv.innerHTML = ""
    for (var i = 0; i < 15; i++) {
        leftMenuDiv.insertAdjacentHTML(
            "beforeend",
            `<div id="user` +
            i + `" onclick="selectActive('user', ` +
            i + `)" class="user-icon"><img src="tmp/koala.jpg" alt="Koala"></div>`
        )
    }
}

function listServers() {
    listing = 2;
    leftMenuDiv.innerHTML = ""
    for (var i = 0; i < 4; i++) {
        leftMenuDiv.insertAdjacentHTML(
            "beforeend",
            `<div id="user` +
            i + `" onclick="selectActive('user', ` +
            i + `)" class="user-icon"><img src="tmp/koala.jpg" alt="Koala"></div>`
        )
    }
}

async function toggleList() {
    if (listing == 1) {
        listServers()
    } else if (listing == 2) {
        listMessages()
    }
    var sleepTime = time1 / 90
    for (var i = 0; i < 361; i += 4) {
        if (i >= 360) {
            i = 0;
            alternateIcon.style.transform = "rotate(" + i + "deg)"
            break
        }
        alternateIcon.style.transform = "rotate(" + i + "deg)"
        await new Promise(r => setTimeout(r, sleepTime))
    }
}

var listing = 0
listMessages()