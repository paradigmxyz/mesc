package mesc_test

import (
	"context"
	"os"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
	"github.com/paradigmxyz/mesc/go/pkg/mesc"
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
})

func setAndResetEnv(name string, value string) error {
	originalValue := os.Getenv(name)
	DeferCleanup(func() {
		_ = os.Setenv(name, originalValue)
	})
	return os.Setenv(name, value)
}
