#!/bin/bash
curl -b cookies.txt -H 'Content-Type: application/json' -X GET http://localhost:8080/derive_address/0/1