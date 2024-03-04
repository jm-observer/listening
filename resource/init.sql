CREATE TABLE if not EXISTS words (
	word_id INTEGER PRIMARY KEY,
	word TEXT,
	zpk_path TEXT
);

CREATE TABLE if not EXISTS learn_plan (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	learn_batch_num INTEGER,
	review_batch_num INTEGER,
	review_interval_hour INTEGER
);

CREATE TABLE if not EXISTS learned_word(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	word_id INTEGER REFERENCES words(word_id) on DELETE CASCADE,
	start_time INTEGER,
	last_time INTEGER,
	next_time INTEGER,
	err_times INTEGER,
	total_learned_times INTEGER,
	current_learned_times INTEGER
);

INSERT into learn_plan(learn_batch_num, review_batch_num, review_interval_hour) values(20, 20, 12);

INSERT INTO learned_word (word_id, start_time, last_time, next_time, err_times, total_learned_times, current_learned_times)
SELECT lt.topic_id , lt.create_at / 1000, lt.create_at / 1000, 1709456431 + lt.topic_day , 0, 1, 1 
FROM learned_topic lt
;

INSERT INTO words (word_id, word, zpk_path)
SELECT d.topic_id , d.word , bt.zpk_path  FROM dict d left join book_topic_410 bt on d.topic_id = bt.topic
;

