package model

import "time"

// Profile describes a profile to contextualize resolved endpoint metadata
type Profile struct {
	Name            string             // Name is the name of the profile
	DefaultEndpoint *string            // DefaultEndpoint contains the ChainID of the default network under this profile, if available
	NetworkDefaults map[ChainID]string // NetworkDefaults maps ChainID values identifying networks to the name of the default endpoint of that network
	ProfileMetadata map[string]any     // ProfileMetadata is a combination of both well-defined and custom-defined metadata for this profile
	UseMESC         bool               // UseMESC, if false, indicates that MESC resolution should not be used when this profile is enabled
}

// Conceal gets, from the profile metadata, whether tools should avoid casually revealing the RPC URLs without prompting.
func (p Profile) Conceal() (bool, bool) {
	concealAny, hasConceal := getMetadataAny(concealKey, p.ProfileMetadata)
	if !hasConceal {
		return false, false
	}

	concealBool, isBool := concealAny.(bool)
	if !isBool {
		return false, false
	}

	return concealBool, true
}

// GetAPIKeys gets from the profile metadata the provided API keys, if present.
func (p Profile) GetAPIKeys() (map[string]string, bool) {
	apiKeysAny, hasAPIKeys := getMetadataAny(apiKeysKey, p.ProfileMetadata)
	if !hasAPIKeys {
		return nil, false
	}

	apiKeysMapToAny, isMap := apiKeysAny.(map[string]any)
	if !isMap {
		return nil, false
	}

	apiKeys := make(map[string]string, len(apiKeysMapToAny))
	for serviceName, apiKeyAny := range apiKeysMapToAny {
		apiKeyString, isString := apiKeyAny.(string)
		if !isString {
			return nil, false
		}
		apiKeys[serviceName] = apiKeyString
	}

	return apiKeys, true
}

// GetEndpointNamesForGrouping gets from the profile metadata the endpoint names that are found in the given grouping, if present.
func (p Profile) GetEndpointNamesForGrouping(grouping string) ([]string, bool) {
	groupsAny, hasGroupsAny := getMetadataAny(groupsKey, p.ProfileMetadata)
	if !hasGroupsAny {
		return nil, false
	}

	groupsMap, isMap := groupsAny.(map[string]any)
	if !isMap {
		return nil, false
	}

	groupedEndpointsAny, hasGroup := groupsMap[grouping]
	if !hasGroup {
		return nil, false
	}

	groupedEndpointsSlice, isSlice := groupedEndpointsAny.([]string)
	if !isSlice {
		return nil, false
	}

	return groupedEndpointsSlice, true
}

// GetCreationTime gets from the metadata the date and time at which the profile was created, present.
func (p Profile) GetCreationTime() (time.Time, bool) {
	floatTime, hasFloat := getMetadataFloat64(creationTimeKey, p.ProfileMetadata)
	if !hasFloat {
		return time.Time{}, false
	}

	return time.Unix(int64(floatTime), 0), true
}

// GetLastModifiedBy gets from the metadata the versioned identifier of the tool used to create the configuration, if present.
func (p Profile) GetLastModifiedBy() (string, bool) {
	return getMetadataString(lastModifiedByKey, p.ProfileMetadata)
}

// GetLastModifiedTime gets from the metadata the date and time when this profile was last modified, if present.
func (p Profile) GetLastModifiedTime() (time.Time, bool) {
	floatTime, hasFloat := getMetadataFloat64(lastModifiedTimeKey, p.ProfileMetadata)
	if !hasFloat {
		return time.Time{}, false
	}

	return time.Unix(int64(floatTime), 0), true
}
