use crate::audioprovider::TrackId;

#[async_trait::async_trait]
pub trait Library {
    async fn playlists(&self) -> anyhow::Result<Vec<PlaylistId>>;
    async fn lookup(&self, pid: &PlaylistId) -> anyhow::Result<Playlist>;
    async fn ratings(&self) -> anyhow::Result<Vec<(TrackId, Rating)>>;
    async fn rating_for(&self, track: &TrackId) -> anyhow::Result<Option<Rating>>;
}

pub struct PlaylistId {
    pub name: String,
}

pub struct Playlist {
    pub songs: Vec<TrackId>,
}

#[derive(PartialEq, Eq)]
pub enum Rating {
    Updoot,
    Downdoot,
}
