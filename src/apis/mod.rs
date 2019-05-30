use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod alert_api;
pub use self::alert_api::{ AlertApi, AlertApiClient };
mod facility_api;
pub use self::facility_api::{ FacilityApi, FacilityApiClient };
mod line_api;
pub use self::line_api::{ LineApi, LineApiClient };
mod live_facility_api;
pub use self::live_facility_api::{ LiveFacilityApi, LiveFacilityApiClient };
mod prediction_api;
pub use self::prediction_api::{ PredictionApi, PredictionApiClient };
mod route_api;
pub use self::route_api::{ RouteApi, RouteApiClient };
mod route_pattern_api;
pub use self::route_pattern_api::{ RoutePatternApi, RoutePatternApiClient };
mod schedule_api;
pub use self::schedule_api::{ ScheduleApi, ScheduleApiClient };
mod service_api;
pub use self::service_api::{ ServiceApi, ServiceApiClient };
mod shape_api;
pub use self::shape_api::{ ShapeApi, ShapeApiClient };
mod stop_api;
pub use self::stop_api::{ StopApi, StopApiClient };
mod trip_api;
pub use self::trip_api::{ TripApi, TripApiClient };
mod vehicle_api;
pub use self::vehicle_api::{ VehicleApi, VehicleApiClient };

pub mod configuration;
pub mod client;
