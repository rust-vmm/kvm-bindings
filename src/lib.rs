// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
#[cfg(all(
    feature = "fam-wrappers",
    any(target_arch = "x86", target_arch = "x86_64")
))]
extern crate vmm_sys_util;

#[cfg(feature = "with-serde")]
extern crate serde;

#[cfg(feature = "with-serde")]
extern crate serde_derive;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::bindings::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::Version;

#[cfg(target_arch = "aarch")]
mod arm;
#[cfg(target_arch = "aarch")]
pub use self::arm::bindings::*;
#[cfg(target_arch = "aarch")]
pub use self::arm::Version;

#[cfg(target_arch = "aarch64")]
mod arm64;
#[cfg(target_arch = "aarch64")]
pub use self::arm64::bindings::*;
#[cfg(target_arch = "aarch64")]
pub use self::arm64::Version;

#[cfg(all(test, feature = "with-serde"))]
mod tests {
    extern crate serde_json;

    use super::Version;

    #[test]
    fn test_version_ser_deser() {
        let ver = Version {
            arch: "x86_64",
            kernel_ver: "v4.20.0",
            crate_ver: "v0.2.0",
        };
        let ver_str = "{\"arch\":\"x86_64\",\"kernel_ver\":\"v4.20.0\",\"crate_ver\":\"v0.2.0\"}";
        let ver_ser = serde_json::to_string(&ver).unwrap();
        assert_eq!(ver_ser, ver_str.to_string());

        let ver_deser = serde_json::from_str::<Version>(ver_str).unwrap();
        assert_eq!(ver, ver_deser);
    }
}
