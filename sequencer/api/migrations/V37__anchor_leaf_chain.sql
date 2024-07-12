ALTER TABLE anchor_leaf
    DROP COLUMN id,
    DROP COLUMN height,
    ADD CONSTRAINT anchor_leaf_pk PRIMARY KEY (view);
