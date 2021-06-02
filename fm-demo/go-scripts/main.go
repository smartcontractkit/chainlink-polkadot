package main

import (
	"fmt"
	"integrations-framework/client"
	"net/http"
)

var spec = `{
	"initiators": [
	  {
		"type": "external",
		"params": {
		  "name": "substrate",
		  "body": {
			"endpoint": "substrate",
			"feed_id": 0,
			"account_id": "0x7c522c8273973e7bcf4a5dbfcc745dba4a3ab08c1e410167d7b1bdf9cb924f6c",
			"fluxmonitor": {
			  "requestData": {
				"data": { "from": "DOT", "to": "USD" }
			  },
			  "feeds": [{ "url": "http://localhost:8083" }],
			  "threshold": 0.5,
			  "absoluteThreshold": 0,
			  "precision": 8,
			  "pollTimer": { "period": "3s" },
			  "idleTimer": { "duration": "10s" }
			}
		  }
		}
	  }
	],
	"tasks": [
	  {
		"type": "ethtx",
		"params": { "multiply": 1e8 }
	  }
	]
  }`

func main() {
	c := newDefaultClient("http://localhost:6688")
	c.SetClient(&http.Client{})
	c.SetSessionCookie()
	r, err := c.CreateSpec(spec)
	fmt.Println(r, err)
}

func newDefaultClient(url string) client.Chainlink {
	cl := client.NewChainlink(&client.ChainlinkConfig{
		Email:    "notreal@fakeemail.ch",
		Password: "twochains",
		URL:      url,
	})
	return cl
}
