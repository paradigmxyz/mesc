package mesc

import (
    "context"
    "errors"

    model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"
    criteria "github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/criteria"
    resolution "github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/resolution"
)

// UnresolvableEndpointError defines when an endpoint cannot be resolved.
type UnresolvableEndpointError struct{}

func (e *UnresolvableEndpointError) Error() string {
    return "no endpoint metadata could be resolved"
}

// GetDefaultEndpoint resolves the endpoint metadata, if available, for the default endpoint.
// This will return UnresolvableEndpointError if no endpoint metadata can be resolved.
func GetDefaultEndpoint(ctx context.Context, options ...resolution.EndpointResolutionOption) (model.EndpointMetadata, error) {
    // TODO: implement
    return EndpointMetadata{}, errors.New("not yet implemented")
}

// GetEndpointByNetwork resolves the endpoint metadata, if available, for the given chain ID.
// This will return UnresolvableEndpointError if no endpoint metadata can be resolved.
func GetEndpointByNetwork(ctx context.Context, chainID model.ChainID, options ..resolution..EndpointResolutionOption) (model.EndpointMetadata, error) {
    // TODO: implement
    return EndpointMetadata{}, errors.New("not yet implemented")
}

// GetEndpointByName resolves the endpoint metadata, if available, for the network for the given name.
// This will return UnresolvableEndpointError if no endpoint metadata can be resolved.
func GetEndpointByName(ctx context.Context, name string, options ...resolution.EndpointResolutionOption) (model.EndpointMetadata, error) {
    // TODO: implement
    return EndpointMetadata{}, errors.New("not yet implemented")
}

// GetEndpointsByQuery resolves the endpoint metadata, if found, for the given query.
// This will return UnresolvableEndpointError if no endpoint metadata can be resolved.
func GetEndpointsByQuery(ctx context.Context, query string, options ...resolution.EndpointResolutionOption) (model.EndpointMetadata, error) {
    // TODO: implement
    return EndpointMetadata{}, errors.New("not yet implemented")
}

// FindEndpoints will find all endpoint metadata matching the given criteria.
// If no criteria is provied, then all endpoints will be returned.
// If no endpoints are found, then an empty slice of EndpointMetadata will be returned.
func FindEndpoints(ctx context.Context, findCriteria []criteria.FindEndpointsCriteria, options ...resolution.EndpointResolutionOption) ([]model.EndpointMetadata, error) {
    // TODO: implement
    return nil, errors.New("not implemented")
}
