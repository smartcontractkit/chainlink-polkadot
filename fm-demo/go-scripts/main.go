package main

import (
	"fmt"
	"integrations-framework/client"
	"io/ioutil"
	"log"
	"net/http"
	"os"
)

func main() {
	files, err := ioutil.ReadDir("../specs")
	if err != nil {
		log.Fatal(err)
	}

	for _, file := range files {
		jsonFile, err := os.Open("../specs/" + file.Name())
		if err != nil {
			fmt.Println(err)
		}
		defer jsonFile.Close()

		spec, _ := ioutil.ReadAll(jsonFile)
		c := newDefaultClient("http://localhost:6688")
		c.SetClient(&http.Client{})
		c.SetSessionCookie()
		r, err := c.CreateSpec(string(spec))
		fmt.Println(r, err)
	}
}

func newDefaultClient(url string) client.Chainlink {
	cl := client.NewChainlink(&client.ChainlinkConfig{
		Email:    "notreal@fakeemail.ch",
		Password: "twochains",
		URL:      url,
	})
	return cl
}
