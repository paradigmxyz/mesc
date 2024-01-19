package model

const (
	apiKeyKey                         = "api_key"
	cloudRegionKey                    = "cloud_region"
	ecosystemKey                      = "ecosystem"
	explorerKey                       = "explorer"
	hostKey                           = "host"
	jwtSecretkey                      = "jwt_secret"
	labelsKey                         = "labels"
	locationKey                       = "location"
	namespacesKey                     = "namespaces"
	nodeClientKey                     = "node_client"
	rateLimitComputeUnitsPerSecondKey = "rate_limit_cups"
	rateLimitPerMethodKey             = "rate_limit_per_method"
	rateLimitRequestsPerSecondKey     = "rate_limit_rps"
)

func getMetdataAny(parameterName string, metadata map[string]any) (any, bool) {
	if metadata == nil {
		return nil, false
	}

	metadatumAny, hasMetadatum := metadata[parameterName]
	return metadatumAny, hasMetadatum
}

func getMetadataFloat64(parameterName string, metadata map[string]any) (float64, bool) {
	metadatumAny, hasMetadatum := getMetdataAny(parameterName, metadata)
	if !hasMetadatum {
		return 0.0, false
	}

	metadatumFloat, isFloat := metadatumAny.(float64)
	if !isFloat {
		return 0.0, false
	}

	return metadatumFloat, true
}

func getMetadataString(parameterName string, metadata map[string]any) (string, bool) {
	metadatumAny, hasMetadatum := getMetdataAny(parameterName, metadata)
	if !hasMetadatum {
		return "", false
	}

	metadatumString, isString := metadatumAny.(string)
	if !isString {
		return "", false
	}

	return metadatumString, true
}

func getMetadataStringSlice(parameterName string, metadata map[string]any) ([]string, bool) {
	metadatumAny, hasMetadatum := getMetdataAny(parameterName, metadata)
	if !hasMetadatum {
		return nil, false
	}

	metadatumAnySlice, isAnySlice := metadatumAny.([]any)
	if !isAnySlice {
		return nil, false
	}

	stringSlice := make([]string, len(metadatumAnySlice))
	for metadatumIndex, metadatumAny := range metadatumAnySlice {
		metadatumString, isString := metadatumAny.(string)
		if !isString {
			return nil, false
		}
		stringSlice[metadatumIndex] = metadatumString
	}
	return stringSlice, true
}
