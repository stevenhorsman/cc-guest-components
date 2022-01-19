// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0

/// Environment macro for `image-rs` work dir.
pub const CC_IMAGE_WORK_DIR: &str = "CC_IMAGE_WORK_DIR";

pub mod bundle;
pub mod config;
pub mod decoder;
pub mod decrypt;
pub mod image;
pub mod pull;
pub mod snapshots;
pub mod unpack;