// RGB standard library for working with smart contracts on Bitcoin & Lightning
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RGB contract interface provides a mapping between identifiers of RGB schema-
//! defined contract state and operation types to a human-readable and
//! standardized wallet APIs.

mod forge;
mod iface;
mod imp;

pub use imp::{IfaceImpl, NamedType};

use crate::LIB_NAME_RGB_STD;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_STD, tags = repr, into_u8, try_from_u8)]
#[repr(u8)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub enum IfaceStd {
    #[strict_type(dumb)]
    #[display("RGB20")]
    Rgb20Fungible = 20,

    #[display("RGB21")]
    Rgb21Collectible = 21,

    #[display("RGB22")]
    Rgb22Identity = 22,

    #[display("RGB23")]
    Rgb23Audit = 23,

    #[display("RGB24")]
    Rgb24Naming = 24,
}
