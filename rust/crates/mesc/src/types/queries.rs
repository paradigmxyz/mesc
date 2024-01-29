use crate::{ChainId, MescError, TryIntoChainId};

/// Multi Endpoint Query
#[derive(Debug, Default, Clone)]
pub struct MultiEndpointQuery {
    /// chain_id
    pub chain_id: Option<ChainId>,
    /// name_contains
    pub name_contains: Option<String>,
    /// url_contains
    pub url_contains: Option<String>,
}

/// builder for MultiEndpointQuery
impl MultiEndpointQuery {
    /// new MultiEndpointQuery
    pub fn new() -> Self {
        Self::default()
    }

    /// set chain_id
    pub fn chain_id<T: TryIntoChainId>(mut self, chain_id: T) -> Result<Self, MescError> {
        self.chain_id = Some(chain_id.try_into_chain_id()?);
        Ok(self)
    }

    /// set name
    pub fn name<T: AsRef<str>>(mut self, query: T) -> Result<Self, MescError> {
        self.name_contains = Some(query.as_ref().to_string());
        Ok(self)
    }

    /// set url
    pub fn url<T: AsRef<str>>(mut self, query: T) -> Result<Self, MescError> {
        self.url_contains = Some(query.as_ref().to_string());
        Ok(self)
    }
}

//
// // individual queries
//

/// EndpointQuery
#[derive(Debug, Clone)]
pub struct EndpointQuery {
    /// query_type
    pub query_type: EndpointQueryType,
    /// fields
    pub fields: EndpointQueryFields,
}

/// EndpointQueryType
#[derive(Debug, Clone)]
pub enum EndpointQueryType {
    /// DefeaultEndpoint
    DefaultEndpoint,
    /// EndpointByName
    EndpointByName,
    /// EndpointByNetwork
    EndpointByNetwork,
    /// UserInput
    UserInput,
}

/// EndpointQueryFields
#[derive(Debug, Clone)]
pub enum EndpointQueryFields {
    /// DefaultEndpoint
    DefaultEndpoint(DefaultEndpointQuery),
    /// EndpointName
    EndpointName(EndpointNameQuery),
    /// EndpointNetwork
    EndpointNetwork(EndpointNetworkQuery),
    /// UserInput
    UserInput(UserInputQuery),
}

/// DefaultEndpointQuery
#[derive(Debug, Clone)]
pub struct DefaultEndpointQuery {
    /// profile
    pub profile: Option<String>,
}

/// EndpointNameQuery
#[derive(Debug, Clone)]
pub struct EndpointNameQuery {
    /// name
    pub name: String,
}

/// EndpointNetworkQuery
#[derive(Debug, Clone)]
pub struct EndpointNetworkQuery {
    /// profile
    pub profile: Option<String>,
    /// chain_id
    pub chain_id: String,
}

/// UserInputQuery
#[derive(Debug, Clone)]
pub struct UserInputQuery {
    /// profile
    pub profile: Option<String>,
    /// user_input
    pub user_input: String,
}

//
// // global metadata
//

/// GlobalMetadataQuery
#[derive(Debug, Clone)]
pub struct GlobalMetadataQuery {
    /// profile
    pub profile: Option<String>,
    /// path
    pub path: Option<Vec<String>>,
}

//
// // general MESC queries
//

/// MescQuery
#[derive(Debug, Clone)]
pub struct MescQuery {
    /// query_type
    pub query_type: MescQueryType,
    /// field
    pub fields: MescQueryFields,
}

/// MescQueryType
#[derive(Debug, Clone)]
pub enum MescQueryType {
    /// DefaultEndpoint
    DefaultEndpoint,
    /// EndpointByName
    EndpointByName,
    /// EndpointByNetwork
    EndpointByNetwork,
    /// UserInput
    UserInput,
    /// MultiEndpoint
    MultiEndpoint,
    /// GlobalMetadata
    GlobalMetadata,
}

/// MescQueryFields
#[derive(Debug, Clone)]
pub enum MescQueryFields {
    /// DefaultEndpoint
    DefaultEndpoint(DefaultEndpointQuery),
    /// EndpointName
    EndpointName(EndpointNameQuery),
    /// EndpointNetwork
    EndpointNetwork(EndpointNetworkQuery),
    /// UserInput
    UserInput(UserInputQuery),
    /// MultiEndpoint
    MultiEndpoint(MultiEndpointQuery),
    /// GlobalMetadata
    GlobalMetadata(GlobalMetadataQuery),
}
