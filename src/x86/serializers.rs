// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "with-serde")]

//! Custom serialization support for 4.20 bindings.

use serde::de::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

use super::bindings::*;

impl<'de> Deserialize<'de> for kvm_lapic_state {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let regs: Vec<::std::os::raw::c_char> = Vec::deserialize(deserializer)?;
        let mut val = kvm_lapic_state::default();
        // This panics if the source and destination have different lengths.
        val.regs.copy_from_slice(&regs[..]);
        Ok(val)
    }
}

impl Serialize for kvm_lapic_state {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let regs = &self.regs[..];
        regs.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_xsave {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let region: Vec<__u32> = Vec::deserialize(deserializer)?;
        let mut val = kvm_xsave::default();
        // This panics if the source and destination have different lengths.
        val.region.copy_from_slice(&region[..]);
        Ok(val)
    }
}

impl Serialize for kvm_xsave {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let region = &self.region[..];
        region.serialize(serializer)
    }
}
