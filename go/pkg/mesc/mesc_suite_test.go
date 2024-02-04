package mesc_test

import (
	"testing"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
)

func TestMesc(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "Mesc Suite")
}
