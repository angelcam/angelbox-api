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

use std::{io, process::Stdio};

use actix_web::{web::Bytes, Error, ResponseError};
use futures::{Stream, StreamExt};
use tokio::{io::AsyncWriteExt, process::Command};

/// Play a given audio stream.
///
/// The audio stream is expected to be in mu-Law encoding with a single channel
/// and sample rate of 8000.
pub async fn play_audio<S, E>(mut stream: S) -> Result<(), Error>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: ResponseError + 'static,
{
    log::info!("Starting audio playback");

    let mut child = Command::new("aplay")
        .args(&["-t", "raw", "-f", "MU_LAW", "-c", "1", "-r", "8000", "-q"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    log::info!("Audio playback started");

    while let Some(chunk) = stream.next().await.transpose()? {
        stdin.write_all(&*chunk).await?;
    }

    stdin.shutdown().await?;

    // the pipe won't get closed until it's dropped
    std::mem::drop(stdin);

    let status = child.wait().await?;

    log::info!("Audio playback stopped");

    if status.success() {
        Ok(())
    } else {
        Err(Error::from(io::Error::new(
            io::ErrorKind::Other,
            "audio playback failed",
        )))
    }
}
