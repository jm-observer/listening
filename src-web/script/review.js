let review_words = [];
async function _to_review(id, name) {
    const tableBody = document.getElementById("tabs");
    const tab = document.getElementById("tab-" + id);
    if(!tab) {
        const tab = init_tab(id, name);
        tableBody.appendChild(tab);
        fetch('review_template.html')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.text();
            })
            .then(htmlString => {
                var parser = new DOMParser();
                let content = htmlString.replaceAll("__id__", id);
                var doc = parser.parseFromString(content, 'text/html');
                return doc.body.children[0]; // 或者 doc.documentElement，视情况而定
            })
            .then(htmlElement => {
                var targetElement = document.getElementById('tabs-content'); // 目标元素
                targetElement.appendChild(htmlElement);
            })
            .catch(error => {
                console.error('There has been a problem with your fetch operation:', error);
            });
    }
    await init_review_words();
    display_tab(id);
}

async function init_review_words() {
    review_words = await invoke("review_info");
    await init_review_word(review_words[0])
}
async function init_review_word(word) {
    if (word.image) {
        const tab = document.getElementById("review_image");
        const assetUrl = await window.__TAURI__.tauri.convertFileSrc(word.image);
        console.log(assetUrl);
        console.log(word.image);
        tab.src = assetUrl;
    }
    const word_ele = document.getElementById("review_word");
    word_ele.innerText = word.word.word;

    const accent = document.getElementById("review_accent");
    accent.innerText = word.word.accent_us;

    const cn_means = document.getElementById("review_cn_mean");
    while (cn_means.firstChild) {
        cn_means.removeChild(cn_means.firstChild);
    }
    for (const item of word.cn_mean) {
        cn_mean = document.createElement('p');
        cn_mean.innerText = item; // 设置按钮文本
        cn_mean.className = "p-0.5";
        cn_means.appendChild(cn_mean);
    }

    const index = document.getElementById("review_index");
    index.value = 0;
}