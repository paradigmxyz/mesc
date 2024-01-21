package model

// RPCConfig describes a collection of RPC configurations
type RPCConfig struct {
	MESCVersion     string                      // MESCVersion describes the version of the MESC specification to which this adheres
	DefaultEndpoint *string                     // DefaultEndpoint is the name of the default endpoint to be used; if nil, then there is no named default endpoint
	NetworkDefaults map[ChainID]string          // NetworkDefaults maps chain IDs to the names of the default network for each chain
	NetworkNames    map[string]ChainID          // NetworkNames maps network names to chain ID values
	Endpoints       map[string]EndpointMetadata // Endpoints maps endpoint metadata by endpoint name
	Profiles        map[string]Profile          // Profiles maps profiles by their profile names
	GlobalMetadata  map[string]any              // GlobalMetadata contains metadata relevant across all profiles
}
