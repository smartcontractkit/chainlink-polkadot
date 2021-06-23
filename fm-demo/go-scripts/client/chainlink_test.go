package client

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var spec = `{
  "initiators": [
    {
      "type": "runlog"
    }
  ],
  "tasks": [
    {
      "type": "httpget"
    },
    {
      "type": "jsonparse"
    },
    {
      "type": "multiply"
    },
    {
      "type": "ethuint256"
    },
    {
      "type": "ethtx"
    }
  ]
}`

// Mocks the creation, read, delete cycle for any job type
func TestNodeClient_CreateReadDeleteJob(t *testing.T) {
	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/jobs", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, Job{
				Data: JobData{
					ID: "1",
				},
			})
		case http.MethodGet:
			assert.Equal(t, "/v2/jobs/1", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		case http.MethodDelete:
			assert.Equal(t, "/v2/jobs/1", req.URL.Path)
			writeResponse(t, rw, http.StatusNoContent, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	s, err := c.CreateJob("schemaVersion = 1")
	assert.NoError(t, err)

	err = c.ReadJob(s.Data.ID)
	assert.NoError(t, err)

	err = c.DeleteJob(s.Data.ID)
	assert.NoError(t, err)
}

// Mocks the creation, read, delete cycle for any job spec
func TestNodeClient_CreateReadDeleteSpec(t *testing.T) {
	specID := "c142042149f64911bb4698fb08572040"

	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/specs", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, Spec{
				Data: SpecData{ID: specID},
			})
		case http.MethodGet:
			assert.Equal(t, fmt.Sprintf("/v2/specs/%s", specID), req.URL.Path)
			writeResponse(t, rw, http.StatusOK, Response{
				Data: map[string]interface{}{},
			})
		case http.MethodDelete:
			assert.Equal(t, fmt.Sprintf("/v2/specs/%s", specID), req.URL.Path)
			writeResponse(t, rw, http.StatusNoContent, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	s, err := c.CreateSpec(spec)
	assert.NoError(t, err)

	_, err = c.ReadSpec(s.Data.ID)
	assert.NoError(t, err)

	err = c.DeleteSpec(s.Data.ID)
	assert.NoError(t, err)
}

// Mocks the creation, read, delete cycle for Chainlink bridges
func TestNodeClient_CreateReadDeleteBridge(t *testing.T) {
	bta := BridgeTypeAttributes{
		Name: "example",
		URL:  "https://example.com",
	}

	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/bridge_types", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		case http.MethodGet:
			assert.Equal(t, "/v2/bridge_types/example", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, BridgeType{
				Data: BridgeTypeData{
					Attributes: bta,
				},
			})
		case http.MethodDelete:
			assert.Equal(t, "/v2/bridge_types/example", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	err := c.CreateBridge(&bta)
	assert.NoError(t, err)

	bt, err := c.ReadBridge(bta.Name)
	assert.NoError(t, err)

	assert.Equal(t, bt.Data.Attributes.Name, bta.Name)
	assert.Equal(t, bt.Data.Attributes.URL, bta.URL)

	err = c.DeleteBridge(bta.Name)
	assert.NoError(t, err)
}

// Mocks the creation, read, delete cycle for OCR keys
func TestNodeClient_CreateReadDeleteOCRKey(t *testing.T) {
	ocrKeyData := OCRKeyData{
		Attributes: OCRKeyAttributes{
			ID:                    "1",
			ConfigPublicKey:       "someNon3sens3",
			OffChainPublicKey:     "mor3Non3sens3",
			OnChainSigningAddress: "thisActuallyMak3sS3ns3",
		},
	}

	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/keys/ocr", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, OCRKey{ocrKeyData})
		case http.MethodGet:
			writeResponse(t, rw, http.StatusOK, OCRKeys{
				Data: []OCRKeyData{ocrKeyData},
			})
		case http.MethodDelete:
			assert.Equal(t, "/v2/keys/ocr/1", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	receivedKey, err := c.CreateOCRKey()
	assert.NoError(t, err)

	keys, err := c.ReadOCRKeys()
	assert.NoError(t, err)
	assert.Contains(t, keys.Data, receivedKey.Data)

	err = c.DeleteOCRKey("1")
	assert.NoError(t, err)
}

// Mocks the creation, read, delete cycle for P2P keys
func TestNodeClient_CreateReadDeleteP2PKey(t *testing.T) {
	p2pKeyData := P2PKeyData{
		P2PKeyAttributes{
			ID:        1,
			PeerID:    "someNon3sens3",
			PublicKey: "mor3Non3sens3",
		},
	}

	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/keys/p2p", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, P2PKey{p2pKeyData})
		case http.MethodGet:
			writeResponse(t, rw, http.StatusOK, P2PKeys{
				Data: []P2PKeyData{p2pKeyData},
			})
		case http.MethodDelete:
			assert.Equal(t, "/v2/keys/p2p/1", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	receivedKey, err := c.CreateP2PKey()
	assert.NoError(t, err)

	keys, err := c.ReadP2PKeys()
	assert.NoError(t, err)
	assert.Contains(t, keys.Data, receivedKey.Data)

	err = c.DeleteP2PKey(1)
	assert.NoError(t, err)
}

// Mocks the creation, read, delete cycle for ETH keys
func TestNodeClient_ReadETHKeys(t *testing.T) {
	ethKeyData := ETHKeyData{
		Attributes: ETHKeyAttributes{
			Address: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
		},
	}

	server := mockedServer(func(rw http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPost:
			assert.Equal(t, "/v2/keys/ocr", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, ETHKey{ethKeyData})
		case http.MethodGet:
			writeResponse(t, rw, http.StatusOK, ETHKeys{
				Data: []ETHKeyData{ethKeyData},
			})
		case http.MethodDelete:
			assert.Equal(t, "/v2/keys/ocr/1", req.URL.Path)
			writeResponse(t, rw, http.StatusOK, nil)
		}
	})
	defer server.Close()

	c := newDefaultClient(server.URL)
	c.SetClient(server.Client())

	receivedKeys, err := c.ReadETHKeys()
	assert.NoError(t, err)
	assert.Contains(t, receivedKeys.Data, ethKeyData)
}

func newDefaultClient(url string) Chainlink {
	cl := NewChainlink(&ChainlinkConfig{
		Email:    "admin@node.local",
		Password: "twochains",
		URL:      url,
	})
	return cl
}

func mockedServer(handlerFunc http.HandlerFunc) *httptest.Server {
	return httptest.NewServer(handlerFunc)
}

func writeResponse(t *testing.T, rw http.ResponseWriter, statusCode int, obj interface{}) {
	rw.WriteHeader(statusCode)
	if obj == nil {
		return
	}
	b, err := json.Marshal(obj)
	require.Nil(t, err)
	_, err = rw.Write(b)
	require.Nil(t, err)
}
