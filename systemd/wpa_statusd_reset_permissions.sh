#!/bin/sh
socket=$(cat "/etc/wpa_statusd.ini" | grep "^socket =" | awk -F "="  '{print $2}' | tr -d ' ')
chmod 777 "$socket"
