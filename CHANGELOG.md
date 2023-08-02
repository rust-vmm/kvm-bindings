# Changelog
## [Unreleased]

- arm64: add KVM_REG_ARM64_SVE_VLS const
- API change in the bindings from upstream kernel changes:
* system_event has been made into a new union
- The x86 module has been renamed to x86_64 for consistency (matches the kernel
  architecture directory name)
- Dropped "x86" (32-bit) x86 support

## [0.6.0]

### Changed

- Updated vmm-sys-utils dependency to 0.11.0
- Switched to specifying dependencies using caret requirements
  instead of comparision requirements

### Added

- Implement `PartialEq` for fam\_wrappers 

## [0.5.0]

### Changed

- Replaced the v4.20 bindings with the v5.13 ones.

### Removed

- Removed v4.14 bindings.

## [0.4.0]

- vmm-sys-utils dependency bumped to match kvm-ioctls.

## [0.3.0]

### Added

- Enabled `fam-wrappers` support on arm and arm64.
- Added fam-wrapper for the arm specific `kvm_reg_list` struct.

## [0.2.0]

### Added

- Added opt-in feature `fam-wrappers` that enables exporting
  safe wrappers over generated structs with flexible array
  members. This optional feature has an external dependency
  on `vmm-sys-util`.
- Added safe fam-wrappers for `kvm_msr_list`, `kvm_msrs`,
  and `kvm_cpuid2`.

## [0.1.1]

### Changed

- Do not enforce rust Edition 2018.

## [0.1.0]

### Added

- KVM bindings for Linux kernel version 4.14 and 4.20 with
  support for arm, arm64, x86 and x86_64.
