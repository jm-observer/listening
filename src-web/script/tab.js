function init_tab(id, name) {
    let template = "<li id='tab-__id__' class='mr-1 shadow rounded-md justify-center'>\n" +
        "                            <div class='flex px-4'>\n" +
        "                                <a id='display___id__' class='items-center bg-white py-2 px-1 flex' href='#'>\n" +
        "                                    <span id='status-__id__' class='h-3 w-3 bg-gray-400 rounded-full mr-2'></span>\n" +
        "                                    #name#</a>\n" +
        "                                <a id='close___id__' class='items-center bg-white px-1 pb-1 pt-1.5  hover:text-teal-800 flex'><i class='layui-icon layui-icon-close'></i></a>\n" +
        "                            </div>\n" +
        "                        </li>";

    const htmlString = template.replaceAll("__id__", id).replaceAll("#name#", name);

    var tempDiv = document.createElement('div');
    tempDiv.innerHTML = htmlString;
    const tableBody = document.getElementById("tabs");
    tableBody.appendChild(tempDiv.children[0]);

    document.getElementById('display_' + id).addEventListener('click', async function(event) {
        event.stopPropagation();
        display_tab(id);
    });
    document.getElementById('close_' + id).addEventListener('click', async function(event) {
        event.stopPropagation();
        close_tab(id);
    })
}


function display_tab(tab_id) {
    let parentElement = document.getElementById('tabs');
    for (let i = 0; i < parentElement.children.length; i++) {
        let tab = parentElement.children[i];
        if(tab.id.endsWith(tab_id)) {
            tab.classList.remove('text-gray-500');
            tab.classList.add('text-teal-500');
        } else {
            tab.classList.remove('text-teal-500');
            tab.classList.add('text-gray-500');
        }
    }

    parentElement = document.getElementById('tabs-content');
    for (let i = 0; i < parentElement.children.length; i++) {
        let tab = parentElement.children[i];
        if(tab.id.endsWith(tab_id)) {
            tab.style.display = 'block';
        } else {
            tab.style.display = 'none';
        }
    }
}


async function close_tab(tab_id) {
    let tab_full_id = get_tab_full_id(tab_id);
    var element = document.getElementById(tab_full_id);
    if (element) {
        element.parentNode.removeChild(element);
    }
    let tab_content_id = 'tab-content-' + tab_id;
    element = document.getElementById(tab_content_id);
    if (element) {
        element.parentNode.removeChild(element);
    }
    display_tab("home");
    // await get_invoke()("disconnect", { id : tab_id});
}

function get_tab_full_id(tab_id) {
    return 'tab-' + tab_id
}
