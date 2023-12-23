use crate::{ChainId, MescError, TryIntoChainId};

#[derive(Debug, Default, Clone)]
pub struct MultiEndpointQuery {
    pub chain_id: Option<ChainId>,
    pub name_contains: Option<String>,
    pub url_contains: Option<String>,
}

/// builder for MultiEndpointQuery
impl MultiEndpointQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chain_id<T: TryIntoChainId>(mut self, chain_id: T) -> Result<Self, MescError> {
        self.chain_id = Some(chain_id.try_into_chain_id()?);
        Ok(self)
    }

    pub fn name<T: AsRef<str>>(mut self, query: T) -> Result<Self, MescError> {
        self.name_contains = Some(query.as_ref().to_string());
        Ok(self)
    }

    pub fn url<T: AsRef<str>>(mut self, query: T) -> Result<Self, MescError> {
        self.url_contains = Some(query.as_ref().to_string());
        Ok(self)
    }
}

//
// // individual queries
//

#[derive(Debug, Clone)]
pub struct EndpointQuery {
    pub query_type: EndpointQueryType,
    pub fields: EndpointQueryFields,
}

#[derive(Debug, Clone)]
pub enum EndpointQueryType {
    DefaultEndpoint,
    EndpointByName,
    EndpointByNetwork,
    UserInput,
}

#[derive(Debug, Clone)]
pub enum EndpointQueryFields {
    DefaultEndpoint(DefaultEndpointQuery),
    EndpointName(EndpointNameQuery),
    EndpointNetwork(EndpointNetworkQuery),
    UserInput(UserInputQuery),
}

#[derive(Debug, Clone)]
pub struct DefaultEndpointQuery {
    pub profile: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EndpointNameQuery {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct EndpointNetworkQuery {
    pub profile: Option<String>,
    pub chain_id: String,
}

#[derive(Debug, Clone)]
pub struct UserInputQuery {
    pub profile: Option<String>,
    pub user_input: String,
}

//
// // global metadata
//

#[derive(Debug, Clone)]
pub struct GlobalMetadataQuery {
    pub path: Option<Vec<String>>,
}

//
// // general MESC queries
//

#[derive(Debug, Clone)]
pub struct MescQuery {
    pub query_type: MescQueryType,
    pub fields: MescQueryFields,
}

#[derive(Debug, Clone)]
pub enum MescQueryType {
    DefaultEndpoint,
    EndpointByName,
    EndpointByNetwork,
    UserInput,
    MultiEndpoint,
    GlobalMetadata,
}

#[derive(Debug, Clone)]
pub enum MescQueryFields {
    DefaultEndpoint(DefaultEndpointQuery),
    EndpointName(EndpointNameQuery),
    EndpointNetwork(EndpointNetworkQuery),
    UserInput(UserInputQuery),
    MultiEndpoint(MultiEndpointQuery),
    GlobalMetadata(GlobalMetadataQuery),
}
