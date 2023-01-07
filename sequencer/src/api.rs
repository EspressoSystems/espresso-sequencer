use crate::state::State;
use std::io;
use tide_disco::{Api, App};

#[derive(Debug, PartialEq)]
struct DummyError;

async fn _serve(_port: u16) -> io::Result<()> {
    // This is here just to avoid a dead code warning
    let my_error = DummyError {};
    dbg!(my_error);

    let mut app = App::<State, DummyError>::with_state(State::default());
    let api = Api::<State, DummyError>::from_file("src/api.toml").unwrap();

    match app.register_module("submit", api) {
        Ok(_) => println!("ok! :)"),
        Err(_) => println!("error :("),
    }

    // serve here

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::api::_serve;
    use std::io;

    #[async_std::test]
    async fn placeholder_test() -> io::Result<()> {
        _serve(8080).await
    }
}
