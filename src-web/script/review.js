let review_words = [];
let listening = false;
let index = 0;
let playCount = 0;

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

                document.getElementById('review_accent_all').addEventListener('click', function(event) {
                    event.stopPropagation();
                    const accent_audio = document.getElementById('review_accent_audio');
                    accent_audio.play();
                });

                document.getElementById('review_listening').addEventListener('click', async function(event) {
                    event.stopPropagation();
                    if(listening) {
                        return;
                    }
                    await init_review_word(review_words[index % review_words.length]);
                    const video = document.getElementById('review_accent_audio');
                    playCount = 0;
                    video.onended = async () => {
                        playCount++;
                        if (playCount < 5) {
                            setTimeout(() => { video.play(); }, 1000); // Wait 2 seconds before replaying
                        } if (playCount >= 5) {
                            index ++;
                            playCount = 0;
                            await init_review_word(review_words[index % review_words.length]);
                            setTimeout(() => { video.play(); }, 3000);
                        }
                    };
                    video.play();
                    listening = true;
                });
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
    for (const word of review_words) {
        word.word.audio_us = await convertFileSrc(word.word.audio_us);
        word.word.audio_uk = await convertFileSrc(word.word.audio_uk);
        if (word.image) {
            word.image = await convertFileSrc(word.image);
        }
        for (const sentence of word.sentences) {
            sentence.audio = await convertFileSrc(sentence.audio);
        }
    }
    await init_review_word(review_words[0])
}
async function init_review_word(word) {
    if (word.image) {
        const tab = document.getElementById("review_image");
        tab.src = word.image;
    }
    if (word.sentences[0]) {
        const tab = document.getElementById("review_audio");
        tab.src = word.sentences[0].audio;
    }
    const word_ele = document.getElementById("review_word");
    word_ele.innerText = word.word.word;

    const accent = document.getElementById("review_accent");
    accent.innerText = word.word.accent_us;
    const accent_audio = document.getElementById('review_accent_audio');
    accent_audio.src = word.word.audio_us;

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