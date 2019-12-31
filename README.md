# WPA_statusd
WPA_statusd is a tiny system daemon which provides a silly little readonly interface to wpa_supplicant.

It is meant to be used in a way where the user starts up wpa_supplicant manually.

## Config
Located at "/etc/wpa_statusd.ini".

### Example

```ini
socket = /tmp/wpa_statusd.sok
wpa_supplicant_socket = /run/wpa_supplicant/wls1
```

## Systemd
Support for systemd is optional, the Makefile has it per default.
You might want to edit the service file, it runs a script upon initialization that makes the socket writable by normal users.

## Building
The Makefile has a bunch of variables which lets you control stuff, most notably:
    - SYSTEMD
        - boolean that controls wether anything systemd related is installed.
    - BIN
        - defaults to "sbin", you might want to change that.

Also:
```sh
make
sudo make install
```

## Protocol
It sends sends BSON around. BSON is very cursed.

There is also a [python client library](https://github.com/AlexFence/wpa_status) available, it should illustrate how to communicate with wpa_statusd.

### Example Request
```json
{
  "type": "Request",
  "method": "PING",
  "id": "bla"
}
```

### Example Response
```json
{
  "type": "Response",
  "id": "bla",
  "method": "PING",
  "result": "Pong"
}
```

### Example Error
```json
{
  "type": "Error",
  "id": null,
  "method": null,
  "code": "MalformedRequest"
}
```

### Commands

It offers the following commands:
- PING
    - Pongs back.
- SUPPLICANT_RUNNING.
    - Checks if wpa_supplicant is running.
- LIST_NETWORKS
    - Lists all configured networks.
    - Forwarded form wpa_supplicant's interface.
- STATUS
    - Returns the current state of the supplicant.
    - Forwarded form wpa_supplicant's interface.


