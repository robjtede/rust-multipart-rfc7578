// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate bytes;
extern crate common_multipart_rfc7578 as common_multipart;
extern crate futures;
extern crate hyper;

mod body;

pub mod client {
    pub use common_multipart::client::Error;

    pub mod multipart {
        pub use body::Body;
        pub use common_multipart::client::multipart::{BoundaryGenerator, Form};
    }
}