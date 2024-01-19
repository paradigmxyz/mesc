package model_test

import (
	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"

	model "github.com/paradigmxyz/mesc/go/pkg/mesc/model"
)

var _ = Describe("ChainID", func() {
	Context("ToInt", func() {
		When("the chain ID is expressed as a ten-base number", func() {
			It("successfully parses the integer", func() {
				chainID := model.ChainID("512")
				asInt, err := chainID.ToInt()
				Expect(err).ToNot(HaveOccurred(), "parsing the integer should not fail")
				Expect(asInt).To(BeNumerically("==", 512), "the parsed integer should be correct")
			})
		})

		When("the chain ID is expressed as a hexadecimal number", func() {
			It("successfully parses the integer", func() {
				chainID := model.ChainID("0xddb99ac060e2")
				asInt, err := chainID.ToInt()
				Expect(err).ToNot(HaveOccurred(), "parsing the integer should not fail")
				Expect(asInt).To(BeNumerically("==", 243789234987234), "the parsed integer should be correct")
			})
		})
	})
})
