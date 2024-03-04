
DELETE FROM dict_bcz WHERE topic_id in (
SELECT topic_id from (
SELECT topic_id , 'dict_a_b' as dict_name from dict_a_b dab union
SELECT topic_id , 'dict_bcz' as dict_name from dict_bcz db union
SELECT topic_id , 'dict_c' as dict_name from dict_c dc 	union
SELECT topic_id , 'dict_d_f' as dict_name from dict_d_f ddf 	union
SELECT topic_id , 'dict_g_k' as dict_name from dict_g_k dgk 	union
SELECT topic_id , 'dict_l_o' as dict_name from dict_l_o dlo 	union
SELECT topic_id , 'dict_p_r' as dict_name from dict_p_r dpr 	union
SELECT topic_id , 'dict_s' as dict_name from dict_s ds 	union
SELECT topic_id , 'dict_t_z' as dict_name from dict_t_z dtz 
) group by topic_id HAVING COUNT(*) > 1);



CREATE VIEW dict_view AS
SELECT * from dict_a_b dab union
SELECT * from dict_bcz db union
SELECT * from dict_c dc 	union
SELECT * from dict_d_f ddf 	union
SELECT * from dict_g_k dgk 	union
SELECT * from dict_l_o dlo 	union
SELECT * from dict_p_r dpr 	union
SELECT * from dict_s ds 	union
SELECT * from dict_t_z dtz ;
