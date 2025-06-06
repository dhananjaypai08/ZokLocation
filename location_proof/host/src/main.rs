// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    LOCATION_PROOF_ELF, LOCATION_PROOF_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // A default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // For example:
    let latitude2: f64 = 19.2437;
    let longitude2: f64 = 73.1355;
    let latitude1: f64 = 19.243715741538303;
    let longitude1: f64 = 73.13354731565501;
    let threshold: f64 = 100.0;
    let env = ExecutorEnv::builder()
        .write(&(latitude1, longitude1, latitude2, longitude2, threshold))
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let prove_info = prover
        .prove(env, LOCATION_PROOF_ELF)
        .unwrap();


    let receipt = prove_info.receipt;
    // println!("Receipt: {:?}", receipt);

    let _output: u32 = receipt.journal.decode().unwrap();

    receipt
        .verify(LOCATION_PROOF_ID)
        .unwrap();
}
