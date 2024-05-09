# Changelog
## [Unreleased]

### Added

- arm64 KVM_ARM_VCPU_HAS_EL2 feature bit for checking nested virtualization
  capability.

### Changed

### Removed

## [0.8.1]

### Fixed

- Implement `Default` for `kvm_xsave2`, which fixes usage of `Xsave`
  unconditionally causing compile errors in downstream crates.

## [0.8.0]

### Added

- An opt-in feature `serde` that enables [`serde`](https://serde.rs)-based
  (de)serialization of various bindings. 

## [0.7.0]

### Changed
- API change in the bindings from upstream kernel changes:
  * system\_event has been made into a new union
- The x86 module has been renamed to x86\_64 for consistency (matches the kernel
  architecture directory name)
- Added all features to the generated docs.rs documentation.

### Removed

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
