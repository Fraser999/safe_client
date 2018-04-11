// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

//! Build script for generating C header files from FFI modules.

extern crate jni;
extern crate ffi_utils;
extern crate rust_sodium;
extern crate routing;
extern crate safe_bindgen;
#[macro_use]
extern crate unwrap;

use jni::signature::{JavaType, Primitive};
use routing::XOR_NAME_LEN;
use rust_sodium::crypto::{box_, secretbox, sign};
use safe_bindgen::{Bindgen, FilterMode, LangCSharp, LangJava};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if env::var("CARGO_FEATURE_BINDINGS").is_err() {
        return;
    }

    gen_bindings_c();
    gen_bindings_csharp();
    gen_bindings_java();
}

fn gen_bindings_c() {
    unwrap!(ffi_utils::header_gen::gen_headers(
        &unwrap!(env::var("CARGO_PKG_NAME")),
        "../bindings/c/",
        "src/lib.rs",
    ));
}

fn gen_bindings_java() {
    let target_dir = Path::new("../bindings/java/safe_authenticator");

    let mut type_map = HashMap::new();
    type_map.insert(
        "XorNameArray",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "SignSecretKey",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "SignPublicKey",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "SymSecretKey",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "SymNonce",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "AsymPublicKey",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "AsymSecretKey",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert(
        "AsymNonce",
        JavaType::Array(Box::new(JavaType::Primitive(Primitive::Byte))),
    );
    type_map.insert("CipherOptHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("EncryptPubKeyHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("EncryptSecKeyHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("MDataEntriesHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert(
        "MDataEntryActionsHandle",
        JavaType::Primitive(Primitive::Long),
    );
    type_map.insert(
        "MDataPermissionsHandle",
        JavaType::Primitive(Primitive::Long),
    );
    type_map.insert(
        "SelfEncryptorReaderHandle",
        JavaType::Primitive(Primitive::Long),
    );
    type_map.insert(
        "SelfEncryptorWriterHandle",
        JavaType::Primitive(Primitive::Long),
    );
    type_map.insert("SEReaderHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("SEWriterHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("SignPubKeyHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("SignSecKeyHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("FileContextHandle", JavaType::Primitive(Primitive::Long));
    type_map.insert("App", JavaType::Primitive(Primitive::Long));
    type_map.insert("Authenticator", JavaType::Primitive(Primitive::Long));

    let mut bindgen = unwrap!(Bindgen::new());
    let mut lang = LangJava::new(type_map);

    lang.set_namespace("net.maidsafe.safe_authenticator");
    lang.set_model_namespace("net.maidsafe.safe_authenticator");

    let mut outputs = HashMap::new();

    bindgen.source_file("../safe_core/src/lib.rs");
    lang.set_lib_name("safe_core");
    unwrap!(bindgen.compile(&mut lang, &mut outputs, false));

    bindgen.source_file("../ffi_utils/src/lib.rs");
    lang.set_lib_name("ffi_utils");
    unwrap!(bindgen.compile(&mut lang, &mut outputs, false));

    bindgen.source_file("src/lib.rs");
    lang.set_lib_name(unwrap!(env::var("CARGO_PKG_NAME")));
    unwrap!(bindgen.compile(&mut lang, &mut outputs, true));

    unwrap!(bindgen.write_outputs(target_dir, &outputs));
}

fn gen_bindings_csharp() {
    let target_dir = Path::new("../bindings/csharp/safe_authenticator");

    let mut bindgen = unwrap!(Bindgen::new());
    let mut lang = LangCSharp::new();

    lang.set_lib_name(unwrap!(env::var("CARGO_PKG_NAME")));

    lang.set_interface_section(
        "SafeAuth.Utilities/IAuthBindings.cs",
        "SafeAuth.Utilities",
        "IAuthBindings",
    );
    lang.set_functions_section(
        "SafeAuth.AuthBindings/AuthBindings.cs",
        "SafeAuth.AuthBindings",
        "AuthBindings",
    );
    lang.set_consts_section(
        "SafeAuth.Utilities/AuthConstants.cs",
        "SafeAuth.Utilities",
        "AuthConstants",
    );
    lang.set_types_section("SafeAuth.Utilities/AuthTypes.cs", "SafeAuth.Utilities");
    lang.set_utils_section(
        "SafeAuth.Utilities/BindingUtils.cs",
        "SafeAuth.Utilities",
        "BindingUtils",
    );

    lang.add_const("ulong", "ASYM_PUBLIC_KEY_LEN", box_::PUBLICKEYBYTES);
    lang.add_const("ulong", "ASYM_SECRET_KEY_LEN", box_::SECRETKEYBYTES);
    lang.add_const("ulong", "ASYM_NONCE_LEN", box_::NONCEBYTES);
    lang.add_const("ulong", "SYM_KEY_LEN", secretbox::KEYBYTES);
    lang.add_const("ulong", "SYM_NONCE_LEN", secretbox::NONCEBYTES);
    lang.add_const("ulong", "SIGN_PUBLIC_KEY_LEN", sign::PUBLICKEYBYTES);
    lang.add_const("ulong", "SIGN_SECRET_KEY_LEN", sign::SECRETKEYBYTES);
    lang.add_const("ulong", "XOR_NAME_LEN", XOR_NAME_LEN);
    lang.add_opaque_type("Authenticator");

    lang.reset_filter(FilterMode::Blacklist);
    lang.filter("AuthFuture");

    bindgen.source_file("../safe_core/src/lib.rs");
    bindgen.compile_or_panic(&mut lang, &mut HashMap::new(), false);

    let mut outputs = HashMap::new();
    bindgen.source_file("src/lib.rs");
    bindgen.compile_or_panic(&mut lang, &mut outputs, true);
    apply_patches(&mut outputs);
    bindgen.write_outputs_or_panic(target_dir, &outputs);

    // Hand-written code.
    let resource_path = Path::new("resource");
    if resource_path.is_dir() {
        unwrap!(ffi_utils::bindgen_utils::copy_files(
            resource_path,
            target_dir,
            ".cs",
        ));
    }
}

fn apply_patches(outputs: &mut HashMap<PathBuf, String>) {
    {
        let content = fetch_mut(outputs, "SafeAuth.AuthBindings/AuthBindings.cs");
        insert_using_utilities(content);
        insert_using_obj_c_runtime(content);
        insert_guard(content);
    }

    for content in outputs.values_mut() {
        fix_names(content);
    }
}

fn insert_using_utilities(content: &mut String) {
    content.insert_str(0, "using SafeAuth.Utilities;\n");
}

fn insert_using_obj_c_runtime(content: &mut String) {
    content.insert_str(0, "#if __IOS__\nusing ObjCRuntime;\n#endif\n");
}

fn insert_guard(content: &mut String) {
    content.insert_str(0, "#if !NETSTANDARD1_2 || __DESKTOP__\n");
    content.push_str("#endif\n");
}

fn fix_names(content: &mut String) {
    *content = content.replace("Idata", "IData").replace("Mdata", "MData");
}

fn fetch_mut<T: AsRef<Path>>(outputs: &mut HashMap<PathBuf, String>, key: T) -> &mut String {
    let key = key.as_ref();
    unwrap!(outputs.get_mut(key), "key {:?} not found in outputs", key)
}
