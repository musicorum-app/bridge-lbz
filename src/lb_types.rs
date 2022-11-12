use std::marker::PhantomData;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ValidateToken<'a> {
    pub code: u16,
    pub user_name: &'a str,
    pub message: &'a str,
    pub valid: bool
}

#[derive(Debug)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct APIResponse<'a> {
    pub code: u16,
    pub status: Option<&'a str>,
    pub message: &'a str
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SubmitListens<'a> {
    pub listen_type: String,
    pub payload: Vec<Listen<'a>>
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Listen<'a> {
    pub listened_at: Option<u64>,
    pub track_metadata: TrackMetadata<'a>
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TrackMetadata<'a> {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: String,
    pub additional_info: Option<AdditionalInfo<'a>>
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AdditionalInfo<'r> {
    pub recording_mbid: Option<String>,
    pub release_mbid: Option<String>,
    pub artist_mbids: Option<Vec<String>>,
    pub tags: Option<Vec<String>>, 
    pub isrc: Option<String>,
    pub spotify_id: Option<String>,
    pub duration_ms: Option<u64>,
    phantom: PhantomData<&'r ()>
}