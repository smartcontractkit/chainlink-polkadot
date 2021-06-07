package client_test

import (
	"os"
	"testing"

	"github.com/onsi/ginkgo/reporters"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

func TestClient(t *testing.T) {
	RegisterFailHandler(Fail)
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})

	junitReporter := reporters.NewJUnitReporter("junit.xml")
	RunSpecsWithCustomReporters(t, "Client Suite", []Reporter{junitReporter})
}
