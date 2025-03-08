use rand::prelude::*;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint32};
use tfhe::prelude::*;


fn server(e_client_value: FheUint32, server_hashes: Vec<u32>, server_r: u32, pk: ServerKey) -> FheUint32 {
    let e_server_r = FheUint32::encrypt(server_r, pk);
    let e_server_hashes = server_hashes
        .iter()
        .map(|&server_hash| FheUint32::encrypt(server_hash, pk));

    let test_equal = e_server_hashes
        .map(|e_server_hash| &e_client_value - &e_server_hash)
        .fold(FheUint32::encrypt(1u32, pk), |acc, e| &acc * &e);

    return &test_equal * &e_server_r;
}

fn main() {
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let input_a = 621234u32;
    let a = FheUint32::encrypt(input_a, &client_key);

    println!("Test: Test");

    //Server-side
    set_server_key(server_key);
    let server_hashes = vec![123456u32, 621234u32, 113458u32];
    let mut rng = rand::rng();
    let server_r: u32 = rng.random::<u32>();
    let result = server(a, server_hashes, server_r);

    //Client-side
    let clear_res: u32 = result.decrypt(&client_key);
    println!("CLIENT: Decrypted server response is {}", clear_res);

    if clear_res == 0 {
        println!("CLIENT: value is in the database 👏");
    } else {
        println!("CLIENT: value is NOT in the database 👎");
    }
}
