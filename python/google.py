
import requests
import json

url = "***YOUR GCHAT WEBHOOKS URL***"
def send_message(msg):
    message_headers = { 'Content-Type': 'application/json; charset=UTF-8'}
    payload = {'text': msg}
    res = requests.post(url, data=json.dumps(payload), headers=message_headers)

send_message("Hello World!")

