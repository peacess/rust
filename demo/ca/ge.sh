#!/bin/bash

out="./out"

openssl req -x509 -newkey rsa:4096 -sha256 -nodes -keyout $out/ca.key -out $out/ca.pem -days 36500 -subj "/C=CN/O=Condition/CN=xx.com"

openssl genrsa -out $out/server.key 4096
openssl req -new -key $out/server.key -out $out/server.req -sha256 -subj "/C=CN/O=Condition/CN=xx.com"
openssl x509 -req -in $out/server.req -CA $out/ca.pem -CAkey $out/ca.key -set_serial 100 -extensions server -days 36500 -outform PEM -out $out/server.pem -sha256
rm $out/server.req

openssl genrsa -out $out/client.key 4096
openssl req -new -key $out/client.key -out $out/client.req -subj "/C=CN/O=Condition/CN=xx.com"
openssl x509 -req -in $out/client.req -CA $out/ca.pem -CAkey $out/ca.key -set_serial 101 -extensions client -days 36500 -outform PEM -out $out/client.pem -sha256
#openssl pkcs12 -export -inkey client.key -in client.cer -out client.p12
rm $out/client.req

openssl req -x509 -nodes -days 36500 -newkey rsa:2048 -keyout $out/nginx.key -out $out/nginx.crt -subj "/C=CN/O=Condition/CN=xx.com"
