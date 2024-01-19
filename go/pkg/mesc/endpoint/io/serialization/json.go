package serialization

import (
	"encoding/json"
	"fmt"
	"io"

	"github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

// DeserializeJSON deserializes the JSON contained within the given io.Reader into a model.RPCConfig object.
// This does not apply any validations prescribed by the MESC specification.
func DeserializeJSON(reader io.Reader) (*model.RPCConfig, error) {
	var jsonConfig *jsonRPCConfig
	if decodeErr := json.NewDecoder(reader).Decode(&jsonConfig); decodeErr != nil {
		return nil, fmt.Errorf("failed to decode MESC JSON: %w", decodeErr)
	}

	return jsonConfig.toModel(), nil
}

func toOptionalChainID(v *string) *model.ChainID {
	if v == nil {
		return nil
	}

	asModel := model.ChainID(*v)
	return &asModel
}

func toChainIDMap[V any](source map[string]V) map[model.ChainID]V {
	if source == nil {
		return nil
	}

	copy := make(map[model.ChainID]V, len(source))
	for chainID, value := range source {
		copy[model.ChainID(chainID)] = value
	}

	return copy
}

func toMappedChainID[K comparable](source map[K]string) map[K]model.ChainID {
	if source == nil {
		return nil
	}

	copy := make(map[K]model.ChainID, len(source))
	for key, chainID := range source {
		copy[key] = model.ChainID(chainID)
	}

	return copy
}

type jsonRPCConfig struct {
	MESCVersion     string            `json:"mesc_version"`
	DefaultEndpoint *string           `json:"default_endpoint"`
	NetworkDefaults map[string]string `json:"network_defaults"`
	NetworkNames    map[string]string `json:"network_names"`
	Endpoints       map[string]*jsonEndpoint
	Profiles        map[string]*jsonProfile
	GlobalMetadata  map[string]any `json:"global_metadata"`
}

func (j *jsonRPCConfig) toModel() *model.RPCConfig {
	networkDefaults := toChainIDMap(j.NetworkDefaults)
	networkNames := toMappedChainID(j.NetworkNames)

	rpcConfig := &model.RPCConfig{
		MESCVersion:     j.MESCVersion,
		DefaultEndpoint: j.DefaultEndpoint,
		NetworkDefaults: networkDefaults,
		NetworkNames:    networkNames,
		GlobalMetadata:  j.GlobalMetadata,
	}

	if jsonEndpoints := j.Endpoints; jsonEndpoints != nil {
		modelEndpoints := make(map[string]model.EndpointMetadata, len(jsonEndpoints))
		for endpointName, jsonEndpoint := range jsonEndpoints {
			modelEndpoints[endpointName] = jsonEndpoint.toModel()
		}
		rpcConfig.Endpoints = modelEndpoints
	}

	if jsonProfiles := j.Profiles; jsonProfiles != nil {
		modelProfiles := make(map[string]model.Profile, len(jsonProfiles))
		for profileName, jsonProfile := range jsonProfiles {
			modelProfiles[profileName] = jsonProfile.toModel()
		}
		rpcConfig.Profiles = modelProfiles
	}

	return rpcConfig
}

type jsonEndpoint struct {
	Name             string
	URL              string
	ChainID          *string        `json:"chain_id"`
	EndpointMetadata map[string]any `json:"endpoint_metadata"`
}

func (j *jsonEndpoint) toModel() model.EndpointMetadata {
	return model.EndpointMetadata{
		Name:             j.Name,
		URL:              j.URL,
		ChainID:          toOptionalChainID(j.ChainID),
		EndpointMetadata: j.EndpointMetadata,
	}
}

type jsonProfile struct {
	Name            string
	DefaultEndpoint *string           `json:"default_endpoint"`
	NetworkDefaults map[string]string `json:"network_defaults"`
	ProfileMetadata map[string]any    `json:"profile_metadata"`
	UseMESC         bool              `json:"use_mesc"`
}

func (j *jsonProfile) toModel() model.Profile {
	return model.Profile{
		Name:            j.Name,
		DefaultEndpoint: j.DefaultEndpoint,
		NetworkDefaults: toChainIDMap(j.NetworkDefaults),
		ProfileMetadata: j.ProfileMetadata,
		UseMESC:         j.UseMESC,
	}
}
