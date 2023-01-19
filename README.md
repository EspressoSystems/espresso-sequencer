# espresso-sequencer

![Architecture diagram](./doc/architecture.svg)

## Development

- Obtain code: `git clone --recursive git@github.com:EspressoSystems/espresso-sequencer`.
- Make sure [nix](https://nixos.org/download.html) is installed.
- Activate the enviorment with `nix-shell`, or `nix develop`, or `direnv allow`
  if using [direnv](https://direnv.net/).

## Run the tests

    cargo test

## Building figures

    make doc

## Run the sequencer locally

A sequencer network can be run locally for easy experimentation and testing. Docker images and a
`docker-compose.yaml` are provided for convenience. The Docker-based demo fetches the images from
the `ghcr` repository, where they are updated with every push to `main` on GitHub. For testing
uncommitted changes, you can also run the same demo by manually building and running the services.

### Running with Docker

To get the latest images: `just pull`

To start the demo: `just demo`

To start the demo in the background: `just demo-background`. This can be useful because the command should exit sucessfully only once the demo is running.

To stop the demo: `just down`

### Running natively

Build all executables with `cargo build --release`. You may then start a single CDN server and
connect as many sequencer nodes as you'd like. To start the CDN, choose a port `$PORT` to run it on
and decide how many sequencer nodes `$N` you will use, then run
`target/release/cdn-server -p $PORT -n $N`.

The sequencer will distribute a HotShot configuration to all the nodes which connect to it, which
specifies consensus parameters like view timers. There is a default config, but you can override any
parameters you want by passing additional options to the `cdn-server` executable. Run
`target/release/cdn-server --help` to see a list of available options.

Once you have started the CDN server, you must connect `$N` sequencer nodes to it, after which the
network will start up automatically. To start one node, run
`target/release/sequencer --cdn-url tcp://localhost:$PORT`. A useful Bash snippet for running `$N`
nodes simultaneously in the background of your shell is:
```bash
for i in `seq $N`; do
    target/release/sequencer --cdn-url tcp://localhost:$PORT &
done
```

Note: if the sequencer shows a `"Connection refused"` error you may need to use
`127.0.0.1` instead of `localhost` when connecting to the CDN. This is because
`localhost` may resolve to `::1` if dual stack (ipv4 and ipv6) networking is
enabled.

## Implementation Plan

We will work towards the architecture illustrated above in three phases.

### Phase I: Basic Sequencing

Replace the Hermez trusted sequencer with a HotShot-based permissionless sequencer.

![Architecture Diagram Phase I](./doc/architecture_phase1.svg)

### Phase II: Off-Chain Data Availability

Only store batch commitments, not full batches, in the rollup contract. Use HotShot for data
availability.

![Architecture Diagram Phase II](./doc/architecture_phase2.svg)

### Phase III: Final Integration

Move adaptor service into zkEVM node for a smoother integration.

![Architecture Diagram Phase III](./doc/architecture.svg)
