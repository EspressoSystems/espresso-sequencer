# Rollup Contract

```solidity

/// Does not use the Espresso Light client contract for fetching the state
contract RollupContract1 {

    State previousState;
    State currentState;
    uint256 lastEspressoBlockNumber;


    function updateRollupState(
            newState,
            commTxsRollup,
            commTxsEspresso,
            blockCommEspresso,
            blockNumberEspresso,
            snarkProof){

        previousState = currentState;

        // TODO call to Espresso contract
        if (escapeHatch){
            bytes[] publicInputs = [
                        previousState,
                        newState,
                        commTxsRollup
                ];

            SnarkVerify(
                publicInputs,
                snarkProof,
                vkRollup
            );

        } else { // No escape hatch, use the state of the Espresso sequencer

            if (blockNumberEspresso <= lastBlockNumberEspresso) {
                revert();
            }

            bytes[] publicInputs = [
                        previousState,
                        newState,
                        commTxsRollup,
                        commTxsEspresso,
                        blockCommEspresso,
                        blockNumberEspresso
                    ];

            SnarkVerify(
                publicInputs,
                snarkProof,
                vkEspresso
            );

        }

        currentState = newState;
        lastBlockNumberEspresso =  blockNumberEspresso;
    }

}

/// Connects to the Espresso Light Client contract to fetch the last state
contract RollupContract2 {

    State previousState;
    State currentState;
    uint256 lastEspressoBlockNumber;


    function updateRollupState(
        newState,
        commTxsRollup,
        commTxsEspresso,
        snarkProof){

        previousState = currentState;

        // TODO call to Espresso contract
        if (escapeHatch){
            bytes[] publicInputs = [
            previousState,
            newState,
            commTxsRollup
            ];

            SnarkVerify(
            publicInputs,
            snarkProof,
            vkRollup
        );

        } else { // No escape hatch, use the state of the Espresso sequencer

            (blockCommEspresso,  blockNumberEspresso) = EspressoLightClientContract.getLastBlockCommitment();

            if (blockNumberEspresso <= lastBlockNumberEspresso) {
                revert();
            }

            bytes[] publicInputs = [
                previousState,
                newState,
                commTxsRollup,
                commTxsEspresso,
                blockCommEspresso,
                blockNumberEspresso
            ];

            SnarkVerify(
                publicInputs,
                snarkProof,
                vkEspresso
            );

        }

        currentState = newState;
        lastBlockNumberEspresso =  blockNumberEspresso;
    }
}

```
