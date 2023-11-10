CREATE TABLE leaf
(
	height BIGINT  PRIMARY KEY,
	hash   VARCHAR NOT NULL
);
CREATE UNIQUE INDEX leaf_hash ON leaf (hash);

CREATE TABLE block
(
	height       BIGINT    PRIMARY KEY REFERENCES leaf (height),
	hash         VARCHAR   NOT NULL,
	timestamp    TIMESTAMP NOT NULL,
	l1_head      BIGINT    NOT NULL,
	l1_finalized BIGINT    NOT NULL REFERENCES l1_block (number)
);
CREATE UNIQUE INDEX block_hash ON block (hash);

CREATE TABLE l1_block
(
	number    BIGINT    PRIMARY KEY,
	hash      VARCHAR   NOT NULL,
	timestamp TIMESTAMP NOT NULL
);
