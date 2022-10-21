package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os/exec"
	"time"
)

type ForwardRequest struct {
	URL  string
	Port int
}

func main() {
	handleForward := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var fr ForwardRequest
		decoder := json.NewDecoder(r.Body)
		if err := decoder.Decode(&fr); err != nil {
			log.Printf("Body decode error: %+v", err)
			http.Error(w, err.Error(), 400)
			return
		}
		defer r.Body.Close()

		log.Printf("Request: %+v", fr)

		var cbUrl *url.URL
		cbSrv := &http.Server{
			Addr: fmt.Sprintf(":%d", fr.Port),
		}
		handleForward := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			log.Printf("Callback URL: %s", r.URL)
			cbUrl = r.URL
			io.WriteString(w, "OK!")
			if flusher, ok := w.(http.Flusher); ok {
				flusher.Flush()
			}
			r.Body.Close()

			go func() {
				time.Sleep(time.Second)
				cbSrv.Shutdown(context.Background())
			}()
		})
		cbSrv.Handler = handleForward

		cmd := exec.Command("C:\\Program Files\\Mozilla Firefox\\firefox.exe", fr.URL)
		if err := cmd.Start(); err != nil {
			log.Printf("Fail to start browser: %+v", err)
		}
		cbSrv.ListenAndServe()

		log.Printf("Callback query: %s", cbUrl.RawQuery)

		if _, err := w.Write([]byte(cbUrl.RawQuery)); err != nil {
			log.Printf("Body write error: %+v", err)
		}

		log.Printf("Done!\n\n")
	})
	srv := http.Server{
		Addr:    ":9080",
		Handler: handleForward,
	}
	log.Printf("Listening...")
	srv.ListenAndServe()
}
