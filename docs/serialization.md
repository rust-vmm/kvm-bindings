# Serializing `kvm-bindings`

## Purpose

The purpose of this document is to identify requirements for `kvm-bindings`
serialization. The requirements will be iterated upon in order to find the best
solution that fits all the use cases.

## Why serialize?

[Firecracker](https://firecracker-microvm.github.io/) has
[snapshotting support](https://github.com/firecracker-microvm/firecracker/issues/1184)
on the roadmap. In order to snapshot a microVM, the internal KVM state needs to
be saved to a file, from which it will be loaded in a new microVM at a later
point in time. Part of a microVM's state is stored in data structures consumed
from the `kvm-bindings` crate.

[cloud-hypervisor](https://github.com/intel/cloud-hypervisor) has
[live migration](https://github.com/intel/cloud-hypervisor/issues/75) on the
roadmap. Part of the live migration process also boils down to serializing
internal state in order to migrate it. Same as Firecracker, cloud-hypervisor
consumes the `kvm-bindings` crate to model the VM.

## Current state

2 proposals have already been submitted as RFCs in the `kvm-bindings` repo:
[`#4`](https://github.com/rust-vmm/kvm-bindings/pull/4) and 
[`#7`](https://github.com/rust-vmm/kvm-bindings/pull/7). In their current
state, both proposals include functionality to serialize *all* the data
structures exposed by the `kvm-bindings` crate, leveraging their
[C-struct](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) nature by
serializing their byte representations as `u8` slices. While this approach is
convenient and easy to implement, it poses some problems:
* inherent unsafety due to direct pointer manipulation in `unsafe` blocks for
  each data structure;
* irrelevant data structures are serializable, even though there is no need;
* the format is inflexible and error prone;
* versioning support cannot be added.

## What to serialize?

There is no need to serialize all the data structures exposed by the crate.
Only those that belong in the VM state that needs to be snapshotted / migrated
must be serializable. This may introduce an overhead of manual work, as
opposed to autogenerating the functionality, for each new addition of a
supported kernel version / platform.

## Requirements

### Versioning

While there is no set decision upon whether or not the structures should be
versioned, and whether backwards compatibility of deserialization is a
requirement, the aim should be to avoid one way doors and leave room for
versioning support, should it prove necessary. Upon deserializing bindings, 2
versions become apparent:

1. the crate version: this is relevant, for instance, in case a security issue
   is fixed in `kvm-bindings`, and the crate consumer wants to ensure that they
   are not deserializing a potentially vulnerable source.
1. the kernel version: this is relevant when deserializing on a host with a
   different kernel version than the one the original structures have embedded.
1. the architecture: this is relevant, for instance, when deserializing a
   snapshot generated on `aarch64` on an `x86_64` host (or vice versa).

#### Implementation

Following are 2 options to extend `kvm-bindings` with version support:

1. Each data structure embeds its version in its serialized representation.
1. A global `version` object is kept per crate. It is up to the consumer to
   include this object in the serialized data, along with the bindings, and
   check version compatibility when deserializing.

### Portability

The bindings' byte representation is arch-dependent, rendering both RFCs
submitted so far un-portable.

### Safety

Aside from the inherent unsafety of raw pointer manipulation and serialization
by `memcpy` and `ptr::read`, many of the bindings contain `union` members.
Union serialization needs to be handled carefully in order to avoid
(de)serializing padding data, which can be manipulated into being malicious.

### Maintainability

Ideally, when adding support for a new kernel version, the maintainer would
have minimal manual work to do for serialization. For instance, manually
implementing `serde` support implies writing 2 custom `serialize` and 2 custom
`deserialize` function per data structure - because `serde` supports 2 data
access patterns (sequential, i.e. a struct's fields are serialized one after
the other, and map, i.e. a struct's fields are serialized in an associative
fashion). This would be cumbersome, especially for a maintainer who has no
interest in serialization. At the other end of the spectrum, autogenerating
serialization support based on a template is attractive, but may pose more
problems in the long term (for instance, if there's a bug in the template, all
the data structures will be affected).

### Conditional compilation

As serialization is not required by all the existing and potential consumers of
the `kvm-bindings` crate (for instance, [`enarx`](https://github.com/enarx/)),
serialization support needs to be optional, and controlled via
[build features](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-sectiona).

## Implementation

### `serde`

[`serde`](https://github.com/serde-rs/serde) is a popular Rust serialization
framework. It includes support for serializing primitives, and defines the API
for `serde` implementers to fill in. This means that deriving / implementing
`serde::Serialize` and `serde::Deserialize` alone is **not** enough to obtain
the serialized representation of a struct. A `serde`-compatible implementer
does the job of actually arranging the struct's data in another format.

`serde` has a large community and is well maintained, with over 15 million
 downloads, 188 releases and 120 contributors.
 
 The ideal would be for `kvm-bindings` to expose the minimum required set of
 `serde`-compatible serializable data structures, leaving it to the consumer
 to pick which serializer to plug in.
 
 Following are 3 `serde`-compatible serializers.

#### `serde-json`

[`serde-json`](https://github.com/serde-rs/json) implements the `serde`
framework for the JSON output format. It leverages `serde`'s
[map access pattern](https://docs.serde.rs/serde/de/trait.MapAccess.html).

`serde-json` has a large community and is well maintained, with over 10 million
downloads, 73 releases and 66 contributors. 

#### `serde-cbor`

[`serde-cbor`](https://github.com/pyfisch/cbor) implements the `serde`
framework for the [CBOR](https://tools.ietf.org/html/rfc7049) format. It
combines the strong type definition specific to JSON with a binary format for
small snapshot sizes.

`serde-cbor` has a small community, with over 715,000 downloads, 21 releases
and 24 contributors. 

#### `bincode`

[`bincode`](https://github.com/servo/bincode) implements the `serde` framework
for a binary format. It leverages `serde`'s
[sequence access pattern](https://docs.serde.rs/serde/de/trait.SeqAccess.html).
`bincode` serialized objects are very compact and sometimes can occupy less
memory than the original structure.

`bincode` has a medium-sized community, with over 2 million downloads, 55
releases and 52 contributors. 
