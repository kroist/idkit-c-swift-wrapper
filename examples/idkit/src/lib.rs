use std::{collections::HashMap, str::FromStr, task::Poll};

use ffi::{PollResult, WrappedProof};
use idkit::{session::{AppId, BridgeUrl, Status, VerificationLevel}, Session};

#[swift_bridge::bridge]
mod ffi {

    #[swift_bridge(swift_repr = "struct")]
    struct WrappedProof {
        proof: String,
        merkle_root: String,
        nullifier_hash: String,
        credential_type: String,
    }

    enum PollResult {
        WaitingForConnection,
        AwaitingConfirmation,
        Confirmed(WrappedProof),
        Failed(String)
    }

    extern "Rust" {
        type WrappedSession;

        #[swift_bridge(init)]
        fn new(app_id: String, action: String) -> WrappedSession;

        fn get_url(&self) -> String;

        fn poll(&self) -> PollResult;
    }
}

pub struct WrappedSession {
    session: Session
}

impl WrappedSession {
    fn new(app_id: String, action: String) -> Self {
        let handle = tokio::runtime::Runtime::new().unwrap();
        let session = handle.block_on(async {
            Session::new(
                AppId::from_str(&app_id).unwrap(),
                &action,
                VerificationLevel::Device,
                BridgeUrl::default(),
                (),
                None
            ).await.unwrap()
        });
        WrappedSession {
            session
        }
    }
    fn get_url(&self) -> String {
        let url = self.session.connect_url().to_string();
        url
    }
    fn poll(&self) -> PollResult {
        let handle = tokio::runtime::Runtime::new().unwrap();
        let status = handle.block_on(async {
            match self.session.poll_for_status().await.unwrap() {
                Status::WaitingForConnection => PollResult::WaitingForConnection,
                Status::AwaitingConfirmation => PollResult::AwaitingConfirmation,
                Status::Failed(error) => PollResult::Failed(error.to_string()),
                Status::Confirmed(proof) => PollResult::Confirmed(
                    WrappedProof {
                        proof: proof.proof,
                        merkle_root: proof.merkle_root,
                        nullifier_hash: proof.nullifier_hash,
                        credential_type: format!("{}", proof.credential_type),
                    }
                ),
            }
        });
        status
    }
}
