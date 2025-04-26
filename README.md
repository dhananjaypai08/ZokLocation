## Zk Based Location authenticity 
Research Points :
1. Running zokrates on UAV 
2. Creating risc-zero prover

### Quickstart Guidelines 
1. in `circuits/` are defined zokrates based circuit
2. in `contracts/` is the contract that can be defined and the function `verifyTx` can be called with proof and inputs as params 
3. in `scripts/` contains zokrates-js for compiling, generating witness, setup phase and generating and verifying proof script.                                        
**Note** : All in one script
4. in `location_proof` contains the risc-zero prover and verifier code. 

### Easy work-around using zokrates-js
1. `circuits` are already defined 
2. Run 
```sh
cd scripts 
npm install 
node proof.js
```
**Note**:  make sure to include your private and public inputs within the proof.json generate witness step 

3. (Optional) Deploy `contracts/VerifierLocation.sol` on any chain and call the function `verifyTx`.
You will need Proof tuples and inputs present in your generated proof in file `proof.json`

### Risc-zero work around 
This requires more in depth working for UAV machines 
1. Run
```sh
cd location_proof
cargo build 
cargo run
```
