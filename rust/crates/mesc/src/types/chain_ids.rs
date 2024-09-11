use crate::MescError;
use serde::{Deserialize, Serialize};

/// ChainId is a string representation of an integer chain id
/// - TryFrom conversions allow specifying as String, &str, uint, or binary data
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(into = "ChainIdSerde", try_from = "ChainIdSerde")]
pub struct ChainId(u64);

impl ChainId {
    /// create new chain id
    pub fn new(chain_id: u64) -> ChainId {
        ChainId(chain_id)
    }

    /// parse chain id from string
    pub fn parse(chain_id: &str) -> Result<ChainId, MescError> {
        chain_id.try_into_chain_id()
    }

    /// get chain id value
    pub fn get(&self) -> u64 {
        self.0
    }

    /// convert to hex representation
    pub fn to_hex(&self) -> String {
        let ChainId(chain_id) = self;
        format!("{chain_id:#x}")
    }

    /// convert to hex representation, zero-padded to 256 bits
    pub fn to_hex_256(&self) -> String {
        let ChainId(chain_id) = self;
        format!("0x{chain_id:064x}")
    }
}

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! impl_from_uint_for_chainid {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ChainId {
                fn from(value: $t) -> ChainId {
                    ChainId(value as _)
                }
            }
        )*
    };
}

impl_from_uint_for_chainid!(u8, u16, u32, u64, usize);

/// use custom trait instead of TryInto so that Error type is always the same
pub trait TryIntoChainId {
    /// try to convert into chain id
    fn try_into_chain_id(self) -> Result<ChainId, MescError>;
}

impl TryIntoChainId for ChainId {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        Ok(self)
    }
}

impl TryIntoChainId for String {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        self.as_str().try_into_chain_id()
    }
}

impl TryIntoChainId for &str {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        if !self.is_empty() {
            let (radix, s) = match self.get(0..2) {
                Some("0x") => (16, &self[2..]),
                _ => (10, self),
            };
            if let Ok(chain_id) = u64::from_str_radix(s, radix) {
                return Ok(ChainId(chain_id));
            }
        }
        Err(MescError::InvalidChainId(self.to_string()))
    }
}

macro_rules! impl_try_into_chain_id_for_integer {
    ($($t:ty),*) => {
        $(
            impl TryIntoChainId for $t {
                fn try_into_chain_id(self) -> Result<ChainId, MescError> {
                    Ok(ChainId::from(self))
                }
            }
        )*
    };
}

impl_try_into_chain_id_for_integer!(u8, u16, u32, u64, usize);

impl TryIntoChainId for &[u8] {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        Err(MescError::NotImplemented("binary chain_id".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
struct ChainIdSerde(String);

impl From<ChainId> for ChainIdSerde {
    fn from(chain_id: ChainId) -> ChainIdSerde {
        ChainIdSerde(chain_id.to_string())
    }
}

impl TryFrom<ChainIdSerde> for ChainId {
    type Error = MescError;

    fn try_from(value: ChainIdSerde) -> Result<ChainId, MescError> {
        value.0.try_into_chain_id()
    }
}
