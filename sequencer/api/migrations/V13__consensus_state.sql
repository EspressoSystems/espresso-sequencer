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
