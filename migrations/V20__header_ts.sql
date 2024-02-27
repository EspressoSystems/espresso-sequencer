ALTER TABLE header
ADD COLUMN "timestamp" BIGINT NOT NULL;

CREATE INDEX header_timestamp_idx ON header ("timestamp");