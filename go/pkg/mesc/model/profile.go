package model

// Profile describes a profile to contextualize resolved endpoint metadata
type Profile struct {
	Name            string             // Name is the name of the profile
	DefaultEndpoint *string            // DefaultEndpoint contains the ChainID of the default network under this profile, if available
	NetworkDefaults map[ChainID]string // NetworkDefaults maps ChainID values identifying networks to the name of the default endpoint of that network
	ProfileMetadata map[string]any     // ProfileMetadata is a combination of both well-defined and custom-defined metadata for this profile
	UseMESC         bool               // UseMESC, if false, indicates that MESC resolution should not be used when this profile is enabled
}

// TODO: implement helper methods resolving profile metadata, per the specification here:
// https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#metadata
