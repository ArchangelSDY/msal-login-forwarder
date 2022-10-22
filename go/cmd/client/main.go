package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"os"
	"strconv"
)

type ForwardRequest struct {
	URL  string `json:"url"`
	Port int    `json:"port"`
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Need url")
		os.Exit(1)
	}

	u, err := url.Parse(os.Args[1])
	if err != nil {
		fmt.Println("Fail to parse url", err)
		os.Exit(1)
	}

	ru, err := url.Parse(u.Query().Get("redirect_uri"))
	if err != nil {
		fmt.Println("Fail to parse redirect url", err)
		os.Exit(1)
	}

	port, _ := strconv.Atoi(ru.Port())
	fmt.Println("MSAL Port:", port)

	bodyData, _ := json.Marshal(&ForwardRequest{
		URL:  os.Args[1],
		Port: port,
	})
	body := bytes.NewBuffer(bodyData)
	resp, err := http.Post("http://192.168.98.1:9080", "application/json", body)
	if err != nil {
		fmt.Println("Fail to forward", err)
		os.Exit(1)
	}
	defer resp.Body.Close()

	fmt.Println("Waiting for response")

	rbody, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Printf("Fail to read forwarded body", err)
		os.Exit(1)
	}

	ru.RawQuery = string(rbody)

	fmt.Println("Calling MSAL:", ru.String())

	resp, err = http.Get(ru.String())
	if err != nil {
		fmt.Printf("Fail to forward", err)
		os.Exit(1)
	}
	defer resp.Body.Close()
}
