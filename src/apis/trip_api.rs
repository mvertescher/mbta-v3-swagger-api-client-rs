/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct TripApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TripApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TripApiClient<C> {
        TripApiClient {
            configuration: configuration,
        }
    }
}

pub trait TripApi {
    fn api_web_trip_controller_index(&self, page_offset: i32, page_limit: i32, sort: &str, fields_trip: &str, include: &str, filter_date: String, filter_direction_id: &str, filter_route: &str, filter_route_pattern: &str, filter_id: &str, filter_name: &str) -> Box<Future<Item = ::models::Trips, Error = Error<serde_json::Value>>>;
    fn api_web_trip_controller_show(&self, id: &str, fields_trip: &str, include: &str) -> Box<Future<Item = ::models::Trip, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>TripApi for TripApiClient<C> {
    fn api_web_trip_controller_index(&self, page_offset: i32, page_limit: i32, sort: &str, fields_trip: &str, include: &str, filter_date: String, filter_direction_id: &str, filter_route: &str, filter_route_pattern: &str, filter_id: &str, filter_name: &str) -> Box<Future<Item = ::models::Trips, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("x-api-key".to_owned(), val);
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("api_key".to_owned(), val);
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("page[offset]", &page_offset.to_string());
            query.append_pair("page[limit]", &page_limit.to_string());
            query.append_pair("sort", &sort.to_string());
            query.append_pair("fields[trip]", &fields_trip.to_string());
            query.append_pair("include", &include.to_string());
            query.append_pair("filter[date]", &filter_date.to_string());
            query.append_pair("filter[direction_id]", &filter_direction_id.to_string());
            query.append_pair("filter[route]", &filter_route.to_string());
            query.append_pair("filter[route_pattern]", &filter_route_pattern.to_string());
            query.append_pair("filter[id]", &filter_id.to_string());
            query.append_pair("filter[name]", &filter_name.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/trips?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Trips, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn api_web_trip_controller_show(&self, id: &str, fields_trip: &str, include: &str) -> Box<Future<Item = ::models::Trip, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("x-api-key".to_owned(), val);
        };
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_query.insert("api_key".to_owned(), val);
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("fields[trip]", &fields_trip.to_string());
            query.append_pair("include", &include.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/trips/{id}?{}", configuration.base_path, query_string, id=id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::Trip, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
