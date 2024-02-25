package mesc_test

import (
	"context"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"

	"github.com/paradigmxyz/mesc/go/pkg/mesc"
	"github.com/paradigmxyz/mesc/go/pkg/mesc/endpoint/resolution"
	"github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

var _ = Describe("Endpoints", func() {
	var ctx context.Context

	BeforeEach(func() {
		ctx = context.Background()
	})

	Context("GetDefaultEndpoint", func() {
		It("resolves the default endpoint from the RPC configuration", func() {
			endpointName := "endpoint"
			endpointURL := "http://desired.endpoint"
			rpcConfig := &model.RPCConfig{
				DefaultEndpoint: &endpointName,
				Endpoints: map[string]model.EndpointMetadata{
					endpointName: {
						URL: endpointURL,
					},
					"otherEndpoint": {
						Name: "To Protect Against False Positives",
					},
				},
			}

			defaultEndpoint, err := mesc.GetDefaultEndpoint(ctx, resolution.WithRPCConfig(*rpcConfig))
			Expect(err).ToNot(HaveOccurred(), "getting the default endpoint should not fail")
			Expect(defaultEndpoint).ToNot(BeNil(), "a default endpoint should be resolved")
			Expect(defaultEndpoint.URL).To(Equal(endpointURL), "the correct default endpoint should be resolved")
		})

		When("the RPC configuration has no default endpoint defined", func() {
			It("returns no default endpoint", func() {
				defaultEndpoint, err := mesc.GetDefaultEndpoint(ctx, resolution.WithRPCConfig(model.RPCConfig{}))
				Expect(err).ToNot(HaveOccurred(), "getting the default endpoint should not fail")
				Expect(defaultEndpoint).To(BeNil(), "no default endpoint should be resolved")
			})
		})

		When("the configured endpoint can't be found", func() {
			It("returns nil", func() {
				endpointName := "notProvided"
				rpcConfig := &model.RPCConfig{
					DefaultEndpoint: &endpointName,
				}

				defaultEndpoint, err := mesc.GetDefaultEndpoint(ctx, resolution.WithRPCConfig(*rpcConfig))
				Expect(err).ToNot(HaveOccurred(), "getting the default endpoint should not fail")
				Expect(defaultEndpoint).To(BeNil(), "a default endpoint should not be resolved")
			})
		})

		When("a profile is supplied", func() {
			It("uses the profile's default endpoint to resolve the default endpoint", func() {
				endpointName := "endpoint"
				otherEndpointName := "otherEndpoint"
				profileName := "aProfile"
				endpointURL := "http://profile.endpoint"
				rpcConfig := &model.RPCConfig{
					DefaultEndpoint: &otherEndpointName,
					Endpoints: map[string]model.EndpointMetadata{
						endpointName: {
							URL: endpointURL,
						},
						otherEndpointName: {
							Name: "To Protect Against False Positives",
						},
					},
					Profiles: map[string]model.Profile{
						profileName: {
							DefaultEndpoint: &endpointName,
						},
					},
				}

				defaultEndpoint, err := mesc.GetDefaultEndpoint(ctx, resolution.WithRPCConfig(*rpcConfig), resolution.WithProfile(profileName))
				Expect(err).ToNot(HaveOccurred(), "getting the default endpoint should not fail")
				Expect(defaultEndpoint).ToNot(BeNil(), "a default endpoint should be resolved")
				Expect(defaultEndpoint.URL).To(Equal(endpointURL), "the profile's default endpoint should be resolved")
			})

			When("the profile has no default endpoint defined", func() {
				It("defers to the RPC configuration's default endpoint", func() {
					endpointName := "fallbackEndpoint"
					profileName := "noDefaultEndpointProfile"
					endpointURL := "http://fallback.endpoint"
					rpcConfig := &model.RPCConfig{
						DefaultEndpoint: &endpointName,
						Endpoints: map[string]model.EndpointMetadata{
							endpointName: {
								URL: endpointURL,
							},
						},
						Profiles: map[string]model.Profile{
							profileName: {},
						},
					}

					defaultEndpoint, err := mesc.GetDefaultEndpoint(ctx, resolution.WithRPCConfig(*rpcConfig), resolution.WithProfile(profileName))
					Expect(err).ToNot(HaveOccurred(), "getting the default endpoint should not fail")
					Expect(defaultEndpoint).ToNot(BeNil(), "a default endpoint should be resolved")
					Expect(defaultEndpoint.URL).To(Equal(endpointURL), "the configuration's default endpoint should be resolved")
				})
			})
		})
	})
})
