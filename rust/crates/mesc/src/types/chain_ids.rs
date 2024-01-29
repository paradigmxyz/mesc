use crate::MescError;
use serde::{Deserialize, Serialize};

/// ChainId is a string representation of an integer chain id
/// - TryFrom conversions allow specifying as String, &str, uint, or binary data
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ChainId(String);

impl ChainId {
    /// convert to hex representation
    pub fn to_hex(&self) -> Result<String, MescError> {
        let ChainId(chain_id) = self;
        if chain_id.starts_with("0x") {
            Ok(chain_id.clone())
        } else {
            match chain_id.parse::<u64>() {
                Ok(number) => Ok(format!("0x{:x}", number)),
                Err(_) => Err(MescError::IntegrityError("bad chain_id".to_string())),
            }
        }
    }

    /// convert to hex representation, zero-padded to 256 bits
    pub fn to_hex_256(&self) -> Result<String, MescError> {
        let ChainId(chain_id) = self;
        if let Some(stripped) = chain_id.strip_prefix("0x") {
            Ok(format!("0x{:0>64}", stripped))
        } else {
            match chain_id.parse::<u128>() {
                Ok(number) => Ok(format!("0x{:064x}", number)),
                Err(_) => {
                    Err(MescError::InvalidChainId("cannot convert chain_id to hex".to_string()))
                }
            }
        }
    }

    /// return chain_id as &str
    pub fn as_str(&self) -> &str {
        let ChainId(chain_id) = self;
        chain_id.as_str()
    }
}

impl std::hash::Hash for ChainId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.to_hex_256() {
            Ok(as_hex) => {
                as_hex.hash(state);
            }
            _ => {
                let ChainId(contents) = self;
                contents.hash(state);
            }
        }
    }
}

impl PartialOrd for ChainId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ChainId {
    fn eq(&self, other: &Self) -> bool {
        let self_string: String = match self.to_hex() {
            Ok(s) => s[2..].to_string(),
            Err(_) => return self == other,
        };
        let other_string = match other.to_hex() {
            Ok(s) => s[2..].to_string(),
            Err(_) => return self == other,
        };
        let self_str = format!("{:0>79}", self_string);
        let other_str = format!("{:0>79}", other_string);
        self_str.eq(&other_str)
    }
}

impl Ord for ChainId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_string: String = match self.to_hex() {
            Ok(s) => s[2..].to_string(),
            Err(_) => return std::cmp::Ordering::Greater,
        };
        let other_string = match other.to_hex() {
            Ok(s) => s[2..].to_string(),
            Err(_) => return std::cmp::Ordering::Greater,
        };
        let self_str = format!("{:>079}", self_string);
        let other_str = format!("{:>079}", other_string);
        self_str.cmp(&other_str)
    }
}

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_from_uint_for_chainid {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ChainId {
                fn from(value: $t) -> ChainId {
                    ChainId(value.to_string())
                }
            }
        )*
    };
}

impl_from_uint_for_chainid!(u8, u16, u32, u64, u128, usize);

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
        if !self.is_empty() &&
            (self.chars().all(|c| c.is_ascii_digit()) ||
                (self.starts_with("0x") && self[2..].chars().all(|c| c.is_ascii_hexdigit())))
        {
            Ok(ChainId(self))
        } else {
            Err(MescError::InvalidChainId(self))
        }
    }
}

impl TryIntoChainId for &str {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        if !self.is_empty() &&
            (self.chars().all(|c| c.is_ascii_digit()) ||
                (self.starts_with("0x") && self[2..].chars().all(|c| c.is_ascii_hexdigit())))
        {
            Ok(ChainId(self.to_string()))
        } else {
            Err(MescError::InvalidChainId(self.to_string()))
        }
    }
}

macro_rules! impl_try_into_chain_id_for_integer {
    ($($t:ty),*) => {
        $(
            impl TryIntoChainId for $t {
                fn try_into_chain_id(self) -> Result<ChainId, MescError> {
                    Ok(ChainId(self.to_string()))
                }
            }
        )*
    };
}

impl_try_into_chain_id_for_integer!(u8, u16, u32, u64, u128, usize);

impl TryIntoChainId for &[u8] {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        Err(MescError::NotImplemented("binary chain_id".to_string()))
    }
}
