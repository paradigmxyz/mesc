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

pub struct EndpointQuery {
    pub query_type: EndpointQueryType,
    pub fields: QueryFields,
}

pub enum EndpointQueryType {
    DefaultEndpoint,
    EndpointByName,
    EndpointByNetwork,
    UserInputQuery,
}

pub enum QueryFields {
    DefaultEndpointQuery(DefaultEndpointQuery),
    EndpointNameQuery(EndpointNameQuery),
    EndpointNetworkQuery(EndpointNetworkQuery),
    UserInputQuery(UserInputQuery),
}

pub struct DefaultEndpointQuery {
    pub profile: Option<String>,
}

pub struct EndpointNameQuery {
    pub profile: Option<String>,
    pub name: String,
}

pub struct EndpointNetworkQuery {
    pub profile: Option<String>,
    pub chain_id: String,
}

pub struct UserInputQuery {
    pub profile: Option<String>,
    pub user_input: String,
}
