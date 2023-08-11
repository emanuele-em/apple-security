//! Access functionality.

use core_foundation::base::TCFType;
use apple_security_sys::access::SecAccessGetTypeID;
use apple_security_sys::base::SecAccessRef;

declare_TCFType! {
    /// A type representing access settings.
    SecAccess, SecAccessRef
}
impl_TCFType!(SecAccess, SecAccessRef, SecAccessGetTypeID);

unsafe impl Sync for SecAccess {}
unsafe impl Send for SecAccess {}
