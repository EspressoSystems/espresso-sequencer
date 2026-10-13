-- The new protocol stores no legacy QC chain, and nothing has read it since the light client
-- stopped falling back to it.
DROP TABLE latest_qc_chain;
