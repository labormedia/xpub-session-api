#!/bin/bash
pwd
curl -c cookies.txt -H 'Content-Type: application/json' -X POST \
-d '{"witness":[32, 243, 236, 222, 145, 94, 145, 229, 196, 148, 30, 99, 26, 123, 127, 218, 89, 8, 46, 210, 29, 232, 55, 248, 201, 99, 5, 215, 167, 216, 56, 149, 226, 90, 48, 26, 191, 78, 203, 130, 108, 175, 30, 161, 80, 163, 62, 230, 169, 1, 89, 185, 86, 0, 185, 223, 35, 5, 218, 2, 201, 49, 97, 108, 36],
"xpub":{"bytes":[4, 53, 135, 207, 2, 50, 21, 121, 81, 0, 0, 0, 0, 41, 166, 29, 62, 45, 38, 147, 163, 94, 95, 3, 18, 184, 157, 43, 71, 224, 157, 142, 250, 12, 165, 107, 211, 69, 31, 5, 205, 185, 5, 157, 252, 3, 114, 92, 45, 56, 5, 58, 7, 124, 15, 61, 204, 161, 180, 126, 206, 39, 242, 95, 188, 190, 39, 221, 14, 116, 6, 68, 159, 243, 58, 47, 254, 43]},
"nonce":0}' \
http://localhost:8080/login 