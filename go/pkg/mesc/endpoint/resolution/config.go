package mesc

import "github.com/paradigmxyz/mesc/go/pkg/mesc/model"

// EndpointResolutionConfig describes configuration and context to be applied during endpoint resolution.
type EndpointResolutionConfig struct {
	profile   *string
	rpcConfig *model.RPCConfig
}

// GetProfile gets the profile, if any, that was supplied.
// It returns true for the bool if there is a profile supplied; false if not.
func (e *EndpointResolutionConfig) GetProfile() (string, bool) {
	if e.profile == nil {
		return "", false
	}

	return *e.profile, true
}

// GetRPCConfig gets the RPC configuration, if any, that was supplied.
// It returns false for the bool if there is an RPC configuration supplied; false if not.
func (e *EndpointResolutionConfig) GetRPCConfig() (model.RPCConfig, bool) {
	if e.rpcConfig == nil {
		return model.RPCConfig{}, false
	}

	return *e.rpcConfig, true
}

// EndpointResolutionOption describes a way of configuring the resolution of an endpoint
type EndpointResolutionOption func(*EndpointResolutionConfig)

// WithProfile will configure endpoint resolution to resolve with the given profile enabled.
func WithProfile(profile string) EndpointResolutionOption {
	return func(cfg *EndpointResolutionConfig) {
		cfg.profile = &profile
	}
}

// WithRPCConfig will configure endpoint resolution to resolve from the given RPCConfig.
func WithRPCConfig(rpcConfig model.RPCConfig) EndpointResolutionOption {
	return func(cfg *EndpointResolutionConfig) {
		cfg.rpcConfig = &rpcConfig
	}
}
