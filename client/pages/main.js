var activeType = "none";
var active = -1;

var rightSide = document.getElementById("rightSide");
var configIcon = document.getElementById("configIcon");
var alternateIcon = document.getElementById("alternateIcon");
var leftMenuDiv = document.getElementById("leftMenuDiv");
var time1 = Math.round(getComputedStyle(document.documentElement).getPropertyValue('--t1').slice(0, -2));

const configs = fs.readFileSync("pages/configs.html", { encoding: "utf8", flag: "r" });
const theme = JSON.parse(fs.readFileSync("data/theme/theme.json", { encoding: "utf-8", flag: "r" }));


var servers = [
    {
        name: "Em breve",
        img: "tmp/error.svg"
    }
];
var koala = {
    name: "Koala",
    img: "tmp/koala.jpg"
};
var chats = [ koala, koala, koala, koala, koala, koala, ];

function openActive() {
    if (activeType === "none")
        rightSide.innerHTML = `<h1 class="ns text-nochatselected"></h1><h2 class="ns text-nochatselected1"></h2>`;

    else if (activeType === "server")
        rightSide.innerHTML = `<h1 class="ns text-soon"></h1><h2 class="ns text-soon1"></h2>`;

    else if (activeType === "config")
        rightSide.innerHTML = configs;

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
            `<div id="user${i}" onclick="selectActive('user', ${i})" class="user-icon" style="margin-top: -${h}; opacity: 0.5;"><img src="${chats[i].img}" alt="${chats[i].name}"></div>`
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
            `<div id="server${i}" onclick="selectActive('server', ${i})" class="user-icon" style="margin-top: -${h}; opacity: 0.5;"><img src="${servers[i].img}" alt="${servers[i].name}"></div>`
        )
    }
}

async function popItems() {
    var h = window.getComputedStyle(document.getElementsByClassName("user-icon")[0], null).getPropertyValue("height");
    var icons = document.getElementsByClassName("user-icon")

    var sleepTime = time1 / (icons.length - 2);

    for (var i = 2; i < icons.length; i++) {
        icons[i].style = `margin-top: -${h}; opacity: 0.5;`;
        await new Promise(r => setTimeout(r, sleepTime));
    }
}

async function pushItems() {
    var icons = document.getElementsByClassName("user-icon")

    var sleepTime = time1 / (icons.length - 2);

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
            alternateIcon.style.transform = `rotate(${i}deg)`
            break
        }
        alternateIcon.style.transform = `rotate(${i}deg)`
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

function toggleConfig(id) {
    var obj = document.getElementsByClassName("configItem")[id];
    obj.classList.toggle("open");

    if (obj.classList.contains("open")) {
        var height = 2;
        for (var i = 0; i < obj.children.length; i++) {
            height += Math.round(window.getComputedStyle(obj.children[i], null).getPropertyValue("height").slice(0, -2));
        }
        obj.style.height = height + "px";
    } else {
        obj.style.height = "calc(8vh + 2px)";
    }
}

function writeConfig() {
    fs.writeFile("data/configs.json", JSON.stringify(settings), function(){});
}

function toggleCheck(id, key) {
    var obj = document.getElementsByClassName("checkbox")[id];
    obj.classList.toggle("on");
    settings[key] = obj.classList.contains("on");
    writeConfig();
}

function setConfig(id, key) {
    var obj = document.getElementsByClassName("config-input")[id];
    settings[key] = obj.value;
    writeConfig();
}

function toggleClass(_class, id, classname, add) {
    var obj = document.getElementsByClassName(_class)[id];
    if (add) {
        obj.classList.add(classname);
    } else {
        obj.classList.remove(classname);
    }
}

function loopClass(classname, value) {
    var t = document.getElementsByClassName(classname);
    for (var i = 0; i < t.length; i++) {
        if (!(t[i].classList.contains(value)))
            t[i].classList.add(value);
    }
}

function loopClassValue(classname, value) {
    var t = document.getElementsByClassName(classname);
    for (var i = 0; i < t.length; i++) {
        if ((t[i].value != value) && (!(t[i].classList.contains("changing"))))
            t[i].value = value;
    }
}

async function listSettings() {
    while (await new Promise(r => setTimeout(()=>r(1), 200))) {
        if (settings.notify) {
            loopClass("status-notify", "on");
        }
        if (settings.online) {
            loopClass("status-iamonline", "on");
        }
        if (settings.messagesfromall) {
            loopClass("status-msgfromall", "on");
        }
        loopClassValue("status-language", settings.language);
    }
}

async function listColors() {
    while (await new Promise(r => setTimeout(() => r(true), 200))) {
        for (key in theme) {
            try {
                var t = document.getElementById(`p-${key}`)
                if ((t.value != theme[key]) && !(t.classList.contains("changing"))) {
                    t.value = theme[key];
                }
                
            } catch { }
        }
    }
}

function setTheme(data, openfile) {
    if (openfile) {
        data = fs.readFileSync(`data/theme/${data}`, { encoding: "utf8", flag: "r" });
    }
    
    fs.writeFile("data/theme/theme.json", data, () => null);
    var { c1, c2, c3, c4, cl1, cl2, cl3, cm1, cc1, cc2 } = JSON.parse(data);
    fs.writeFile(
        "pages/theme.css",
        `:root {
            --c1: ${c1};
            --c2: ${c2};
            --c3: ${c3};
            --c4: ${c4};
            --cl1: ${cl1};
            --cl2: ${cl2};
            --cl3: ${cl3};
            --cm1: ${cm1};
            --cc1: ${cc1};
            --cc2: ${cc2};
            --t1: 250ms;
        }
        `,
        ()=>null
    )
}

function updateColors() {
    data = JSON.parse(fs.readFileSync(`data/theme/theme.json`, { encoding: "utf8", flag: "r" }));
    for (key in data) {
        console.log(key);
        if (document.body.contains(document.getElementById(`p-${key}`))) {
            data[key] = document.getElementById(`p-${key}`).value;
        }
    }
    setTheme(JSON.stringify(data), false);
}


var listing = 0;
listMessages();
pushItems();
writeText();
listSettings();
listColors();
