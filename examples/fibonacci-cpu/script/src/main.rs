use sp1_sdk::{include_elf, utils, Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::time::Instant;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("fibonacci-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '500' to it.
    let n = 1000u32;

    // The input stream that the program will read from using `sp1_zkvm::io::read`. Note that the
    // types of the elements in the input stream must match the types being read in the program.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    println!("The number is {}", n);
    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    println!("Creating Fibonacci program");
    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("executed program with {} cycles", report.total_instruction_count());

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);

    println!("Generating proof now 01");
    let start = Instant::now();
    let mut proof = client.prove(&pk, &stdin).compressed().run().unwrap();
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);

    println!("generated proof in {:?} seconds", duration.as_secs());

    println!("Generating proof now 02");
    let start = Instant::now();
    let mut proof = client.prove(&pk, &stdin).compressed().run().unwrap();
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);

    println!("generated proof in {:?} seconds", duration.as_secs());

    // Read and verify the output.
    //
    // Note that this output is read from values committed to in the program using
    // `sp1_zkvm::io::commit`.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();

    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof.save("proof-with-pis.bin").expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, &vk).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}
