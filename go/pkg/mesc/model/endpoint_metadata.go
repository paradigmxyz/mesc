package model

// EndpointMetadata describes metadata for an RPC endpoint.
type EndpointMetadata struct {
	Name             string         // Name is the name of the endpoint
	URL              string         // URL is the location of the endpoint
	ChainID          *ChainID       // ChainID is the chain ID of the string
	EndpointMetadata map[string]any // EndpointMetadata is a map of both well-defined endpoint metadata and custom metadata
}

// GetAPIKey gets the API key in the endpoint metadata, if present.
func (e EndpointMetadata) GetAPIKey() (string, bool) {
	return getMetadataString(apiKeyKey, e.EndpointMetadata)
}

// GetCloudRegion gets the cloud region from the endpoint metadata, if present.
func (e EndpointMetadata) GetCloudRegion() (string, bool) {
	return getMetadataString(cloudRegionKey, e.EndpointMetadata)
}

// GetEcosystem gets the ecosystem in the endpoint metadata, if present.
func (e EndpointMetadata) GetEcosystem() (string, bool) {
	return getMetadataString(ecosystemKey, e.EndpointMetadata)
}

// GetExplorer gets the URL of the explorer from the endpoint metadata, if present.
func (e EndpointMetadata) GetExplorer() (string, bool) {
	return getMetadataString(explorerKey, e.EndpointMetadata)
}

// GetHost gets the host from the endpoint metadata, if present.
func (e EndpointMetadata) GetHost() (string, bool) {
	return getMetadataString(hostKey, e.EndpointMetadata)
}

// GetJWTSecret gets the JWT secret from the endpoint metadata, if present.
func (e EndpointMetadata) GetJWTSecret() (string, bool) {
	return getMetadataString(jwtSecretkey, e.EndpointMetadata)
}

// GetLabels gets the labels from the endpoint metadata, if present.
func (e EndpointMetadata) GetLabels() ([]string, bool) {
	return getMetadataStringSlice(labelsKey, e.EndpointMetadata)
}

// GetLocation gets the location from the endpoint metadata, if present.
func (e EndpointMetadata) GetLocation() (string, bool) {
	return getMetadataString(locationKey, e.EndpointMetadata)
}

// GetMethodRateLimit gets the rate limit, if present, of requests per second for the given RPC method name.
func (e EndpointMetadata) GetMethodRateLimit(methodName string) (float64, bool) {
	rateLimitPerMethodAny, hasRateLimits := getMetdataAny(rateLimitPerMethodKey, e.EndpointMetadata)
	if !hasRateLimits {
		return 0.0, false
	}

	rateLimitPerMethodMap, isMap := rateLimitPerMethodAny.(map[string]any)
	if !isMap {
		return 0.0, false
	}

	rateLimitAny, hasRateLimit := rateLimitPerMethodMap[methodName]
	if !hasRateLimit {
		return 0.0, false
	}

	rateLimitFloat, isFloat := rateLimitAny.(float64)
	if !isFloat {
		return 0.0, false
	}

	return rateLimitFloat, true
}

// GetNamespaces gets the method namespaces defined in the endpoint metadata, if present.
func (e EndpointMetadata) GetNamespaces() ([]string, bool) {
	return getMetadataStringSlice(namespacesKey, e.EndpointMetadata)
}

// GetNodeClient gets the node client information in the metadata, if present.
func (e EndpointMetadata) GetNodeClient() (string, bool) {
	return getMetadataString(nodeClientKey, e.EndpointMetadata)
}

// GetRateLimitCUPS gets the rate limit in compute units per second that the endpoint will allow.
func (e EndpointMetadata) GetRateLimitCUPS() (float64, bool) {
	return getMetadataFloat64(rateLimitComputeUnitsPerSecondKey, e.EndpointMetadata)
}

// GetRateLimitRPS gets the rate limit of requests per second that the endpoint will allow.
func (e EndpointMetadata) GetRateLimitRPS() (float64, bool) {
	return getMetadataFloat64(rateLimitRequestsPerSecondKey, e.EndpointMetadata)
}
