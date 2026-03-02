use components::playlist_detail::PlaylistDetail;
use dioxus::prelude::*;
use player::player;
use reader::{Library, PlaylistStore};

#[component]
pub fn PlaylistsPage(
    playlist_store: Signal<PlaylistStore>,
    library: Signal<Library>,
    config: Signal<config::AppConfig>,
    player: Signal<player::Player>,
    mut is_playing: Signal<bool>,
    mut current_playing: Signal<u64>,
    mut current_song_cover_url: Signal<String>,
    mut current_song_title: Signal<String>,
    mut current_song_artist: Signal<String>,
    mut current_song_duration: Signal<u64>,
    mut current_song_progress: Signal<u64>,
    mut queue: Signal<Vec<reader::models::Track>>,
    mut current_queue_index: Signal<usize>,
    mut selected_playlist_id: Signal<Option<String>>,
) -> Element {
    let store = playlist_store.read();
    let mut has_fetched_jellyfin_playlists = use_signal(|| false);
    let mut last_source = use_signal(|| config.read().active_source.clone());

    if *last_source.read() != config.read().active_source {
        selected_playlist_id.set(None);
        last_source.set(config.read().active_source.clone());
    }

    use_effect(move || {
        let is_jellyfin = config.read().active_source == config::MusicSource::Jellyfin;
        if is_jellyfin && !*has_fetched_jellyfin_playlists.read() {
            has_fetched_jellyfin_playlists.set(true);
            spawn(async move {
                let (server_config, device_id) = {
                    let conf = config.peek();
                    if let Some(server) = &conf.server {
                        if let (Some(token), Some(user_id)) =
                            (&server.access_token, &server.user_id)
                        {
                            (
                                Some((server.url.clone(), token.clone(), user_id.clone())),
                                conf.device_id.clone(),
                            )
                        } else {
                            (None, conf.device_id.clone())
                        }
                    } else {
                        (None, conf.device_id.clone())
                    }
                };

                if let Some((url, token, user_id)) = server_config {
                    let remote = server::jellyfin::JellyfinRemote::new(
                        &url,
                        Some(&token),
                        &device_id,
                        Some(&user_id),
                    );
                    if let Ok(playlists) = remote.get_playlists().await {
                        let mut jelly_playlists = Vec::new();
                        for p in playlists {
                            if let Ok(items) = remote.get_playlist_items(&p.id).await {
                                let tracks: Vec<String> =
                                    items.into_iter().map(|item| item.id).collect();
                                jelly_playlists.push(reader::models::JellyfinPlaylist {
                                    id: p.id.clone(),
                                    name: p.name.clone(),
                                    tracks,
                                });
                            } else {
                                jelly_playlists.push(reader::models::JellyfinPlaylist {
                                    id: p.id.clone(),
                                    name: p.name.clone(),
                                    tracks: vec![],
                                });
                            }
                        }
                        let mut store_write = playlist_store.write();
                        store_write.jellyfin_playlists = jelly_playlists;
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: "p-8",
            if let Some(pid) = selected_playlist_id.read().clone() {
                 PlaylistDetail {
                     playlist_id: pid,
                     playlist_store: playlist_store,
                     library: library,
                     config: config,
                     player: player,
                     is_playing: is_playing,
                     current_playing: current_playing,
                     current_song_cover_url: current_song_cover_url,
                     current_song_title: current_song_title,
                     current_song_artist: current_song_artist,
                     current_song_duration: current_song_duration,
                     current_song_progress: current_song_progress,
                     queue: queue,
                     current_queue_index: current_queue_index,
                     on_close: move |_| selected_playlist_id.set(None),
                 }
            } else {
                div { class: "flex items-center justify-between mb-8",
                    h1 { class: "text-3xl font-bold text-white", "Playlists" }
                }

                if (config.read().active_source == config::MusicSource::Local && store.playlists.is_empty())
                    || (config.read().active_source == config::MusicSource::Jellyfin
                        && store.jellyfin_playlists.is_empty())
                {
                    div { class: "flex flex-col items-center justify-center h-64 text-slate-500",
                        i { class: "fa-regular fa-folder-open text-4xl mb-4 opacity-50" }
                        p { "No playlists yet. Add songs from your library!" }
                    }
                } else {
                        div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                            if config.read().active_source == config::MusicSource::Local {
                                {store.playlists.iter().map(|playlist| {
                                    let cover_url = if let Some(first_track_path) = playlist.tracks.first() {
                                        let lib = library.peek();
                                        lib.tracks.iter()
                                            .find(|t| t.path == *first_track_path)
                                            .and_then(|t| {
                                                lib.albums.iter()
                                                    .find(|a| a.id == t.album_id)
                                                    .and_then(|a| a.cover_path.as_ref())
                                                    .and_then(|cp| utils::format_artwork_url(Some(cp)))
                                            })
                                    } else {
                                        None
                                    };
                                    rsx! {
                                        div {
                                            key: "{playlist.id}",
                                            class: "bg-white/5 border border-white/5 rounded-2xl p-6 hover:bg-white/10 transition-all cursor-pointer group",
                                            onclick: {
                                                let id = playlist.id.clone();
                                                move |_| selected_playlist_id.set(Some(id.clone()))
                                            },
                                            div {
                                                class: "mb-4 w-full aspect-square rounded-xl flex items-center justify-center overflow-hidden transition-all bg-white/5",
                                                if let Some(url) = cover_url {
                                                    img { src: "{url}", class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" }
                                                } else {
                                                    div {
                                                        class: "w-full h-full flex items-center justify-center",
                                                        style: "background: color-mix(in srgb, var(--color-indigo-500), transparent 80%); color: var(--color-indigo-400)",
                                                        i { class: "fa-solid fa-list-ul text-2xl" }
                                                    }
                                                }
                                            }
                                            h3 { class: "text-xl font-bold text-white mb-1 truncate", "{playlist.name}" }
                                            p { class: "text-sm text-slate-400", "{playlist.tracks.len()} tracks" }
                                        }
                                    }
                                })}
                            }
                            if config.read().active_source == config::MusicSource::Jellyfin {
                                {store.jellyfin_playlists.iter().map(|playlist| {
                                    let cover_url = if let Some(first_track_id) = playlist.tracks.first() {
                                        let lib = library.peek();
                                        lib.jellyfin_tracks.iter()
                                            .find(|t| t.path.to_string_lossy().contains(first_track_id))
                                            .and_then(|t| {
                                                let conf = config.peek();
                                                if let Some(server) = &conf.server {
                                                    let path_str = t.path.to_string_lossy();
                                                    let parts: Vec<&str> = path_str.split(':').collect();
                                                    if parts.len() >= 2 {
                                                        let id = parts[1];
                                                        let mut url = format!("{}/Items/{}/Images/Primary", server.url, id);
                                                        if let Some(token) = &server.access_token {
                                                            url.push_str(&format!("?api_key={}", token));
                                                        }
                                                        Some(url)
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            })
                                    } else {
                                        None
                                    };

                                    rsx! {
                                        div {
                                            key: "{playlist.id}",
                                            class: "bg-white/5 border border-white/5 rounded-2xl p-6 hover:bg-white/10 transition-all cursor-pointer group",
                                            onclick: {
                                                let id = playlist.id.clone();
                                                move |_| selected_playlist_id.set(Some(id.clone()))
                                            },
                                            div {
                                                class: "mb-4 w-full aspect-square rounded-xl flex items-center justify-center overflow-hidden transition-all bg-white/5",
                                                if let Some(url) = cover_url {
                                                    img { src: "{url}", class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" }
                                                } else {
                                                    div {
                                                        class: "w-full h-full flex items-center justify-center",
                                                        style: "background: color-mix(in srgb, var(--color-indigo-500), transparent 80%); color: var(--color-indigo-400)",
                                                        i { class: "fa-solid fa-server text-2xl" }
                                                    }
                                                }
                                            }
                                            h3 { class: "text-xl font-bold text-white mb-1 truncate", "{playlist.name}" }
                                            p { class: "text-sm text-slate-400", "Jellyfin • {playlist.tracks.len()} tracks" }
                                        }
                                    }
                                })}
                            }
                        }
                }
            }
        }
    }
}
