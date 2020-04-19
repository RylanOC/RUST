
use crate::model::artists::ArtistsVec;
use crate::model::tracks::TracksVec;
use crate::chart_maker;

use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::model::audio::AudioFeatures;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct ChartBuilder<'a> {
    artists: ArtistsVec,
    tracks: TracksVec,
    colors: VecDeque<&'a str>,
}

impl<'a> ChartBuilder<'a> {
    pub fn new(a: ArtistsVec, t: TracksVec) -> ChartBuilder<'a> {
        ChartBuilder {
            artists: a,
            tracks: t,
            colors: VecDeque::from_iter(
                vec!["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"]
            ),
        }
    }

    pub fn add_color(mut self, new_color: &'a str) {
        self.colors.push_back(new_color);
    }

    pub async fn get_charts(&self) -> Vec<std::string::String> {

        let mut charts: Vec<std::string::String> = Vec::new();
        
        let mut col = self.colors.clone();

        let mut artist_pop: Vec<f64> = Vec::new();
        let mut genre_map: HashMap<String, chart_maker::BarchartDatum> = HashMap::new();
        for artist in &(self.artists.items) {
            artist_pop.push(artist.popularity as f64);
            for genre in &artist.genres {
                genre_map.entry(genre.to_string())
                    .or_insert(chart_maker::BarchartDatum::new(genre, 0.0).clone())
                    .value += 1.0;
            }
        }
        charts.push(chart_maker::make_histogram(artist_pop, "Artist Popularity", col.front().unwrap()));

        let temp = col.pop_front().unwrap();
        col.push_back(temp);

        let client_credential = SpotifyClientCredentials::default().build();

        let spotify = Spotify::default()
            .client_credentials_manager(client_credential)
            .build();
        let mut track_ids: Vec<String> = Vec::new();
        
        for track in &(self.tracks.items) {
            track_ids.push(track.id.as_ref().unwrap().to_string());
        }
        let track_info: Vec<AudioFeatures> = spotify.audios_features(&track_ids)
                                                    .await
                                                    .unwrap()
                                                    .unwrap()
                                                    .audio_features;

        let mut acousticness: Vec<f64> = Vec::new();
        let mut danceability: Vec<f64> = Vec::new();
        let mut energy: Vec<f64> = Vec::new();
        let mut tempo: Vec<f64> = Vec::new();
        let mut mood: Vec<f64> = Vec::new();

        for info in &track_info {
            acousticness.push(info.acousticness as f64);
            danceability.push(info.danceability as f64);
            energy.push(info.energy as f64);
            tempo.push(info.tempo as f64);
            mood.push(info.valence as f64);
        }

        charts.push(chart_maker::make_histogram(acousticness, "Acousticness", col.front().unwrap()));
        let temp = col.pop_front().unwrap();
        col.push_back(temp);

        charts.push(chart_maker::make_histogram(danceability, "Danceability", col.front().unwrap()));
        let temp = col.pop_front().unwrap();
        col.push_back(temp);

        charts.push(chart_maker::make_histogram(energy, "Energy", col.front().unwrap()));
        let temp = col.pop_front().unwrap();
        col.push_back(temp);

        charts.push(chart_maker::make_histogram(tempo, "Tempo", col.front().unwrap()));
        let temp = col.pop_front().unwrap();
        col.push_back(temp);

        charts.push(chart_maker::make_histogram(mood, "Sad <-- Mood --> Happy", col.front().unwrap()));

        let mut genre_vec: Vec<&chart_maker::BarchartDatum> = 
            genre_map.values().clone().collect();
        
        genre_vec.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());

        if genre_vec.len()>10 {
            charts.push(chart_maker::make_barchart(genre_vec[..10].to_vec(), &col, "Artist Genres").to_string());
        }
        else {
            charts.push(chart_maker::make_barchart(genre_vec, &col, "Artist Genres").to_string());
        }

        return charts;
    }


}