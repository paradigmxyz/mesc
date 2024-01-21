package serialization_test

import (
	"bytes"
	_ "embed"
	"time"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
	"github.com/onsi/gomega/types"

	"github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/io/serialization"
	"github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

//go:embed example.json
var exampleJSON string

var _ = Describe("JSON Deserialization", func() {
	It("successfully deserializes the example JSON", func() {
		jsonReader := bytes.NewBufferString(exampleJSON)
		rpcConfig, err := serialization.DeserializeJSON(jsonReader)
		Expect(err).ToNot(HaveOccurred(), "deserializing the JSON should not fail")

		Expect(rpcConfig.MESCVersion).To(Equal("MESC 1.0"), "the MESC version should be deserialized correctly")
		Expect(rpcConfig.DefaultEndpoint).ToNot(BeNil(), "there should be a default endpoint")
		Expect(*rpcConfig.DefaultEndpoint).To(Equal("local_ethereum"), "the default endpoint should be correct")
		Expect(rpcConfig.NetworkDefaults).To(HaveKeyWithValue(model.ChainID("1"), "local_ethereum"), "the network defaults should be deserialized")
		Expect(rpcConfig.NetworkNames).To(HaveKeyWithValue("local_ethereum", model.ChainID("1")), "the network names should be deserialized")
		Expect(rpcConfig.GlobalMetadata).To(And(HaveLen(1), HaveKeyWithValue("example_global_metadatum", "example-global-value")), "the global metadata should have been read in")

		// verify endpoints
		Expect(rpcConfig.Endpoints).To(And(HaveLen(1), HaveKey("local_ethereum")), "the local_ethereum endpoint should be deserialized")
		ethereumEndpoint := rpcConfig.Endpoints["local_ethereum"]
		Expect(ethereumEndpoint.Name).To(Equal("local_ethereum"), "the endpoint name should be deserialized")
		Expect(ethereumEndpoint.URL).To(Equal("http://localhost:8545"), "the endpoint URL should be deserialized")
		Expect(ethereumEndpoint.ChainID).ToNot(BeNil(), "the chain ID should be present on the endpoint metadata")
		Expect(*ethereumEndpoint.ChainID).To(Equal(model.ChainID("1")), "the chain ID should be deserialized")
		// The actual verification of the metadata contents follows this by exercising helper methods
		Expect(ethereumEndpoint.EndpointMetadata).To(HaveLen(13), "there should be 13 endpoint metadata elements")

		expectMatches("rate limit RPS", ethereumEndpoint.GetRateLimitRPS, BeNumerically("==", 250))
		expectMatches("rate limit CUPS", ethereumEndpoint.GetRateLimitCUPS, BeNumerically("==", 1000))
		expectMatches("method rate limit", func() (float64, bool) {
			return ethereumEndpoint.GetMethodRateLimit("trace_block")
		}, BeNumerically("==", 200))
		expectMatches("API key", ethereumEndpoint.GetAPIKey, Equal("a2798f237a2398rf7"))
		expectMatches("JWT secret", ethereumEndpoint.GetJWTSecret, ContainSubstring("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"))
		expectMatches("host", ethereumEndpoint.GetHost, Equal("local"))
		expectMatches("ecosystem", ethereumEndpoint.GetEcosystem, Equal("ethereum"))
		expectMatches("node client", ethereumEndpoint.GetNodeClient, Equal("reth/v0.1.0-alpha.10-7b781eb60/x86_64-unknown-linux-gnu"))
		expectMatches("namespaces", ethereumEndpoint.GetNamespaces, ConsistOf([]string{"debug", "eth", "trace"}))
		expectMatches("explorer", ethereumEndpoint.GetExplorer, Equal("https://etherscan.io"))
		expectMatches("location", ethereumEndpoint.GetLocation, Equal("Paris, France"))
		expectMatches("cloud region", ethereumEndpoint.GetCloudRegion, Equal("aws-us-east-1a"))
		expectMatches("labels", ethereumEndpoint.GetLabels, ConsistOf([]string{"archive", "cache", "private_mempool"}))

		// verify profiles
		Expect(rpcConfig.Profiles).To(And(HaveLen(1), HaveKey("xyz")), "the xyz profile should be read in")
		xyzProfile := rpcConfig.Profiles["xyz"]
		Expect(xyzProfile.Name).To(Equal("xyz"), "the profile name should be correct")
		Expect(xyzProfile.DefaultEndpoint).ToNot(BeNil(), "the profile should have a default endpoint")
		Expect(*xyzProfile.DefaultEndpoint).To(Equal("local_ethereum"), "the profile should have the correct default endpoint")
		Expect(xyzProfile.NetworkDefaults).To(And(HaveLen(1), HaveKeyWithValue(model.ChainID("1"), "local_ethereum")), "the profile should have the correct network defaults")
		Expect(xyzProfile.UseMESC).To(BeTrue(), "the profile should have MESC enabled")
		expectMatches("last modified by", xyzProfile.GetLastModifiedBy, Equal("mesc__1.0"))
		expectMatches("last modified time", xyzProfile.GetLastModifiedTime, BeTemporally("==", time.Unix(1705870535, 0)))
		expectMatches("creation time", xyzProfile.GetCreationTime, BeTemporally("==", time.Unix(1700200462, 0)))
		expectMatches("API keys", xyzProfile.GetAPIKeys, And(HaveLen(1), HaveKeyWithValue("etherscan", "abc123")))
		expectMatches("load balancer groups", func() ([]string, bool) {
			loadBalancerGroups, hasGroup := xyzProfile.GetEndpointNamesForGrouping("load_balancer")
			return loadBalancerGroups, hasGroup
		}, ConsistOf([]string{"alchemy_optimism", "quicknode_optimism"}))
		expectMatches("conceal", xyzProfile.Conceal, BeTrue())
	})
})

func expectMatches[V any](valueDescriptor string, getter func() (V, bool), valueMatcher types.GomegaMatcher) {
	actualValue, hasValue := getter()
	Expect(hasValue).To(BeTrue(), "%s should be present", valueDescriptor)
	Expect(actualValue).To(valueMatcher, "%s should match", valueDescriptor)
}
