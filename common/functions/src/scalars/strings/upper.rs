// Copyright 2021 Datafuse Labs.
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

use bstr::ByteSlice;
use bytes::BufMut;

use super::string2string::String2StringFunction;
use super::string2string::StringOperator;

#[derive(Clone, Default)]
pub struct Upper;

impl StringOperator for Upper {
    #[inline]
    fn apply_with_no_null<'a>(&'a mut self, s: &'a [u8], mut buffer: &mut [u8]) -> usize {
        for (start, end, ch) in s.char_indices() {
            if ch == '\u{FFFD}' {
                // If char is not valid, just copy it.
                buffer.put_slice(&s.as_bytes()[start..end]);
            } else if ch.is_ascii() {
                buffer.put_u8(ch.to_ascii_uppercase() as u8);
            } else {
                for x in ch.to_uppercase() {
                    buffer.put_slice(x.encode_utf8(&mut [0; 4]).as_bytes());
                }
            }
        }
        s.len()
    }
}

pub type UpperFunction = String2StringFunction<Upper>;
