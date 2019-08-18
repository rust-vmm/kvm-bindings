// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "with_serde")]
include!("build_serde.rs");

#[cfg(not(feature = "with_serde"))]
fn main() {}
