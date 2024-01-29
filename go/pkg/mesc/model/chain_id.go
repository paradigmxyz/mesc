package model

import (
	"fmt"
	"strconv"
	"strings"
)

// ChainID is the definition of a representation of a chain ID,
// conforming to the format as described in the MESC specification.
type ChainID string

// ToInt will parse an integer representation of the chain ID.
func (c ChainID) ToInt() (int64, error) {
	asString := string(c)
	if strings.HasPrefix(asString, "0x") {
		parsedInt, parseErr := strconv.ParseInt(asString[2:], 16, 64)
		if parseErr != nil {
			return 0, fmt.Errorf("failed to parse chain ID '%s' as a hex value: %w", asString, parseErr)
		}

		return parsedInt, nil
	}

	// Try to just parse it as a ten-base digit
	parsedInt, parseErr := strconv.ParseInt(asString, 10, 64)
	if parseErr != nil {
		return 0, fmt.Errorf("failed to parse chain ID '%s' as a ten-base value: %w", asString, parseErr)
	}

	return parsedInt, nil
}
