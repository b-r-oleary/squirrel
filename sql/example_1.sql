select e.*, af.event_id AS event_id, datetime_utc AS "event_start_time", another_column, and_another_column from public.actions_fat af join event e on e.id = af.event_id;

select e.*, af.event_id AS event_id from public.actions_fat af join event e on e.id = af.event_id;

with a_cte AS (
select 1 AS something
)
SELECT * FROM a_cte
