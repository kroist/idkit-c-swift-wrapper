use std::{collections::HashMap, str::FromStr};

use idkit::{session::{AppId, BridgeUrl, VerificationLevel}, Session};

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct MyIpAddress {
        origin: String,
    }

    extern "Rust" {
        async fn get_my_ip_from_rust() -> MyIpAddress;
        async fn get_url(app_id: String, action: String) -> String;
    }
}

async fn get_url(
    app_id: String,
    action: String
) -> String {
    let session = Session::new(
        AppId::from_str(&app_id).unwrap(),
        &action,
        VerificationLevel::Device,
        BridgeUrl::default(),
        (),
        None
    ).await.unwrap();

    let url = session.connect_url().to_string();
    url
}

// TODO: Return a `Result<MyIpAddress, SomeErrorType>`
//  Once we support returning Result from an async function.
async fn get_my_ip_from_rust() -> ffi::MyIpAddress {
    println!("Starting HTTP request from the Rust side...");

    let origin = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap()
        .remove("origin")
        .unwrap();

    println!("HTTP request complete. Returning the value to Swift...");

    ffi::MyIpAddress { origin }
}
