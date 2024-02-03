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
