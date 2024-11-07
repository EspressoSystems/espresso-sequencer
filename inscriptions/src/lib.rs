pub mod api;
pub mod service;

use api::inscriptions::v0::{
    create_inscriptions_api::{create_inscriptions_processing, InscriptionsConfig},
    Error, HotshotQueryServiceBlockStreamRetriever, ProcessProduceBlockStreamTask,
    StateClientMessageSender, STATIC_VER_0_1,
};
use async_std::sync::RwLock;
use clap::Parser;
use espresso_types::NamespaceId;
use futures::channel::mpsc::{self, Sender};
use service::{client_message::InternalClientMessage, espresso_inscription::EspressoInscription};
use std::sync::Arc;
use tide_disco::App;
use url::Url;

/// Options represents the configuration options that are available for running
/// the inscriptions service via the [run_standalone_service] function.
/// These options are configurable via command line arguments or environment
/// variables.
#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// block_stream_source_base_url is the base URL for the availability API
    /// endpoint that is provided by Espresso Sequencers.
    ///
    /// This endpoint is expected to point to the version root path of the
    /// URL.
    /// Example:
    ///   - https://query.cappuccino.testnet.espresso.network/v0/
    #[clap(long, env = "ESPRESSO_INSCRIPTIONS_BLOCK_STREAM_SOURCE_BASE_URL")]
    block_stream_source_base_url: Url,

    /// submit_base_url is the base URL for the submit endpoint that is
    /// used for submitting data to the Espresso Block Chain.
    ///
    /// Please note that this can be either the submission endpoint for
    /// the Espresso Block Chain (which can be either the public mempool
    /// URL or the private mempool URL) or the submission endpoint for
    /// the Espresso Block Chain's Ingestion Service.
    ///
    /// Example:
    ///   - https://query.cappucino.testnet.espresso.network/v0/submit
    #[clap(long, env = "ESPRESSO_INSCRIPTIONS_SUBMIT_BASE_URL")]
    submit_base_url: Url,

    /// signer_mnemonic is the mnemonic that is used to generate the private
    /// key that will be used to sign the Inscriptions that are being
    /// submitted to the Espresso Block Chain.
    #[clap(
        long,
        env = "ESPRESSO_INSCRIPTIONS_SIGNER_MNEMONIC",
        default_value = "test test test test test test test test test test test test test test switch"
    )]
    signer_mnemonic: String,

    /// port is the port that the API service will be listening on.  If not
    /// specified, it will default to 9001.
    #[clap(long, env = "ESPRESSO_INSCRIPTIONS_PORT", default_value = "9001")]
    port: u16,

    /// put_inscription_buffer_size is the size of the buffer that is used
    /// to store pending inscriptions that are waiting to be submitted to
    /// the Espresso Block Chain.
    ///
    /// If the queue is at capacity, the service will return an error indicating
    /// that the service is too busy to accept new requests.
    #[clap(
        long,
        env = "ESPRESSO_INSCRIPTIONS_PUT_INSCRIPTION_BUFFER_SIZE",
        default_value = "1024"
    )]
    put_inscription_buffer_size: usize,

    /// put_inscriptions_per_second is the number of inscriptions that can be
    /// submitted to the Espresso Block Chain per second.
    ///
    /// This is used to throttle the number of inscriptions that are being
    /// submitted to the Espresso Block Chain.
    #[clap(
        long,
        env = "ESPRESSO_INSCRIPTIONS_PUT_INSCRIPTIONS_PER_SECOND",
        default_value = "50"
    )]
    put_inscriptions_per_second: u32,

    /// inscriptions_namespace_id represents the [espresso_types::NamespaceId]
    /// for the Espresso Inscription transactions that will be submitted to the
    /// Espresso Block Chain.
    ///
    /// Note: Just for fun, the the default value of the
    /// NamespaceId is the ASCII representation of "SIGN" in Big Endian Order.
    #[clap(
        long,
        env = "ESPRESSO_INSCRIPTIONS_NAMESPACE_ID",
        default_value = "0x5349474e"
    )]
    inscriptions_namespace_id: u32,
}

impl Options {
    /// block_stream_source_base_url returns the base URL for the availability
    /// API endpoint that is provided by Espresso Sequencers.
    pub fn block_stream_source_base_url(&self) -> Url {
        self.block_stream_source_base_url.clone()
    }

    /// submit_base_url returns the base URL for the submit endpoint that is
    /// used for submitting data to the Espresso Block Chain.
    pub fn submit_base_url(&self) -> Url {
        self.submit_base_url.clone()
    }

    /// signer_mnemonic returns the mnemonic that is used to generate the
    /// private key that will be used to sign the Inscriptions that are being
    /// submitted to the Espresso Block Chain.
    ///
    /// It is also utilized to verify the data coming back from the Espresso
    /// Block Chain.
    pub fn signer_mnemonic(&self) -> &str {
        &self.signer_mnemonic
    }

    /// port returns the port that the API service will be listening on.
    /// If not specified, it will default to 9001.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// put_inscription_buffer_size returns the size of the buffer that is used
    /// to store pending inscriptions that are waiting to be submitted to
    /// the Espresso Block Chain.
    pub fn put_inscription_buffer_size(&self) -> usize {
        self.put_inscription_buffer_size
    }

    /// put_inscriptions_per_second returns the number of inscriptions that can
    /// be submitted to the Espresso Block Chain per second.
    pub fn put_inscriptions_per_second(&self) -> u32 {
        self.put_inscriptions_per_second
    }

    /// inscriptions_namespace_id returns the [espresso_types::NamespaceId]
    /// for the Espresso Inscription transactions that will be submitted to the
    /// Espresso Block Chain.
    pub fn inscriptions_namespace_id(&self) -> NamespaceId {
        NamespaceId::from(self.inscriptions_namespace_id)
    }
}

/// MainState represents the State of the application this is available to
/// tide_disco.
struct MainState<K> {
    internal_client_message_sender: Sender<InternalClientMessage<K>>,

    put_inscription_sender: Arc<RwLock<Sender<EspressoInscription>>>,
}

#[async_trait::async_trait]
impl<K> StateClientMessageSender<K> for MainState<K>
where
    K: Send,
{
    fn internal_client_message_sender(&self) -> Sender<InternalClientMessage<K>> {
        self.internal_client_message_sender.clone()
    }

    async fn put_inscription(&self, inscription: EspressoInscription) -> Result<(), Error> {
        let mut sender = self.put_inscription_sender.write_arc().await;

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

/// Run the service by itself.
///
/// This function will run the inscription demo as its own service.  It has some
/// options that allow it to be configured in order for it to operate
/// effectively.
pub async fn run_standalone_service(options: Options) {
    let (internal_client_message_sender, internal_client_message_receiver) = mpsc::channel(32);
    let (put_inscription_sender, put_inscription_receiver) =
        mpsc::channel(options.put_inscription_buffer_size);
    let state = MainState {
        internal_client_message_sender,
        put_inscription_sender: Arc::new(RwLock::new(put_inscription_sender)),
    };

    let mut app: App<_, api::inscriptions::v0::Error> = App::with_state(state);
    let inscriptions_api =
        api::inscriptions::v0::define_api().expect("error defining inscriptions api");

    match app.register_module("inscriptions", inscriptions_api) {
        Ok(_) => {}
        Err(err) => {
            panic!("error registering inscriptions api: {:?}", err);
        }
    }

    let (block_sender, block_receiver) = mpsc::channel(10);

    let _process_consume_blocks = ProcessProduceBlockStreamTask::new(
        HotshotQueryServiceBlockStreamRetriever::new(options.block_stream_source_base_url()),
        block_sender,
    );

    let signer = match alloy::signers::local::MnemonicBuilder::<
        alloy::signers::local::coins_bip39::English,
    >::default()
    .phrase(options.signer_mnemonic())
    .build()
    {
        Ok(signer) => signer,
        Err(err) => {
            panic!(
                "failed to generate private key signer from mnemonic: {:?}",
                err
            );
        }
    };

    let _inscriptions_task_state = match create_inscriptions_processing(
        InscriptionsConfig::from(&options),
        internal_client_message_receiver,
        block_receiver,
        put_inscription_receiver,
        signer,
    )
    .await
    {
        Ok(inscriptions_task_state) => inscriptions_task_state,

        Err(err) => {
            panic!("error defining inscriptions api: {:?}", err);
        }
    };

    let port = options.port();
    // We would like to wait until being signaled
    let app_serve_handle = async_std::task::spawn(async move {
        let app_serve_result = app.serve(format!("0.0.0.0:{}", port), STATIC_VER_0_1).await;
        tracing::info!("app serve result: {:?}", app_serve_result);
    });

    app_serve_handle.await;
}
