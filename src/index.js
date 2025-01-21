const invoke = window.__TAURI__.core.invoke;
window.__TAURI__.event.listen('tauri://window-shown', () => {
    const searchBar = document.getElementById("search-bar");
    if (searchBar) {
        console.log("Focus")
        searchBar.focus();
    }
});

let debounceTimeout;
document.getElementById('search-bar').addEventListener('keyup',(event) => {
    const query = event.target.value;

    clearTimeout(debounceTimeout);

    debounceTimeout = setTimeout(() => {
        if (query.length > 2) {
            invoke('task_manager', { action: "search", parameter: query})
                .then((message) => console.log(message));
        }
    }, 3000)
});

// Initiation Variables
const icon_3 = document.getElementById("i3");
const invDiv = document.getElementById("other_options");
const middleLine = document.getElementById("middle-line-3");
const topLine = document.getElementById("top-line-3");
const bottomLine = document.getElementById("bottom-line-3");

icon_3.addEventListener('click', () => {
    invDiv.classList.toggle('show');
    topLine.classList.toggle("animate");
    middleLine.classList.toggle("animate");
    bottomLine.classList.toggle("animate");
});

///////////////// DEMO //////////////////

const playDemo = () => {
    setTimeout(() => { icon_3.click(); }, 5000);
    setTimeout(() => { icon_3.click(); }, 6000);
};

document.addEventListener("DOMContentLoaded", () => {
    //playDemo();
});

document.addEventListener("DOMContentLoaded", () => {
    const field = document.querySelector(".field");

    if (field) {
        field.addEventListener("keydown", (event) => {
            if (event.key === "Backspace" && field.value === "") {
                invoke('task_manager', { action: "stop", parameter: ""}).then((message) => console.log(message));
            }
        });
    }
});