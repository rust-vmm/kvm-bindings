// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

use vmm_serde::*;

#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_cpuid, nent, kvm_cpuid_entry);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_cpuid2, nent, kvm_cpuid_entry2);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_irq_routing, nr, kvm_irq_routing_entry);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_msrs, nmsrs, kvm_msr_entry);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_msr_list, nmsrs, __u32);
#[cfg(all(feature = "with-serde", not(feature = "kvm-v4_14_0")))]
serde_ffi_fam_impl!(kvm_nested_state, size, __u8);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_signal_mask, len, __u8);
#[cfg(feature = "with-serde")]
serde_ffi_fam_impl!(kvm_reg_list, n, __u64);

#[cfg(feature = "with-serde")]
impl SizeofFamStruct for kvm_coalesced_mmio_ring {
    fn size_of(&self) -> usize {
        (self.last - self.first) as usize * std::mem::size_of::<kvm_coalesced_mmio>()
            + std::mem::size_of::<kvm_coalesced_mmio_ring>()
    }
}

#[cfg(all(test, feature = "with-serde"))]
mod tests
{
    use super::*;

    #[test]
    fn ffi_test_ffi_fam_struct() {
        let data = vec![
            kvm_msrs {
                nmsrs: 1,
                pad: 0,
                entries: __IncompleteArrayField::new(),
            },
            kvm_msrs {
                nmsrs: 0x1,
                pad: 0x2,
                entries: __IncompleteArrayField::new(),
            },
        ];
        let ser = serde_json::to_string(&data[0]).unwrap();
        let mut deserializer = serde_json::Deserializer::from_str(&ser);
        let content: Vec<kvm_msrs> = kvm_msrs::deserialize(&mut deserializer).unwrap();
        // let decoded: FamStructWrapper<kvm_msrs> = content.into();

        assert_eq!(content[0].nmsrs, 1);
        assert_eq!(content[0].pad, 0);
    }

    #[test]
    fn ffi_test_ffi_struct() {
        let data = kvm_dtable {
            base: 0x1000,
            limit: 0x2000,
            ..Default::default()
        };
        let ser = serde_json::to_string(&data).unwrap();
        let content: kvm_dtable = serde_json::from_str(&ser).unwrap();

        assert_eq!(content.base, 0x1000);
        assert_eq!(content.limit, 0x2000);
    }
}
