use crate::audioprovider::TrackId;

#[async_trait::async_trait]
pub trait License {
    async fn can_license(&self, tid: &TrackId) -> anyhow::Result<bool>;

    /// if needed, notify the service that the user is playing the track
    ///
    /// for example, an implementer that verifies which songs are in the public
    /// domain would return Ok(()) if the
    async fn take_license(&self, tid: &TrackId) -> anyhow::Result<MaybeLicense>;
}

#[derive(PartialEq, Eq)]
pub enum MaybeLicense {
    /// Success, the user may play the song once
    Yes,
    /// not capable of granting that license
    CantLicense,
}
