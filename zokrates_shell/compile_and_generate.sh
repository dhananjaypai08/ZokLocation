#!/bin/bash
set -e

cd ../circuits
# echo "Compiling and generating proofs for circuits..."
# zokrates compile -i auth.zok
# echo "Setting up the circuit..."
# zokrates setup
# echo "Computing witness..."
# zokrates compute-witness -a 123 123
# echo "Generating proof..."
# zokrates generate-proof
# echo "Exporting verifier..."
# zokrates export-verifier -o ../contracts/VerifierAuth.sol
# echo "Copying proof to zokrates directory..."
# cp proof.json ../zokrates/auth_proof.json
echo "Compiling and generating proofs for location circuit..."
zokrates compile -i location.zok
zokrates setup
zokrates compute-witness -a 5 5 5 0 0 0 10 10 10
zokrates generate-proof
zokrates export-verifier -o ../contracts/VerifierLocation.sol
cp proof.json ../zokrates/location_proof.json
