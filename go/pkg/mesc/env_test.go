package mesc_test

import (
	"context"
	"os"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
	"github.com/paradigmxyz/mesc/go/pkg/mesc"
	"github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

var _ = Describe("Env", func() {
	var ctx context.Context

	BeforeEach(func() {
		ctx = context.Background()
	})

	Context("reading from environmental variables", func() {
		When("the mode is set to ENV", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_MODE", "ENV")).To(Succeed(), "setting the mode should succeed")
			})

			It("reads the configuration from the environment", func() {
				Expect(setAndResetEnv("MESC_ENV", `{ "mesc_version": "MESC 1.0", "default_endpoint": "ethereum_env" }`))

				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configuration should not fail")
				Expect(rpcConfig).ToNot(BeNil(), "the resolved RPC configuration should not be nil")
				Expect(rpcConfig.DefaultEndpoint).ToNot(BeNil(), "the RPC configuration should have a default endpoint set")
				Expect(*rpcConfig.DefaultEndpoint).To(Equal("ethereum_env"), "the RPC config should be read from the environmental variable")
			})
		})

		When("there is no mode set", func() {
			When("there is JSON in MESC_ENV", func() {
				BeforeEach(func() {
					Expect(setAndResetEnv("MESC_ENV", `{ "mesc_version": "MESC 1.0", "default_endpoint": "ethereum_env_nomode" }`))
				})

				It("resolves the JSON from the environmental variable", func() {
					rpcConfig, err := mesc.ResolveRPCConfig(ctx)
					Expect(err).ToNot(HaveOccurred(), "resolving the RPC configuration should not fail")
					Expect(rpcConfig).ToNot(BeNil(), "the resolved RPC configuration should not be nil")
					Expect(rpcConfig.DefaultEndpoint).ToNot(BeNil(), "the RPC configuration should have a default endpoint set")
					Expect(*rpcConfig.DefaultEndpoint).To(Equal("ethereum_env_nomode"), "the RPC config should be read from the environmental variable")
				})
			})
		})
	})

	Context("applying overrides", func() {
		BeforeEach(func() {
			Expect(setAndResetEnv("MESC_MODE", "ENV")).To(Succeed(), "setting the mode should not fail")
			Expect(setAndResetEnv("MESC_ENV", `{ "mesc_version": "MESC 1.0" }`)).To(Succeed(), "setting the JSON env should not fail")
		})

		When("there is a default endpoint override", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_DEFAULT_ENDPOINT", "localhost.default:9999")).To(Succeed(), "setting the default endpoint override should succeed")
			})

			It("overrides the default endpoint", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configuration should not fail")
				Expect(rpcConfig.DefaultEndpoint).ToNot(BeNil(), "the default endpoint should be set")
				Expect(*rpcConfig.DefaultEndpoint).To(Equal("localhost.default:9999"), "the default endpoint override should be applied")
			})
		})

		When("there are network defaults set", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_NETWORK_DEFAULTS", "5=alchemy_optimism 1=local_mainnet")).To(Succeed(), "setting the network defaults override should succeed")
			})

			It("applies the network default overrides", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configurations should not fail")
				Expect(rpcConfig.NetworkDefaults).To(And(
					HaveLen(2),
					HaveKeyWithValue(model.ChainID("1"), "local_mainnet"),
					HaveKeyWithValue(model.ChainID("5"), "alchemy_optimism"),
				), "the network default override should have been applied")
			})
		})

		When("there are network names set", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_NETWORK_NAMES", "zora=7777777 scroll=534352")).To(Succeed(), "setting the network names override should work")
			})

			It("applies the network names override", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configurations should not fail")
				Expect(rpcConfig.NetworkNames).To(And(
					HaveLen(2),
					HaveKeyWithValue("zora", model.ChainID("7777777")),
					HaveKeyWithValue("scroll", model.ChainID("534352")),
				), "the network names override should be applied")
			})
		})

		When("there are endpoint overrides set", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_ENDPOINTS", "alchemy_optimism=https://alchemy.com/fjsj local_goerli:5=localhost:8545")).To(Succeed(), "setting the endpoint overrides should succeed")
			})

			It("applies the endpoint overrides", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configurations should not fail")
				Expect(rpcConfig.Endpoints).To(And(
					HaveLen(2),
					HaveKey("alchemy_optimism"),
					HaveKey("local_goerli"),
				), "the endpoints should be loaded into the RPC configuration")

				optimismEndpoint := rpcConfig.Endpoints["alchemy_optimism"]
				Expect(optimismEndpoint.Name).To(Equal("alchemy_optimism"), "the Optimism endpoint should inherit its key as its name")
				Expect(optimismEndpoint.URL).To(Equal("https://alchemy.com/fjsj"), "the Optimism URL should be set")
				Expect(optimismEndpoint.ChainID).To(BeNil(), "no chain ID should be set for the Optimism endpoint")

				goerliEndpoint := rpcConfig.Endpoints["local_goerli"]
				Expect(goerliEndpoint.Name).To(Equal("local_goerli"), "the Goerli endpoint should inherit its key as its name")
				Expect(goerliEndpoint.URL).To(Equal("localhost:8545"), "the Goerli endpoint should have the right URL")
				Expect(goerliEndpoint.ChainID).ToNot(BeNil(), "the Goerli endpoint should have a chain ID set")
				Expect(*goerliEndpoint.ChainID).To(Equal(model.ChainID("5")), "the Goerli endpoint should have the correct chain ID")
			})
		})

		When("there are profile overrides set", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_PROFILES", "foundry.default_endpoint=local_goerli foundry.network_defaults.5=alchemy_optimism foundry.profile_metadata.metadatum_key=metadatum_value foundry.use_mesc=true")).To(Succeed(), "setting the profiles override should not fail")
			})

			It("applies the profile overrides", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configurations should not fail")
				Expect(rpcConfig.Profiles).To(And(
					HaveLen(1),
					HaveKey("foundry"),
				), "the profile should be loaded")

				profile := rpcConfig.Profiles["foundry"]
				Expect(profile.Name).To(Equal("foundry"), "the profile should inherit the key as the name")
				Expect(profile.DefaultEndpoint).ToNot(BeNil(), "the profile should have a default endpoint")
				Expect(*profile.DefaultEndpoint).To(Equal("local_goerli"), "the profile should have the correct default endpoint")
				Expect(profile.NetworkDefaults).To(And(
					HaveLen(1),
					HaveKeyWithValue(model.ChainID("5"), "alchemy_optimism"),
				), "the profile should have the correct network defaults loaded")
				Expect(profile.ProfileMetadata).To(And(
					HaveLen(1),
					HaveKeyWithValue("metadatum_key", "metadatum_value"),
				), "the profile should have the correct metadata")
				Expect(profile.UseMESC).To(BeTrue(), "the profile should have MESC enabled")
			})
		})

		When("there is a global metadata override", func() {
			BeforeEach(func() {
				Expect(setAndResetEnv("MESC_GLOBAL_METADATA", `{ "bool_field": true, "string_field": "str_value" }`))
			})

			It("assigns the configured metadata as global metadata", func() {
				rpcConfig, err := mesc.ResolveRPCConfig(ctx)
				Expect(err).ToNot(HaveOccurred(), "resolving the RPC configurations should not fail")
				Expect(rpcConfig.GlobalMetadata).To(And(
					HaveLen(2),
					HaveKeyWithValue("bool_field", true),
					HaveKeyWithValue("string_field", "str_value"),
				), "the global metadata should be overridden")
			})
		})
	})

	When("there is no resolvable RPC configuration", func() {
		It("fails to resolve an RPC configuration", func() {
			_, err := mesc.ResolveRPCConfig(ctx)
			Expect(err).To(HaveOccurred(), "resolving the RPC configuration should fail")
			Expect(err.Error()).To(Equal("unable to resolve MESC configuration"), "it should fail for the correct reason")
		})
	})
})

func setAndResetEnv(name string, value string) error {
	originalValue := os.Getenv(name)
	DeferCleanup(func() {
		_ = os.Setenv(name, originalValue)
	})
	return os.Setenv(name, value)
}
