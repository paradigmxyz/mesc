package mesc

import model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"

// EndpointFindCriteriaConfig describes the criteria to be used with FindEndpoints
type EndpointFindCriteriaConfig struct {
    chainID      *model.ChainID
    nameContains *string
    urlContains  *string
}

// FindEndpointsCriteria provides a means of customizing the finding criteria for endpoints in FindEndpoints.
type FindEndpointsCriteria func(*EndpointFindCriteriaConfig)

// HasChainID will find endpoints that contain endpoint metadata for the given chain ID.
func HasChainID(chainID model.ChainID) FindEndpointsCriteria {
    return func(e *EndpointFindCriteriaConfig) {
        e.chainID = &chainID
    }
}

// NameContains describes a criteria to find endpoints whose name contains the given substring.
func NameContains(name string) FindEndpointsCriteria {
    return func(e *EndpointFindCriteriaConfig) {
        e.nameContains = &name
    }
}

// URLContains describes a criteria to find endpoints whose URLs contain the given substring.
func URLContains(url string) FindEndpointsCriteria {
    return func(e *EndpointFindCriteriaConfig) {
        e.urlContains = &url
    }
}
