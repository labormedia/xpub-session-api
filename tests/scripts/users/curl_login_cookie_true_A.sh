#!/bin/bash
pwd
curl -b cookies.txt -H 'Content-Type: application/json' -X POST \
-d '{"witness":[31, 144, 160, 233, 234, 243, 143, 186, 243, 193, 70, 88, 164, 114, 10, 72, 246, 57, 11, 107, 156, 32, 89, 250, 198, 162, 35, 74, 193, 222, 19, 194, 223, 12, 36, 108, 254, 71, 77, 255, 61, 186, 153, 84, 216, 82, 159, 55, 146, 154, 160, 144, 124, 158, 127, 165, 61, 160, 168, 23, 14, 99, 117, 204, 31],
"xpub":{"bytes":[4, 53, 135, 207, 2, 205, 24, 204, 87, 0, 0, 0, 0, 25, 100, 90, 64, 101, 74, 135, 255, 193, 220, 130, 107, 63, 78, 211, 122, 222, 154, 111, 132, 206, 133, 157, 200, 122, 240, 92, 186, 102, 84, 140, 92, 2, 71, 47, 83, 161, 141, 100, 232, 253, 241, 236, 85, 8, 85, 223, 244, 103, 216, 186, 60, 99, 157, 27, 214, 152, 104, 218, 15, 142, 99, 4, 47, 9]},
"nonce":0}' \
http://localhost:8080/login 