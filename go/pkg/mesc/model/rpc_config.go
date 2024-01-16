package mesc

// RPCConfig describes a collection of RPC configurations
type RPCConfig struct {
    MESCVersion     string                      // MESCVersion describes the version of the MESC specification to which this adheres
    DefaultEndpoint string                      // DefaultEndpoint is the name of the default endpoint to be used
    NetworkDefaults map[ChainID]string          // NetworkDefaults maps chain IDs to the names of the default network for each chain
    Endpoints       map[string]EndpointMetadata // Endpoints maps endpoint metadata by endpoint name
    Profiles        map[string]Profile          // Profiles maps profiles by their profile names
    GlobalMetadata  map[string]any              // GlobalMetadata contains metadata relevant across all profiles
}

// TODO: implement helper methods resolving global metadata, per the specification here:
// https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#metadata
