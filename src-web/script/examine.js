let examine_words = [];
let exam = false;
let examine_index = 0;
let examine_playCount = 0;
let is_submit = false;
let is_pause = false;

let error_times = 0;
let right_times = 0;
let examine_error_words = [];
let examine_right_words = [];
let const_play_count = 6;

async function _to_exam(id, name) {
    const tab = document.getElementById("tab-" + id);
    if (!tab) {
        init_tab(id, name);
        fetch('examine_template.html')
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

                document.getElementById('examine_pause').addEventListener('click', async function (event) {
                    const video = document.getElementById('examine_accent_audio');
                    video.pause();
                    exam = false;
                    is_pause = true;
                });

                document.getElementById('examine_turn_to_review').addEventListener('click', async function (event) {
                    if (examine_error_words.length == 0) {
                        //todo tell people
                    }
                    await _to_review("review", "review", examine_error_words);
                    start_listen();
                });

                document.getElementById('examine_turn_to_review_right').addEventListener('click', async function (event) {
                    if (examine_right_words.length == 0) {
                        //todo tell people
                    }
                    await _to_review("review", "review", examine_right_words);
                    start_listen();
                });

                document.getElementById('examine_submit').addEventListener('click', submit);
                const element = document.getElementById('examine_word'); // Replace 'yourElementId' with your actual element's ID
                element.addEventListener('keydown', async function (event) {
                    if (event.key === 'Enter') {
                        await submit(event);
                    }
                });

                const video = document.getElementById('examine_accent_audio');
                video.onended = async () => {
                    examine_playCount++;
                    if (examine_playCount < const_play_count) {
                        if (is_submit) {
                            if (await next_exam_word()) {
                                setTimeout(() => {
                                    if (exam) {
                                        video.play();
                                    }
                                }, 2000);
                            } else {
                                // end;
                            }
                        } else {
                            setTimeout(() => {
                                if (exam) {
                                    video.play();
                                }
                            }, 1800); // Wait 2 seconds before replaying
                        }
                    }
                    if (examine_playCount >= const_play_count) {
                        await submit();
                        if (await next_exam_word()) {
                            setTimeout(() => {
                                if (exam) {
                                    video.play();
                                }
                            }, 2000);
                        } else {
                            // end;
                        }
                    }
                };

                document.getElementById('examine').addEventListener('click', async function (event) {
                    event.stopPropagation();
                    if (exam) {
                        return;
                    }
                    start_to_examine();
                });
            })
            .catch(error => {
                console.error('There has been a problem with your fetch operation:', error);
            });
    }
    await init_examine_words();
    display_tab(id);
    update_exam_remaining_count();
    setTimeout(async () => {
        await start_to_examine()
    }, 2000);

}

async function submit(manual) {
    if (is_submit) {
        return;
    }
    let user_word = document.getElementById('examine_word').value;
    if (manual) {
        if (user_word === "") {
            return;
        }
    }
    let exam_word = examine_words[examine_index];
    const icon_right = document.getElementById('exam_right_icon');
    const icon_false = document.getElementById('exam_false_icon');
    if (user_word == exam_word.word.word) {
        right_times++;
        icon_right.classList.remove("hidden");
        icon_false.classList.add("hidden");
        let rs = await invoke("exam", {
            "rs": "success",
            "wordId": exam_word.word.word_id,
        });
        log(rs);
        examine_right_words.push(exam_word);
    } else {
        error_times++;
        icon_false.classList.remove("hidden");
        icon_right.classList.add("hidden");
        let rs = await invoke("exam", {"rs": "fail", "wordId": exam_word.word.word_id});
        log(rs);
        examine_error_words.push(exam_word);
    }
    examine_index++;
    update_exam_remaining_count()
    init_input();
    is_submit = true;
}

async function init_examine_words() {
    examine_error_words = await _get_words("review");
}

async function next_exam_word() {
    examine_playCount = 0;
    is_submit = false;
    let next_index = examine_index % examine_words.length;
    if (next_index != 0) {
        await init_examine_word(examine_words[next_index]);
        return true
    } else {
        // end
        display_rs();
        exam = false;
        examine_words = examine_error_words;
        return false
    }
}

async function start_to_examine() {
    if (is_pause) {
        document.getElementById('examine_accent_audio').play();
        is_pause = false;
        return;
    }
    if (examine_error_words.length == 0) {
        await init_examine_words();
    }
    is_submit = 0;
    error_times = 0;
    right_times = 0;
    examine_words = examine_error_words;
    examine_error_words = [];
    examine_right_words = [];
    exam = true;
    examine_index = 0;
    examine_playCount = 0;
    document.getElementById("examine_turn_to_review").classList.add("hidden");
    document.getElementById("examine_turn_to_review_right").classList.add("hidden");
    document.getElementById('exam_right_icon').classList.add("hidden");
    document.getElementById('exam_false_icon').classList.add("hidden");
    await init_examine_word(examine_words[0]);
    update_exam_remaining_count();
    init_input();
    document.getElementById('examine_accent_audio').play();
}


async function init_variate() {
    if (is_pause) {
        document.getElementById('examine_accent_audio').play();
        is_pause = false;
        return;
    }
    if (examine_error_words.length == 0) {
        await init_examine_words();
    }
    is_submit = 0;
    error_times = 0;
    right_times = 0;
    examine_words = examine_error_words;
    examine_error_words = [];
    examine_right_words = [];
    exam = true;
    examine_index = 0;
    examine_playCount = 0;
    document.getElementById("examine_turn_to_review").classList.add("hidden");
    document.getElementById("examine_turn_to_review_right").classList.add("hidden");
    document.getElementById('exam_right_icon').classList.add("hidden");
    document.getElementById('exam_false_icon').classList.add("hidden");
    await init_examine_word(examine_words[0]);
    update_exam_remaining_count();
    init_input();
    document.getElementById('examine_accent_audio').play();
}

function update_exam_remaining_count() {
    const word_ele = document.getElementById("examine_remaining_count");
    if (examine_words.length == 0) {
        word_ele.innerText = "剩余数：";
    } else {
        word_ele.innerText = "剩余数：" + (examine_words.length - examine_index % examine_words.length);
    }
    const right_count = document.getElementById("examine_right_count");
    right_count.innerText = "正确数：" + right_times;
    const false_count = document.getElementById("examine_false_count");
    false_count.innerText = "错误数：" + error_times;
}

function display_rs() {
    const word_ele = document.getElementById("examine_remaining_count");
    word_ele.innerText = "单词数：" + examine_words.length;
    const right_count = document.getElementById("examine_right_count");
    right_count.innerText = "正确数：" + right_times;
    const false_count = document.getElementById("examine_false_count");
    false_count.innerText = "错误数：" + error_times;
    if (error_times > 0) {
        const turn_to_review = document.getElementById("examine_turn_to_review");
        turn_to_review.classList.remove("hidden");
    }
    if (right_times > 0) {
        const turn_to_review = document.getElementById("examine_turn_to_review_right");
        turn_to_review.classList.remove("hidden");
    }
}

function init_examine_word(word) {
    const accent_audio = document.getElementById('examine_accent_audio');
    accent_audio.src = word.word.audio_us;
}

function init_input() {
    document.getElementById('examine_word').value = "";
    document.getElementById('examine_word').focus();
}
