ALTER TABLE payload DROP CONSTRAINT IF EXISTS payload_height_fkey;
ALTER TABLE vid DROP CONSTRAINT IF EXISTS vid_height_fkey;
ALTER TABLE leaf DROP CONSTRAINT IF EXISTS leaf_height_fkey;
ALTER TABLE leaf DROP CONSTRAINT IF EXISTS leaf_block_hash_fkey;
ALTER TABLE transaction DROP CONSTRAINT IF EXISTS transaction_block_height_fkey;

ALTER TABLE payload
ADD CONSTRAINT payload_height_fkey FOREIGN KEY (height) REFERENCES header(height) ON DELETE CASCADE;

ALTER TABLE vid
ADD CONSTRAINT vid_height_fkey FOREIGN KEY (height) REFERENCES header(height) ON DELETE CASCADE;

ALTER TABLE leaf
ADD CONSTRAINT leaf_height_fkey FOREIGN KEY (height) REFERENCES header(height) ON DELETE CASCADE;

ALTER TABLE transaction
ADD CONSTRAINT transaction_block_height_fkey FOREIGN KEY (block_height) REFERENCES header(height) ON DELETE CASCADE;

ALTER TABLE leaf
ADD CONSTRAINT leaf_block_hash_fkey FOREIGN KEY (block_hash) REFERENCES header(hash) ON DELETE CASCADE;