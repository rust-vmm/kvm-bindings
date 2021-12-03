// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use vmm_sys_util::fam::{FamStruct, FamStructWrapper};

use arm64::bindings::*;

// There is no constant in the kernel as far as the maximum number
// of registers on arm, but KVM_GET_REG_LIST usually returns around 450.
const ARM64_REGS_MAX: usize = 500;

// Implement the FamStruct trait for kvm_reg_list.
generate_fam_struct_impl!(kvm_reg_list, u64, reg, u64, n, ARM64_REGS_MAX);

// Implement the PartialEq trait for kvm_reg_list.
impl PartialEq for kvm_reg_list {
    fn eq(&self, other: &kvm_reg_list) -> bool {
        // No need to call entries's eq, FamStructWrapper's PartialEq will do it for you
        self.n == other.n
    }
}

/// Wrapper over the `kvm_reg_list` structure.
///
/// The `kvm_reg_list` structure contains a flexible array member. For details check the
/// [KVM API](https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt)
/// documentation on `kvm_reg_list`. To provide safe access to
/// the array elements, this type is implemented using
/// [FamStructWrapper](../vmm_sys_util/fam/struct.FamStructWrapper.html).
pub type RegList = FamStructWrapper<kvm_reg_list>;

#[cfg(test)]
mod tests {
    use super::RegList;

    #[test]
    fn test_reg_list_eq() {
        let mut wrapper = RegList::new(1).unwrap();
        assert_eq!(wrapper.as_slice().len(), 1);

        let mut wrapper2 = wrapper.clone();
        assert!(wrapper == wrapper2);

        wrapper.as_mut_slice()[0] = 1;
        assert!(wrapper != wrapper2);
        wrapper2.as_mut_slice()[0] = 1;
        assert!(wrapper == wrapper2);
    }
}
