### Simple live Espresso block scanner
### Scan all blocks looking for transaction on each VM. 
### For each found transaction, display the VM block containing that transaction, including inclusion proofs for transactions in the Espresso block.

import urllib.request
import json;
import time;
import sys;
host="localhost"
port="50000"
if len(sys.argv) > 1:
	host=sys.argv[1]
if len(sys.argv) > 2:
	port=sys.argv[2]
url="http://"+host+":"+port+"/availability/block/"

# Keep track of transaction to avoid showing duplicates
payloads = set()

# Scan block by block
block_number = 0
while True:
	try:
		page = urllib.request.urlopen(url+str(block_number))
	except urllib.error.HTTPError as e:
		# "Block not produced yet. waiting for 1 second
		time.sleep(1)
		continue;
	except urllib.error.URLError as ex:
		# Server is down, reset block number to 0

		print("URLERROR trying again in 10 secs")
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
			txns = struct['block']['transaction_nmt']
			# we only need to show the block once per vm, so if several transactions are seen for same vm in same block, treat them as duplicates
			seen_vms = set();
			for txn in txns:
				vm = txn['vm'] # extract Rollup ID
				payload = payload_hex=bytearray(txn['payload']).hex()
				# if transaction already seen, skip
				if payload in payloads:
					continue
				payloads.add(payload)
				
				# if vm already seen in block skip
				if vm in seen_vms:
					continue
				seen_vms.add(vm)

				print("### New transactions for vm "+str(vm) + " found on Espresso Block "+str(block_number)+" ###")

				# find block info for Rollup
				vm_page = urllib.request.urlopen(url+str(block_number)+"/namespace/"+str(vm))
				vm_block_str = vm_page.read()
				vm_block_struct = json.loads(vm_block_str)
				
				# Omit unnecessary data
				del vm_block_struct['proof']['phantom']
				del vm_block_struct['proof']['left_boundary_proof']
				del vm_block_struct['proof']['right_boundary_proof']
				del vm_block_struct['header']['metadata']['l1_finalized']
				for proof in vm_block_struct['proof']['proofs']:
					# proof is too large to see, Eliding details
					proof['proof'] = "[...] # Merkle proof for inclusion of each vm transaction in the Espresso Block"
					del proof['_phantom_arity']
				
				# transform byte arrays in hex strings for visualization
				root_as_hex = bytearray(vm_block_struct['header']['transactions_root']['root']).hex()
				vm_block_struct['header']['transactions_root']['root'] = root_as_hex
				for txn in vm_block_struct['transactions']:
					payload_hex = bytearray(txn['payload']).hex()
					txn['payload']=payload_hex
				
				# print trimmed block
				print(json.dumps(vm_block_struct, indent=2))
				print()
						
					
					
					
			
		except ValueError:
			print("Error: invalid json structture")
			exit(0)
		block_number +=1;


