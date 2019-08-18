// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "with_serde")]
extern crate libc;
#[cfg(feature = "with_serde")]
extern crate serde;
#[cfg(feature = "with_serde")]
extern crate serde_bytes;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::bindings::*;

#[cfg(target_arch = "aarch")]
mod arm;
#[cfg(target_arch = "aarch")]
pub use self::arm::bindings::*;

#[cfg(target_arch = "aarch64")]
mod arm64;
#[cfg(target_arch = "aarch64")]
pub use self::arm64::bindings::*;


// Serializers for x86 and x86_64.
#[cfg(all(feature = "with_serde", feature = "kvm-v4_14_0", any(target_arch = "x86", target_arch = "x86_64")))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/x86/serializers_v4_14_0.rs"));
#[cfg(all(feature = "with_serde", feature = "kvm-v4_20_0", any(target_arch = "x86", target_arch = "x86_64")))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/x86/serializers_v4_20_0.rs"));
#[cfg(all(feature = "with_serde", not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0"), any(target_arch = "x86", target_arch = "x86_64")))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/x86/serializers_v4_20_0.rs"));

// Serializers for aarch.
#[cfg(all(feature = "with_serde", feature = "kvm-v4_14_0", target_arch = "aarch"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm/serializers_v4_14_0.rs"));
#[cfg(all(feature = "with_serde", feature = "kvm-v4_20_0", target_arch = "aarch"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm/serializers_v4_20_0.rs"));
#[cfg(all(feature = "with_serde", not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0"), target_arch = "aarch"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm/serializers_v4_20_0.rs"));

// Serializers for aarch64.
#[cfg(all(feature = "with_serde", feature = "kvm-v4_14_0", target_arch = "aarch64"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm64/serializers_v4_14_0.rs"));
#[cfg(all(feature = "with_serde", feature = "kvm-v4_20_0", target_arch = "aarch64"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm64/serializers_v4_20_0.rs"));
#[cfg(all(feature = "with_serde", not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0"), target_arch = "aarch64"))]
include!(concat!(env!("OUT_DIR"), "/serialization/src/arm64/serializers_v4_20_0.rs"));
