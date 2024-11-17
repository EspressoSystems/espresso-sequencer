-- Add block_merkle_tree_root column as a generated column
ALTER TABLE header
ADD COLUMN block_merkle_tree_root TEXT
GENERATED ALWAYS AS (json_extract(data, '$.block_merkle_tree_root')) STORED;

-- Add fee_merkle_tree_root column as a generated column
ALTER TABLE header
ADD COLUMN fee_merkle_tree_root TEXT
GENERATED ALWAYS AS (json_extract(data, '$.fee_merkle_tree_root')) STORED;
CREATE INDEX header_block_merkle_tree_root_idx ON header (block_merkle_tree_root);
CREATE INDEX header_fee_merkle_tree_root_idx ON header (fee_merkle_tree_root);