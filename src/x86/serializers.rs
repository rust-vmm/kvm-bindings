// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "with-serde")]

//! Custom serialization support for 4.14 and 4.20 bindings.

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

impl<'de> Deserialize<'de> for kvm_ioapic_state__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This union can either express `bits` or `fields`, both occupying `sizeof(u64)`.
        // It's therefore save to deserialize just the `bits` field.
        let bits = __u64::deserialize(deserializer)?;
        Ok(kvm_ioapic_state__bindgen_ty_1 { bits })
    }
}

impl Serialize for kvm_ioapic_state__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // This union can either express `bits` or `fields`, both occupying `sizeof(u64)`.
        // It's therefore save to serialize just the `bits` field.
        let bits = unsafe { self.bits };
        bits.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_irqchip__bindgen_ty_1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // This union can either hold a `kvm_pic_state` or a `kvm_ioapic_state`.
        // With no way of knowing which of the union's fields is needed, we deserialize the
        // larger one - `kvm_ioapic_state`.
        let ioapic = kvm_ioapic_state::deserialize(deserializer)?;
        Ok(kvm_irqchip__bindgen_ty_1 { ioapic })
    }
}

impl Serialize for kvm_irqchip__bindgen_ty_1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // This union can either hold a `kvm_pic_state` or a `kvm_ioapic_state`.
        // With no way of knowing which of the union's fields is used, we can only serialize the
        // larger one - `kvm_ioapic_state`.
        let ioapic = unsafe { self.ioapic };
        ioapic.serialize(serializer)
    }
}
