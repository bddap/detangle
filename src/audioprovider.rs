#[async_trait::async_trait]
pub trait AudioProvider {
    async fn lookup(&self, track: &TrackId) -> anyhow::Result<Option<Track>>;

    /// whether the track can be played from this source without a requiring a separate
    /// license
    ///
    /// as of this writing, most streaming services bundle licensing with provisioning
    /// this function allows an implementer to specify whether the service does that
    async fn license_required(&self, track: &TrackId) -> anyhow::Result<bool>;
}

// a fully downloaded and uncompressed track
// will probably need to add seeking and compression in the future
pub struct Track {
    pub sample_rate: u16,
    pub samples: Vec<i16>,
}

#[derive(Hash, PartialEq, Eq)]
pub struct TrackId {
    pub isrc: Option<Isrc>,
    pub search: Option<String>,
    pub sha256: Option<[u8; 32]>,
}

// https://en.wikipedia.org/wiki/International_Standard_Recording_Code
#[derive(Hash, PartialEq, Eq)]
pub struct Isrc {
    pub country_code: [char; 2],
    pub registrant_code: [char; 3],
    pub year_of_reference: [char; 2],
    pub designation_code: [char; 5],
}

pub struct Search {
    pub track_name: String,
    pub album_name: Option<String>,
    pub artist_name: Option<String>,
    pub release_year: Option<i32>,
}
