use std::{
    num::{NonZero, NonZeroU32},
    sync::Arc,
};

use crate::{
    service::{
        client_id::ClientId,
        client_message::InternalClientMessage,
        client_state::{
            ClientThreadState, InternalClientMessageProcessingTask,
            ProcessDistributeServerMessageHandlingTask, ProcessPutInscriptionToChainTask,
            ProcessRecordPutInscriptionRequestTask,
        },
        data_state::{DataState, ProcessBlockStreamTask, Stats, MAX_LOCAL_INSCRIPTION_HISTORY},
        espresso_inscription::EspressoInscription,
        server_message::ServerMessage,
        storage::{
            in_memory::HeightCachingInMemory, postgres::PostgresPersistence,
            InscriptionPersistence, RetrieveLastReceivedBlockError,
        },
    },
    Options,
};
use alloy::signers::local::PrivateKeySigner;
use async_std::sync::RwLock;
use espresso_types::NamespaceId;
use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt,
};
use governor::{Quota, RateLimiter};
use url::Url;

use super::{HotshotQueryServiceBlockStreamRetriever, ProcessProduceBlockStreamTask};

pub struct InscriptionsAPI {
    pub process_internal_client_message_handle: Option<InternalClientMessageProcessingTask>,
    pub process_block_stream_handle: Option<ProcessBlockStreamTask>,
    pub process_distribute_inscription_handle: Option<ProcessDistributeServerMessageHandlingTask>,
    pub process_put_inscription_to_chain_handle: Option<ProcessPutInscriptionToChainTask>,
    pub process_record_put_inscription_handle: Option<ProcessRecordPutInscriptionRequestTask>,
    pub process_produce_blocks_handle: Option<ProcessProduceBlockStreamTask>,
}

#[derive(Debug, Clone)]
pub struct InscriptionsConfig {
    /// The URL to the block stream source
    block_stream_source_base_url: Url,

    /// The URL to submit inscriptions to
    submit_url: Url,

    /// inscription_namespace_id is the NamespaceId that is used to identify
    /// the Inscriptions that are being submitted to the Espresso Block Chain.
    inscription_namespace_id: NamespaceId,

    /// put_inscriptions_per_second is the setting to effectively rate limit
    /// the number of inscriptions that can be submitted to the Espresso
    /// Transaction mempool per second.
    put_inscriptions_per_second: u32,

    /// postgres_url is the URL to the Postgres database that is used to store
    /// the inscriptions.
    postgres_url: Url,

    /// minimum_start_block is the minimum block that the inscriptions service
    /// should start processing from.
    minimum_start_block: u64,
}

#[derive(Debug)]
pub enum CreateInscriptionsConfigError {
    UrlParseError(url::ParseError),
    SetPathError,
    SetPortError,
    SetUsernameError,
    SetPasswordError,
    SetHostError,
}

impl From<url::ParseError> for CreateInscriptionsConfigError {
    fn from(err: url::ParseError) -> Self {
        CreateInscriptionsConfigError::UrlParseError(err)
    }
}

impl TryFrom<&Options> for InscriptionsConfig {
    type Error = CreateInscriptionsConfigError;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        let mut postgres_url = Url::parse("postgres://localhost/")?;

        postgres_url.set_path(options.postgres_database().as_str());
        postgres_url
            .set_port(Some(options.postgres_port()))
            .or(Err(CreateInscriptionsConfigError::SetPortError))?;
        postgres_url
            .set_username(options.postgres_user().as_str())
            .or(Err(CreateInscriptionsConfigError::SetUsernameError))?;
        postgres_url
            .set_password(Some(options.postgres_password().as_str()))
            .or(Err(CreateInscriptionsConfigError::SetPasswordError))?;
        postgres_url
            .set_host(Some(options.postgres_host().as_str()))
            .or(Err(CreateInscriptionsConfigError::SetHostError))?;

        Ok(InscriptionsConfig {
            block_stream_source_base_url: options.block_stream_source_base_url(),
            submit_url: options.submit_base_url(),
            inscription_namespace_id: options.inscriptions_namespace_id(),
            postgres_url,
            put_inscriptions_per_second: options.put_inscriptions_per_second(),
            minimum_start_block: options.minimum_block_height(),
        })
    }
}

impl InscriptionsConfig {
    /// submit_url returns the URL that is used to submit inscriptions to the
    /// Espresso Block Chain.
    fn submit_url(&self) -> Url {
        self.submit_url.clone()
    }

    /// inscription_namespace_id returns the NamespaceId that is used to
    /// identify the Inscriptions that are being submitted to the Espresso
    /// Block Chain.
    fn inscription_namespace_id(&self) -> NamespaceId {
        self.inscription_namespace_id
    }

    /// put_inscriptions_per_second returns the number of inscriptions that can
    /// be submitted to the Espresso Block Chain per second.
    fn put_inscriptions_per_second(&self) -> u32 {
        self.put_inscriptions_per_second
    }

    /// minimum_start_block returns the minimum block that the inscriptions
    /// service should start processing from.
    fn minimum_start_block(&self) -> u64 {
        self.minimum_start_block
    }

    /// postgres_url returns the URL to the Postgres database that is used to
    /// store the inscriptions service persistent state.
    fn postgres_url(&self) -> Url {
        self.postgres_url.clone()
    }

    /// block_stream_source_base_url returns the base URL to the block stream
    /// source that originates from the availability API of the HotShot
    /// Query Service.
    fn block_stream_source_base_url(&self) -> Url {
        self.block_stream_source_base_url.clone()
    }
}

#[cfg(test)]
impl InscriptionsConfig {
    fn new_testing(block_stream_source_base_url: Url, submit_url: Url, postgres_url: Url) -> Self {
        InscriptionsConfig {
            block_stream_source_base_url,
            submit_url,
            postgres_url,
            inscription_namespace_id: NamespaceId::from(0x7e57u32),
            put_inscriptions_per_second: 20,
            minimum_start_block: 0,
        }
    }
}

#[derive(Debug)]
pub enum CreateInscriptionsProcessingError {
    PostgresError(sqlx::Error),
    MigrationError(sqlx::migrate::MigrateError),
    CreateInscriptionsConfigError(CreateInscriptionsConfigError),
    RetrieveLastReceivedBlockError(RetrieveLastReceivedBlockError),
}

impl From<sqlx::Error> for CreateInscriptionsProcessingError {
    fn from(err: sqlx::Error) -> Self {
        CreateInscriptionsProcessingError::PostgresError(err)
    }
}

impl From<sqlx::migrate::MigrateError> for CreateInscriptionsProcessingError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        CreateInscriptionsProcessingError::MigrationError(err)
    }
}

impl From<CreateInscriptionsConfigError> for CreateInscriptionsProcessingError {
    fn from(err: CreateInscriptionsConfigError) -> Self {
        CreateInscriptionsProcessingError::CreateInscriptionsConfigError(err)
    }
}

impl From<RetrieveLastReceivedBlockError> for CreateInscriptionsProcessingError {
    fn from(err: RetrieveLastReceivedBlockError) -> Self {
        CreateInscriptionsProcessingError::RetrieveLastReceivedBlockError(err)
    }
}

/**
 * create_inscriptions_processing is a function that creates a inscriptions
 * service processing environment.  This function will create a number of tasks
 * that will be responsible for processing the data streams that are coming in
 * from the various sources.  This function will also create the data state that
 * will be used to store the state of the network.
 */
pub async fn create_inscriptions_processing(
    config: InscriptionsConfig,
    internal_client_message_receiver: Receiver<InternalClientMessage<Sender<ServerMessage>>>,
    put_inscription_record_receiver: Receiver<EspressoInscription>,
    put_inscription_to_chain_receiver: Receiver<EspressoInscription>,
    put_inscription_to_chain_sender: Sender<EspressoInscription>,
    signer: PrivateKeySigner,
) -> Result<InscriptionsAPI, CreateInscriptionsProcessingError> {
    let client_thread_state = ClientThreadState::<Sender<ServerMessage>>::new(
        Default::default(),
        ClientId::from_count(1),
    );

    let rate_limiter = {
        let Some(non_zero_limit) = NonZeroU32::new(config.put_inscriptions_per_second()) else {
            panic!("Failed to create non-zero limit");
        };

        RateLimiter::direct(Quota::per_second(non_zero_limit))
    };

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(config.postgres_url().to_string().as_str())
        .await?;

    // Run all of the migrations
    sqlx::migrate!().run(&pool).await?;

    let persistence = Arc::new(HeightCachingInMemory::new(PostgresPersistence::new(pool)));

    let (block_sender, block_receiver) = mpsc::channel(10);

    let mut data_state = DataState::new(Default::default(), persistence.clone(), signer.address());

    let Stats {
        num_blocks: block_height,
        num_transactions,
        num_inscriptions,
    } = persistence.retrieve_last_received_block().await?;

    tracing::debug!("launching service with the following recovered stats: block_height: {}, num_transactions: {}, num_inscriptions: {}", block_height, num_transactions, num_inscriptions);

    // Restore the previous number of transactions and the block height.
    data_state.add_num_transactions(block_height, num_transactions);
    data_state.add_num_inscriptions(block_height, num_inscriptions);

    let limit = NonZero::<usize>::new(MAX_LOCAL_INSCRIPTION_HISTORY).unwrap();
    let latest_inscriptions_result: Result<
        Vec<crate::service::espresso_inscription::InscriptionAndChainDetails>,
        crate::service::storage::RetrieveLatestInscriptionAndChainDetailsError,
    > = persistence
        .retrieve_latest_inscription_and_chain_details(limit)
        .await;

    {
        // Retrieve the latest inscriptions
        let latest_inscriptions = match latest_inscriptions_result {
            Ok(latest_inscriptions) => latest_inscriptions,
            Err(err) => {
                tracing::error!("error retrieving latest inscriptions: {:?}", err);
                panic!("error retrieving latest inscriptions: {:?}", err);
            }
        };

        tracing::info!(
            "bootstrapping with {} previously loaded inscriptions",
            latest_inscriptions.len()
        );

        // populate the latest inscriptions
        for inscription_and_chain_details in latest_inscriptions {
            data_state.add_latest_inscription(inscription_and_chain_details);
        }
    }

    // Retrieve all pending put inscriptions request, and front-load them.
    {
        let pending_inscriptions_result = persistence.retrieve_pending_put_inscriptions().await;

        let pending_inscriptions = match pending_inscriptions_result {
            Ok(pending_inscriptions) => pending_inscriptions,
            Err(err) => {
                tracing::error!("error retrieving pending inscriptions: {:?}", err);
                panic!("error retrieving pending inscriptions: {:?}", err);
            }
        };

        tracing::info!(
            "bootstrapping with {} previous put inscription requests",
            pending_inscriptions.len()
        );

        let mut put_inscription_to_chain_sender = put_inscription_to_chain_sender;

        for inscription in pending_inscriptions {
            if let Err(err) = put_inscription_to_chain_sender.send(inscription).await {
                tracing::error!("error sending inscription to chain: {:?}", err);
                panic!("error sending inscription to chain: {:?}", err);
            }
        }
    }

    let process_produce_blocks = ProcessProduceBlockStreamTask::new(
        HotshotQueryServiceBlockStreamRetriever::new(config.block_stream_source_base_url()),
        persistence.clone(),
        config.minimum_start_block(),
        block_sender,
    );

    let data_state = Arc::new(RwLock::new(data_state));
    let client_thread_state = Arc::new(RwLock::new(client_thread_state));
    let (server_message_sender, server_message_receiver) = mpsc::channel(32);

    let process_internal_client_message_handle = InternalClientMessageProcessingTask::new(
        internal_client_message_receiver,
        data_state.clone(),
        client_thread_state.clone(),
    );

    let process_block_stream_handle = ProcessBlockStreamTask::new(
        config.inscription_namespace_id(),
        block_receiver,
        data_state.clone(),
        server_message_sender,
    );

    let process_distribute_server_message_handle = ProcessDistributeServerMessageHandlingTask::new(
        client_thread_state.clone(),
        server_message_receiver,
    );

    let process_record_put_inscription_handle = ProcessRecordPutInscriptionRequestTask::new(
        data_state.clone(),
        put_inscription_record_receiver,
    );

    let process_put_inscription_to_chain_handle = ProcessPutInscriptionToChainTask::new(
        rate_limiter,
        data_state.clone(),
        config.inscription_namespace_id(),
        signer,
        put_inscription_to_chain_receiver,
        config.submit_url(),
    );

    Ok(InscriptionsAPI {
        process_produce_blocks_handle: Some(process_produce_blocks),
        process_internal_client_message_handle: Some(process_internal_client_message_handle),
        process_block_stream_handle: Some(process_block_stream_handle),
        process_distribute_inscription_handle: Some(process_distribute_server_message_handle),
        process_put_inscription_to_chain_handle: Some(process_put_inscription_to_chain_handle),
        process_record_put_inscription_handle: Some(process_record_put_inscription_handle),
    })
}

#[cfg(test)]
mod test {
    use crate::{
        api::inscriptions::v0::{Error, StateClientMessageSender, STATIC_VER_0_1},
        service::{
            client_message::InternalClientMessage, espresso_inscription::EspressoInscription,
            server_message::ServerMessage,
        },
    };
    use alloy::signers::local::PrivateKeySigner;
    use futures::channel::mpsc::{self, Sender};
    use tide_disco::App;
    use url::Url;

    #[derive(Clone)]
    struct TestState(
        Sender<InternalClientMessage<Sender<ServerMessage>>>,
        Sender<EspressoInscription>,
        Sender<EspressoInscription>,
    );

    #[async_trait::async_trait]
    impl StateClientMessageSender<Sender<ServerMessage>> for TestState {
        fn internal_client_message_sender(
            &self,
        ) -> Sender<InternalClientMessage<Sender<ServerMessage>>> {
            self.0.clone()
        }

        async fn put_inscription(&self, inscription: EspressoInscription) -> Result<(), Error> {
            let mut sender1 = self.1.clone();
            let mut sender2 = self.2.clone();

            match sender2.try_send(inscription.clone()) {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("error sending inscription: {:?}", err);
                    return Err(Error::TooManyRequests);
                }
            }

            match sender1.try_send(inscription) {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    tracing::error!("error sending inscription: {:?}", err);
                    Err(Error::TooManyRequests)
                }
            }
        }
    }

    #[async_std::test]
    #[ignore]
    async fn test_full_setup_example() {
        let (internal_client_message_sender, internal_client_message_receiver) = mpsc::channel(32);
        let (put_inscription_to_chain_sender, put_inscription_to_chain_receiver) =
            mpsc::channel(32);
        let (put_inscription_record_sender, put_inscription_record_receiver) = mpsc::channel(32);
        let state = TestState(
            internal_client_message_sender,
            put_inscription_record_sender,
            put_inscription_to_chain_sender.clone(),
        );

        let mut app: App<_, crate::api::inscriptions::v0::Error> = App::with_state(state);
        let inscriptions_api_result = super::super::define_api::<TestState>();
        let inscriptions_api = match inscriptions_api_result {
            Ok(inscriptions_api) => inscriptions_api,
            Err(err) => {
                panic!("error defining inscriptions api: {:?}", err);
            }
        };

        match app.register_module("inscriptions", inscriptions_api) {
            Ok(_) => {}
            Err(err) => {
                panic!("error registering inscriptions api: {:?}", err);
            }
        }

        let inscriptions_task_state = match super::create_inscriptions_processing(
            super::InscriptionsConfig::new_testing(
                Url::parse("http://query.main.net.espresso.network/v0/").unwrap(),
                Url::parse("http://localhost:8000/v0/submit/submit").unwrap(),
                Url::parse("postgres://localhost/inscriptions").unwrap(),
            ),
            internal_client_message_receiver,
            put_inscription_record_receiver,
            put_inscription_to_chain_receiver,
            put_inscription_to_chain_sender,
            PrivateKeySigner::random(),
        )
        .await
        {
            Ok(inscriptions_task_state) => inscriptions_task_state,

            Err(err) => {
                panic!("error defining inscriptions api: {:?}", err);
            }
        };

        // We would like to wait until being signaled
        let app_serve_handle = async_std::task::spawn(async move {
            let app_serve_result = app.serve("0.0.0.0:9000", STATIC_VER_0_1).await;
            tracing::info!("app serve result: {:?}", app_serve_result);
        });
        tracing::info!("now listening on port 9000");

        app_serve_handle.await;

        drop(inscriptions_task_state);
    }
}
