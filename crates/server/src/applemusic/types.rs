use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Artwork {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlayParams {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArtistRef {
    pub id: String,
    #[serde(default)]
    pub attributes: Option<ArtistRefAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArtistRefAttributes {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumRef {
    pub id: String,
    #[serde(default)]
    pub attributes: Option<AlbumRefAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumRefAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artist_name: String,
    #[serde(default)]
    pub artwork: Option<Artwork>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub track_type: String,
    #[serde(default)]
    pub attributes: TrackAttributes,
    #[serde(default)]
    pub relationships: TrackRelationships,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artist_name: String,
    #[serde(default)]
    pub album_name: String,
    #[serde(default)]
    pub artwork: Artwork,
    #[serde(default)]
    pub durationInMillis: u64,
    #[serde(default)]
    pub trackNumber: u32,
    #[serde(default)]
    pub discNumber: u32,
    #[serde(default)]
    pub genreNames: Vec<String>,
    #[serde(default)]
    pub releaseDate: String,
    #[serde(default)]
    pub isrc: String,
    #[serde(default)]
    pub audioTraits: Vec<String>,
    #[serde(default)]
    pub contentRating: String,
    #[serde(default)]
    pub playParams: Option<PlayParams>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackRelationships {
    #[serde(default)]
    pub artists: RelationshipData<Vec<ArtistRef>>,
    #[serde(default)]
    pub albums: RelationshipData<Vec<AlbumRef>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RelationshipData<T> {
    #[serde(default)]
    pub data: T,
    #[serde(default)]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackResp {
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<TrackData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SongResp {
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<TrackData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub album_type: String,
    #[serde(default)]
    pub attributes: AlbumAttributes,
    #[serde(default)]
    pub relationships: AlbumRelationships,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artist_name: String,
    #[serde(default)]
    pub artwork: Artwork,
    #[serde(default)]
    pub genreNames: Vec<String>,
    #[serde(default)]
    pub trackCount: u32,
    #[serde(default)]
    pub releaseDate: String,
    #[serde(default)]
    pub copyright: String,
    #[serde(default)]
    pub upc: String,
    #[serde(default)]
    pub isSingle: bool,
    #[serde(default)]
    pub isCompilation: bool,
    #[serde(default)]
    pub playParams: Option<PlayParams>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumRelationships {
    #[serde(default)]
    pub artists: RelationshipData<Vec<ArtistRef>>,
    #[serde(default)]
    pub tracks: TrackResp,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AlbumResp {
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<AlbumData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlaylistData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub playlist_type: String,
    #[serde(default)]
    pub attributes: PlaylistAttributes,
    #[serde(default)]
    pub relationships: AlbumRelationships,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlaylistAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artwork: Option<Artwork>,
    #[serde(default)]
    pub description: Option<PlaylistDescription>,
    #[serde(default)]
    pub trackCount: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlaylistDescription {
    #[serde(default)]
    pub short: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlaylistResp {
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<PlaylistData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchResp {
    #[serde(default)]
    pub results: SearchResults,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchResults {
    #[serde(default)]
    pub songs: Option<SearchResultList<TrackData>>,
    #[serde(default)]
    pub albums: Option<SearchResultList<AlbumData>>,
    #[serde(default)]
    pub artists: Option<SearchResultList<ArtistSearchData>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchResultList<T> {
    #[serde(default)]
    pub href: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArtistSearchData {
    pub id: String,
    #[serde(default)]
    pub attributes: ArtistSearchAttributes,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArtistSearchAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub genreNames: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WebPlaybackResp {
    #[serde(default)]
    pub songList: Vec<WebPlaybackSong>,
    #[serde(default)]
    pub status: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WebPlaybackSong {
    #[serde(default)]
    #[serde(rename = "hls-playlist-url")]
    pub hls_playlist_url: String,
    #[serde(default)]
    #[serde(rename = "hls-key-cert-url")]
    pub hls_key_cert_url: String,
    #[serde(default)]
    pub assets: Vec<WebPlaybackAsset>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct WebPlaybackAsset {
    #[serde(default)]
    pub flavor: String,
    #[serde(default)]
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryAlbumResp {
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<LibraryAlbumData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryAlbumData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(default)]
    pub attributes: LibraryAlbumAttributes,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryAlbumAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artistName: String,
    #[serde(default)]
    pub artwork: Artwork,
    #[serde(default)]
    pub trackCount: u32,
    #[serde(default)]
    pub genreNames: Vec<String>,
    #[serde(default)]
    pub releaseDate: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibrarySongResp {
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<LibrarySongData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibrarySongData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(default)]
    pub attributes: LibrarySongAttributes,
    #[serde(default)]
    pub relationships: TrackRelationships,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibrarySongAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artistName: String,
    #[serde(default)]
    pub albumName: String,
    #[serde(default)]
    pub artwork: Artwork,
    #[serde(default)]
    pub durationInMillis: u64,
    #[serde(default)]
    pub trackNumber: u32,
    #[serde(default)]
    pub discNumber: u32,
    #[serde(default)]
    pub genreNames: Vec<String>,
    #[serde(default)]
    pub releaseDate: String,
    #[serde(default)]
    pub playParams: Option<PlayParams>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryPlaylistResp {
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub data: Vec<LibraryPlaylistData>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryPlaylistData {
    pub id: String,
    #[serde(default)]
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(default)]
    pub attributes: LibraryPlaylistAttributes,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryPlaylistAttributes {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub artwork: Option<Artwork>,
    #[serde(default)]
    pub trackCount: u32,
    #[serde(default)]
    pub canEdit: bool,
}
