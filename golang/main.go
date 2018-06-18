package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
)

func send_message(message string) {
	m := map[string]string{"text": message}
	jsonValue, _ := json.Marshal(m)
	b := bytes.NewBuffer(jsonValue)
	client := &http.Client{}
	url := "***YOUR GCHAT WEBHOOKS URL***"
	r, _ := http.NewRequest("POST", url, b)
	r.Header.Add("Content-Type", "application/json; charset=UTF-8")
	resp, _ := client.Do(r)
	fmt.Println(resp.Status)
}
func main() {
	send_message("Hello World!")
}
