# AngelBox REST API

Right now, the API is intended to be available only remotely via Angelcam
Cloud. Only authenticated users who have paired their AngelBox with their user
account are authorized to access the AngelBox API. As a result of this, the
AngelBox API itself does not need any authentication mechanism as long as it is
available only on the localhost network interface (communication with Angelcam
Cloud will be tunneled by
[Arrow Client](https://github.com/angelcam/arrow-client)).

## Get device info and API capabilities

```
GET /api/v1/
```

### Response example

```json
{
    "device": "AngelBox",
    "vendor": "Angelcam",
    "api": {
        "version": "0.1.0",
        "capabilities": {
            "speaker": {
                "formats": {
                    "audio/basic": {
                        "endpoint": "/api/v1/audio/play",
                        "codecs": [
                            "pcm_ulaw"
                        ]
                    }
                }
            }
        }
    }
}
```

* `device` - device type, right now the only possible value is "AngelBox"
* `vendor` - who created/sold the device
* `api` - AngelBox API description
    * `version` - version of the application
    * `capabilities` - API capabilities; a dictionary where each key is a
            capability name and each value describes the capability
        * `speaker` - direct audio playback
            * `formats` - acceptable audio stream formats
                * `audio/basic` - single channel mu-Law audio with sample rate
                    of 8000 transferred as an HTTP body within a POST request
                    * `endpoint` - API endpoint accepting the audio format
                    * `codecs` - a list of codecs accepted by the endpoint; the
                        codec names correspond to those used by FFmpeg

## Play audio stream

```
POST /api/v1/audio/play
```

To play an audio stream, simply POST it as a request body. The audio stream
must be a single channel mu-Law audio with sample rate of 8000. The
`Content-Type` header field must be set to `audio/basic`. If the size of the
body is not known in advance, chunked transfer encoding may be used.

### Request example

```text
POST /api/v1/audio/play HTTP/1.1
Host: localhost
Content-Type: audio/basic
Transfer-Encoding: chunked

1000
HERE GOES 4096 BYTES OF AUDIO DATA...
1000
ANOTHER 4096 BYTES OF AUDIO DATA...
...
0

```
