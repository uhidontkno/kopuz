pub mod api;
pub mod auth;
pub mod signin;
pub mod types;

pub use api::AppleMusicApi;

use config::MusicService;
use reader::models::{TrackId, Track};

pub fn apple_music_id(adam_id: impl Into<String>) -> TrackId {
    TrackId::Server {
        service: MusicService::AppleMusic,
        item_id: adam_id.into(),
    }
}

pub fn artwork_url(template: &str, size: u32) -> String {
    template
        .replace("{w}", &size.to_string())
        .replace("{h}", &size.to_string())
}

/// Convert a catalog track response to a reader::Track.
pub fn track_from_song_data(song: &types::TrackData) -> Track {
    let cover = if !song.attributes.artwork.url.is_empty() {
        Some(artwork_url(&song.attributes.artwork.url, 600))
    } else {
        None
    };

    let artist = if !song.relationships.artists.data.is_empty() {
        song.relationships
            .artists
            .data
            .iter()
            .map(|a| {
                a.attributes
                    .as_ref()
                    .map(|att| att.name.as_str())
                    .unwrap_or("Unknown Artist")
            })
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        song.attributes.artist_name.clone()
    };

    let artists = if song.relationships.artists.data.is_empty() {
        vec![song.attributes.artist_name.clone()]
    } else {
        song.relationships
            .artists
            .data
            .iter()
            .map(|a| {
                a.attributes
                    .as_ref()
                    .map(|att| att.name.clone())
                    .unwrap_or_else(|| "Unknown Artist".to_string())
            })
            .collect()
    };

    let album_id = song
        .relationships
        .albums
        .data
        .first()
        .map(|a| format!("applemusic:{}", a.id))
        .unwrap_or_default();

    Track {
        id: apple_music_id(&song.id),
        cover,
        album_id,
        title: song.attributes.name.clone(),
        artist,
        album: song.attributes.album_name.clone(),
        duration: song.attributes.durationInMillis / 1000,
        khz: 0,
        bitrate: 0,
        track_number: Some(song.attributes.trackNumber),
        disc_number: Some(song.attributes.discNumber),
        musicbrainz_release_id: None,
        musicbrainz_recording_id: None,
        musicbrainz_track_id: None,
        playlist_item_id: None,
        artists,
    }
}

/// Convert a library song resource to a reader::Track.
pub fn track_from_library_song(song: &types::LibrarySongResource) -> Track {
    let cover = song
        .attributes
        .artwork
        .as_ref()
        .filter(|a| !a.url.is_empty())
        .map(|a| artwork_url(&a.url, 600));

    Track {
        id: apple_music_id(&song.id),
        cover,
        album_id: String::new(),
        title: song.attributes.name.clone(),
        artist: song.attributes.artistName.clone(),
        album: song.attributes.albumName.clone(),
        duration: song.attributes.durationInMillis / 1000,
        khz: 0,
        bitrate: 0,
        track_number: Some(song.attributes.trackNumber),
        disc_number: Some(song.attributes.discNumber),
        musicbrainz_release_id: None,
        musicbrainz_recording_id: None,
        musicbrainz_track_id: None,
        playlist_item_id: None,
        artists: vec![song.attributes.artistName.clone()],
    }
}

/// Convert a library album resource to a reader::Album.
pub fn album_from_library(album: &types::LibraryAlbumResource) -> reader::Album {
    reader::Album {
        id: format!("applemusic:{}", album.id),
        title: album.attributes.name.clone(),
        artist: album.attributes.artistName.clone(),
        genre: album.attributes.genreNames.join(", "),
        year: album
            .attributes
            .releaseDate
            .split('-')
            .next()
            .and_then(|y| y.parse().ok())
            .unwrap_or(0),
        cover_path: album
            .attributes
            .artwork
            .as_ref()
            .filter(|a| !a.url.is_empty())
            .map(|a| std::path::PathBuf::from(format!(
                "applemusic:{}:{}",
                album.id,
                artwork_url(&a.url, 600)
            ))),
        manual_cover: false,
    }
}
