use std::collections::HashSet;

use audioprovider::{AudioProvider, TrackId};
use library::{Library, Rating};
use license::License;

use crate::license::MaybeLicense;

pub mod audioprovider;
pub mod library;
pub mod license;

/// get a random track to play
pub async fn shuffle(
    lib: impl Library,
    lic: impl License,
    aud: impl AudioProvider,
) -> anyhow::Result<audioprovider::Track> {
    let mut track_ids = HashSet::<TrackId>::new();
    let dislikes: HashSet<TrackId> = lib
        .ratings()
        .await?
        .into_iter()
        .filter(|(_track, rating)| rating != &Rating::Downdoot)
        .map(|(track, _)| track)
        .collect();
    for pl in lib.playlists().await? {
        for track in lib.lookup(&pl).await?.songs {
            if !dislikes.contains(&track) {
                track_ids.insert(track);
            }
        }
    }
    // track_ids is probably already in random-ish order because it is a hashmap
    for track_id in track_ids {
        let license_required = aud.license_required(&track_id).await?;
        if license_required && !lic.can_license(&track_id).await? {
            // try the next song
            continue;
        }
        let track = match aud.lookup(&track_id).await? {
            Some(track) => track,
            None => continue,
        };
        if license_required && lic.take_license(&track_id).await? == MaybeLicense::CantLicense {
            // unexpected, can_license was called earlier
            // try the next song
            continue;
        }
        return Ok(track);
    }
    anyhow::bail!("found no licensable tracks");
}

// Notes
//
// albums can be identified by UPC or EAN
//
// https://github.com/sigma67/ytmusicapi
