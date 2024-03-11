-- DELETE AFTER TESTING

-- This table maps 32-byte hash values to short 4-byte identifiers which are
-- unique within the database.
CREATE TABLE hash
(
	id    SERIAL PRIMARY KEY,
	value BYTEA  NOT NULL UNIQUE,
);

CREATE TABLE node
(
	pos     LTREE NOT NULL,  -- Position of the node in the tree
	created BIGINT NOT NULL, -- Version where this node was last created/modified
	
	-- The hash value of the node. We store hashes as references to a `hash` table,
	-- as the hashes themselves are large (32 bytes) and the references can be small
	-- (4 bytes). Since each hash is referenced twice in the tree (once in the node
	-- itself and once as a child of its parent node) this gives us almost a 2x
	-- compression for hash-related storage.
	value INT NOT NULL REFERENCES hash (id),

	-- The hash values of the children, also stored as a short foreign key. Note
	-- that the hashes uniquely identify the children, but don't reference them in
	-- the relational database sense. This is because we never need to do joins over
	-- the tree structure.
	--
	-- NULL for leaf nodes
	children []INT,
	-- Bitmap indicating which children are present. This allows the `children`
	-- array to store only non-empty child hashes, which reduces storage
	-- requirements for sparse, high-arity trees.
	--
	-- NULL for leaf nodes. May be omitted entirely for low-arity trees, where it
	-- may be more efficient to just store the empty children explicitly.
	children_bitmap BIT(2),

	index JSONB, -- The application-specific key identifying this entry (leaf only).
	entry JSONB, -- The application-specific contents (leaf only).
);
-- We store each version of each position exactly once.
ALTER TABLE node ADD CONSTRAINT node_pk PRIMARY KEY (pos, created)
-- This index lets us query for ancestor/descendant relationships on the `pos`
-- field.
CREATE INDEX node_path ON node USING GIST (pos);

