CREATE TABLE quorum_proposals (
    view BIGINT PRIMARY KEY,
    data BLOB,
    leaf_hash TEXT
);