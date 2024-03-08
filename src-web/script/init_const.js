function isTauriEnvironment() {
    return typeof window.__TAURI__ !== 'undefined';
}

const listen = isTauriEnvironment() ?
    window.__TAURI__.event.listen :
    function (method, data) {
    };

const invoke = isTauriEnvironment() ?
    window.__TAURI__.tauri.invoke :
    function (method, data) {
        console.log(method, data);
        return method
    };

const shell_open = isTauriEnvironment() ?
    window.__TAURI__.shell.open:
    function (url) {
        return caches.open(url).then(function(cache) {
            return cache;
        });
    };
const dialog_open = isTauriEnvironment() ?
    window.__TAURI__.dialog.open:
    function () {
    };

const convertFileSrc = isTauriEnvironment() ?
    window.__TAURI__.tauri.convertFileSrc:
    async function () {
    };
// const homeDir = isTauriEnvironment() ?
//     window.__TAURI__.path.homeDir:
//     async function () {};
//
// const base_home = isTauriEnvironment() ?
//     window.__TAURI__.fs.BaseDirectory.Home:
//     11;

// const _readBinaryFile = isTauriEnvironment() ?
//     window.__TAURI__.fs.readBinaryFile:
//     function () {
//     };
//
// const join = isTauriEnvironment() ?
//     window.__TAURI__.path.join:
//     async function () {
//     };
//
// async function readBinaryFile(path) {
//     try {
//         return await _readBinaryFile(path, { dir: base_home })
//     } catch (error) {
//         console.error('Error readBinaryFile file:', path);
//     }
// }

async function loading() {
    init_info();
    document.getElementById('github').addEventListener('click', function() {
        shell_open('https://github.com/url').catch(console.error);
    });
    document.getElementById('tips').addEventListener('click', function(event) {
        event.stopPropagation();
        let modal = document.getElementById('tips-msg');
        modal.style.display = 'block';
        modal.style.top = (event.target.getBoundingClientRect().top + 20) + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要
    });
    let config = await invoke("loading");
    document.getElementById("tips-msg-pre").innerText = config["hint"];
    if (!config["debug"]) {
        document.addEventListener('contextmenu', function(event) {
            event.preventDefault(); // 阻止默认的右键菜单
        });
    }
    document.getElementById('to_review').addEventListener('click', function(event) {
        event.stopPropagation();
        to_review();
    });
    document.getElementById('to_exam').addEventListener('click', function(event) {
        event.stopPropagation();
        to_exam();
    });

    document.getElementById('display_home').addEventListener('click', function(event) {
        event.stopPropagation();
        display_tab('home');
    });
}

function display_info() {
    var modal = document.getElementById('modal');
    modal.style.display = 'block';
}

function init_info() {
    const modal = document.getElementById('modal');
    const triggerButton = document.getElementById('tabs-content');
    const rect = triggerButton.getBoundingClientRect();
    modal.style.top = rect.top + 'px'; // 或者使用 rect.bottom + 'px'，取决于需要

    document.getElementById('self_signed_ca').addEventListener('click', function() {
        select_file()
    });
}

function button_click_style(button) {
    button.classList.add('bg-yellow-600');
    setTimeout(() => {
        button.classList.remove('bg-yellow-600');
        button.classList.add('bg-yellow-500');
    }, 200);
}
