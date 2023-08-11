#![allow(bad_style)]
#![allow(unused)]
#![allow(clippy::all)]
#![allow(deprecated)]
#![allow(deref_nullptr)]
#![allow(invalid_value)] // mem::uninitialized has to stay

use core_foundation_sys::base::{CFOptionFlags, OSStatus};
use core_foundation_sys::string::CFStringRef;
use std::os::raw::*;

#[cfg(target_os = "macos")]
use apple_security_sys::access::*;
use apple_security_sys::access_control::*;
use apple_security_sys::authorization::*;
use apple_security_sys::base::*;
use apple_security_sys::certificate::*;
#[cfg(target_os = "macos")]
use apple_security_sys::certificate_oids::*;
use apple_security_sys::cipher_suite::*;
#[cfg(target_os = "macos")]
use apple_security_sys::code_signing::*;
#[cfg(target_os = "macos")]
use apple_security_sys::digest_transform::*;
#[cfg(target_os = "macos")]
use apple_security_sys::encrypt_transform::*;
use apple_security_sys::identity::*;
use apple_security_sys::import_export::*;
use apple_security_sys::item::*;
use apple_security_sys::key::*;
#[cfg(target_os = "macos")]
use apple_security_sys::keychain::*;
#[cfg(target_os = "macos")]
use apple_security_sys::keychain_item::*;
use apple_security_sys::policy::*;
use apple_security_sys::random::*;
use apple_security_sys::secure_transport::*;
#[cfg(target_os = "macos")]
use apple_security_sys::transform::*;
use apple_security_sys::trust::*;
use apple_security_sys::trust_settings::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
