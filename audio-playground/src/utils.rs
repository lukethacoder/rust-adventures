use std::collections::HashMap;

use crate::schema::{FacetResult, FacetResults, FieldSchema, OrderBy};
use regex::Regex;
use tantivy::{collector::FacetCounts, query::QueryParser, schema::Facet};

pub fn path2name(x: String) -> String {
    norm(&x)
        .as_str()
        .split("/")
        .into_iter()
        .last()
        .map(|x| x.to_string())
        .unwrap_or("".to_string())
}

pub fn adapt_text(parser: &QueryParser, text: &str) -> String {
    match text {
        "" => text.to_string(),
        text => parser
            .parse_query(text)
            .map(|_| text.to_string())
            .unwrap_or_else(|e| {
                println!("Error during parsing query: {e}. Input query: {text}");
                format!("\"{}\"", text.replace('"', ""))
            }),
    }
}

pub fn norm(path: &str) -> String {
    str::replace(path, "\\", "/")
}

pub fn get_genre_regex() -> regex::Regex {
    return Regex::new(r#"[/,;]"#).unwrap();
}

pub fn file_ext(file_name: &str) -> &str {
    if !file_name.contains(".") {
        return "";
    }
    file_name.split(".").last().unwrap_or("")
}

pub fn get_genre_bracket_regex() -> regex::Regex {
    return Regex::new(r"[()]").unwrap();
}

pub fn genre_string_to_vec(genre: &str) -> Vec<String> {
    let genres_as_string = genre.replace("\0", ";");

    // Split the string by "/,;" chars
    let genres_array = get_genre_regex()
        .split(&genres_as_string)
        .collect::<Vec<_>>();

    // Attempt to store values as ID3v1 Genre keys, fallback to the string value
    let genres: Vec<String> = genres_array
        .iter()
        .map(|&genre_string| {
            let index_genre = ID3V1_GENRES.iter().position(|&r| r == genre_string);
            if let Some(index) = index_genre {
                return index.to_string();
            } else {
                let genre_without_brackets = get_genre_bracket_regex()
                    .replace_all(genre_string, "")
                    .into_owned();

                return genre_without_brackets.trim().to_string();
            }
        })
        .collect::<Vec<_>>();

    genres
}

pub fn is_valid_facet(maybe_facet: &str) -> bool {
    Facet::from_text(maybe_facet)
        .map_err(|_| println!("Invalid facet: {maybe_facet}"))
        .is_ok()
}

pub fn facet_count(facet: &str, facets_count: &FacetCounts) -> Vec<FacetResult> {
    facets_count
        .top_k(facet, 50)
        .into_iter()
        .map(|(facet, count)| FacetResult {
            tag: facet.to_string(),
            total: count as i32,
        })
        .collect()
}

pub fn create_facets(
    facets: Vec<String>,
    facets_count: FacetCounts,
) -> HashMap<String, FacetResults> {
    facets
        .into_iter()
        .map(|facet| (&facets_count, facet))
        .map(|(facets_count, facet)| (facet_count(&facet, facets_count), facet))
        .filter(|(r, _)| !r.is_empty())
        .map(|(facet_results, facet)| (facet, FacetResults { facet_results }))
        .collect()
}

pub fn get_order_field(
    field_schema: &FieldSchema,
    order: &Option<OrderBy>,
) -> Option<tantivy::schema::Field> {
    match order {
        Some(order) => match order.field.as_str() {
            "created_date" => Some(field_schema.created_date),
            "modified_date" => Some(field_schema.modified_date),
            "indexed_date" => Some(field_schema.indexed_date),
            _ => {
                println!("Order by {} is not currently supported.", order.field);
                None
            }
        },
        None => None,
    }
}

pub fn subs(str: &str) -> Vec<String> {
    if let Ok(paths) = std::fs::read_dir(str) {
        return paths
            .into_iter()
            .map(|x| x.unwrap().path().to_str().unwrap().to_string())
            .collect();
    }
    vec![]
}

const ID3V1_GENRES: [&str; 192] = [
    "Blues",
    "Classic Rock",
    "Country",
    "Dance",
    "Disco",
    "Funk",
    "Grunge",
    "Hip-Hop",
    "Jazz",
    "Metal",
    "New Age",
    "Oldies",
    "Other",
    "Pop",
    "Rhythm and Blues",
    "Rap",
    "Reggae",
    "Rock",
    "Techno",
    "Industrial",
    "Alternative",
    "Ska",
    "Death Metal",
    "Pranks",
    "Soundtrack",
    "Euro-Techno",
    "Ambient",
    "Trip-Hop",
    "Vocal",
    "Jazz & Funk",
    "Fusion",
    "Trance",
    "Classical",
    "Instrumental",
    "Acid",
    "House",
    "Game",
    "Sound clip",
    "Gospel",
    "Noise",
    "Alternative Rock",
    "Bass",
    "Soul",
    "Punk",
    "Space",
    "Meditative",
    "Instrumental Pop",
    "Instrumental Rock",
    "Ethnic",
    "Gothic",
    "Darkwave",
    "Techno-Industrial",
    "Electronic",
    "Pop-Folk",
    "Eurodance",
    "Dream",
    "Southern Rock",
    "Comedy",
    "Cult",
    "Gangsta",
    "Top 40",
    "Christian Rap",
    "Pop/Funk",
    "Jungle music",
    "Native US",
    "Cabaret",
    "New Wave",
    "Psychedelic",
    "Rave",
    "Showtunes",
    "Trailer",
    "Lo-Fi",
    "Tribal",
    "Acid Punk",
    "Acid Jazz",
    "Polka",
    "Retro",
    "Musical",
    "Rock ’n’ Roll",
    "Hard Rock",
    "Folk",
    "Folk-Rock",
    "National Folk",
    "Swing",
    "Fast Fusion",
    "Bebop",
    "Latin",
    "Revival",
    "Celtic",
    "Bluegrass",
    "Avantgarde",
    "Gothic Rock",
    "Progressive Rock",
    "Psychedelic Rock",
    "Symphonic Rock",
    "Slow Rock",
    "Big Band",
    "Chorus",
    "Easy Listening",
    "Acoustic",
    "Humour",
    "Speech",
    "Chanson",
    "Opera",
    "Chamber Music",
    "Sonata",
    "Symphony",
    "Booty Bass",
    "Primus",
    "Porn Groove",
    "Satire",
    "Slow Jam",
    "Club",
    "Tango",
    "Samba",
    "Folklore",
    "Ballad",
    "Power Ballad",
    "Rhythmic Soul",
    "Freestyle",
    "Duet",
    "Punk Rock",
    "Drum Solo",
    "A cappella",
    "Euro-House",
    "Dance Hall",
    "Goa music",
    "Drum & Bass",
    "Club-House",
    "Hardcore Techno",
    "Terror",
    "Indie",
    "BritPop",
    "Negerpunk",
    "Polsk Punk",
    "Beat",
    "Christian Gangsta Rap",
    "Heavy Metal",
    "Black Metal",
    "Crossover",
    "Contemporary Christian",
    "Christian Rock",
    "Merengue",
    "Salsa",
    "Thrash Metal",
    "Anime",
    "Jpop",
    "Synthpop",
    "Abstract",
    "Art Rock",
    "Baroque",
    "Bhangra",
    "Big beat",
    "Breakbeat",
    "Chillout",
    "Downtempo",
    "Dub",
    "EBM",
    "Eclectic",
    "Electro",
    "Electroclash",
    "Emo",
    "Experimental",
    "Garage",
    "Global",
    "IDM",
    "Illbient",
    "Industro-Goth",
    "Jam Band",
    "Krautrock",
    "Leftfield",
    "Lounge",
    "Math Rock",
    "New Romantic",
    "Nu-Breakz",
    "Post-Punk",
    "Post-Rock",
    "Psytrance",
    "Shoegaze",
    "Space Rock",
    "Trop Rock",
    "World Music",
    "Neoclassical",
    "Audiobook",
    "Audio Theatre",
    "Neue Deutsche Welle",
    "Podcast",
    "Indie-Rock",
    "G-Funk",
    "Dubstep",
    "Garage Rock",
    "Psybient",
];
