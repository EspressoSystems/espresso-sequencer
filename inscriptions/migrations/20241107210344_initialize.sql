-- Add migration script here

-- last_read_block is meant to be a table with a single row that stores the
-- last read block number.
CREATE TABLE last_read_block (
    id INTEGER PRIMARY KEY,
    block_number BIGINT NOT NULL
);

-- We shouldn't need an index for this table, as there **SHOULD** only be a
-- single row in this table.

-- Write a single row into this table to seed it for future updates.
INSERT INTO last_read_block (id, block_number) VALUES (0, 0);


-- pending_put_inscription_request is meant to store the requests that are
-- pending to be sent to the espresso chain.
CREATE TABLE pending_put_inscription_request (
    id SERIAL PRIMARY KEY,
    ins_hash BYTEA NOT NULL UNIQUE,
    ins_address VARCHAR NOT NULL,
    ins_time BIGINT NOT NULL
);

-- We would like to be able to query this table by the hash of the inscription,
-- as well as the time that the inscription was created.
CREATE INDEX pending_put_inscription_request_hash_index ON pending_put_inscription_request (ins_hash);
CREATE INDEX pending_put_inscription_request_time_index ON pending_put_inscription_request (ins_time);

-- sync_event is an enum that represents the two different types of events
-- that we want to keep track of for tracking the performance of our
-- inscription submission process.
CREATE TYPE sync_event AS ENUM ('submit', 'confirmed');

-- pending_put_inscriptions_event is meant to store the key events that occur
-- during the submission of an inscription to the espresso chain, as well as
-- the receipt of that inscription.
CREATE TABLE pending_put_inscriptions_event (
    id SERIAL PRIMARY KEY,
    ins_id BIGINT NOT NULL REFERENCES pending_put_inscription_request (id) ON DELETE RESTRICT,
    event_type sync_event NOT NULL,
    event_time BIGINT NOT NULL
);

-- We would like to be able to query this table by the hash of the inscription,
-- as well as the type of event that occurred.
--
-- We want to be able to retrieve the put_inscription requests that are lacking
-- a confirmation event.
CREATE INDEX pending_put_inscriptions_event_id_and_sync_event ON pending_put_inscriptions_event (ins_id, event_type);

-- confirmed_inscriptions is meant to store the inscriptions that have been
-- confirmed by the espresso chain.
CREATE TABLE confirmed_inscriptions (
    id SERIAL PRIMARY KEY,
    ins_hash BYTEA NOT NULL UNIQUE,
    ins_address VARCHAR NOT NULL,
    ins_time BIGINT NOT NULL,

    chain_block_height BIGINT NOT NULL,
    chain_txn_offset BIGINT NOT NULL
);

-- We would like to be able to retrieve the confirmed inscriptions by their
-- hash, the address of the wallet, and the time that the inscription was
-- created.
CREATE INDEX confirmed_inscriptions_hash_index ON confirmed_inscriptions (ins_hash);
CREATE INDEX confirmed_inscriptions_time_index ON confirmed_inscriptions (ins_time);
CREATE INDEX confirmed_inscriptions_block_index ON confirmed_inscriptions (ins_address);
