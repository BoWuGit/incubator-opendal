// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! [`type_alias_impl_trait`](https://github.com/rust-lang/rust/issues/63063) is not stable yet.
//! So we can't write the following code:
//!
//! ```txt
//! impl Accessor for S3Backend {
//!     type Writer = impl oio::Write;
//! }
//! ```
//!
//! Which means we have to write the type directly like:
//!
//! ```txt
//! impl Accessor for OssBackend {
//!     type Writer = oio::TwoWaysWriter<
//!         oio::MultipartUploadWriter<OssWriter>,
//!         oio::AppendObjectWriter<OssWriter>,
//!     >;
//! }
//! ```
//!
//! This module is used to provide some enums for the above code. We should remove this module once
//! type_alias_impl_trait has been stabilized.

use async_trait::async_trait;
use bytes::Bytes;

use crate::raw::oio::Streamer;
use crate::raw::*;
use crate::*;

/// TwoWaysWrite is used to implement [`Write`] based on two ways.
///
/// Users can wrap two different writers together.
pub enum TwoWaysWriter<ONE: oio::Write, TWO: oio::Write> {
    /// The first type for the [`TwoWaysWriter`].
    One(ONE),
    /// The second type for the [`TwoWaysWriter`].
    Two(TWO),
}

#[async_trait]
impl<ONE: oio::Write, TWO: oio::Write> oio::Write for TwoWaysWriter<ONE, TWO> {
    async fn write(&mut self, bs: Bytes) -> Result<()> {
        match self {
            Self::One(one) => one.write(bs).await,
            Self::Two(two) => two.write(bs).await,
        }
    }

    async fn sink(&mut self, size: u64, s: Streamer) -> Result<()> {
        match self {
            Self::One(one) => one.sink(size, s).await,
            Self::Two(two) => two.sink(size, s).await,
        }
    }

    async fn abort(&mut self) -> Result<()> {
        match self {
            Self::One(one) => one.abort().await,
            Self::Two(two) => two.abort().await,
        }
    }

    async fn close(&mut self) -> Result<()> {
        match self {
            Self::One(one) => one.close().await,
            Self::Two(two) => two.close().await,
        }
    }
}

/// ThreeWaysWriter is used to implement [`Write`] based on three ways.
///
/// Users can wrap three different writers together.
pub enum ThreeWaysWriter<ONE: oio::Write, TWO: oio::Write, THREE: oio::Write> {
    /// The first type for the [`ThreeWaysWriter`].
    One(ONE),
    /// The second type for the [`ThreeWaysWriter`].
    Two(TWO),
    /// The third type for the [`ThreeWaysWriter`].
    Three(THREE),
}

#[async_trait]
impl<ONE: oio::Write, TWO: oio::Write, THREE: oio::Write> oio::Write
    for ThreeWaysWriter<ONE, TWO, THREE>
{
    async fn write(&mut self, bs: Bytes) -> Result<()> {
        match self {
            Self::One(one) => one.write(bs).await,
            Self::Two(two) => two.write(bs).await,
            Self::Three(three) => three.write(bs).await,
        }
    }

    async fn sink(&mut self, size: u64, s: Streamer) -> Result<()> {
        match self {
            Self::One(one) => one.sink(size, s).await,
            Self::Two(two) => two.sink(size, s).await,
            Self::Three(three) => three.sink(size, s).await,
        }
    }

    async fn abort(&mut self) -> Result<()> {
        match self {
            Self::One(one) => one.abort().await,
            Self::Two(two) => two.abort().await,
            Self::Three(three) => three.abort().await,
        }
    }

    async fn close(&mut self) -> Result<()> {
        match self {
            Self::One(one) => one.close().await,
            Self::Two(two) => two.close().await,
            Self::Three(three) => three.close().await,
        }
    }
}
