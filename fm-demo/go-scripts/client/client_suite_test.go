package client_test

import (
	"github.com/onsi/ginkgo/reporters"
	"testing"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

func TestClient(t *testing.T) {
	RegisterFailHandler(Fail)

	junitReporter := reporters.NewJUnitReporter("junit.xml")
	RunSpecsWithCustomReporters(t, "Client Suite", []Reporter{junitReporter})
}
