use crate::{
    transaction::{SequencerTransaction, Transaction},
    SeqTypes,
};
use async_std::{
    sync::Mutex,
    task::{spawn, JoinHandle},
};
use futures::FutureExt;
use hotshot::traits::election::static_committee::StaticCommittee;
use hotshot::traits::implementations::MemoryStorage;
use hotshot::traits::NodeImplementation;
use hotshot::types::HotShotHandle;
use std::io;
use tide_disco::{error::ServerError, Api, App, StatusCode};

pub fn serve<
    I: NodeImplementation<
        SeqTypes,
        Storage = MemoryStorage<SeqTypes>,
        Election = StaticCommittee<SeqTypes>,
    >,
>(
    init_handle: HotShotHandle<SeqTypes, I>,
    port: u16,
) -> io::Result<JoinHandle<io::Result<()>>> {
    type StateType<I> = Mutex<HotShotHandle<SeqTypes, I>>;

    let mut app = App::<StateType<I>, ServerError>::with_state(Mutex::new(init_handle));

    // Include API specification in binary
    let toml = toml::from_str::<toml::value::Value>(include_str!("api.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    let mut api = Api::<StateType<I>, ServerError>::new(toml)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    // Pass transaction from request body into HotShot handle
    api.post("submit", |req, state| {
        async move {
            state
                .submit_transaction(SequencerTransaction::Wrapped(
                    req.body_auto::<Transaction>()?,
                ))
                .await
                .map_err(|err| ServerError {
                    status: StatusCode::InternalServerError,
                    message: err.to_string(),
                })
        }
        .boxed()
    })
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    app.register_module("api", api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    Ok(spawn(app.serve(format!("0.0.0.0:{}", port))))
}

#[cfg(test)]
mod test {
    use crate::{
        api::serve,
        test::{init_hotshot_handles, wait_for_decide_on_handle},
        transaction::{SequencerTransaction, Transaction},
        vm::VmId,
    };
    use portpicker::pick_unused_port;
    use surf_disco::Client;
    use tide_disco::error::ServerError;

    #[async_std::test]
    async fn submit_test() {
        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{}", port).parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        // Get list of HotShot handles, take the first one, and submit a transaction to it
        let handles = init_hotshot_handles().await;
        serve(handles[0].clone(), port).unwrap();

        client.connect(None).await;

        client
            .post::<()>("api/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(handles[0].clone(), SequencerTransaction::Wrapped(txn))
            .await
            .unwrap()
    }
}
