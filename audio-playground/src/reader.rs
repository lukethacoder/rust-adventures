use log::{error, info, trace, warn};
use std::fs::Metadata;
use std::path::Path;

use audiotags::{AudioTag, Tag};
use id3;
use mpeg_audio_header::{Header, ParseMode};

use crate::schema::TrackJson;
use crate::utils::{file_ext, norm};

pub fn get_duration_for_path(path_string: &String) -> Option<f64> {
    // Duration is usually not stored in ID3 tags, so lets calculate it from the audio file itself
    let path = Path::new(&path_string);
    let ext = file_ext(&path_string);

    // `wav` files don't seem to play nice here, so just ignore them for now
    if ext != "wav" {
        let header = Header::read_from_path(&path, ParseMode::PreferVbrHeaders);
        if header.is_err() {
            error!("Error fetching duration for {:?}", &path);
        } else {
            return Some(header.unwrap().total_duration.as_secs_f64());
        }
    } else {
        warn!("Skipping wav file duration fetching");
    }

    return None;
}

pub fn get_track_from_path(path_string: &String) -> Option<TrackJson> {
    let path: &Path = Path::new(&path_string);
    return get_track_from_path_instance(&path_string, &path);
}

pub fn get_track_from_path_instance(path_string: &String, path: &Path) -> Option<TrackJson> {
    let metadata: Metadata = path.metadata().unwrap();
    let ext: &str = file_ext(&path_string);

    // audiotags does not support wav files, so we must handle them directly with the ID3 package
    if ext == "wav" {
        let tag: id3::Tag = id3::Tag::read_from_wav_path(&path_string).unwrap();
        return Some(TrackJson::new_wav(norm(&path_string), metadata, tag));
    } else {
        let tag: Result<Box<dyn AudioTag>, _> = Tag::new().read_from_path(&path_string);
        if tag.is_err() {
            error!("Error parsing tag {:?}", &path_string);
        }
        return Some(TrackJson::new(norm(&path_string), metadata, tag.unwrap()));
    }
}
