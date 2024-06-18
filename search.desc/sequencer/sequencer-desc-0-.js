searchState.loadedDescShard("sequencer", 0, "SNAFU context selector for the <code>Error::BlockBuilding</code> variant\nGlobal variables for an Espresso blockchain.\nSNAFU context selector for the <code>Error::GenesisWrongSize</code> …\nA header is like a [<code>Block</code>] with the body replaced by a …\nSNAFU context selector for the <code>Error::IncorrectParent</code> …\nSNAFU context selector for the <code>Error::IncorrectView</code> variant\nSNAFU context selector for the <code>Error::MerkleTreeError</code> …\nSNAFU context selector for the <code>Error::MissingGenesis</code> …\nThe Sequencer node is generic over the hotshot CommChannel.\nSNAFU context selector for the <code>Error::UnexpectedGenesis</code> …\nMinimum fee in WEI per byte of payload\nRoot Commitment of Block Merkle Tree\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nAccount (etheruem address) of builder\nThe address where a CDN marshal is located\nA commitment to a ChainConfig or a full ChainConfig.\nEspresso chain ID\nThe underlying event\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nFee contract address on L1.\nFee paid by the block builder\nRoot Commitment of <code>FeeMerkleTree</code>\nAccount that receives sequencing fees.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nL1 Client\nThe Espresso block header includes information a bout the …\nThe Espresso block header includes a reference to the …\nThe address to send to other Libp2p nodes to contact us\nThe address to bind to for Libp2p\nThe (optional) bootstrap node addresses for Libp2p. If …\nMaximum size in bytes of a block\nSequencer node persistence.\nUtilities for generating and storing the most recent light …\nThe view number that this event originates from\nSequencer-specific API endpoint handlers.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSequencer-specific API options and initialization.\nUpdate loop for query API state.\nProvider for fetching missing data for the query service.\nThis struct defines the public Hotshot configuration …\nThis struct defines the public Hotshot validator …\nA data source with sequencer-specific functionality.\nInstantiate a data source from command line options.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the state of the requested <code>account</code>.\nGet the blocks Merkle tree frontier.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a provider for fetching missing data from a list of …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nOptions for the catchup API module.\nOptions for the config API module.\nOptions for the explorer API module.\nOptions for the Hotshot events streaming API module.\nThe minimal HTTP API.\nOptions for the query API module.\nOptions for the state API module.\nOptions for the status API module.\nOptions for the submission API module.\nAdd a catchup API module.\nAdd a config API module.\nPort that the HTTP Hotshot Event streaming API will use.\nAdd an explorer API module.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nWhether these options will run the query API.\nAdd a Hotshot events streaming API module.\nInitialize the modules for interacting with HotShot.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMaximum number of concurrent HTTP connections the server …\nPeers for fetching missing data for the query service.\nPort that the HTTP API will use.\nAdd a query API module backed by the file system.\nAdd a query API module backed by a Postgres database.\nStart the server.\nAdd a state API module.\nAdd a status API module.\nAdd a submit API module.\nDefault options for running a web server on the given port.\nDefault options for running a web server on the given port.\nTest the catchup API with custom options.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTest the state signature API.\nTest the status API with custom options.\nTest the submit API with custom options.\nProof of correctness for namespace payload bytes in a …\nRaw binary data for a namespace table.\nReturn type for <code>NsTable::validate</code>.\nRaw payload data for an entire block.\nSerialization (and deserialization) of primitive unsigned …\nTypes related to a namespace table.\nProof of correctness for namespace payload bytes in a …\nReturn all transactions in the namespace whose payload is …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the payload bytes for the <code>index</code>th namespace, along …\nVerify a <code>NsProof</code> against a payload commitment. Returns <code>None</code>…\nByte lengths for the different items that could appear in …\nIndex for an entry in a ns table.\nReturn type for [<code>Payload::ns_iter</code>].\nRaw binary data for a namespace table.\nReturn type for <code>NsTable::validate</code>.\nNumber of entries in a namespace table.\nAdd an entry to the namespace table.\nByte length of a single namespace table entry.\nSearch the namespace table for the ns_index belonging to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nByte length of a namespace table header.\nDoes the <code>index</code>th entry exist in the namespace table?\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSerialize to bytes and consume self.\nIterator over all unique namespaces in the namespace table.\nNumber of entries in the namespace table.\nRead subslice range for the <code>index</code>th namespace from the …\nRead the namespace id from the <code>index</code>th entry from the …\nLike <code>Self::read_ns_id</code> except <code>index</code> is not checked. Use …\nRead the namespace offset from the <code>index</code>th entry from the …\nRead the number of namespaces declared in the namespace …\nAre the bytes of this <code>NsTable</code> uncorrupted?\nRaw payload data for an entire block.\nByte length of a block payload, which includes all …\nReturns the argument unchanged.\nReturns the argument unchanged.\nNeed a sync version of <code>BlockPayload::from_transactions</code> in …\nExtract payload byte length from a <code>VidCommon</code> and construct …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nIs the payload byte length declared in a <code>VidCommon</code> equal …\nConvenience wrapper for <code>Self::read_ns_payload</code>.\nLike <code>QueryablePayload::transaction_with_proof</code> except …\nTypes related to a namespace payload and its transaction …\nCartesian product of <code>NsIter</code>, <code>TxIter</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRaw binary data for a single namespace’s payload.\nReturn all transactions in this namespace. The namespace …\nReturn a transaction from this namespace. Set its …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nIterator over all transactions in this namespace.\nPrivate helper\nCrazy boilerplate code to make it so that <code>NsPayloadOwned</code> …\nRead and parse bytes from the ns payload.\nPrivate helper. (Could be pub if desired.)\nPrivate helper\nIndex range for a namespace payload inside a block payload.\nAccess the underlying index range for this namespace …\nConvert a <code>NsPayloadBytesRange</code> into a range that’s …\nReturn the byte length of this namespace.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTODO restrict visibility?\nProof of correctness for transaction bytes in a block.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the <code>Transaction</code> indicated by <code>index</code>, along with a …\nVerify a <code>TxProof</code> for <code>tx</code> against a payload commitment. …\nData that can be deserialized from a subslice of namespace …\nByte lengths for the different items that could appear in …\nBuild an individual namespace payload one transaction at a …\nByte length of a namespace payload.\nSpecifies a subslice of namespace payload bytes to read.\nNumber of txs in a namespace.\nByte range for the part of a tx table that declares the …\nThe part of a tx table that declares the number of txs in …\nIndex for an entry in a tx table.\nA transaction’s payload data.\nByte range for a transaction’s payload data.\nEntries from a tx table in a namespace for use in a …\nByte range for entries from a tx table for use in a …\nAdd a transaction’s payload to this namespace\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nDeserialize <code>Self</code> from namespace payload bytes.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSerialize to bytes and consume self.\nReturns the minimum of:\nRange relative to this ns payload\nByte length of a single tx table entry.\nByte length of a tx table header.\nImpl <code>serde</code> for type <code>$T</code> with methods named <code>$to_bytes</code>, …\nCan <code>n</code> fit into <code>byte_len</code> bytes?\nDeserialize <code>bytes</code> in little-endian form into a <code>$T</code>, padding …\nReturn the largest <code>$T</code> value that can fit into <code>byte_len</code> …\nSerialize <code>n</code> into <code>BYTE_LEN</code> bytes in little-endian form, …\nCan <code>n</code> fit into <code>byte_len</code> bytes?\nDeserialize <code>bytes</code> in little-endian form into a <code>$T</code>, padding …\nReturn the largest <code>$T</code> value that can fit into <code>byte_len</code> …\nSerialize <code>n</code> into <code>BYTE_LEN</code> bytes in little-endian form, …\nFetch the given list of accounts, retrying on transient …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA catchup implementation that falls back to a remote …\nFetch and remember the blocks frontier, retrying on …\nTry to fetch the given account state, failing without …\nTry to fetch and remember the blocks frontier, failing …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nGlobal variables for an Espresso blockchain.\nMinimum fee in WEI per byte of payload\nEspresso chain ID\nFee contract address on L1.\nAccount that receives sequencing fees.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMaximum size in bytes of a block\nThe consensus handle\nThe sequencer context contains a consensus handle and …\nReturn a reference to the underlying consensus handle.\nAllow this node to continue participating in consensus …\nStream consensus events.\nget event streamer\nevents streamer to stream hotshot events to external …\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe consensus handle\nInternal reference to the underlying [<code>SystemContext</code>]\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWait for consensus to complete.\nWait for all background tasks to complete.\nConstructor\nStop participating in consensus.\nStop all background tasks.\nSpawn a background task attached to this context.\nSpawn a background task attached to this <code>TaskList</code>.\nStart participating in consensus.\nReturn a reference to the consensus state signer.\nContext for generating state signatures.\nBackground tasks to shut down when the node is dropped.\nWait for a signal from the orchestrator before starting …\nAn orchestrator to wait for before starting consensus.\nAdd a list of tasks to the given context.\nSNAFU context selector for the <code>SigningError</code> error\nConsume the selector and return the associated error\nConsume the selector and return a <code>Result</code> with the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nR value\nS Value\nV value\nComplete block info.\nGenesis of an Espresso chain.\nInformation about the genesis state which feeds into the …\nAn L1 block from which an Espresso chain should start …\nAn L1 block number to sync from.\nInitial configuration of an Espresso stake table.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA header is like a [<code>Block</code>] with the body replaced by a …\nSNAFU context selector for the <code>InvalidBlockHeader</code> error\nRoot Commitment of Block Merkle Tree\nConsume the selector and return the associated error\nCommit over fee_amount, payload_commitment and metadata\nAccount (etheruem address) of builder\nA commitment to a ChainConfig or a full ChainConfig.\nConsume the selector and return a <code>Result</code> with the …\nFee paid by the block builder\nRoot Commitment of <code>FeeMerkleTree</code>\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe Espresso block header includes information a bout the …\nThe Espresso block header includes a reference to the …\nreward_balance at the moment is only implemented as a …\nTrait for generalized query service for parts of the …\nError type during synchronization between data sources …\nprepare the transaction from new leaves (with QC) from …\nIf specified, sequencing attempts will be delayed by …\nReturns the argument unchanged.\nReturns the argument unchanged.\nAddress of HotShot contract on layer 1.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChain ID for layer 1 Ethereum.\nURL of layer 1 Ethereum JSON-RPC provider.\nURL of HotShot Query Service\nClient-side timeout for HTTP requests.\nmain logic for the commitment task, which sync the latest …\nIndex of a funded account derived from sequencer-mnemonic.\nMnemonic phrase for a funded wallet.\nmain logic for catching up with HotShot contract on L1\nAn Http Provider and configuration to interact with the L1.\nMaximum number of L1 blocks that can be scanned for events …\nThe snapshot also includes information about the latest …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nProxy to <code>Provider.get_block_number</code>.\nProxy to <code>get_finalized_block</code>.\nGet fee info for each <code>Deposit</code> occurring between <code>prev</code> and …\nThe relevant snapshot of the L1 includes a reference to …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nInstantiate an <code>L1Client</code> for a given <code>Url</code>.\n<code>Provider</code> from <code>ethers-provider</code>.\nGet a snapshot from the l1.\nGet information about the given block.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe broker definition for the Push CDN. Uses the TCP …\nThe client definition for the Push CDN. Uses the Quic …\nThe DA topic\nThe global topic\nThe production run definition for the Push CDN. Uses the …\nThe testing run definition for the Push CDN. Uses the real …\nThe enum for the topics we can subscribe to in the Push CDN\nThe user definition for the Push CDN. Uses the Quic …\nA wrapped <code>SignatureKey</code>. We need to implement the Push CDN…\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSign a message of arbitrary data and return the serialized …\nVerify a message of arbitrary data and return the result\nSplit off the peer ID from a multiaddress, returning the …\nRun the state catchup API module.\nRun the explorer API module.\nRun the hotshot events API module.\nRun an HTTP server.\nSNAFU context selector for the <code>ParseDurationError</code> error\nSNAFU context selector for the <code>ParseSizeError</code> error\nRun the query API module.\nRun the merklized state  API module.\nRun the status API module.\nAlias for storage-fs.\nUse the file system for persistent storage.\nUse a Postgres database for persistent storage.\nRun the transaction submission API module.\nAdd this as an optional module. Return the next optional …\nConsume the selector and return the associated error\nConsume the selector and return the associated error\nThe socket address of the HotShot CDN’s main entry point …\nConsume the selector and return a <code>Result</code> with the …\nConsume the selector and return a <code>Result</code> with the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nPath to TOML file containing genesis state.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhether or not we are a DA node.\nPath to file containing private keys.\nMaximum number of L1 blocks that can be scanned for events …\nUrl we will use for RPC communication with L1.\nThe address we advertise to other nodes as being a Libp2p …\nThe address to bind to for Libp2p (in <code>host:port</code> form)\nA comma-separated list of Libp2p multiaddresses to use as …\nAdd optional modules to the service.\nAdd more optional modules.\nURL of the HotShot orchestrator.\nPrivate staking key.\nPrivate state signing key.\nPeer nodes use to fetch missing state\nURL of the Light Client State Relay Server\nbuilder to use\nThe maximum amount of time a leader can wait to get a …\nThe address for the Push CDN’s “marshal”, A.K.A. …\ncombined network config\nthe commit this run is based on\nthe hotshot config\nthe data availability web server config\ntime to wait until we request data associated with a …\nUpdate storage based on an event from consensus.\nwhether DA membership is determined by index. if true, the …\nUse this storage as a state catchup backend, if supported.\nname of the key type (for debugging)\nthe libp2p config\nLoad the latest leaf saved with <code>save_anchor_leaf</code>.\nLoad the orchestrator config from storage.\nLoad the latest known consensus state.\nLoad the highest view saved with <code>save_voted_view</code>.\nLoad the proposals saved by consensus\nLoad undecided state saved by consensus before we shut …\npassword to have the orchestrator start the network, …\ntimeout before starting the next view\nMock implementation of persistence, for testing.\nglobal index of node (for testing purposes a uid)\nnumber of bootstrap nodes\nrandom builder config\nnumber of views to run\nSaves the latest decided leaf.\nSave the orchestrator config to storage.\nunique seed (for randomness? TODO)\ndelay before beginning consensus\nsize of transactions\nnumber of transactions per view\ntimeout before starting next view sync round\nthe webserver config\nOptions for file system backed persistence.\nFile system backed persistence.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStorage path for persistent data.\nOverwrite a file if a condition is met.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nOptions for Postgres-backed persistence.\nPostgres-backed persistence.\nPruning parameters.\nThe minimum delay between active fetches in a stream.\nDisable pruning and reconstruct previously pruned data.\nBatch size for pruning. This is the number of blocks data …\nThe minimum delay between loading chunks in a stream.\nName of database to connect to.\nSpecifies the maximum number of concurrent fetch requests …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nHostname for the remote Postgres database server.\nInterval for running the pruner.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMaximum disk usage (in basis points).\nMinimum retention period. Data is retained for at least …\nPassword for Postgres user.\nPort for the remote Postgres database server.\nThis will enable the pruner and set the default pruning …\nPruning parameters.\nThreshold for pruning, specified in bytes. If the disk …\nTarget retention period. Data older than this is pruned to …\nPostgres URI.\nUse TLS for an encrypted connection to the database.\nPostgres user to connect as.\nPossible builder validation failures\nA proof of the balance of an account in the fee ledger.\nPossible charge fee failures\n<code>FeeInfo</code> holds data related to builder fees.\nPossible proposal validation failures\nThis enum is not used in code but functions as an index of …\nReturn inner <code>Address</code>\nUpdates the <code>ValidatedState</code> if a protocol upgrade has …\nReturn byte slice representation of inner <code>Address</code> type\nThe minimum fee paid by the given builder account for a …\nFrontier of Block Merkle Tree\nCharge a fee to an account, transferring the funds to the …\nFee Merkle Tree\nFind accounts that are not in memory.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct the state with the given block header.\nConstruct a genesis validated state.\nRetrieves the <code>ChainConfig</code>.\nInsert a fee deposit receipt\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCheck if the merkle tree is available\nPrefund an account with a given amount. Only for demo …\nReturn array containing underlying bytes of inner <code>Address</code> …\nValidate parent against known values (from state) and …\nValidate builder account by verifying signature\nCapacity for the in memory signature storage.\nType for stake table commitment\nA rolling in-memory storage for the most recent light …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturn a signature of a light client state at given height.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nKey pair for signing a new light client state\nA relay server that’s collecting and serving the light …\nThe state relay server url\nSign the light client state at given height and store it.\nThe most recent light client state signatures\nCommitment for current fixed stake table\nHelper function for stake table commitment\nConnect to the given state relay server to send signed …\nconfigurability options for the web server\nState that checks the light client state update and the …\npath to API\nSignatures bundles for each block height\nSet up APIs for relay server\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the latest available signatures bundle.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStake table\nThe latest state signatures bundle whose total weight …\nThe block height of the latest available state signature …\nPost a signature to the relay server\nA ordered queue of block heights, used for garbage …\nshutdown signal\nMinimum weight to form an available state signature bundle\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTODO <code>NamespaceId</code> has historical debt to repay:\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUseful for when we want to test size of transaction(s)")