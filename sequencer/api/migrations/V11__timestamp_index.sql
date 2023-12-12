CREATE INDEX header_timestamp ON header (((data->'timestamp')::bigint));
