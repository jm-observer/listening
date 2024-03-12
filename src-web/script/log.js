log_limit_count = 20;
current_log_count = 0;

function init_log_plugin() {
    document.getElementById("log_latest").addEventListener('click', function (event) {
        event.stopPropagation();
        let classList = document.getElementById("log_all").classList;
        if (classList.contains("hidden")) {
            classList.remove("hidden");
        } else {
            classList.add("hidden");
        }
    });
}

function hidden_logs() {
    document.getElementById("log_all").classList.add("hidden");
}

function log(msg) {
    document.getElementById("log_latest").innerText = msg;
    current_log_count++;
    let logs = document.getElementById("log_all");
    while (current_log_count > log_limit_count && logs.firstChild) {
        current_log_count--;
        logs.removeChild(logs.firstChild)
    }
    let log_msg = document.createElement("pre");
    log_msg.classList.add("p-0.5");
    log_msg.innerText = msg;
    logs.appendChild(log_msg);
}