use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  alert_api: Box<::apis::AlertApi>,
  facility_api: Box<::apis::FacilityApi>,
  line_api: Box<::apis::LineApi>,
  live_facility_api: Box<::apis::LiveFacilityApi>,
  prediction_api: Box<::apis::PredictionApi>,
  route_api: Box<::apis::RouteApi>,
  route_pattern_api: Box<::apis::RoutePatternApi>,
  schedule_api: Box<::apis::ScheduleApi>,
  service_api: Box<::apis::ServiceApi>,
  shape_api: Box<::apis::ShapeApi>,
  stop_api: Box<::apis::StopApi>,
  trip_api: Box<::apis::TripApi>,
  vehicle_api: Box<::apis::VehicleApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      alert_api: Box::new(::apis::AlertApiClient::new(rc.clone())),
      facility_api: Box::new(::apis::FacilityApiClient::new(rc.clone())),
      line_api: Box::new(::apis::LineApiClient::new(rc.clone())),
      live_facility_api: Box::new(::apis::LiveFacilityApiClient::new(rc.clone())),
      prediction_api: Box::new(::apis::PredictionApiClient::new(rc.clone())),
      route_api: Box::new(::apis::RouteApiClient::new(rc.clone())),
      route_pattern_api: Box::new(::apis::RoutePatternApiClient::new(rc.clone())),
      schedule_api: Box::new(::apis::ScheduleApiClient::new(rc.clone())),
      service_api: Box::new(::apis::ServiceApiClient::new(rc.clone())),
      shape_api: Box::new(::apis::ShapeApiClient::new(rc.clone())),
      stop_api: Box::new(::apis::StopApiClient::new(rc.clone())),
      trip_api: Box::new(::apis::TripApiClient::new(rc.clone())),
      vehicle_api: Box::new(::apis::VehicleApiClient::new(rc.clone())),
    }
  }

  pub fn alert_api(&self) -> &::apis::AlertApi{
    self.alert_api.as_ref()
  }

  pub fn facility_api(&self) -> &::apis::FacilityApi{
    self.facility_api.as_ref()
  }

  pub fn line_api(&self) -> &::apis::LineApi{
    self.line_api.as_ref()
  }

  pub fn live_facility_api(&self) -> &::apis::LiveFacilityApi{
    self.live_facility_api.as_ref()
  }

  pub fn prediction_api(&self) -> &::apis::PredictionApi{
    self.prediction_api.as_ref()
  }

  pub fn route_api(&self) -> &::apis::RouteApi{
    self.route_api.as_ref()
  }

  pub fn route_pattern_api(&self) -> &::apis::RoutePatternApi{
    self.route_pattern_api.as_ref()
  }

  pub fn schedule_api(&self) -> &::apis::ScheduleApi{
    self.schedule_api.as_ref()
  }

  pub fn service_api(&self) -> &::apis::ServiceApi{
    self.service_api.as_ref()
  }

  pub fn shape_api(&self) -> &::apis::ShapeApi{
    self.shape_api.as_ref()
  }

  pub fn stop_api(&self) -> &::apis::StopApi{
    self.stop_api.as_ref()
  }

  pub fn trip_api(&self) -> &::apis::TripApi{
    self.trip_api.as_ref()
  }

  pub fn vehicle_api(&self) -> &::apis::VehicleApi{
    self.vehicle_api.as_ref()
  }


}
