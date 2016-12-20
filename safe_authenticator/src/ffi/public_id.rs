// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License, version 1.0 or later, or (2) The General Public License
// (GPL), version 3, depending on which licence you accepted on initial access
// to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be bound by the terms of the MaidSafe Contributor
// Agreement, version 1.0.
// This, along with the Licenses can be found in the root directory of this
// project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed under the GPL Licence is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations relating to use of the SAFE Network Software.

use Authenticator;
use ffi_utils::{FfiString, OpaqueCtx, catch_unwind_cb};
use futures::Future;
use public_id;
use safe_core::FutureExt;
use std::os::raw::c_void;

/// Create Public ID.
#[no_mangle]
pub unsafe extern "C" fn public_id_create(auth: *const Authenticator,
                                          public_id: FfiString,
                                          user_data: *mut c_void,
                                          o_cb: extern "C" fn(*mut c_void, i32)) {
    catch_unwind_cb(user_data, o_cb, || {
        let user_data = OpaqueCtx(user_data);
        let public_id = public_id.to_string()?;

        (*auth).send(move |client| {
            public_id::create(client, &public_id)
                .then(move |res| {
                    o_cb(user_data.0, ffi_result_code!(res));
                    Ok(())
                })
                .into_box()
                .into()
        })
    })
}

#[cfg(test)]
mod tests {
    use errors::ERR_ENTRY_EXISTS;

    use ffi_utils::FfiString;
    use ffi_utils::test_utils::call_0;
    use safe_core::utils;
    use super::*;
    use test_utils::create_authenticator;

    #[test]
    fn create() {
        let authenticator = create_authenticator();
        let public_id = unwrap!(utils::generate_random_string(10));
        let ffi_public_id = FfiString::from_str(&public_id);

        // Create public first time succeeds.
        unsafe { unwrap!(call_0(|ud, cb| public_id_create(&authenticator, ffi_public_id, ud, cb))) }

        // Attempt to create already existing public id fails.
        let res =
            unsafe { call_0(|ud, cb| public_id_create(&authenticator, ffi_public_id, ud, cb)) };

        match res {
            Err(code) if code == ERR_ENTRY_EXISTS => (),
            Err(err) => panic!("Unexpected {:?}", err),
            Ok(_) => panic!("Unexpected success"),
        }
    }
}
