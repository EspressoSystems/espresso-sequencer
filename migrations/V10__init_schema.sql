CREATE TABLE header
(
    height    BIGINT  PRIMARY KEY,
    hash      VARCHAR NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,

    -- For convenience, we store the entire application-specific header type as JSON. Just like
    -- `leaf.leaf` and `leaf.qc`, this allows us to easily reconstruct the entire header using
    -- `serde_json`, and to run queries and create indexes on application-specific header fields
    -- without having a specific column for those fields. In many cases, this will enable new
    -- application-specific API endpoints to be implemented without altering the schema (beyond
    -- possibly adding an index for performance reasons).
    data JSONB NOT NULL
);
-- TODO this index should be unique, but HotShot does not currently give the application a way to
-- put the block height or parent hash in the header, so headers are not guaranteed to be distinct.
-- This will be fixed soon with the block header refactor.
-- https://github.com/EspressoSystems/hotshot-query-service/issues/284
CREATE INDEX header_hash ON header (hash);

CREATE TABLE payload
(
    height BIGINT PRIMARY KEY REFERENCES header (height),
    size   INTEGER,
    data   BYTEA
);

CREATE TABLE leaf
(
    height     BIGINT  PRIMARY KEY REFERENCES header (height),
    hash       VARCHAR NOT NULL UNIQUE,
    proposer   VARCHAR NOT NULL,
    -- TODO we should be able to add a `REFERENCES header (hash)` here, except that `header (hash)`
    -- is currently not unique. Upcoming changes to HotShot to allow us to include the block height
    -- and/or parent hash in the header hash will fix this.
    -- https://github.com/EspressoSystems/hotshot-query-service/issues/284
    block_hash VARCHAR NOT NULL,

    -- For convenience, we store the entire leaf and justifying QC as JSON blobs. There is a bit of
    -- redundancy here with the indexed fields above, but it makes it easy to reconstruct the entire
    -- leaf without depending on the specific fields of the application-specific leaf type. We
    -- choose JSON over a binary format, even though it has a larger storage footprint, because
    -- Postgres actually has decent JSON support: we don't have to worry about escaping non-ASCII
    -- characters in inputs, and we can even do queries on the JSON and add indices over sub-objects
    -- of the JSON blobs.
    leaf JSONB NOT NULL,
    qc   JSONB NOT NULL
);
CREATE INDEX leaf_proposer ON leaf (proposer);

CREATE TABLE transaction
(
    id   SERIAL PRIMARY KEY,
    hash VARCHAR NOT NULL,
    -- Block containing this transaction.
    block_height BIGINT NOT NULL REFERENCES header(height),
    -- Position within the block. Transaction indices are an application-specific type, so we store
    -- it as a serialized blob. We use JSON instead of a binary format so that the application can
    -- make use of the transaction index in its own SQL queries.
    index JSONB NOT NULL
);
-- This index is not unique, because nothing stops HotShot from sequencing duplicate transactions.
CREATE INDEX transaction_hash ON transaction (hash);
