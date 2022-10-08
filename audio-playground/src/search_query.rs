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
            let facet_term = Term::from_facet(schema.facets, &facet);
            let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
            queries.push((Occur::Should, Box::new(facet_term_query)));
        });
    Box::new(BooleanQuery::new(queries))
}

// pub fn do_search(
//     &self,
//     request: &DocumentSearchRequest,
//     facet_only_flag: bool,
// ) -> DocumentSearchResponse {
//     use crate::search_query::create_query;
//     let query_parser = {
//         let mut query_parser = QueryParser::for_index(&self.index, vec![self.schema.text]);
//         query_parser.set_conjunction_by_default();
//         query_parser
//     };
//     let text = FieldReaderService::adapt_text(&query_parser, &request.body);
//     let query = if !request.body.is_empty() {
//         create_query(&query_parser, request, &self.schema, &text)
//     } else {
//         Box::new(AllQuery) as Box<dyn Query>
//     };

//     // Offset to search from
//     let results = request.result_per_page as usize;
//     let offset = results * request.page_number as usize;
//     let extra_result = results + 1;
//     let order_field = self.get_order_field(&request.order);
//     let facets = request
//         .faceted
//         .as_ref()
//         .map(|v| {
//             v.tags
//                 .iter()
//                 .filter(|s| FieldReaderService::is_valid_facet(*s))
//                 .cloned()
//                 .collect()
//         })
//         .unwrap_or_default();
//     let mut facet_collector = FacetCollector::for_field(self.schema.facets);
//     for facet in &facets {
//         match Facet::from_text(facet) {
//             Ok(facet) => facet_collector.add_facet(facet),
//             Err(_) => error!("Invalid facet: {}", facet),
//         }
//     }
//     let searcher = self.reader.searcher();
//     match order_field {
//         _ if !facet_only_flag => {
//             // Just a facet search
//             let facets_count = searcher.search(&query, &facet_collector).unwrap();
//             self.convert_bm25_order(
//                 SearchResponse {
//                     facets,
//                     query: &text,
//                     top_docs: vec![],
//                     facets_count,
//                     order_by: request.order.clone(),
//                     page_number: request.page_number,
//                     results_per_page: results as i32,
//                 },
//                 &searcher,
//             )
//         }
//         Some(order_field) => {
//             let mut multicollector = MultiCollector::new();
//             let facet_handler = multicollector.add_collector(facet_collector);
//             let topdocs_collector = TopDocs::with_limit(extra_result)
//                 .and_offset(offset)
//                 .order_by_u64_field(order_field);
//             let topdocs_handler = multicollector.add_collector(topdocs_collector);
//             let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
//             let facets_count = facet_handler.extract(&mut multi_fruit);
//             let top_docs = topdocs_handler.extract(&mut multi_fruit);
//             self.convert_int_order(
//                 SearchResponse {
//                     facets_count,
//                     facets,
//                     top_docs,
//                     query: &text,
//                     order_by: request.order.clone(),
//                     page_number: request.page_number,
//                     results_per_page: results as i32,
//                 },
//                 &searcher,
//             )
//         }
//         None => {
//             let mut multicollector = MultiCollector::new();
//             let facet_handler = multicollector.add_collector(facet_collector);
//             let topdocs_collector = TopDocs::with_limit(extra_result).and_offset(offset);
//             let topdocs_handler = multicollector.add_collector(topdocs_collector);
//             let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
//             let facets_count = facet_handler.extract(&mut multi_fruit);
//             let top_docs = topdocs_handler.extract(&mut multi_fruit);
//             self.convert_bm25_order(
//                 SearchResponse {
//                     facets_count,
//                     facets,
//                     top_docs,
//                     query: &text,
//                     order_by: request.order.clone(),
//                     page_number: request.page_number,
//                     results_per_page: results as i32,
//                 },
//                 &searcher,
//             )
//         }
//     }
// }
