package model

const (
	apiKeyKey                         = "api_key"
	apiKeysKey                        = "api_keys"
	cloudRegionKey                    = "cloud_region"
	concealKey                        = "conceal"
	creationTimeKey                   = "creation_time"
	ecosystemKey                      = "ecosystem"
	explorerKey                       = "explorer"
	groupsKey                         = "groups"
	hostKey                           = "host"
	jwtSecretkey                      = "jwt_secret"
	labelsKey                         = "labels"
	lastModifiedByKey                 = "last_modified_by"
	lastModifiedTimeKey               = "last_modified_time"
	locationKey                       = "location"
	namespacesKey                     = "namespaces"
	nodeClientKey                     = "node_client"
	rateLimitComputeUnitsPerSecondKey = "rate_limit_cups"
	rateLimitPerMethodKey             = "rate_limit_per_method"
	rateLimitRequestsPerSecondKey     = "rate_limit_rps"
)

func getMetadataAny(parameterName string, metadata map[string]any) (any, bool) {
	if metadata == nil {
		return nil, false
	}

	metadatumAny, hasMetadatum := metadata[parameterName]
	return metadatumAny, hasMetadatum
}

func getMetadataFloat64(parameterName string, metadata map[string]any) (float64, bool) {
	metadatumAny, hasMetadatum := getMetadataAny(parameterName, metadata)
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
	metadatumAny, hasMetadatum := getMetadataAny(parameterName, metadata)
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
	metadatumAny, hasMetadatum := getMetadataAny(parameterName, metadata)
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
