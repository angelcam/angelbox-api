// Copyright 2021 Angelcam, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{
    http::{header, StatusCode},
    web, Error, HttpRequest, HttpResponse, Responder,
};

use crate::speaker;

/// API root.
async fn root(request: HttpRequest) -> impl Responder {
    let play_audio_url = request
        .url_for_static("play-audio")
        .unwrap()
        .path()
        .to_string();

    web::Json(serde_json::json!({
        "device": "AngelBox",
        "vendor": "Angelcam",
        /*"client": {
            "version": "TODO",
            "uuid": "TODO",
        },*/
        "api": {
            "version": env!("CARGO_PKG_VERSION"),
            "capabilities": {
                "speaker": {
                    "formats": {
                        "audio/basic": {
                            "endpoint": play_audio_url,
                            "codecs": [
                                "pcm_ulaw"
                            ]
                        }
                    }
                }
            }
        }
    }))
}

/// Handler for the audio playback endpoint.
async fn play_audio(
    content_type: web::Header<header::ContentType>,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    if !content_type
        .essence_str()
        .eq_ignore_ascii_case("audio/basic")
    {
        return Ok(HttpResponse::new(StatusCode::UNSUPPORTED_MEDIA_TYPE));
    }

    let play = speaker::play_audio(body);

    match play.await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::NO_CONTENT)),
        Err(err) => {
            let err = err.as_response_error();

            if err.status_code() == StatusCode::INTERNAL_SERVER_ERROR {
                log::warn!("Audio playback error: {}", err);

                Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
            } else {
                Ok(err.error_response())
            }
        }
    }
}

/// Configure the API service.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(root)))
        .service(
            web::resource("/audio/play")
                .name("play-audio")
                .route(web::post().to(play_audio)),
        );
}
