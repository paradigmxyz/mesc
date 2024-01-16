package mesc

const (
    rateLimitRequestsPerSecondKey = "rate_limit_rps"
)

// EndpointMetadata describes metadata for an RPC endpoint.
type EndpointMetadata struct {
    Name             string         // Name is the name of the endpoint
    URL              string         // URL is the location of the endpoint
    ChainID          *ChainID       // ChainID is the chain ID of the string
    EndpointMetadata map[string]any // EndpointMetadata is a map of both well-defined endpoint metadata and custom metadata
}

// GetRateLimitPerSecond gets the rate limit of requests per second that the endpoint will allow.
func (e EndpointMetadata) GetRateLimitPerSecond() (float64, bool) {
    if e.EndpointMetadata == nil {
        return 0.0, false
    }

    rpsAny, hasRPS := e.EndpointMetadata[rateLimitRequestsPerSecondKey]
    if !hasRPS {
        return 0.0, false
    }

    rpsFloat64, isFloat := rpsAny.(float64)
    if isFloat {
        return rpsFloat64, true
    }

    return 0.0, false
}

// TODO: implement other convenience methods for the other well-defined parameters here:
// https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#metadata
