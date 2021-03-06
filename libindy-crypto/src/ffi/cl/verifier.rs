use cl::verifier::*;
use cl::*;
use errors::ToErrorCode;
use ffi::ErrorCode;
use utils::ctypes::CTypesUtils;
use utils::json::{JsonEncodable, JsonDecodable};

use libc::c_char;

use std::os::raw::c_void;

/// Creates random nonce.
///
/// Note that nonce deallocation must be performed by calling indy_crypto_cl_nonce_free.
///
/// # Arguments
/// * `nonce_p` - Reference that will contain nonce instance pointer.
#[no_mangle]
pub extern fn indy_crypto_cl_verifier_new_nonce(nonce_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_cl_verifier_new_nonce: >>> {:?}", nonce_p);

    check_useful_c_ptr!(nonce_p, ErrorCode::CommonInvalidParam1);

    let res = match Verifier::new_nonce() {
        Ok(nonce) => {
            trace!("indy_crypto_cl_verifier_new_nonce: nonce: {:?}", nonce);
            unsafe {
                *nonce_p = Box::into_raw(Box::new(nonce)) as *const c_void;
                trace!("indy_crypto_cl_verifier_new_nonce: *nonce_p: {:?}", *nonce_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_verifier_new_nonce: <<< res: {:?}", res);
    res
}

/// Returns json representation of nonce.
///
/// # Arguments
/// * `nonce` - Reference that contains nonce instance pointer.
/// * `nonce_json_p` - Reference that will contain nonce json.
#[no_mangle]
pub extern fn indy_crypto_cl_nonce_to_json(nonce: *const c_void,
                                           nonce_json_p: *mut *const c_char) -> ErrorCode {
    trace!("indy_crypto_cl_nonce_to_json: >>> nonce: {:?}, nonce_json_p: {:?}", nonce, nonce_json_p);

    check_useful_c_reference!(nonce, Nonce, ErrorCode::CommonInvalidParam1);
    check_useful_c_ptr!(nonce_json_p, ErrorCode::CommonInvalidParam2);

    trace!("indy_crypto_cl_nonce_to_json: entity >>> nonce: {:?}", nonce);

    let res = match nonce.to_json() {
        Ok(nonce_json) => {
            trace!("indy_crypto_cl_nonce_to_json: nonce_json: {:?}", nonce_json);
            unsafe {
                let nonce_json = CTypesUtils::string_to_cstring(nonce_json);
                *nonce_json_p = nonce_json.into_raw();
                trace!("indy_crypto_cl_nonce_to_json: nonce_json_p: {:?}", *nonce_json_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_nonce_to_json: <<< res: {:?}", res);
    res
}

/// Creates and returns nonce json.
///
/// Note: Nonce instance deallocation must be performed by calling indy_crypto_cl_nonce_free.
///
/// # Arguments
/// * `nonce_json` - Reference that contains nonce json.
/// * `nonce_p` - Reference that will contain nonce instance pointer.
#[no_mangle]
pub extern fn indy_crypto_cl_nonce_from_json(nonce_json: *const c_char,
                                             nonce_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_cl_nonce_from_json: >>> nonce_json: {:?}, nonce_p: {:?}", nonce_json, nonce_p);

    check_useful_c_str!(nonce_json, ErrorCode::CommonInvalidParam1);
    check_useful_c_ptr!(nonce_p, ErrorCode::CommonInvalidParam2);

    trace!("indy_crypto_cl_nonce_from_json: entity: nonce_json: {:?}", nonce_json);

    let res = match Nonce::from_json(&nonce_json) {
        Ok(nonce) => {
            trace!("indy_crypto_cl_nonce_from_json: nonce: {:?}", nonce);
            unsafe {
                *nonce_p = Box::into_raw(Box::new(nonce)) as *const c_void;
                trace!("indy_crypto_cl_nonce_from_json: *nonce_p: {:?}", *nonce_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_nonce_from_json: <<< res: {:?}", res);
    res
}

/// Deallocates nonce instance.
///
/// # Arguments
/// * `nonce` - Reference that contains nonce instance pointer.
#[no_mangle]
pub extern fn indy_crypto_cl_nonce_free(nonce: *const c_void) -> ErrorCode {
    trace!("indy_crypto_cl_nonce_free: >>> nonce: {:?}", nonce);

    check_useful_c_ptr!(nonce, ErrorCode::CommonInvalidParam1);

    let nonce = unsafe { Box::from_raw(nonce as *mut Nonce); };
    trace!("indy_crypto_cl_nonce_free: entity: nonce: {:?}", nonce);

    let res = ErrorCode::Success;

    trace!("indy_crypto_cl_nonce_free: <<< res: {:?}", res);
    res
}

/// Creates and returns proof verifier.
///
/// Note that proof verifier deallocation must be performed by
/// calling indy_crypto_cl_proof_verifier_finalize.
///
/// # Arguments
/// * `proof_verifier_p` - Reference that will contain proof verifier instance pointer.
#[no_mangle]
pub extern fn indy_crypto_cl_verifier_new_proof_verifier(proof_verifier_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_cl_verifier_new_proof_verifier: >>> {:?}", proof_verifier_p);

    check_useful_c_ptr!(proof_verifier_p, ErrorCode::CommonInvalidParam1);

    let res = match Verifier::new_proof_verifier() {
        Ok(proof_verifier) => {
            trace!("indy_crypto_cl_verifier_new_proof_verifier: proof_verifier: {:?}", proof_verifier);
            unsafe {
                *proof_verifier_p = Box::into_raw(Box::new(proof_verifier)) as *const c_void;
                trace!("indy_crypto_cl_verifier_new_proof_verifier: *proof_verifier_p: {:?}", *proof_verifier_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_verifier_new_proof_verifier: <<< res: {:?}", res);
    res
}

/// Add sub proof request to proof verifier.
/// Part of proof request related to a particular schema-key.
///
/// # Arguments
/// * `proof_verifier` - Reference that contain proof verifier instance pointer.
/// * `key_id` - Reference that contains unique claim identifier.
/// * `sub_proof_request` - Reference that contain requested attributes and predicates instance pointer.
/// * `claim_schema` - Reference that contain claim schema instance pointer.
/// * `issuer_pub_key` - Reference that contain public key instance pointer.
/// * `rev_reg_pub` - (Optional) Reference that contain public revocation registry instance pointer.
#[no_mangle]
pub extern fn indy_crypto_cl_proof_verifier_add_sub_proof_request(proof_verifier: *const c_void,
                                                                  key_id: *const c_char,
                                                                  sub_proof_request: *const c_void,
                                                                  claim_schema: *const c_void,
                                                                  issuer_pub_key: *const c_void,
                                                                  rev_reg_pub: *const c_void) -> ErrorCode {
    trace!("indy_crypto_cl_proof_verifier_add_sub_proof_request: >>> proof_verifier: {:?}, key_id: {:?}, sub_proof_request: {:?} ,\
            issuer_pub_key: {:?}, rev_reg_pub: {:?}", proof_verifier, key_id, sub_proof_request, issuer_pub_key, rev_reg_pub);

    check_useful_mut_c_reference!(proof_verifier, ProofVerifier, ErrorCode::CommonInvalidParam1);
    check_useful_c_str!(key_id, ErrorCode::CommonInvalidParam2);
    check_useful_c_reference!(sub_proof_request, SubProofRequest, ErrorCode::CommonInvalidParam3);
    check_useful_c_reference!(claim_schema, ClaimSchema, ErrorCode::CommonInvalidParam4);
    check_useful_c_reference!(issuer_pub_key, IssuerPublicKey, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_reference!(rev_reg_pub, RevocationRegistryPublic, ErrorCode::CommonInvalidParam6);

    trace!("indy_crypto_cl_proof_verifier_add_sub_proof_request: entities: proof_verifier: {:?}, key_id: {:?}, sub_proof_request: {:?},\
            issuer_pub_key: {:?}, rev_reg_pub: {:?}", proof_verifier, key_id, sub_proof_request, issuer_pub_key, rev_reg_pub);

    let res = match proof_verifier.add_sub_proof_request(&key_id,
                                                         sub_proof_request,
                                                         claim_schema,
                                                         issuer_pub_key,
                                                         rev_reg_pub) {
        Ok(()) => ErrorCode::Success,
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_proof_verifier_add_sub_proof_request: <<< res: {:?}", res);
    res
}


/// Verifies proof and deallocates proof verifier.
///
/// # Arguments
/// * `proof_verifier` - Reference that contain proof verifier instance pointer.
/// * `proof` - Reference that contain proof instance pointer.
/// * `nonce` - Reference that contain nonce instance pointer.
/// * `valid_p` - Reference that will be filled with true - if proof valid or false otherwise.
#[no_mangle]
pub extern fn indy_crypto_cl_proof_verifier_verify(proof_verifier: *const c_void,
                                                   proof: *const c_void,
                                                   nonce: *const c_void,
                                                   valid_p: *mut bool) -> ErrorCode {
    trace!("indy_crypto_cl_proof_verifier_verify: >>> proof_verifier: {:?}, proof: {:?}, nonce: {:?}, valid_p: {:?}", proof_verifier, proof, nonce, valid_p);

    check_useful_c_ptr!(proof_verifier, ErrorCode::CommonInvalidParam1);
    check_useful_c_reference!(proof, Proof, ErrorCode::CommonInvalidParam2);
    check_useful_c_reference!(nonce, Nonce, ErrorCode::CommonInvalidParam3);
    check_useful_c_ptr!(valid_p, ErrorCode::CommonInvalidParam4);

    let proof_verifier = unsafe { Box::from_raw(proof_verifier as *mut ProofVerifier) };

    trace!("indy_crypto_cl_proof_verifier_verify: entities: >>> proof_verifier: {:?}, proof: {:?}, nonce: {:?}", proof_verifier, proof, nonce);

    let res = match proof_verifier.verify(proof, nonce) {
        Ok(valid) => {
            trace!("indy_crypto_cl_proof_verifier_verify: valid: {:?}", valid);
            unsafe {
                *valid_p = valid;
                trace!("indy_crypto_cl_proof_verifier_verify: *valid_p: {:?}", *valid_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_cl_proof_verifier_verify: <<< res: {:?}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CString;
    use std::ptr;
    use ffi::cl::mocks::*;
    use super::mocks::*;
    use super::super::issuer::mocks::*;
    use super::super::prover::mocks::*;

    #[test]
    fn indy_crypto_cl_verifier_new_nonce_works() {
        let mut nonce_p: *const c_void = ptr::null();
        let err_code = indy_crypto_cl_verifier_new_nonce(&mut nonce_p);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!nonce_p.is_null());

        _free_nonce(nonce_p)
    }

    #[test]
    fn indy_crypto_cl_nonce_to_json_works() {
        let nonce = _nonce();

        let mut nonce_json_p: *const c_char = ptr::null();
        let err_code = indy_crypto_cl_nonce_to_json(nonce, &mut nonce_json_p);
        assert_eq!(err_code, ErrorCode::Success);

        _free_nonce(nonce)
    }

    #[test]
    fn indy_crypto_cl_nonce_from_json_works() {
        let nonce = _nonce();

        let mut nonce_json_p: *const c_char = ptr::null();
        let err_code = indy_crypto_cl_nonce_to_json(nonce, &mut nonce_json_p);
        assert_eq!(err_code, ErrorCode::Success);

        let mut nonce_p: *const c_void = ptr::null();
        let err_code = indy_crypto_cl_nonce_from_json(nonce_json_p, &mut nonce_p);
        assert_eq!(err_code, ErrorCode::Success);

        _free_nonce(nonce)
    }

    #[test]
    fn indy_crypto_cl_nonce_free_works() {
        let nonce = _nonce();

        let err_code = indy_crypto_cl_nonce_free(nonce);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_cl_verifier_new_proof_verifier_works() {
        let key_id = CString::new("key_id").unwrap();
        let (issuer_pub_key, issuer_priv_key) = _issuer_keys();
        let (rev_reg_pub, rev_reg_priv) = _revocation_registry(issuer_pub_key);
        let master_secret = _master_secret();
        let (blinded_master_secret, master_secret_blinding_data) = _blinded_master_secret(issuer_pub_key, master_secret);
        let nonce = _nonce();
        let claim_signature = _claim_signature(blinded_master_secret, issuer_pub_key, issuer_priv_key, rev_reg_pub, rev_reg_priv);
        let claim_schema = _claim_schema();
        let sub_proof_request = _sub_proof_request();
        _process_claim_signature(claim_signature, master_secret_blinding_data, issuer_pub_key, rev_reg_pub);
        let proof = _proof(issuer_pub_key, rev_reg_pub, claim_signature, nonce, master_secret);

        let mut proof_verifier_p: *const c_void = ptr::null();
        let err_code = indy_crypto_cl_verifier_new_proof_verifier(&mut proof_verifier_p);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!proof_verifier_p.is_null());

        _add_sub_proof_request(proof_verifier_p, key_id, claim_schema, issuer_pub_key, rev_reg_pub, sub_proof_request);
        _free_proof_verifier(proof_verifier_p, proof, nonce);
        _free_issuer_keys(issuer_pub_key, issuer_priv_key);
        _free_revocation_registry(rev_reg_pub, rev_reg_priv);
        _free_master_secret(master_secret);
        _free_blinded_master_secret(blinded_master_secret, master_secret_blinding_data);
        _free_nonce(nonce);
        _free_claim_schema(claim_schema);
        _free_sub_proof_request(sub_proof_request);
        _free_claim_signature(claim_signature);
    }

    #[test]
    fn indy_crypto_cl_proof_verifier_add_sub_proof_request_works() {
        let key_id = CString::new("key_id").unwrap();
        let (issuer_pub_key, issuer_priv_key) = _issuer_keys();
        let (rev_reg_pub, rev_reg_priv) = _revocation_registry(issuer_pub_key);
        let master_secret = _master_secret();
        let (blinded_master_secret, master_secret_blinding_data) = _blinded_master_secret(issuer_pub_key, master_secret);
        let claim_schema = _claim_schema();
        let sub_proof_request = _sub_proof_request();
        let nonce = _nonce();
        let claim_signature = _claim_signature(blinded_master_secret, issuer_pub_key, issuer_priv_key, rev_reg_pub, rev_reg_priv);
        _process_claim_signature(claim_signature, master_secret_blinding_data, issuer_pub_key, rev_reg_pub);
        let proof = _proof(issuer_pub_key, rev_reg_pub, claim_signature, nonce, master_secret);
        let proof_verifier = _proof_verifier();

        let err_code = indy_crypto_cl_proof_verifier_add_sub_proof_request(proof_verifier,
                                                                           key_id.as_ptr(),
                                                                           sub_proof_request,
                                                                           claim_schema,
                                                                           issuer_pub_key,
                                                                           rev_reg_pub);
        assert_eq!(err_code, ErrorCode::Success);

        _free_proof_verifier(proof_verifier, proof, nonce);
        _free_issuer_keys(issuer_pub_key, issuer_priv_key);
        _free_revocation_registry(rev_reg_pub, rev_reg_priv);
        _free_master_secret(master_secret);
        _free_blinded_master_secret(blinded_master_secret, master_secret_blinding_data);
        _free_nonce(nonce);
        _free_claim_schema(claim_schema);
        _free_sub_proof_request(sub_proof_request);
        _free_claim_signature(claim_signature);
    }

    #[test]
    fn indy_crypto_cl_proof_verifier_verify_works() {
        let key_id = CString::new("key_id").unwrap();
        let (issuer_pub_key, issuer_priv_key) = _issuer_keys();
        let (rev_reg_pub, rev_reg_priv) = _revocation_registry(issuer_pub_key);
        let master_secret = _master_secret();
        let (blinded_master_secret, master_secret_blinding_data) = _blinded_master_secret(issuer_pub_key, master_secret);
        let claim_schema = _claim_schema();
        let claim_signature = _claim_signature(blinded_master_secret, issuer_pub_key, issuer_priv_key, rev_reg_pub, rev_reg_priv);
        _process_claim_signature(claim_signature, master_secret_blinding_data, issuer_pub_key, rev_reg_pub);
        let sub_proof_request = _sub_proof_request();
        let nonce = _nonce();
        let proof = _proof(issuer_pub_key, rev_reg_pub, claim_signature, nonce, master_secret);
        let proof_verifier = _proof_verifier();

        let err_code = indy_crypto_cl_proof_verifier_add_sub_proof_request(proof_verifier,
                                                                           key_id.as_ptr(),
                                                                           sub_proof_request,
                                                                           claim_schema,
                                                                           issuer_pub_key,
                                                                           rev_reg_pub);
        assert_eq!(err_code, ErrorCode::Success);

        let mut valid = false;
        let err_code = indy_crypto_cl_proof_verifier_verify(proof_verifier, proof, nonce, &mut valid);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(valid); //TODO: Uncomment

        _free_issuer_keys(issuer_pub_key, issuer_priv_key);
        _free_revocation_registry(rev_reg_pub, rev_reg_priv);
        _free_master_secret(master_secret);
        _free_blinded_master_secret(blinded_master_secret, master_secret_blinding_data);
        _free_nonce(nonce);
        _free_claim_schema(claim_schema);
        _free_sub_proof_request(sub_proof_request);
        _free_claim_signature(claim_signature);
    }
}

pub mod mocks {
    use super::*;
    use std::ptr;
    use std::ffi::CString;

    pub fn _nonce() -> *const c_void {
        let mut nonce_p: *const c_void = ptr::null();
        let err_code = indy_crypto_cl_verifier_new_nonce(&mut nonce_p);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!nonce_p.is_null());

        nonce_p
    }

    pub fn _free_nonce(nonce: *const c_void) {
        let err_code = indy_crypto_cl_nonce_free(nonce);
        assert_eq!(err_code, ErrorCode::Success);
    }

    pub fn _proof_verifier() -> *const c_void {
        let mut proof_verifier_p: *const c_void = ptr::null();
        let err_code = indy_crypto_cl_verifier_new_proof_verifier(&mut proof_verifier_p);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!proof_verifier_p.is_null());

        proof_verifier_p
    }

    pub fn _add_sub_proof_request(proof_verifier: *const c_void, key_id: CString, claim_schema: *const c_void,
                                  issuer_pub_key: *const c_void, rev_reg_pub: *const c_void, sub_proof_request: *const c_void) {
        let err_code = indy_crypto_cl_proof_verifier_add_sub_proof_request(proof_verifier,
                                                                           key_id.as_ptr(),
                                                                           sub_proof_request,
                                                                           claim_schema,
                                                                           issuer_pub_key,
                                                                           rev_reg_pub);
        assert_eq!(err_code, ErrorCode::Success);
    }

    pub fn _free_proof_verifier(proof_verifier: *const c_void, proof: *const c_void, nonce: *const c_void) {
        let mut valid = false;
        let err_code = indy_crypto_cl_proof_verifier_verify(proof_verifier, proof, nonce, &mut valid);
        assert_eq!(err_code, ErrorCode::Success);
    }
}