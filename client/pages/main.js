var activeUser = -100;

var activeType = "none";
var active = -1;

var rightSide = document.getElementById("rightSide");
var configIcon = document.getElementById("configIcon");
var alternateIcon = document.getElementById("alternateIcon");

var leftMenuDiv = document.getElementById("leftMenuDiv");

var time1 = Math.round(getComputedStyle(document.documentElement).getPropertyValue('--t1').slice(0, -2));

var servers = [
    {
        name: "Em breve",
        img: "tmp/error.svg"
    }
];
var chats = [
    {
        name: "Koala",
        img: "tmp/koala.jpg"
    }, {
        name: "Koala",
        img: "tmp/koala.jpg"
    }, {
        name: "Koala",
        img: "tmp/koala.jpg"
    }, {
        name: "Koala",
        img: "tmp/koala.jpg"
    }, {
        name: "Koala",
        img: "tmp/koala.jpg"
    }, {
        name: "Koala",
        img: "tmp/koala.jpg"
    },
];

function openActive() {
    if (activeType === "none")
        rightSide.innerHTML = `<h1 class="ns">Você não selecionou nenhum chat</h1>
        <h2 class="ns">Selecione um canal ou uma conversa</h2>`;
    else if (activeType === "server")
        rightSide.innerHTML = `<h1 class="ns">Em breve</h1>
        <h2 class="ns">Os servidores seram adicionados em breve, aguarde</h2>`;
    else if (activeType === "config")
        rightSide.innerHTML = `
        <div id="configBox">
            <div class="configItem">
                <div class="configItemTitle">Minha conta</div>
            </div>
        </div>
        `
    else
        rightSide.innerHTML = "";
}

function selectActive(type, id) {
    if (activeType === "config")
        configIcon.classList.remove("active");
    else if (activeType != "none")
        document.getElementById(activeType + active).classList.remove("active");

    if (active != id || activeType != type) {
        active = id;
        activeType = type;
    } else {
        active = -1;
        activeType = "none";
    }

    if (activeType === "config")
        configIcon.classList.add("active");
    else if (activeType != "none")
        document.getElementById(activeType + active).classList.add("active");

    openActive();
}



function listMessages() {
    listing = 1;
    leftMenuDiv.innerHTML = "";
    var h = window.getComputedStyle(document.getElementsByClassName("user-icon")[0], null).getPropertyValue("height");
    for (var i = 0; i < chats.length; i++) {
        leftMenuDiv.insertAdjacentHTML(
            "beforeend",
            `<div id="user` +
            i + `" onclick="selectActive('user', ` +
            i + `)" class="user-icon" style="`+"margin-top: -"+h+"; opacity: 0.5;"+`"><img src="`+chats[i].img+`" alt="`+chats[i].name+`"></div>`
        )
    }
}

function listServers() {
    listing = 2;
    leftMenuDiv.innerHTML = ""
    var h = window.getComputedStyle(document.getElementsByClassName("user-icon")[0], null).getPropertyValue("height");
    for (var i = 0; i < servers.length; i++) {
        leftMenuDiv.insertAdjacentHTML(
            "beforeend",
            `<div id="server` +
            i + `" onclick="selectActive('server', ` +
            i + `)" class="user-icon" style="`+"margin-top: -"+h+"; opacity: 0.5;"+`"><img src="`+servers[i].img+`" alt="`+servers[i].name+`"></div>`
        )
    }
}

async function popItems() {
    var h = window.getComputedStyle(document.getElementsByClassName("user-icon")[0], null).getPropertyValue("height");
    var icons = document.getElementsByClassName("user-icon")

    var sleepTime = time1/(icons.length-2);

    for (var i = 2; i < icons.length; i++) {
        icons[i].style = "margin-top: -"+h+"; opacity: 0.5;";
        await new Promise(r => setTimeout(r, sleepTime));
    }
}

async function pushItems() {
    var icons = document.getElementsByClassName("user-icon")

    var sleepTime = time1/(icons.length-2);

    for (var i = 2; i < icons.length; i++) {
        icons[i].style = "margin-top: 1vh; opacity: 1;";
        await new Promise(r => setTimeout(r, sleepTime));
    }
}

async function rotateAlternate() {
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

async function toggleList() {
    rotateAlternate();
    await popItems();

    selectActive("none", -1)
    
    if (listing == 1) {
        listServers()
    } else if (listing == 2) {
        listMessages()
    }
    pushItems();
    
}

var listing = 0;
listMessages();
pushItems();




