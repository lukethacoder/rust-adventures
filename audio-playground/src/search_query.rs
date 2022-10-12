use log::{error, info, trace, warn};

use tantivy::collector::{Count, FacetCollector, MultiCollector, TopDocs};
use tantivy::schema::{Facet, IndexRecordOption, Term};
use tantivy::{query::*, Document, Index, IndexReader, Searcher};

use crate::schema::{DocumentResult, OrderBy, ResultScore, Track, TrackJson};
use crate::utils::{adapt_text, create_facets, get_order_field, is_valid_facet};

use super::schema::{DocumentSearchRequest, DocumentSearchResponse, FieldSchema, SearchResponse};

pub fn create_query(
    parser: &QueryParser,
    search: &DocumentSearchRequest,
    schema: &FieldSchema,
    text: &str,
) -> Box<dyn Query> {
    let mut queries: Vec<(Occur, Box<dyn Query>)> = vec![];
    let main_q = if text.is_empty() {
        Box::new(AllQuery)
    } else {
        parser.parse_query(text).unwrap()
    };

    queries.push((Occur::Must, main_q));

    // By Year
    // let year_range_query = Box::new(RangeQuery::new_u64(field_schema.year, year_start..year_end));
    // queries.push((Occur::Must, year_range_query));

    // By Created Date
    // let created_date_range_query = Box::new(RangeQuery::new_u64(
    //     field_schema.created_date,
    //     created_date_start..created_date_end,
    // ));
    // queries.push((Occur::Must, created_date_range_query));

    // Fields
    // search.fields.iter().for_each(|value| {
    //     let facet_key: String = format!("/{}", value);
    //     let facet = Facet::from(facet_key.as_str());
    //     let facet_term = Term::from_facet(schema.field, &facet);
    //     let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
    //     queries.push((Occur::Must, Box::new(facet_term_query)));
    // });

    // Facets
    search
        .faceted
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

fn handle_document_with_score(
    field_schema: &FieldSchema,
    doc: Document,
    score: Option<ResultScore>,
) -> DocumentResult {
    let track = Track::with_document(field_schema, doc);

    let doc_response = DocumentResult {
        score: score,
        track: track,
    };
    doc_response
}

fn convert_int_order(
    field_schema: FieldSchema,
    response: SearchResponse<u64>,
    searcher: &Searcher,
) -> DocumentSearchResponse {
    info!("convert_int_order query at {}:{}", line!(), file!());
    let mut total = response.top_docs.len();
    info!("\nfound {} total", &total);

    let next_page: bool;
    if total > response.results_per_page as usize {
        next_page = true;
        total = response.results_per_page as usize;
    } else {
        next_page = false;
    }
    let mut results = Vec::with_capacity(total);

    info!("convert_int_order query at {}:{}", line!(), file!());
    for (id, (_, doc_address)) in response.top_docs.into_iter().enumerate() {
        match searcher.doc(doc_address) {
            Ok(doc) => {
                info!("convert_int_order OK query at {}:{}", line!(), file!());
                let result = handle_document_with_score(
                    &field_schema,
                    doc,
                    Some(ResultScore {
                        bm25: 0.0,
                        booster: id as f32,
                    }),
                );
                results.push(result);
            }
            Err(e) => error!("Error retrieving document from index: {}", e),
        }
    }

    let facets = create_facets(response.facets, response.facets_count);
    info!("Document query at {}:{}", line!(), file!());
    DocumentSearchResponse {
        total: total as i32,
        results,
        facets,
        page_number: response.page_number,
        result_per_page: response.results_per_page,
        query: response.query.to_string(),
        next_page,
        bm25: false,
    }
}

fn convert_bm25_order(
    field_schema: FieldSchema,
    response: SearchResponse<f32>,
    searcher: &Searcher,
) -> DocumentSearchResponse {
    let mut total = response.top_docs.len();

    let next_page: bool;
    if total > response.results_per_page as usize {
        next_page = true;
        total = response.results_per_page as usize;
    } else {
        next_page = false;
    }
    let mut results = Vec::with_capacity(total);
    info!("convert_bm25_order at {}:{}", line!(), file!());

    for (id, (score, doc_address)) in response.top_docs.into_iter().take(total).enumerate() {
        match searcher.doc(doc_address) {
            Ok(doc) => {
                results.push(handle_document_with_score(
                    &field_schema,
                    doc,
                    Some(ResultScore {
                        bm25: score,
                        booster: id as f32,
                    }),
                ));
            }
            Err(e) => error!("Error retrieving document from index: {}", e),
        }
    }

    let facets = create_facets(response.facets, response.facets_count);
    info!("Document query at {}:{}", line!(), file!());
    DocumentSearchResponse {
        total: total as i32,
        results,
        facets,
        page_number: response.page_number,
        result_per_page: response.results_per_page,
        query: response.query.to_string(),
        next_page,
        bm25: true,
    }
}

pub fn do_search(
    index: Index,
    reader: IndexReader,
    field_schema: FieldSchema,
    request: &DocumentSearchRequest,
    facet_only_flag: bool,
) -> DocumentSearchResponse {
    let query_parser = {
        let query_parser = QueryParser::for_index(
            &index,
            vec![
                field_schema.title,
                field_schema.artist,
                field_schema.album,
                field_schema.track,
            ],
        );
        // query_parser.set_conjunction_by_default();
        query_parser
    };
    println!("\nrequest.text {:?} ", &request.text);
    let text = adapt_text(&query_parser, &request.text);

    println!("text {:?} ", &text);

    let query = if !request.text.is_empty() {
        create_query(&query_parser, request, &field_schema, &text)
    } else {
        Box::new(AllQuery) as Box<dyn Query>
    };

    // Offset to search from
    let results = request.result_per_page as usize;
    println!("\nresult_per_page {} ", results);

    let offset = results * request.page_number as usize;
    println!("offset {} ", offset);

    let extra_result = results + 1;
    let order_field = get_order_field(&field_schema, &request.order);
    let facets = request
        .faceted
        .as_ref()
        .map(|v| {
            v.tags
                .iter()
                .filter(|s| is_valid_facet(*s))
                .cloned()
                .collect()
        })
        .unwrap_or_default();

    println!("\nfacets {:?} ", facets);
    println!("field_schema.facets {:?} ", field_schema.facets);

    let mut facet_collector = FacetCollector::for_field(field_schema.facets);
    for facet in &facets {
        match Facet::from_text(facet) {
            Ok(facet) => facet_collector.add_facet(facet),
            Err(_) => println!("Invalid facet: {}", facet),
        }
    }

    let searcher = reader.searcher();

    // TODO: use request.filters to filter by year range and date ranges

    match order_field {
        _ if !facet_only_flag => {
            // Just a facet search
            let facets_count = searcher.search(&query, &facet_collector).unwrap();
            convert_bm25_order(
                field_schema,
                SearchResponse {
                    facets,
                    query: &text,
                    top_docs: vec![],
                    facets_count,
                    order_by: request.order.clone(),
                    page_number: request.page_number,
                    results_per_page: results as i32,
                },
                &searcher,
            )
        }
        Some(order_field) => {
            let mut multicollector = MultiCollector::new();
            let facet_handler = multicollector.add_collector(facet_collector);
            let count_handler = multicollector.add_collector(Count);

            let topdocs_collector = TopDocs::with_limit(extra_result)
                .and_offset(offset)
                .order_by_u64_field(order_field);
            let topdocs_handler = multicollector.add_collector(topdocs_collector);
            let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
            let facets_count = facet_handler.extract(&mut multi_fruit);
            let top_docs = topdocs_handler.extract(&mut multi_fruit);

            let count = count_handler.extract(&mut multi_fruit);

            convert_int_order(
                field_schema,
                SearchResponse {
                    facets_count,
                    facets,
                    top_docs,
                    query: &text,
                    order_by: request.order.clone(),
                    page_number: request.page_number,
                    results_per_page: results as i32,
                },
                &searcher,
            )
        }
        None => {
            let mut multicollector = MultiCollector::new();
            let facet_handler = multicollector.add_collector(facet_collector);
            let topdocs_collector = TopDocs::with_limit(extra_result).and_offset(offset);
            let topdocs_handler = multicollector.add_collector(topdocs_collector);
            let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
            let facets_count = facet_handler.extract(&mut multi_fruit);
            let top_docs = topdocs_handler.extract(&mut multi_fruit);

            convert_bm25_order(
                field_schema,
                SearchResponse {
                    facets_count,
                    facets,
                    top_docs,
                    query: &text,
                    order_by: request.order.clone(),
                    page_number: request.page_number,
                    results_per_page: results as i32,
                },
                &searcher,
            )
        }
    }
}
