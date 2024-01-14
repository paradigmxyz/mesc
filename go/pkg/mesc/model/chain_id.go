package mesc

import "errors"

// ChainID is the definition of a representation of a chain ID,
// conforming to the format as described in the MESC specification.
type ChainID string

// ToInt will parse an integer representation of the chain ID.
func (c ChainID) ToInt() (int64, error) {
    // TODO: implement parsing of the chain ID per the MESC specification
    return 0, errors.New("not implemented")
}
