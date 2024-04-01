CREATE TABLE anchor_leaf (
    -- The ID is always set to 0. Setting it explicitly allows us to enforce with every insert or
    -- update that there is only a single entry in this table: the latest decided leaf.
    id INT PRIMARY KEY,

    height BIGINT,
    leaf   BYTEA
);

CREATE TABLE highest_voted_view (
    -- The ID is always set to 0. Setting it explicitly allows us to enforce with every insert or
    -- update that there is only a single entry in this table: the latest known view.
    id INT PRIMARY KEY,

    view BIGINT
);

CREATE TABLE high_qc (
    id INT PRIMARY KEY,
    view BIGINT,
    data BYTEA
);

CREATE TABLE da_proposal (
    view BIGINT PRIMARY KEY,
    data BYTEA
);

CREATE TABLE da_proposal_vid_share (
    view BIGINT PRIMARY KEY,
    data BYTEA
);
