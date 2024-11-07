use std::{num::NonZeroU32, sync::Arc};

use crate::{
    service::{
        client_id::ClientId,
        client_message::InternalClientMessage,
        client_state::{
            ClientThreadState, InternalClientMessageProcessingTask,
            ProcessDistributeInscriptionHandlingTask, ProcessRecordInscriptionHandlingTask,
        },
        data_state::{DataState, ProcessBlockStreamTask},
        espresso_inscription::EspressoInscription,
        server_message::ServerMessage,
    },
    Options,
};
use alloy::signers::local::PrivateKeySigner;
use async_std::sync::RwLock;
use espresso_types::{NamespaceId, SeqTypes};
use futures::channel::mpsc::{self, Receiver, Sender};
use governor::{Quota, RateLimiter};
use hotshot_query_service::availability::BlockQueryData;
use url::Url;

pub struct InscriptionsAPI {
    pub process_internal_client_message_handle: Option<InternalClientMessageProcessingTask>,
    pub process_block_stream_handle: Option<ProcessBlockStreamTask>,
    pub process_distribute_inscription_handle: Option<ProcessDistributeInscriptionHandlingTask>,
    pub process_record_inscription_handle: Option<ProcessRecordInscriptionHandlingTask>,
}

#[derive(Debug, Clone)]
pub struct InscriptionsConfig {
    /// The URL to submit inscriptions to
    submit_url: Url,

    /// inscription_namespace_id is the NamespaceId that is used to identify
    /// the Inscriptions that are being submitted to the Espresso Block Chain.
    inscription_namespace_id: NamespaceId,

    /// put_inscriptions_per_second is the setting to effectively rate limit
    /// the number of inscriptions that can be submitted to the Espresso
    /// Transaction mempool per second.
    put_inscriptions_per_second: u32,
}

impl From<&Options> for InscriptionsConfig {
    fn from(options: &Options) -> Self {
        InscriptionsConfig {
            submit_url: options.submit_base_url(),
            inscription_namespace_id: options.inscriptions_namespace_id(),
            put_inscriptions_per_second: options.put_inscriptions_per_second(),
        }
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
}

#[cfg(test)]
impl InscriptionsConfig {
    fn new_testing(submit_url: Url) -> Self {
        InscriptionsConfig {
            submit_url,
            inscription_namespace_id: NamespaceId::from(0x7e57u32),
            put_inscriptions_per_second: 20,
        }
    }
}

#[derive(Debug)]
pub enum CreateInscriptionsProcessingError {}

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
    block_receiver: Receiver<BlockQueryData<SeqTypes>>,
    put_inscriptions_receiver: Receiver<EspressoInscription>,
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

    let data_state = DataState::new(Default::default(), signer.address());

    let data_state = Arc::new(RwLock::new(data_state));
    let client_thread_state = Arc::new(RwLock::new(client_thread_state));
    let (inscription_sender, inscription_receiver) = mpsc::channel(32);

    let process_internal_client_message_handle = InternalClientMessageProcessingTask::new(
        internal_client_message_receiver,
        data_state.clone(),
        client_thread_state.clone(),
    );

    let process_block_stream_handle = ProcessBlockStreamTask::new(
        config.inscription_namespace_id(),
        block_receiver,
        data_state.clone(),
        inscription_sender,
    );

    let process_distribute_inscription_handle = ProcessDistributeInscriptionHandlingTask::new(
        client_thread_state.clone(),
        inscription_receiver,
    );

    let process_record_inscription_handle = ProcessRecordInscriptionHandlingTask::new(
        rate_limiter,
        config.inscription_namespace_id(),
        signer,
        put_inscriptions_receiver,
        config.submit_url(),
    );

    Ok(InscriptionsAPI {
        process_internal_client_message_handle: Some(process_internal_client_message_handle),
        process_block_stream_handle: Some(process_block_stream_handle),
        process_distribute_inscription_handle: Some(process_distribute_inscription_handle),
        process_record_inscription_handle: Some(process_record_inscription_handle),
    })
}

#[cfg(test)]
mod test {
    use crate::{
        api::inscriptions::v0::{
            Error, HotshotQueryServiceBlockStreamRetriever, ProcessProduceBlockStreamTask,
            StateClientMessageSender, STATIC_VER_0_1,
        },
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
    );

    #[async_trait::async_trait]
    impl StateClientMessageSender<Sender<ServerMessage>> for TestState {
        fn internal_client_message_sender(
            &self,
        ) -> Sender<InternalClientMessage<Sender<ServerMessage>>> {
            self.0.clone()
        }

        async fn put_inscription(&self, inscription: EspressoInscription) -> Result<(), Error> {
            let mut sender = self.1.clone();

            match sender.try_send(inscription) {
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
        let (put_inscription_sender, put_inscription_receiver) = mpsc::channel(32);
        let state = TestState(internal_client_message_sender, put_inscription_sender);

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

        let (block_sender, block_receiver) = mpsc::channel(10);

        let process_consume_leaves = ProcessProduceBlockStreamTask::new(
            HotshotQueryServiceBlockStreamRetriever::new(
                "https://query.decaf.testnet.espresso.network/v0"
                    .parse()
                    .unwrap(),
            ),
            block_sender,
        );

        let inscriptions_task_state = match super::create_inscriptions_processing(
            super::InscriptionsConfig::new_testing(
                Url::parse("http://localhost:8000/v0/submit/submit").unwrap(),
            ),
            internal_client_message_receiver,
            block_receiver,
            put_inscription_receiver,
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
        drop(process_consume_leaves);
    }
}
