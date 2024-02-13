package mesc

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"os"
	"strconv"
	"strings"

	"github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/io/serialization"
	model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

// ResolveRPCConfig resolves an RPC configuration per the MESC specification rules.
func ResolveRPCConfig(ctx context.Context) (*model.RPCConfig, error) {
	byMode, hasByMode, err := resolveFromMode()
	if err != nil {
		return nil, fmt.Errorf("failed to resolve RPC configuration by mode: %w", err)
	} else if hasByMode {
		return applyOverrides(byMode)
	}

	byPath, hasByPath, err := readRPCConfigFile()
	if err != nil {
		return nil, fmt.Errorf("failed to read RPC configuration from file: %w", err)
	} else if hasByPath {
		return applyOverrides(byPath)
	}

	byEnv, hasByEnv, err := readRPCConfigEnv()
	if err != nil {
		return nil, fmt.Errorf("failed to read RPC configuration from env: %w", err)
	} else if hasByEnv {
		return applyOverrides(byEnv)
	}

	return nil, fmt.Errorf("unable to resolve MESC configuration")
}

func applyEndpointMetadataOverrides(rpcConfig *model.RPCConfig) error {
	endpointMetadataOverrideJSON := os.Getenv("MESC_ENDPOINT_METADATA")
	if endpointMetadataOverrideJSON == "" {
		return nil
	}

	endpointMetadataOverrides, err := serialization.DeserializeEndpointMetadataJSON(bytes.NewBufferString(endpointMetadataOverrideJSON))
	if err != nil {
		return fmt.Errorf("failed to deserialize endpoint metadata overrides: %w", err)
	}

	if rpcConfig.Endpoints == nil {
		rpcConfig.Endpoints = make(map[string]model.EndpointMetadata)
	}

	for endpointName, endpointMetadata := range endpointMetadataOverrides {
		rpcConfig.Endpoints[endpointName] = endpointMetadata
	}

	return nil
}

func applyEndpointOverrides(rpcConfig *model.RPCConfig) error {
	endpointOverrides := os.Getenv("MESC_ENDPOINTS")
	if endpointOverrides == "" {
		return nil
	}

	endpoints := make(map[string]model.EndpointMetadata)
	for _, endpoint := range strings.Split(endpointOverrides, " ") {
		splitEndpoint := strings.Split(endpoint, "=")
		if len(splitEndpoint) != 2 {
			return fmt.Errorf("invalid endpoint override: '%s'", endpoint)
		}

		endpoint := model.EndpointMetadata{}
		endpointKey := splitEndpoint[0]
		if strings.Contains(endpointKey, ":") {
			splitKey := strings.Split(endpointKey, ":")
			endpoint.Name = splitKey[0]
			endpointKey = splitKey[0]
			chainID := model.ChainID(splitKey[1])
			endpoint.ChainID = &chainID
		} else {
			endpoint.Name = splitEndpoint[0]
		}

		endpoint.URL = splitEndpoint[1]

		endpoints[endpointKey] = endpoint
	}
	rpcConfig.Endpoints = endpoints

	return nil
}

func applyGlobalMetadataOverride(rpcConfig *model.RPCConfig) error {
	globalMetadataOverride := os.Getenv("MESC_GLOBAL_METADATA")
	if globalMetadataOverride == "" {
		return nil
	}

	var globalMetadataOverrides map[string]any
	if unmarshalErr := json.Unmarshal([]byte(globalMetadataOverride), &globalMetadataOverrides); unmarshalErr != nil {
		return fmt.Errorf("failed to unmarshal global metadata override: %w", unmarshalErr)
	}

	if rpcConfig.GlobalMetadata == nil {
		rpcConfig.GlobalMetadata = make(map[string]any)
	}

	for overrideName, overrideValue := range globalMetadataOverrides {
		rpcConfig.GlobalMetadata[overrideName] = overrideValue
	}

	return nil
}

func applyNetworkDefaultsOverride(rpcConfig *model.RPCConfig) error {
	networkDefaultsOverride := os.Getenv("MESC_NETWORK_DEFAULTS")
	if networkDefaultsOverride == "" {
		return nil
	}

	networkDefaults := make(map[model.ChainID]string)
	for _, networkDefault := range strings.Split(networkDefaultsOverride, " ") {
		splitNetworkDefault := strings.Split(networkDefault, "=")
		if len(splitNetworkDefault) != 2 {
			return fmt.Errorf("invalid network default override: '%s'", networkDefault)
		}

		networkDefaults[model.ChainID(splitNetworkDefault[0])] = splitNetworkDefault[1]
	}
	rpcConfig.NetworkDefaults = networkDefaults

	return nil
}

func applyNetworkNamesOverride(rpcConfig *model.RPCConfig) error {
	networkNameOverride := os.Getenv("MESC_NETWORK_NAMES")
	if networkNameOverride == "" {
		return nil
	}

	networkNames := make(map[string]model.ChainID)
	for _, networkName := range strings.Split(networkNameOverride, " ") {
		splitNetworkName := strings.Split(networkName, "=")
		if len(splitNetworkName) != 2 {
			return fmt.Errorf("invalid network name overide: '%s'", networkName)
		}

		networkNames[splitNetworkName[0]] = model.ChainID(splitNetworkName[1])
	}
	rpcConfig.NetworkNames = networkNames

	return nil
}

func applyOverrides(rpcConfig *model.RPCConfig) (*model.RPCConfig, error) {
	if err := applyNetworkDefaultsOverride(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed to apply network defaults overrides: %w", err)
	}

	if err := applyNetworkNamesOverride(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed to apply network names overrides: %w", err)
	}

	if err := applyEndpointOverrides(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed apply endpoint overrides: %w", err)
	}

	if err := applyProfileOverrides(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed to apply profile overrides: %w", err)
	}

	if err := applyGlobalMetadataOverride(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed to apply global metadata overrides: %w", err)
	}

	if err := applyEndpointMetadataOverrides(rpcConfig); err != nil {
		return nil, fmt.Errorf("failed to apply endpoint metadata overrides: %w", err)
	}

	if defaultEndpointOverride := os.Getenv("MESC_DEFAULT_ENDPOINT"); defaultEndpointOverride != "" {
		rpcConfig.DefaultEndpoint = &defaultEndpointOverride
		synthesizeEndpoint(rpcConfig, defaultEndpointOverride)
	}

	return rpcConfig, nil
}

func applyProfileOverrides(rpcConfig *model.RPCConfig) error {
	profileOverrides := os.Getenv("MESC_PROFILES")
	if profileOverrides == "" {
		return nil
	}

	profiles := make(map[string]*model.Profile)
	for _, profileOverride := range strings.Split(profileOverrides, " ") {
		keyValue := strings.Split(profileOverride, "=")
		if len(keyValue) != 2 {
			return fmt.Errorf("invalid profile override: '%s'", profileOverride)
		}

		keyParts := strings.Split(keyValue[0], ".")
		if len(keyParts) < 2 {
			return fmt.Errorf("invalid key in profile override key/value pair: '%s'", profileOverride)
		}

		profile, hasProfile := profiles[keyParts[0]]
		if !hasProfile {
			profile = &model.Profile{
				Name: keyParts[0],
			}
			profiles[keyParts[0]] = profile
		}

		switch keyParts[1] {
		case "default_endpoint":
			profile.DefaultEndpoint = &keyValue[1]
		case "network_defaults":
			if len(keyParts) < 3 {
				return fmt.Errorf("invalid network default setting in profile override: '%s'", profileOverride)
			}

			networkDefaults := profile.NetworkDefaults
			if networkDefaults == nil {
				networkDefaults = make(map[model.ChainID]string)
				profile.NetworkDefaults = networkDefaults
			}
			networkDefaults[model.ChainID(keyParts[2])] = keyValue[1]
		case "profile_metadata":
			if len(keyParts) < 3 {
				return fmt.Errorf("invalid profile metadata setting in profile override: '%s'", profileOverride)
			}

			profileMetadata := profile.ProfileMetadata
			if profileMetadata == nil {
				profileMetadata = make(map[string]any)
				profile.ProfileMetadata = profileMetadata
			}
			profileMetadata[keyParts[2]] = keyValue[1]
		case "use_mesc":
			parsedBool, parseErr := strconv.ParseBool(keyValue[1])
			if parseErr != nil {
				return fmt.Errorf("invalid use_mesc setting in profile override: '%s'", profileOverride)
			}

			profile.UseMESC = parsedBool
		default:
			return fmt.Errorf("unrecognized profile override setting: '%s'", profileOverride)
		}
	}

	dereffedProfiles := make(map[string]model.Profile, len(profiles))
	for profileName, profile := range profiles {
		dereffedProfiles[profileName] = *profile
	}
	rpcConfig.Profiles = dereffedProfiles

	return nil
}

func readRPCConfigBytes(jsonBytes []byte) (*model.RPCConfig, error) {
	rpcConfig, serializationErr := serialization.DeserializeJSON(bytes.NewBuffer(jsonBytes))
	if serializationErr != nil {
		return nil, fmt.Errorf("failed to deserialize JSON: %w", serializationErr)
	}

	return rpcConfig, nil
}

func readRPCConfigEnv() (*model.RPCConfig, bool, error) {
	mescJSON := os.Getenv("MESC_ENV")
	if mescJSON == "" {
		return nil, false, nil
	}

	rpcConfig, err := readRPCConfigBytes([]byte(mescJSON))
	if err != nil {
		return nil, false, fmt.Errorf("failed to unmarshal RPC config JSON from env var: %w", err)
	}

	return rpcConfig, true, nil
}

func readRPCConfigFile() (*model.RPCConfig, bool, error) {
	filePath := os.Getenv("MESC_PATH")
	if filePath == "" {
		return nil, false, nil
	}

	jsonBytes, err := os.ReadFile(filePath)
	if err != nil {
		return nil, false, fmt.Errorf("failed to read RPC configuration from file '%s': %w", filePath, err)
	}

	rpcConfig, err := readRPCConfigBytes(jsonBytes)
	if err != nil {
		return nil, false, fmt.Errorf("failed to unmarshal RPC config JSON from file '%s': %w", filePath, err)
	}

	return rpcConfig, true, nil
}

func resolveFromMode() (*model.RPCConfig, bool, error) {
	mescMode := os.Getenv("MESC_MODE")
	switch strings.TrimSpace(mescMode) {
	case "PATH":
		return readRPCConfigFile()
	case "ENV":
		return readRPCConfigEnv()
	default:
		if mescMode != "" {
			return nil, false, fmt.Errorf("invalid MESC_MODE value: '%s'", mescMode)
		}
	}

	return nil, false, nil
}

func synthesizeEndpoint(rpcConfig *model.RPCConfig, endpointName string) {
	endpoints := rpcConfig.Endpoints
	if endpoints == nil {
		endpoints = make(map[string]model.EndpointMetadata)
		rpcConfig.Endpoints = endpoints
	}

	syntheticEndpointName := endpointName
	if url, parseErr := url.Parse(endpointName); parseErr == nil {
		if host := url.Host; host != "" {
			syntheticEndpointName = host
		} else if scheme := url.Scheme; scheme != "" {
			syntheticEndpointName = scheme
		}
	}

	if _, hasEndpoint := endpoints[syntheticEndpointName]; hasEndpoint {
		return
	}

	endpoints[syntheticEndpointName] = model.EndpointMetadata{
		Name: syntheticEndpointName,
		URL:  endpointName,
	}
}
