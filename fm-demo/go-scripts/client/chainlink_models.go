package client

// ChainlinkConfig represents the variables needed to connect to a Chainlink node
type ChainlinkConfig struct {
	URL      string
	Email    string
	Password string
}

// ResponseArray is the generic model that can be used for all Chainlink API responses that are an slice
type ResponseSlice struct {
	Data []map[string]interface{}
}

// Response is the generic model that can be used for all Chainlink API responses
type Response struct {
	Data map[string]interface{}
}

// BridgeType is the model that represents the bridge when read or created on a Chainlink node
type BridgeType struct {
	Data BridgeTypeData `json:"data"`
}

// BridgeTypeData is the model that represents the bridge when read or created on a Chainlink node
type BridgeTypeData struct {
	Attributes BridgeTypeAttributes `json:"attributes"`
}

// BridgeTypeAttributes is the model that represents the bridge when read or created on a Chainlink node
type BridgeTypeAttributes struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

// Session is the form structure used for authenticating
type Session struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

// OCRKeys is the model that represents the created OCR keys when read
type OCRKeys struct {
	Data []OCRKeyData `json:"data"`
}

// OCRKey is the model that represents the created OCR keys when read
type OCRKey struct {
	Data OCRKeyData `json:"data"`
}

// OCRKeyData is the model that represents the created OCR keys when read
type OCRKeyData struct {
	Attributes OCRKeyAttributes `json:"attributes"`
}

// OCRKeyAttributes is the model that represents the created OCR keys when read
type OCRKeyAttributes struct {
	ID                    string `json:"id"`
	ConfigPublicKey       string `json:"configPublicKey"`
	OffChainPublicKey     string `json:"offChainPublicKey"`
	OnChainSigningAddress string `json:"onChainSigningAddress"`
}

// P2PKeys is the model that represents the created P2P keys when read
type P2PKeys struct {
	Data []P2PKeyData `json:"data"`
}

// P2PKey is the model that represents the created P2P keys when read
type P2PKey struct {
	Data P2PKeyData `json:"data"`
}

// P2PKeyData is the model that represents the created P2P keys when read
type P2PKeyData struct {
	Attributes P2PKeyAttributes `json:"attributes"`
}

// P2PKeyAttributes is the model that represents the created P2P keys when read
type P2PKeyAttributes struct {
	ID        int    `json:"id"`
	PeerID    string `json:"peerId"`
	PublicKey string `json:"publicKey"`
}

// ETHKeys is the model that represents the created ETH keys when read
type ETHKeys struct {
	Data []ETHKeyData `json:"data"`
}

// ETHKey is the model that represents the created ETH keys when read
type ETHKey struct {
	Data ETHKeyData `json:"data"`
}

// ETHKeyData is the model that represents the created ETH keys when read
type ETHKeyData struct {
	Attributes ETHKeyAttributes `json:"attributes"`
}

// ETHKeyAttributes is the model that represents the created ETH keys when read
type ETHKeyAttributes struct {
	Address string `json:"address"`
}

// SpecForm is the form used when creating a v2 job spec, containing the TOML of the v2 job
type SpecForm struct {
	TOML string `json:"toml"`
}

// Spec represents a job specification that contains information about the job spec
type Spec struct {
	Data SpecData `json:"data"`
}

// SpecData contains the ID of the job spec
type SpecData struct {
	ID string `json:"id"`
}

// JobForm is the form used when creating a v2 job spec, containing the TOML of the v2 job
type JobForm struct {
	TOML string `json:"toml"`
}

// Job contains the job data for a given job
type Job struct {
	Data JobData `json:"data"`
}

// JobData contains the ID for a given job
type JobData struct {
	ID string `json:"id"`
}
