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

use crate::builder::{ArrayBuilder, FixedSizeBinaryBuilder, PrimitiveBuilder, UInt8BufferBuilder};
use crate::{Array, ArrayRef, FixedSizeBinaryArray};
use arrow_buffer::Buffer;
use arrow_buffer::NullBufferBuilder;
use arrow_data::ArrayData;
use arrow_schema::{ArrowError, DataType};
use std::any::Any;
use std::sync::Arc;
use hashbrown::HashTable;
use crate::types::ArrowDictionaryKeyType;

/// Builder for [`DictionaryArray`] of [`FixedSizeBinaryArray`]
///
/// For example to map a set of byte indices to FixedSizeBinary values. Note that
/// the use of a `HashMap` here will not scale to very large arrays or
/// result in an ordered dictionary.
#[derive(Debug)]
pub struct FixedSizeBinaryDictionaryBuilder<K, T>
where
    K: ArrowDictionaryKeyType,
    T: FixedSizeBinaryArray,
{
    state: ahash::RandomState,
    dedup: HashTable<usize>,

    keys_builder: PrimitiveBuilder<K>,
    values_builder: FixedSizeBinaryBuilder<T>,
}

impl<K, T> Default for FixedSizeBinaryDictionaryBuilder<K, T>
where
    K: ArrowDictionaryKeyType,
    T: FixedSizeBinaryArray,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, T> FixedSizeBinaryDictionaryBuilder<K, T>
where
    K: ArrowDictionaryKeyType,
    T: FixedSizeBinaryArray,
{
    /// Creates a new `FixedSizeBinaryDictionaryBuilder`
    pub fn new(byte_width: i32) -> Self {
        let keys_builder = PrimitiveBuilder::new();
        let values_builder = FixedSizeBinaryBuilder::new(byte_width);
        Self {
            state: Default::default(),
            dedup: HashTable::with_capacity(keys_builder.capacity()),
            keys_builder,
            values_builder,
        }
    }

    // /// Creates a new `FixedSizeBinaryDictionaryBuilder` with the provided capacities
    // ///
    // /// `keys_capacity`: the number of keys, i.e. length of array to build
    // /// `value_capacity`: the number of distinct dictionary values, i.e. size of dictionary
    // pub fn with_capacity(keys_capacity: usize, value_capacity: usize, byte_width: i32) -> Self {
    //     Self {
    //         state: Default::default(),
    //         dedup: Default::default(),
    //         keys_builder: PrimitiveBuilder::with_capacity(keys_capacity),
    //         values_builder: UInt8BufferBuilder::new(capacity * byte_width as usize),
    //     }
    // }
}

