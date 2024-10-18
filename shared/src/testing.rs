pub mod constants {
    //! Shared constants for tests

    use std::time::Duration;

    /// Controls the maximum block size of this protocol instance.
    /// This is an arbitrary default value for testing.
    pub const TEST_PROTOCOL_MAX_BLOCK_SIZE: u64 = 1_000_000;

    /// Controls period between optimistic increments of maximum block size limit.
    /// This is an arbitrary default value for testing.
    pub const TEST_MAX_BLOCK_SIZE_INCREMENT_PERIOD: Duration = Duration::from_secs(60);

    /// Controls the number of nodes that are used in the VID computation for the tests.
    pub const TEST_NUM_NODES_IN_VID_COMPUTATION: usize = 4;

    /// Controls the number of attempts that the simulated consensus will
    /// perform when an error is returned from the Builder when asking for available blocks.
    pub const TEST_NUM_CONSENSUS_RETRIES: usize = 4;

    /// Governs the buffer size used for the test channels.
    /// All of the channels created need a capacity. The concrete capacity isn't
    /// specifically bounded in tests, so it is set to an arbitrary value.
    pub const TEST_CHANNEL_BUFFER_SIZE: usize = 32;
}
