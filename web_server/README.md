# Web Server

This crate implements a web server that HotShot can use to increase its throughput.  The web server is designed to be run behind several Nginx instances for high performance.  

The run the web server: `cargo run --features=full-ci --example web_server`.  
This will start the web server at `0.0.0.0:9000`

## How it works
Unlike the other networking implementations in `HotShot` that use a *pushing* paradigm over a TCP connection, the web server uses a *pulling* paradigm over HTTP.  In the older centralized server, for example, messages are pushed to nodes from the server.  In contrast, in the web server nodes must poll (pull from) the server periodically to download the latest data.  The direction of dataflow is reversed.  This design allows us to take advantage of Nginx's powerful caching mode, which will improve performance in the large networks `HotShot` is designed for.  Keep in mind that `HotShot` will also be using `Libp2p` in tandem with the web server.  The web server allows high bandwidth through the network under optimistic conditions while `Libp2p` protects against centralization and censorship.  

The web server is built on [Tide Disco](https://github.com/EspressoSystems/tide-disco), an expansion on the popular [Tide](https://docs.rs/tide/0.16.0/tide/index.html) Rust web application framework.  It has several endpoints specified in its `api.toml` file.  These endpoints are what nodes use to either POST or GET messages.  For example, a replica node will poll the web server periodically through the `proposal` endpoint to ask if there is a proposal for its current view.  The server will respond with either an error if there is no proposal for that view yet, or with the proposal.  It works similarly for transactions: Nodes can post transactions to the web server at any time.  They can also download all the transactions they haven't received yet from the web server.  



