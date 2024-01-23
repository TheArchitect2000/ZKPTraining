const snarkjs = require("snarkjs");
const fs = require("fs");

async function run() {
    const inputData = {
        fib1: 1, 
        fib2: 17
    };
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(inputData, "../circuit_js/circuit.wasm", "../zkey/phase2_circuit_prover_groth16.zkey");

    console.log("Proof is: ");
    console.log(JSON.stringify(proof, null, 1));

    const verifKey = JSON.parse(fs.readFileSync("../zkey/verification_key_groth16.json"));

    const res = await snarkjs.groth16.verify(verifKey, publicSignals, proof);

    if (res === true) {
        console.log("Verification OK. Proof is valid.");
    } else {
        console.log("Proof is NOT valid.");
    }

}

run().then(() => {
    process.exit(0);
});