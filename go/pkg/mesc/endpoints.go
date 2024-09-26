package mesc

import (
	"context"
	"errors"
	"fmt"

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
	resolutionConfig := resolveEndpointResolutionConfig(options...)

	rpcConfig, hasConfig := resolutionConfig.GetRPCConfig()
	if !hasConfig {
		resolvedRPCConfig, err := ResolveRPCConfig(ctx)
		if err != nil {
			return nil, fmt.Errorf("failed to resolve RPC configuration: %w", err)
		}

		rpcConfig = *resolvedRPCConfig
	}

	var defaultEndpointName string

	profileName, hasProfileName := resolutionConfig.GetProfile()
	if hasProfileName {
		profile, hasProfile := rpcConfig.Profiles[profileName]
		if hasProfile && profile.DefaultEndpoint != nil {
			defaultEndpointName = *profile.DefaultEndpoint
		}
	}

	if defaultEndpointName == "" && rpcConfig.DefaultEndpoint != nil {
		defaultEndpointName = *rpcConfig.DefaultEndpoint
	}

	if defaultEndpointName == "" {
		// unable to resolve default endpoint name to use, so nothing can be found
		return nil, nil
	}

	endpoint, hasEndpoint := rpcConfig.Endpoints[defaultEndpointName]
	if !hasEndpoint {
		return nil, nil
	}

	return &endpoint, nil
}

// GetEndpointByNetwork resolves the endpoint metadata, if available, for the given chain ID.
// This will return nil if no endpoint metadata can be resolved.
func GetEndpointByNetwork(ctx context.Context, chainID model.ChainID, options ...resolution.EndpointResolutionOption) (*model.EndpointMetadata, error) {
	// TODO: implement
	return nil, errors.New("not yet implemented")
}

// GetEndpointByName resolves the endpoint metadata, if available, for the network for the given name.
// This will return nil if no endpoint metadata can be resolved.
// If the given RPC config is non-nil, it will be used to determine the endpoint.
// If the given RPC config is nil, then the RPC config will be resolved according to the MESC specification.
func GetEndpointByName(ctx context.Context, name string, rpcConfig *model.RPCConfig) (*model.EndpointMetadata, error) {
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

func resolveEndpointResolutionConfig(options ...resolution.EndpointResolutionOption) *resolution.EndpointResolutionConfig {
	cfg := &resolution.EndpointResolutionConfig{}
	for _, option := range options {
		option(cfg)
	}

	return cfg
}
