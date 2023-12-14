use crate::{ChainId, MescError, TryIntoChainId};

#[derive(Debug, Default, Clone)]
pub struct EndpointQuery {
    pub chain_id: Option<ChainId>,
    pub name_contains: Option<String>,
    pub url_contains: Option<String>,
}

impl EndpointQuery {
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
