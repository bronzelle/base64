pub mod base64;

pub use self::base64::encode_iterator::EncodeBase64IterExt;
pub use self::base64::{decode, encode};
