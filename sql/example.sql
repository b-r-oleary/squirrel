select e.*, af.event_id AS event_id, datetime_utc AS "event_start_time", another_column, and_another_column from public.actions_fat af join event e on e.id = af.event_id;

select e.*, af.event_id AS event_id from public.actions_fat af join event e on e.id = af.event_id;

with a_cte AS (
select 1 AS something
), another_cte AS (
select e.*, af.event_id AS event_id, datetime_utc AS "event_start_time", another_column, and_another_column from public.actions_fat af join event e on e.id = af.event_id
)
SELECT c_1.*, c_2.* FROM a_cte AS c_1, another_cte AS c_2
JOIN some_other_table t ON t.this_is_a_really_long_column_name_id = c_2.id;

SELECT a_column
FROM this_is_a_table
    , and_this_is_another_table JOIN another_table USING (an_id)
    , and_yet_another_table
        JOIN some_table USING (another_id)
        JOIN some_other_table USING (some_other_id)
        JOIN something_else USING (id)
