use tantivy::query::*;
use tantivy::schema::{Facet, IndexRecordOption};
use tantivy::Term;

use super::schema::{DocumentSearchRequest, FieldSchema};

pub fn create_query(
    parser: &QueryParser,
    search: &DocumentSearchRequest,
    schema: &FieldSchema,
    text: &str,
) -> Box<dyn Query> {
    let mut queries = vec![];
    let main_q = if text.is_empty() {
        Box::new(AllQuery)
    } else {
        parser.parse_query(text).unwrap()
    };

    queries.push((Occur::Must, main_q));
    // Fields
    // search.fields.iter().for_each(|value| {
    //     let facet_key: String = format!("/{}", value);
    //     let facet = Facet::from(facet_key.as_str());
    //     let facet_term = Term::from_facet(schema.field, &facet);
    //     let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
    //     queries.push((Occur::Must, Box::new(facet_term_query)));
    // });

    // Add filter (genres)
    search
        .filter
        .iter()
        .flat_map(|f| f.tags.iter())
        .for_each(|value| {
            let facet = Facet::from(value.as_str());
            let facet_term = Term::from_facet(schema.genres, &facet);
            let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
            queries.push((Occur::Should, Box::new(facet_term_query)));
        });
    Box::new(BooleanQuery::new(queries))
}
