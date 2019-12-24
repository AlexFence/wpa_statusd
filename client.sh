#!/bin/sh
# requires serialization-tools & socat
echo '{"id": "bla", "type": "Request", "method": "STATUS" }' | bson-fromjson > "/tmp/wpa_statusd_req"
socat - UNIX-CONNECT:/tmp/wifi-chan.sok < "/tmp/wpa_statusd_req" | bsondump --quiet | json-pretty

