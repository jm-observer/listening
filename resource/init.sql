CREATE TABLE if not EXISTS words
(
    word_id  INTEGER PRIMARY KEY NOT NULL,
    word     TEXT                NOT NULL,
    zpk_path TEXT,
    zpk_name TEXT
);

CREATE TABLE if not EXISTS learn_plan
(
    id                   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    learn_batch_num      INTEGER                           NOT NULL,
    review_batch_num     INTEGER                           NOT NULL,
    review_interval_hour INTEGER                           NOT NULL
);

CREATE TABLE if not EXISTS learned_word
(
    id                    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word_id               INTEGER                           NOT NULL,
    start_time            INTEGER                           NOT NULL,
    last_time             INTEGER                           NOT NULL,
    next_time             INTEGER                           NOT NULL,
    err_times             INTEGER                           NOT NULL DEFAULT 0,
    total_learned_times   INTEGER                           NOT NULL DEFAULT 0,
    current_learned_times INTEGER                           NOT NULL DEFAULT 0
);

CREATE TABLE test_record
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word_id INTEGER                           NOT NULL,
    time    INTEGER                           NOT NULL,
    result  INTEGER                           NOT NULL DEFAULT 0
);

CREATE TABLE audio_replace_record
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word_id INTEGER                           NOT NULL,
    word    TEXT                              NOT NULL,
    time    TEXT                              NOT NULL
);

INSERT into learn_plan(learn_batch_num, review_batch_num, review_interval_hour)
values (20, 40, 12);

ATTACH DATABASE 'D:\\baicizhan\\lookup.db' AS lookup;
ATTACH DATABASE 'D:\\baicizhan\\baicizhantopic.db' AS baicizhantopic;
ATTACH DATABASE 'D:\\baicizhan\\baicizhantopicproblem.db' AS learned;


INSERT INTO words (word_id, word, zpk_path)
SELECT d.topic_id, d.word, bt.zpk_path
FROM lookup.dict_view d
         left join baicizhantopic.topic_resource_410 bt on d.topic_id = bt.topic
;
UPDATE words
set zpk_name = REPLACE(REPLACE(zpk_path, "/r/", ""), ".zpk", "")
where zpk_path is not null;

INSERT INTO learned_word (word_id, start_time, last_time, next_time, err_times, total_learned_times,
                          current_learned_times)
SELECT lt.topic_id, lt.create_at / 1000, lt.create_at / 1000, 1709456431 + lt.topic_day, 0, 1, 1
FROM learned.ts_learn_offline_dotopic_sync_ids_410 lt
;


