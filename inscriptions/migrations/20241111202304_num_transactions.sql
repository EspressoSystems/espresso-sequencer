-- Start Tracking number of transactions that have been tracked.
ALTER TABLE last_read_block ADD COLUMN num_transaction BIGINT NOT NULL DEFAULT 0;
