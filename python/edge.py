#!/usr/bin/python3
# -*- coding: utf-8 -*-
import json
import http.client
import sys
import urllib.parse


if __name__ == '__main__':
    url = sys.argv[1]
    print('URL:', url)

    u = urllib.parse.urlparse(url)
    qs = urllib.parse.parse_qs(u.query)
    redirect_uri = qs['redirect_uri'][0]
    ru = urllib.parse.urlparse(redirect_uri)
    port = ru.port
    print('MASL listen port:', port)

    # Send to remote server
    conn = http.client.HTTPConnection('192.168.98.1', 9080)
    body = {
        'url': url,
        'port': port,
    }
    conn.request('POST', '/open-url', json.dumps(body))
    resp = conn.getresponse()
    resp_body = resp.read()
    print('Resp body', resp_body)
    qs = resp_body.decode('utf-8')

    conn = http.client.HTTPConnection('127.0.0.1', port)
    conn.request('GET', '/?' + qs)
    resp = conn.getresponse()
    resp_body = resp.read()
    print('MSAL body', resp_body)
