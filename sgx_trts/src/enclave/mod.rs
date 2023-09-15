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
// under the License..

mod atexit;
mod entry;
mod init;
mod mem;
mod uninit;

pub mod parse;
pub mod state;

pub use atexit::{at_exit, cleanup};
pub use init::{ctors, global_init, rtinit};
pub use mem::{
    is_within_enclave, is_within_host, is_within_rts_range, is_within_user_range, EnclaveRange,
    MmLayout,
};
pub use uninit::{global_exit, rtuninit, UNINIT_FLAG};
