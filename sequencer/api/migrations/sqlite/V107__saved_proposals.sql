CREATE TABLE quorum_proposals (
    view BIGINT PRIMARY KEY,
    data BLOB,
    leaf_hash TEXT
);

CREATE UNIQUE INDEX quorum_proposals_leaf_hash_idx ON quorum_proposals (leaf_hash);