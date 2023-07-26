for i in `seq $1`; do
      target/release/sequencer \
              --orchestrator-url http://localhost:8080 \
              --da-server-url http://localhost:8081 \
              --consensus-server-url http://localhost:8082 &
      sleep 15
done
