###### Generate random numbers for Phase 1, which is independent of the circuit ###
mkdir powersoftau
cd powersoftau
# This command initializes a fresh powers of tau file.
# The tau power ceremony is designed to accommodate up to 4096 (2^12) constraints.
# For this setting, the upper limit is 28, allowing the secure generation of zk-snark parameters for circuits with as many as 2^28 
# (approximately 268 million) constraints using snarkjs.
snarkjs powersoftau new bn128 12 phase1_pot12_00.ptau -v 
# The 'contribute' command generates a new contribution in the ptau file.
# You will be prompted to input random text, adding an additional layer of entropy.
# The 'name' argument serves as a label for reference and appears during file verification.
# The -e parameter enables the inclusion of random text directly in the command, allowing for a non-interactive contribution process.
snarkjs powersoftau contribute phase1_pot12_00.ptau phase1_pot12_01.ptau --name="First contribution Name" -v -e="Random text 1"
# Adds an additional layer of contribution.
snarkjs powersoftau contribute phase1_pot12_01.ptau phase1_pot12_02.ptau --name="Second contribution Name" -v -e="Random text 2"
# Introduces a third layer of contribution utilizing third-party software.
# Enables the integration of diverse software types within a single ceremony.
snarkjs powersoftau export challenge phase1_pot12_02.ptau phase1_challenge_03
snarkjs powersoftau challenge contribute bn128 phase1_challenge_03 phase1_response_03 -e="Random text"
snarkjs powersoftau import response phase1_pot12_02.ptau phase1_response_03 phase1_pot12_03.ptau -n="Third contribution name"
# Implements a random beacon in the ptau file for enhanced randomness.
# A random beacon acts as a verifiable source of public randomness, becoming available only after a predetermined time. 
# It typically involves applying a hash function like SHA256 repeatedly on high-entropy, publicly accessible data. 
# Potential data sources include future stock market closing values, national lottery results, or specific blockchain block values. 
snarkjs powersoftau beacon phase1_pot12_03.ptau phase1_pot12_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
# Initiating the readiness for phase 2 of the setup, which is specific to the circuit.
# The 'prepare phase2' command performs encrypted calculations of the Lagrange polynomials at tau, alpha*tau, and beta*tau. 
# Utilizing the beacon ptau file from the previous step, it generates a final ptau file for the creation of circuit-specific 
# proving and verification keys.
snarkjs powersoftau prepare phase2 phase1_pot12_beacon.ptau phase1_pot12_final.ptau -v
# The 'verify' command confirms the integrity of a powers of tau file.
# It's not necessary to generate the powers of tau file from scratch; you can simply download the most recent version from
# the official powers of tau website.
# Prior to circuit creation, a final examination is conducted to validate the final protocol transcript.
snarkjs powersoftau verify phase1_pot12_final.ptau
# Download the bn128 Ptau files with 54 contributions and a beacon from the provided link.
# Visit the webpage [snarkjs GitHub - Prepare Phase 2](https://github.com/iden3/snarkjs?tab=readme-ov-file#7-prepare-phase-2) 
cd ..

###### Compile the Circom circuit to produce the r1cs file ###
cp original/fibonacci.circom     circuit.circom
cp original/fibonacci_input.json input.json
#cp ./original/sudoku.circom     circuit.circom
#cp ./original/sudoku_input.json input.json

# The 'circom' command requires a single input (the circuit file, such as 'circuit.circom') and supports three options:
# 'r1cs': Creates 'circuit.r1cs', the binary format r1cs constraint system of the circuit.
# 'wasm': Produces 'circuit.wasm', which is the WebAssembly code needed for witness generation.
# 'sym': Generates 'circuit.sym', a symbol file crucial for debugging and displaying the constraint system in an annotated format.
circom circuit.circom --r1cs --wasm --sym
# Utilize the 'info' command to display statistics about the circuit.
# The displayed stats align with our conceptual understanding of the designed circuit, 
# which includes two private inputs 'fib1' and 'fib2', a single output 'fibn', and a thousand constraints modeled as 'fib1 * fib2 = fibn'.
snarkjs r1cs info circuit.r1cs
# Execute a command to verify by printing the circuit's constraints.
# Expect to observe one constraint structured.
snarkjs r1cs print circuit.r1cs circuit.sym
# Convert the r1cs file to JSON format for easier human interpretation.
snarkjs r1cs export json circuit.r1cs circuit.r1cs.json

###### Initiate the creation of the witness file ###
node circuit_js/generate_witness.js circuit_js/circuit.wasm input.json circuit_input.wtns 
# Verify the compliance of the generated witness with the r1cs file using this command:
snarkjs wtns check circuit.r1cs circuit_input.wtns

###### Setting Up the Circuit ###
# As of now, snarkjs offers support for three proving systems: Groth16, PLONK, and FFLONK (currently in Beta).
# Initiate proof generation using the PLONK system.
mkdir zkey
snarkjs plonk setup   circuit.r1cs powersoftau/phase1_pot12_final.ptau zkey/phase1_circuit_prover_plonk.zkey
# Initiate proof generation using the FFLONK system.
snarkjs fflonk setup  circuit.r1cs powersoftau/phase1_pot12_final.ptau zkey/phase1_circuit_prover_fflonk.zkey
# Commence proof generation using the Groth16 system.
# Groth16 necessitates a unique trusted ceremony for each circuit, unlike PLONK and FFLONK, 
# which only require a universal powers of tau ceremony.
# This action creates a preliminary zkey file without any contributions from phase 2.

###### Phase 2 - Key Generation, Exclusive to Groth16 ###
# CAUTION: Avoid using this initial zkey in a production environment, as it lacks necessary security contributions.
# The 'zkey new' command produces a starter zkey file devoid of any contributions.
# A zkey, integral to zero-knowledge proofs, encapsulates both the proving and verification keys, along with phase 2 contributions.
# It's critical to verify that a zkey is correctly associated with its specific circuit.
# Keep in mind that circuit_0000.zkey, generated above, is yet to receive any contributions and is unsuitable for final circuit use.
mkdir phase2
cd phase2
snarkjs groth16 setup ../circuit.r1cs ../powersoftau/phase1_pot12_final.ptau phase2_circuit_00.zkey
# Name of the Initial Contributor
snarkjs zkey contribute phase2_circuit_00.zkey phase2_circuit_01.zkey --name="First Contributor Name" -v -e="Random entropy 1"
# Name of the Second Contributor
snarkjs zkey contribute phase2_circuit_01.zkey phase2_circuit_02.zkey --name="Second contribution Name" -v -e="Random entropy 2"
# Name of the Third Contributor
snarkjs zkey export bellman phase2_circuit_02.zkey  phase2_challenge_03
snarkjs zkey bellman contribute bn128 phase2_challenge_03 phase2_response_03 -e="Random text"
snarkjs zkey import bellman phase2_circuit_02.zkey phase2_response_03 phase2_circuit_03.zkey -n="Third Contribution Name"
# Initiate the Generation of the zkey File
snarkjs zkey verify ../circuit.r1cs ../powersoftau/phase1_pot12_final.ptau phase2_circuit_03.zkey
snarkjs zkey beacon phase2_circuit_03.zkey phase2_circuit_prover_groth16.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
# Execute the Verification of the Key
snarkjs zkey verify ../circuit.r1cs ../powersoftau/phase1_pot12_final.ptau phase2_circuit_prover_groth16.zkey
cp phase2_circuit_prover_groth16.zkey ../zkey/phase2_circuit_prover_groth16.zkey
cd ..

###### Proof Generation Process ###
# This step involves creating the proof.
# The 'circuit_proof_plonk.json' file encapsulates the actual proof, while 'output.json' details the values of public outputs.
snarkjs plonk prove   zkey/phase1_circuit_prover_plonk.zkey   circuit_input.wtns circuit_proof_plonk.json   output.json
snarkjs fflonk prove  zkey/phase1_circuit_prover_fflonk.zkey  circuit_input.wtns circuit_proof_fflonk.json  output.json
snarkjs groth16 prove zkey/phase2_circuit_prover_groth16.zkey circuit_input.wtns circuit_proof_groth16.json output.json

###### Proof Verification Process ###
# Utilize this command to validate the proof, incorporating the previously exported verification key.
# A successful verification is indicated by the output 'OK' in the console, confirming the proof's validity.
# Execute proof verification using the PLONK system.
cd zkey
snarkjs zkey    export verificationkey               phase1_circuit_prover_plonk.zkey    verification_key_plonk.json
snarkjs plonk   verify verification_key_plonk.json   ../output.json ../circuit_proof_plonk.json
# Execute proof verification using the FFLONK system.
snarkjs zkey    export verificationkey               phase1_circuit_prover_fflonk.zkey   verification_key_fflonk.json
snarkjs fflonk  verify verification_key_fflonk.json  ../output.json  ../circuit_proof_fflonk.json
# Execute proof verification using the Groth16 system.
snarkjs zkey    export verificationkey               phase2_circuit_prover_groth16.zkey  verification_key_groth16.json
snarkjs groth16 verify verification_key_groth16.json ../output.json ../circuit_proof_groth16.json
cd ..

###### Procedure for Creating and Verifying the Proof in JavaScript
cd nodejs
npm init
npm install snarkjs
cp node_modules/snarkjs/build/snarkjs.min.js .
node verify.js
cd ..

###### Procedure for Creating and Verifying the Proof in Browser and Solidity
# Converting the Verifier into a Smart Contract ###
# Initiating the creation of a verifier within a Solidity smart contract.
# Conclude by exporting the verifier as a Solidity smart contract, enabling its deployment on a blockchain network, 
# such as through Remix.
# Detailed guidance on this process can be found in section 4 of the associated tutorial.
cd solidity
snarkjs zkey export solidityverifier ../zkey/phase2_circuit_prover_groth16.zkey  verifier.sol
# Procedure for Simulating the Verification Process
snarkjs zkey export soliditycalldata ../output.json ../circuit_proof_groth16.json
cd ..

