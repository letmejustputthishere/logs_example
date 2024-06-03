use std::cell::RefCell;

use ic_cdk::{
    api::management_canister::ecdsa::{
        sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, SignWithEcdsaArgument,
    },
    export_candid, println,
};

thread_local! {
    // create a cell that holds a counter
    static COUNTER: std::cell::RefCell<u32> = RefCell::default();
}

#[ic_cdk::update]
async fn sign() -> String {
    println!("pre management canister call");
    match sign_with_ecdsa(SignWithEcdsaArgument {
        derivation_path: vec![],
        message_hash: vec![1; 32],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: "dfx_test_key".to_string(),
        },
    })
    .await
    {
        Ok(_) => println!("management canister call succeeded"),
        Err(err) => println!("management canister call failed: {:?}", err),
    }
    "this returned".to_string()
}

#[ic_cdk::update]
fn produce_logs() {
    // print the counter value
    COUNTER.with_borrow_mut(|counter| {
        println!("counter: {}", *counter);
        *counter += 1;
    });
}

export_candid!();
