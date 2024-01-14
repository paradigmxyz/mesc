package mesc

import model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"

// EndpointResolutionConfig describes configuration and context to be applied during endpoint resolution.
type EndpointResolutionConfig struct {
    profile   *string
    rpcConfig *model.RPCConfig
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
