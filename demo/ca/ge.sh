#!/bin/bash

openssl req -x509 -newkey rsa:4096 -sha256 -nodes -keyout ca.key -out ca.pem -days 36500 -subj "/C=CN/O=Condition/CN=xx.com"

openssl genrsa -out server.key 4096
openssl req -new -key server.key -out server.req -sha256 -subj "/C=CN/O=Condition/CN=xx.com"
openssl x509 -req -in server.req -CA ca.pem -CAkey ca.key -set_serial 100 -extensions server -days 36500 -outform PEM -out server.pem -sha256
rm server.req

openssl genrsa -out client.key 4096
openssl req -new -key client.key -out client.req -subj "/C=CN/O=Condition/CN=xx.com"
openssl x509 -req -in client.req -CA ca.pem -CAkey ca.key -set_serial 101 -extensions client -days 36500 -outform PEM -out client.pem -sha256
#openssl pkcs12 -export -inkey client.key -in client.cer -out client.p12
rm client.req

openssl req -x509 -nodes -days 36500 -newkey rsa:2048 -keyout nginx.key -out nginx.crt -subj "/C=CN/O=Condition/CN=xx.com"
