use std::ffi::{c_char, CString};


#[repr(C)]
pub struct Proof {
    pub proof: *const c_char,
    pub merkle_root: *const c_char,
    pub nullifier_hash: *const c_char,
    pub credential_type: *const c_char,
}

#[no_mangle]
pub extern "C" fn get_proof() -> Proof {
    let proof_ptr = CString::new("123").unwrap();
    let merkle_ptr = CString::new("456").unwrap();
    let nullifier_ptr = CString::new("789").unwrap();
    let cred_ptr = CString::new("000").unwrap();
    Proof {
        proof: proof_ptr.into_raw(),
        merkle_root:  merkle_ptr.into_raw(),
        nullifier_hash: nullifier_ptr.into_raw(), 
        credential_type: cred_ptr.into_raw()
    }
}
