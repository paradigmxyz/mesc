package serialization

import (
	"bytes"
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

// DeserializeEndpointMetadataJSON deserializes the given representation of endpoint metadata mapped by endpoint name.
func DeserializeEndpointMetadataJSON(reader io.Reader) (map[string]model.EndpointMetadata, error) {
	var endpointJSONMetadata map[string]*jsonEndpoint
	if decodeErr := json.NewDecoder(reader).Decode(&endpointJSONMetadata); decodeErr != nil {
		return nil, fmt.Errorf("failed to decode endpoint metadata JSON: %w", decodeErr)
	}

	endpoints := make(map[string]model.EndpointMetadata, len(endpointJSONMetadata))
	for endpointName, endpointJSON := range endpointJSONMetadata {
		endpoints[endpointName] = endpointJSON.toModel()
	}

	return endpoints, nil
}

// SerializeJSON serializes the given RPC configuration to a JSON representation conforming to the MESC specification.
func SerializeJSON(rpcConfig *model.RPCConfig) (io.Reader, error) {
	jsonProfiles := make(map[string]*jsonProfile)
	for profileKey, profile := range jsonProfiles {
		jsonProfiles[profileKey] = &jsonProfile{
			Name:            profile.Name,
			DefaultEndpoint: profile.DefaultEndpoint,
			NetworkDefaults: profile.NetworkDefaults,
			ProfileMetadata: profile.ProfileMetadata,
			UseMESC:         profile.UseMESC,
		}
	}

	jsonEndpoints := make(map[string]*jsonEndpoint)
	for endpointKey, endpoint := range rpcConfig.Endpoints {
		jsonEndpoints[endpointKey] = modelEndpointToJSON(&endpoint)
	}

	jsonRPCConfig := &jsonRPCConfig{
		MESCVersion:     rpcConfig.MESCVersion,
		DefaultEndpoint: rpcConfig.DefaultEndpoint,
		NetworkDefaults: fromChainIDMap(rpcConfig.NetworkDefaults),
		NetworkNames:    fromMappedChainID(rpcConfig.NetworkNames),
		Endpoints:       jsonEndpoints,
		Profiles:        jsonProfiles,
		GlobalMetadata:  rpcConfig.GlobalMetadata,
	}

	jsonBytes, err := json.Marshal(jsonRPCConfig)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal RPC config to bytes: %w", err)
	}

	return bytes.NewBuffer(jsonBytes), nil
}

// SerializeEndpointMetadataJSON serializes the given endpoint model to a JSON form compliant with the MESC specification.
func SerializeEndpointMetadataJSON(endpoint *model.EndpointMetadata) (io.Reader, error) {
	jsonEndpoint := modelEndpointToJSON(endpoint)

	jsonBytes, err := json.Marshal(jsonEndpoint)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal endpoint metadata to bytes: %w", err)
	}

	return bytes.NewBuffer(jsonBytes), nil
}

func toOptionalChainID(v *string) *model.ChainID {
	if v == nil {
		return nil
	}

	asModel := model.ChainID(*v)
	return &asModel
}

func fromChainIDMap[V any](source map[model.ChainID]V) map[string]V {
	if source == nil {
		return nil
	}

	copy := make(map[string]V, len(source))
	for chainID, value := range source {
		copy[string(chainID)] = value
	}

	return copy
}

func fromMappedChainID[K comparable](source map[K]model.ChainID) map[K]string {
	if source == nil {
		return nil
	}

	copy := make(map[K]string, len(source))
	for key, chainID := range source {
		copy[key] = string(chainID)
	}

	return copy
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
	MESCVersion     string                   `json:"mesc_version"`
	DefaultEndpoint *string                  `json:"default_endpoint"`
	NetworkDefaults map[string]string        `json:"network_defaults"`
	NetworkNames    map[string]string        `json:"network_names"`
	Endpoints       map[string]*jsonEndpoint `json:"endpoints"`
	Profiles        map[string]*jsonProfile  `json:"profiles"`
	GlobalMetadata  map[string]any           `json:"global_metadata"`
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

func modelEndpointToJSON(endpoint *model.EndpointMetadata) *jsonEndpoint {
	var chainIDPtr *string
	if endpoint.ChainID != nil {
		v := string(*endpoint.ChainID)
		chainIDPtr = &v
	}

	return &jsonEndpoint{
		Name:             endpoint.Name,
		URL:              endpoint.URL,
		ChainID:          chainIDPtr,
		EndpointMetadata: endpoint.EndpointMetadata,
	}
}

type jsonEndpoint struct {
	Name             string         `json:"name"`
	URL              string         `json:"url"`
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
	Name            string            `json:"name"`
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
