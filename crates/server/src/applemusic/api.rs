use reqwest::Client;

use super::types::*;
use super::auth;

const BASE: &str = "https://amp-api.music.apple.com";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub struct AppleMusicApi {
    http: Client,
    media_user_token: Option<String>,
    storefront: String,
    language: String,
}

impl AppleMusicApi {
    pub fn new(
        media_user_token: Option<String>,
        storefront: impl Into<String>,
        language: impl Into<String>,
    ) -> Self {
        Self {
            http: Client::new(),
            media_user_token,
            storefront: storefront.into(),
            language: language.into(),
        }
    }

    async fn get(&self, path: &str) -> Result<reqwest::Response, String> {
        let bearer = auth::get_bearer_token().await?;
        let url = format!("{BASE}{path}");
        let mut req = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {bearer}"))
            .header("User-Agent", USER_AGENT)
            .header("Origin", "https://music.apple.com");

        if let Some(token) = &self.media_user_token {
            req = req.header("x-apple-music-user-token", token.as_str());
        }

        req.send()
            .await
            .map_err(|e| format!("GET {path}: {e}"))
    }

    async fn post(&self, path: &str, body: &serde_json::Value) -> Result<reqwest::Response, String> {
        let bearer = auth::get_bearer_token().await?;
        let url = format!("{BASE}{path}");
        let mut req = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {bearer}"))
            .header("User-Agent", USER_AGENT)
            .header("Origin", "https://music.apple.com")
            .header("Content-Type", "application/json")
            .json(body);

        if let Some(token) = &self.media_user_token {
            req = req.header("x-apple-music-user-token", token.as_str());
        }

        req.send()
            .await
            .map_err(|e| format!("POST {path}: {e}"))
    }

    async fn delete(&self, path: &str) -> Result<reqwest::Response, String> {
        let bearer = auth::get_bearer_token().await?;
        let url = format!("{BASE}{path}");
        let mut req = self
            .http
            .delete(&url)
            .header("Authorization", format!("Bearer {bearer}"))
            .header("User-Agent", USER_AGENT)
            .header("Origin", "https://music.apple.com");

        if let Some(token) = &self.media_user_token {
            req = req.header("x-apple-music-user-token", token.as_str());
        }

        req.send()
            .await
            .map_err(|e| format!("DELETE {path}: {e}"))
    }

    pub async fn get_song(&self, id: &str) -> Result<TrackData, String> {
        let path = format!(
            "/v1/catalog/{}/songs/{}?include=albums,artists&extend=extendedAssetUrls&l={}",
            self.storefront, id, self.language
        );
        let resp = self.get(&path).await?;
        if !resp.status().is_success() {
            return Err(format!("get_song {id}: HTTP {}", resp.status()));
        }
        let song: SongResp = resp.json().await.map_err(|e| format!("parse song: {e}"))?;
        song.data
            .into_iter()
            .next()
            .ok_or_else(|| format!("song {id} not found"))
    }

    pub async fn get_album(&self, id: &str) -> Result<AlbumData, String> {
        let path = format!(
            "/v1/catalog/{}/albums/{}?include=tracks,artists&extend=extendedAssetUrls&l={}",
            self.storefront, id, self.language
        );
        let resp = self.get(&path).await?;
        if !resp.status().is_success() {
            return Err(format!("get_album {id}: HTTP {}", resp.status()));
        }
        let album: AlbumResp = resp.json().await.map_err(|e| format!("parse album: {e}"))?;
        album
            .data
            .into_iter()
            .next()
            .ok_or_else(|| format!("album {id} not found"))
    }

    pub async fn get_playlist(&self, id: &str) -> Result<PlaylistData, String> {
        let path = format!(
            "/v1/catalog/{}/playlists/{}?include=tracks,artists&extend=extendedAssetUrls&l={}",
            self.storefront, id, self.language
        );
        let resp = self.get(&path).await?;
        if !resp.status().is_success() {
            return Err(format!("get_playlist {id}: HTTP {}", resp.status()));
        }
        let pl: PlaylistResp =
            resp.json().await.map_err(|e| format!("parse playlist: {e}"))?;
        pl.data
            .into_iter()
            .next()
            .ok_or_else(|| format!("playlist {id} not found"))
    }

    pub async fn search(
        &self,
        term: &str,
        types: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchResp, String> {
        let path = format!(
            "/v1/catalog/{}/search?term={}&types={}&limit={}&offset={}&l={}",
            self.storefront,
            urlencoding::encode(term),
            urlencoding::encode(types),
            limit,
            offset,
            self.language,
        );
        let resp = self.get(&path).await?;
        if !resp.status().is_success() {
            return Err(format!("search: HTTP {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("parse search: {e}"))
    }

    pub async fn get_library_albums(&self) -> Result<Vec<LibraryAlbumData>, String> {
        let mut all = Vec::new();
        let mut next = Some("/v1/me/library/albums?limit=100&l=en".to_string());
        while let Some(path) = next.take() {
            let resp = self.get(&path).await?;
            if !resp.status().is_success() {
                return Err(format!("library albums: HTTP {}", resp.status()));
            }
            let page: LibraryAlbumResp =
                resp.json().await.map_err(|e| format!("parse library albums: {e}"))?;
            all.extend(page.data);
            if page.next.is_empty() {
                break;
            }
            // next is a relative path like "/v1/me/library/albums?offset=100&..."
            next = Some(page.next);
        }
        Ok(all)
    }

    pub async fn get_library_songs(&self) -> Result<Vec<LibrarySongData>, String> {
        let mut all = Vec::new();
        let mut next = Some("/v1/me/library/songs?limit=100&l=en".to_string());
        while let Some(path) = next.take() {
            let resp = self.get(&path).await?;
            if !resp.status().is_success() {
                return Err(format!("library songs: HTTP {}", resp.status()));
            }
            let page: LibrarySongResp =
                resp.json().await.map_err(|e| format!("parse library songs: {e}"))?;
            all.extend(page.data);
            if page.next.is_empty() {
                break;
            }
            next = Some(page.next);
        }
        Ok(all)
    }

    pub async fn get_library_playlists(&self) -> Result<Vec<LibraryPlaylistData>, String> {
        let mut all = Vec::new();
        let mut next = Some("/v1/me/library/playlists?limit=100".to_string());
        while let Some(path) = next.take() {
            let resp = self.get(&path).await?;
            if !resp.status().is_success() {
                return Err(format!("library playlists: HTTP {}", resp.status()));
            }
            let page: LibraryPlaylistResp = resp
                .json()
                .await
                .map_err(|e| format!("parse library playlists: {e}"))?;
            all.extend(page.data);
            if page.next.is_empty() {
                break;
            }
            next = Some(page.next);
        }
        Ok(all)
    }

    pub async fn get_library_playlist_tracks(
        &self,
        playlist_id: &str,
    ) -> Result<Vec<TrackData>, String> {
        let mut all = Vec::new();
        let mut next = Some(format!(
            "/v1/me/library/playlists/{}/tracks?limit=100",
            playlist_id
        ));
        while let Some(path) = next.take() {
            let resp = self.get(&path).await?;
            if !resp.status().is_success() {
                return Err(format!("library playlist tracks: HTTP {}", resp.status()));
            }
            let page: TrackResp = resp
                .json()
                .await
                .map_err(|e| format!("parse library playlist tracks: {e}"))?;
            all.extend(page.data);
            if page.next.is_empty() {
                break;
            }
            next = Some(page.next);
        }
        Ok(all)
    }

    pub async fn add_to_library(&self, item_id: &str) -> Result<(), String> {
        let body = serde_json::json!({
            "id": item_id,
            "type": "songs",
        });
        let resp = self.post("/v1/me/library", &body).await?;
        if !resp.status().is_success() {
            return Err(format!("add_to_library: HTTP {}", resp.status()));
        }
        Ok(())
    }

    pub async fn remove_from_library(&self, item_id: &str) -> Result<(), String> {
        let resp = self
            .delete(&format!("/v1/me/library/songs/{}", item_id))
            .await?;
        if !resp.status().is_success() {
            return Err(format!("remove_from_library: HTTP {}", resp.status()));
        }
        Ok(())
    }

    pub async fn create_playlist(
        &self,
        name: &str,
        item_refs: &[String],
    ) -> Result<String, String> {
        let mut attributes = serde_json::json!({ "name": name });
        if !item_refs.is_empty() {
            let mut tracks = Vec::new();
            for id in item_refs {
                tracks.push(serde_json::json!({ "id": id }));
            }
            attributes["tracks"] = serde_json::json!({ "data": tracks });
        }
        let body = serde_json::json!({
            "attributes": attributes,
        });
        let resp = self.post("/v1/me/library/playlists", &body).await?;
        if !resp.status().is_success() {
            return Err(format!("create_playlist: HTTP {}", resp.status()));
        }
        let val: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        val["data"][0]["id"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| "no id in playlist create response".to_string())
    }

    pub async fn add_to_playlist(
        &self,
        playlist_id: &str,
        item_refs: &[String],
    ) -> Result<(), String> {
        let mut tracks = Vec::new();
        for id in item_refs {
            tracks.push(serde_json::json!({ "id": id }));
        }
        let body = serde_json::json!({
            "data": tracks,
        });
        let resp = self
            .post(
                &format!("/v1/me/library/playlists/{}/tracks", playlist_id),
                &body,
            )
            .await?;
        if !resp.status().is_success() {
            return Err(format!("add_to_playlist: HTTP {}", resp.status()));
        }
        Ok(())
    }

    pub async fn remove_from_playlist(
        &self,
        playlist_id: &str,
        track_ids: &[String],
    ) -> Result<(), String> {
        let mut tracks = Vec::new();
        for id in track_ids {
            tracks.push(serde_json::json!({ "id": id }));
        }
        // Note: DELETE with body may not be supported; Apple Music uses a
        // different endpoint for removal. If this fails, we'll need to use
        // the PATCH approach instead. For now, we try the standard approach.
        let _body = serde_json::json!({ "data": tracks });
        let resp = self
            .delete(&format!(
                "/v1/me/library/playlists/{}/tracks",
                playlist_id
            ))
            .await?;
        // Note: DELETE with body may not be supported; Apple Music uses a
        // different endpoint for removal. If this fails, we'll need to use
        // the PATCH approach instead. For now, we try the standard approach.
        if !resp.status().is_success() {
            return Err(format!("remove_from_playlist: HTTP {}", resp.status()));
        }
        Ok(())
    }

    pub async fn validate(&self) -> Result<(), String> {
        let token = self
            .media_user_token
            .as_deref()
            .ok_or("no media user token")?;
        let bearer = auth::get_bearer_token().await?;
        let resp = self
            .http
            .get(format!("{BASE}/v1/me/library"))
            .header("Authorization", format!("Bearer {bearer}"))
            .header("User-Agent", USER_AGENT)
            .header("Origin", "https://music.apple.com")
            .header("x-apple-music-user-token", token)
            .send()
            .await
            .map_err(|e| format!("validate: {e}"))?;
        if resp.status().is_success() {
            Ok(())
        } else if resp.status().as_u16() == 401 {
            Err("expired".to_string())
        } else {
            Err(format!("HTTP {}", resp.status()))
        }
    }
}
