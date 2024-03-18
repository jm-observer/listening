let review_words = [];
let listening = false;
let review_index = 0;
let review_playCount = 0;
let review_loop = 0;
let timeout_id = 0;
let timeout_id_2 = 0;

async function _to_review(id, name, new_review_words) {
    const tab = document.getElementById("tab-" + id);
    if (!tab) {
        init_tab(id, name);
        await fetch('review_template.html')
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

                document.getElementById('review_accent_all').addEventListener('click', function (event) {
                    event.stopPropagation();
                    const accent_audio = document.getElementById('review_accent_audio');
                    accent_audio.play();
                });
                document.getElementById('review_accent_youdao').addEventListener('click', function (event) {
                    event.stopPropagation();
                    const accent_audio = update_global_audio("https://dict.youdao.com/dictvoice?type=2&audio=" + review_words[review_index].word.word);
                    accent_audio.play();
                });
                document.getElementById('review_listening_pause').addEventListener('click', async function (event) {
                    const video = document.getElementById('review_accent_audio');
                    video.pause();
                    listening = false;
                });
                document.getElementById('review_previous').addEventListener('click', async function (event) {
                    const video = document.getElementById('review_accent_audio');
                    if (listening) {
                        video.pause();
                        await previous_word();
                        video.play();
                    } else {
                        await previous_word()
                    }
                });
                const video = document.getElementById('review_accent_audio');
                video.onended = async () => {
                    review_playCount++;
                    if (review_playCount < 5) {
                        timeout_id = setTimeout(() => {
                            if (listening) {
                                video.play();
                            }
                        }, 1000); // Wait 2 seconds before replaying
                    }
                    if (review_playCount >= 5) {
                        await next_word();
                        timeout_id_2 = setTimeout(() => {
                            if (listening) {
                                video.play();
                            }
                        }, 2000);
                    }
                };
                document.getElementById('review_listening').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    start_listen();
                });

                document.getElementById('review_listening_today').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    await review_today();
                    init_global_var();
                    await init_word_by_index();
                    start_listen();
                });
                document.getElementById('review_listening_yesterday').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    await review_yesterday();
                    init_global_var();
                    await init_word_by_index();
                    start_listen();
                });
                document.getElementById('review_listening_today_error').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    await review_today_error();
                    init_global_var();
                    await init_word_by_index();
                    start_listen();
                });
                document.getElementById('review_listening_yesterday_error').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    await review_yesterday_error();
                    init_global_var();
                    await init_word_by_index();
                    start_listen();
                });

                document.getElementById('replace_youdao').addEventListener('click', async function (event) {
                    let word = review_words[review_index];
                    const word_id = word.word.word_id;
                    await relace_audio(word.word.word_id, word.word.word, word.word.audio_us_old)
                });
            })
            .catch(error => {
                console.error('There has been a problem with your fetch operation:', error);
            });


    }
    await init_review_words(new_review_words);
    init_global_var();
    await init_word_by_index(review_index, false);
    display_tab(id);
    start_listen();
}

function start_listen() {
    if (listening) {
        return;
    }
    const video = document.getElementById('review_accent_audio');
    video.play();
    listening = true;
}

async function init_review_words(new_review_words) {
    if (new_review_words) {
        review_words = new_review_words;
    } else {
        await review_review(50);
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

async function init_word_by_index() {
    review_playCount = 0;
    let next_index = review_index % review_words.length;
    await init_review_word(review_words[next_index]);
    clearTimeout(timeout_id);
    clearTimeout(timeout_id_2);
}

async function next_word() {
    review_index++;
    if (review_index % review_words.length == 0) {
        review_loop++;
        update_loop()
    }
    update_remaining_count();
    await init_word_by_index()
}

async function previous_word() {
    if (review_index > 0) {
        if (review_index % review_words.length == 0) {
            review_loop--;
            update_loop()
        }
        review_index--;
        update_remaining_count();
        await init_word_by_index()
    }
}

function update_loop() {
    const word_ele = document.getElementById("looped_count");
    word_ele.innerText = "已循环：" + review_loop;
}

function update_remaining_count() {
    const word_ele = document.getElementById("remaining_count");
    word_ele.innerText = "本次剩余：" + (review_words.length - review_index % review_words.length);
}

function init_global_var() {
    listening = false;
    review_index = 0;
    review_playCount = 0;
    review_loop = 0;
    timeout_id = 0;
    timeout_id_2 = 0;
    update_loop();
    update_remaining_count()
}

async function convert_asserts(word) {
    word.word.audio_us_old = word.word.audio_us;
    word.word.audio_us = await convertFileSrc(word.word.audio_us);
    word.word.audio_uk = await convertFileSrc(word.word.audio_uk);
    if (word.image) {
        word.image = await convertFileSrc(word.image);
    }
    for (const sentence of word.sentences) {
        sentence.audio = await convertFileSrc(sentence.audio);
    }
}

async function review_today() {
    review_words = await _get_words("today");
}

async function review_yesterday() {
    review_words = await _get_words("yesterday");
}

async function review_today_error(ty) {
    review_words = await _get_words("today_error");
}

async function review_yesterday_error() {
    review_words = await _get_words("yesterday_error");
}

async function review_review(limit) {
    review_words = await _get_words("review", limit);
}


async function _get_words(ty, limit) {
    if (!limit) {
        limit = 30;
    }
    let words = await invoke("review_info", {"ty": ty, "limit": limit});
    for (const word of words) {
        await convert_asserts(word);
    }
    return words;
}


async function relace_audio(word_id, word, audio_path) {
    let rs = await invoke("replace_audio", {"wordId": word_id, "word": word, "audioPath": audio_path});
    log(rs);
}