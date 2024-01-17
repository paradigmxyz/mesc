package mesc

import (
	"context"
	"errors"

	criteria "github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/criteria"
	resolution "github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/resolution"
	model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

// FindEndpoints will find all endpoint metadata matching the given criteria.
// If no criteria is provided, then all endpoints will be returned.
// If no endpoints are found, then an empty slice of EndpointMetadata will be returned.
func FindEndpoints(ctx context.Context, findCriteria []criteria.FindEndpointsCriteria, options ...resolution.EndpointResolutionOption) ([]*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not implemented")
}

// GetDefaultEndpoint resolves the endpoint metadata, if available, for the default endpoint.
// This will return nil if no endpoint metadata can be resolved.
func GetDefaultEndpoint(ctx context.Context, options ...resolution.EndpointResolutionOption) (*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not yet implemented")
}

// GetEndpointByNetwork resolves the endpoint metadata, if available, for the given chain ID.
// This will return nil if no endpoint metadata can be resolved.
func GetEndpointByNetwork(ctx context.Context, chainID model.ChainID, options ...resolution.EndpointResolutionOption) (*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not yet implemented")
}

// GetEndpointByName resolves the endpoint metadata, if available, for the network for the given name.
// This will return nil if no endpoint metadata can be resolved.
func GetEndpointByName(ctx context.Context, name string, options ...resolution.EndpointResolutionOption) (*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not yet implemented")
}

// GetEndpointsByQuery resolves the endpoint metadata, if found, for the given query.
// This will return nil if no endpoint metadata can be resolved.
func GetEndpointsByQuery(ctx context.Context, query string, options ...resolution.EndpointResolutionOption) (*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not yet implemented")
}

// IsMESCEnabled determines if MESC is enabled.
func IsMESCEnabled(ctx context.Context) (bool, error) {
	// TODO: implement
	return false, errors.New("not yet implemented")
}
