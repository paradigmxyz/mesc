package mesc

import (
	"bytes"
	"context"
	"fmt"
	"os"
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

func applyOverrides(rpcConfig *model.RPCConfig) (*model.RPCConfig, error) {
	if defaultEndpointOverride := os.Getenv("MESC_DEFAULT_ENDPOINT"); defaultEndpointOverride != "" {
		rpcConfig.DefaultEndpoint = &defaultEndpointOverride
	}

	if networkDefaultsOverride := os.Getenv("MESC_NETWORK_DEFAULTS"); networkDefaultsOverride != "" {
		networkDefaults := make(map[model.ChainID]string)
		for _, networkDefault := range strings.Split(networkDefaultsOverride, " ") {
			splitNetworkDefault := strings.Split(networkDefault, "=")
			if len(splitNetworkDefault) != 2 {
				return nil, fmt.Errorf("invalid network default override: '%s'", networkDefault)
			}

			networkDefaults[model.ChainID(splitNetworkDefault[0])] = splitNetworkDefault[1]
		}
		rpcConfig.NetworkDefaults = networkDefaults
	}

	if networkNameOverride := os.Getenv("MESC_NETWORK_NAMES"); networkNameOverride != "" {
		networkNames := make(map[string]model.ChainID)
		for _, networkName := range strings.Split(networkNameOverride, " ") {
			splitNetworkName := strings.Split(networkName, "=")
			if len(splitNetworkName) != 2 {
				return nil, fmt.Errorf("invalid network name overide: '%s'", networkName)
			}

			networkNames[splitNetworkName[0]] = model.ChainID(splitNetworkName[1])
		}
		rpcConfig.NetworkNames = networkNames
	}

	return rpcConfig, nil
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
