//! The `snow` crate is a straightforward, Hard To Fuck Up™ Noise Protocol implementation.
//!
//! Read the [Noise Protocol Framework Spec](http://noiseprotocol.org/noise.html) for more
//! information.
//!
//! The typical usage flow is to use [`Builder`] to construct a [`Session`], which is main
//! state machine you will want to interact with.
//!
//! # Examples
//! See `examples/simple.rs` for a more complete TCP client/server example with static keys.
//!
//! ```
//! # extern crate snow;
//! # use snow::Error;
//! #
//! # fn try_main() -> Result<(), Error> {
//! static PATTERN: &'static str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";
//!
//! let mut initiator = snow::Builder::new(PATTERN.parse()?)
//!     .build_initiator()?;
//! let mut responder = snow::Builder::new(PATTERN.parse()?)
//!     .build_responder()?;
//!
//! let (mut read_buf, mut first_msg, mut second_msg) =
//!     ([0u8; 1024], [0u8; 1024], [0u8; 1024]);
//!
//! // -> e
//! let len = initiator.write_message(&[], &mut first_msg)?;
//!
//! // responder processes the first message...
//! responder.read_message(&first_msg[..len], &mut read_buf)?;
//!
//! // <- e, ee
//! let len = responder.write_message(&[], &mut second_msg)?;
//!
//! // initiator processes the response...
//! initiator.read_message(&second_msg[..len], &mut read_buf)?;
//!
//! // NN handshake complete, transition into transport mode.
//! let initiator = initiator.into_transport_mode();
//! let responder = responder.into_transport_mode();
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! [`Builder`]: struct.Builder.html
//! [`Session`]: enum.Session.html

#![warn(missing_docs)]

#[cfg(any(feature = "default-resolver", feature = "hacl-star-resolver"))]
#[macro_use]
extern crate arrayref;

#[macro_use]
pub mod error;
#[macro_use]
mod utils;
mod constants;
mod cipherstate;
mod symmetricstate;
mod handshakestate;
mod builder;
mod transportstate;
mod stateless_transportstate;

pub mod params;
pub mod types;
pub mod resolvers;

pub use crate::error::Error;
pub use crate::builder::{Builder, Keypair};
pub use crate::handshakestate::HandshakeState;
pub use crate::transportstate::TransportState;
pub use crate::stateless_transportstate::StatelessTransportState;
