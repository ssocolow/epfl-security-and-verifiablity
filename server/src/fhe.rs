// use concrete_shortint::prelude::*;
// use rand::Rng;

// fn main() {
//     // --- Key Generation ---
//     // Use one of the provided parameter sets. For simplicity, we use PARAMETERS_4.
//     // (See the Concrete shortint docs for other options.)
//     let parameters = PARAMETERS_4;
//     let client_key = ClientKey::new(parameters);
//     let server_key = ServerKey::new(&client_key);

//     // --- Alice's Operation ---
//     // Alice encrypts her input x.
//     let x: u8 = 42;
//     let ct_x = client_key.encrypt(x);
//     println!("Alice sends E(x): {:?}", ct_x);

//     // --- Bob's Operation ---
//     // Bob has a set of values y_j and a constant c.
//     let bob_values: Vec<u8> = vec![10, 20, 30];
//     let c: u8 = 5; // The constant to subtract

//     // For the product, we start with the encryption of 1 (the multiplicative identity).
//     let mut ct_result = client_key.encrypt(1u8);

//     let mut rng = rand::thread_rng();

//     for &y in &bob_values {
//         // Bob encrypts his value y_j.
//         let ct_y = client_key.encrypt(y);
//         // Homomorphically subtract the constant c.
//         let ct_sub = ct_y - c;

//         // Choose a random exponent r between 1 and 5.
//         let r: u8 = rng.gen_range(1..=5);
//         println!("For y = {}: random exponent = {}", y, r);

//         // Compute (E(y) - c)^r by iterated multiplication.
//         // Start with the identity for multiplication.
//         let mut ct_exp = client_key.encrypt(1u8);
//         for _ in 0..r {
//             // Use unchecked multiplication (which does no carry management) for simplicity.
//             ct_exp = server_key.unchecked_mul(&ct_exp, &ct_sub);
//         }
//         // Multiply the result into the overall product.
//         ct_result = server_key.unchecked_mul(&ct_result, &ct_exp);
//     }

//     // --- Decryption ---
//     // In a real protocol, Bob would send ct_result back to Alice for decryption.
//     let result = client_key.decrypt(&ct_result);
//     println!("Bob sends back the aggregated ciphertext.");
//     println!("Decrypted final result: {}", result);
// }