### Simple live Espresso block scanner
### Scan all blocks looking for transaction on each VM.
### For each found transaction, display the VM block containing that transaction, including inclusion proofs for transactions in the Espresso block.

import argparse
import urllib.request
import json
import time
import datetime
import sys

parser = argparse.ArgumentParser(
    prog="Espresso Block Scanner",
    description="Scans the espresso ledger looking for  transactions. FOr each transaction it  prints out the namespace  block containing it",
    epilog="",
)

parser.add_argument("-u", "--url", default="http://localhost:50000")

args = parser.parse_args()

url = f"{args.url}/availability/block/"

# Keep track of transaction to avoid showing duplicates
payloads = set()

# Scan block by block
block_number = 0
while True:
    try:
        page = urllib.request.urlopen(f"{url}{block_number}")
    except urllib.error.HTTPError as e:
        # "Block not produced yet. waiting for 1 second
        time.sleep(1)
        continue
    except urllib.error.URLError as ex:
        # Server is down, reset block number to 0

        print("URLERROR:" + url + ". Ttrying again in 10 secs")
        print(ex)
        time.sleep(10)
        block_number = 0
    else:
        # We are Ok Espresso block found in page
        # If block contains transactions, extract Rollup id and find block with inclusion proofs for the rollup

        block = page.read()
        struct = json.loads(block)

        # throw exception if json is different from expected
        try:
            txns = struct["block"]["transaction_nmt"]
            # we only need to show the block once per vm, so if several transactions are seen for same vm in same block, treat them as duplicates
            seen_vms = set()
            for txn in txns:
                vm = txn["vm"]  # extract Rollup ID
                payload = bytes(txn["payload"]).hex()
                # if transaction already seen, skip
                if payload in payloads:
                    continue
                payloads.add(payload)

                # if vm already seen in block skip
                if vm in seen_vms:
                    continue
                seen_vms.add(vm)

                print(
                    "### New transactions for vm "
                    + str(vm)
                    + " found on Espresso Block "
                    + str(block_number)
                    + " ###"
                )
                namespace_block_url = f"{url}{block_number}/namespace/{vm}"
                print("GET " + namespace_block_url)
                # find block info for Rollup
                vm_page = urllib.request.urlopen(namespace_block_url)
                vm_block_str = vm_page.read()
                vm_block_struct = json.loads(vm_block_str)

                # Omit unnecessary data
                del vm_block_struct["proof"]["phantom"]
                if vm_block_struct["proof"]["left_boundary_proof"] is None:
                    vm_block_struct["proof"][
                        "left_boundary_proof"
                    ] = "Null # left boundary proof not needed"
                else:
                    vm_block_struct["proof"][
                        "left_boundary_proof"
                    ] = "[...]"  # Merkle proof for inclusion of left boundary vm transaction
                if vm_block_struct["proof"]["right_boundary_proof"] == None:
                    vm_block_struct["proof"][
                        "right_boundary_proof"
                    ] = "Null # right boundary proof not needed"
                else:
                    vm_block_struct["proof"][
                        "right_boundary_proof"
                    ] = "[...]"  # Merkle proof for inclusion of righ boundary vm transaction
                del vm_block_struct["header"]["metadata"]["l1_finalized"]
                for proof in vm_block_struct["proof"]["proofs"]:
                    # proof is too large to see, Eliding details
                    proof[
                        "proof"
                    ] = "[...] # Merkle proof for inclusion of each vm transaction in the Espresso Block"
                    del proof["_phantom_arity"]

                # transform byte arrays in hex strings for visualization
                root_as_hex = bytes(
                    vm_block_struct["header"]["transactions_root"]["root"]
                ).hex()
                vm_block_struct["header"]["transactions_root"]["root"] = root_as_hex
                for txn in vm_block_struct["transactions"]:
                    payload_hex = bytes(txn["payload"]).hex()
                    txn["payload"] = payload_hex

                # transform unix timespamp to date
                timestamp = vm_block_struct["header"]["metadata"]["timestamp"]
                date = datetime.datetime.fromtimestamp(timestamp)
                vm_block_struct["header"]["metadata"]["timestamp"] = str(date)

                # print trimmed block
                print(json.dumps(vm_block_struct, indent=2))
                print()

        except ValueError:
            print("Error: invalid json structure")
            exit(1)
        block_number += 1
