function to_review() {
    let _ = _to_review("review", "review");
}

function to_exam() {
    let _ = _to_exam("examine", "examine");
}


async function select_file() {
    const selected = await dialog_open({
        multiple: false,
        directory: false,
    });
    if (Array.isArray(selected)) {
        // user selected multiple files
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        document.getElementById('self_signed_ca').value = selected;
    }
}


async function check_then_save() {
    var form = document.getElementById('info');
    var formData = new FormData(form);
    var formObject = {};
    formData.forEach(function (value, key) {
        formObject[key] = value;
    });
    let result = true;
    if (result) {
        return;
    }
    if (!formObject["name"]) {
        result = false;
        document.getElementById('name').classList.add('input-error')
    } else {
        document.getElementById('name').classList.remove('input-error')
    }
    formObject["auto_connect"] = check_to_bool(formObject["auto_connect"])
    formObject["credential"] = check_to_bool(formObject["credential"])
    if (formObject["credential"]) {
        if (!formObject["user_name"]) {
            result = false;
            document.getElementById('user_name').classList.add('input-error')
        } else {
            document.getElementById('user_name').classList.remove('input-error')
        }
        if (!formObject["password"]) {
            result = false;
            document.getElementById('password').classList.add('input-error')
        } else {
            document.getElementById('password').classList.remove('input-error')
        }
    }

    if (formObject["tls"] === "self_signed") {
        if (formObject["self_signed_ca"] === "") {
            result = false;
            document.getElementById('self_signed_ca').classList.add('file-input-error')
        } else {
            document.getElementById('self_signed_ca').classList.remove('file-input-error')
        }
    }
    formObject["port"] = Number(formObject["port"]);
    formObject["id"] = Number(formObject["id"]);
    if (!formObject["port"]) {
        result = false;
        document.getElementById('port').classList.add('input-error')
    } else {
        document.getElementById('port').classList.remove('input-error')
    }
    try {
        JSON.parse(formObject["params"]);
        document.getElementById('params').classList.remove('textarea-error')
    } catch (error) {
        result = false;
        document.getElementById('params').classList.add('textarea-error')
    }

    if (result) {
        // try {
        //     let name = formObject["name"];
        //     let broker_id = await invoke("update_or_new_broker", { broker : formObject});
        //     console.log("broker_id: " + broker_id);
        //     document.getElementById('modal').style.display = 'none';
        //     broker_list();
        //     return {broker_id, name};
        // } catch (e) {
        //     console.error("Parsing error:", e);
        // }
    }
}


function update_global_audio(url) {
    let global_audio = document.getElementById('global_audio');
    global_audio.src = url;
    return global_audio;
}

async function loading_overview() {
    const overview = await invoke("load_overview");
    document.getElementById('tested_amount').innerText = "已测试单词数：" + overview.tested_amount;
    document.getElementById('waiting_amount').innerText = "待测试单词数：" + overview.waiting_amount;
    document.getElementById('today_all_amount').innerText = "今日总测试单词数：" + overview.today_all_amount;
    document.getElementById('today_error_amount').innerText = "今日错误单词数：" + overview.today_error_amount;
}

async function display_tab_home() {
    await loading_overview();
    display_tab('home');
}

