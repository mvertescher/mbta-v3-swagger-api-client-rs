/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleResourceRelationships {
  #[serde(rename = "trip")]
  trip: Option<::models::VehicleResourceRelationshipsTrip>,
  #[serde(rename = "stop")]
  stop: Option<::models::VehicleResourceRelationshipsStop>,
  #[serde(rename = "route")]
  route: Option<::models::ShapeResourceRelationshipsRoute>
}

impl VehicleResourceRelationships {
  pub fn new() -> VehicleResourceRelationships {
    VehicleResourceRelationships {
      trip: None,
      stop: None,
      route: None
    }
  }

  pub fn set_trip(&mut self, trip: ::models::VehicleResourceRelationshipsTrip) {
    self.trip = Some(trip);
  }

  pub fn with_trip(mut self, trip: ::models::VehicleResourceRelationshipsTrip) -> VehicleResourceRelationships {
    self.trip = Some(trip);
    self
  }

  pub fn trip(&self) -> Option<&::models::VehicleResourceRelationshipsTrip> {
    self.trip.as_ref()
  }

  pub fn reset_trip(&mut self) {
    self.trip = None;
  }

  pub fn set_stop(&mut self, stop: ::models::VehicleResourceRelationshipsStop) {
    self.stop = Some(stop);
  }

  pub fn with_stop(mut self, stop: ::models::VehicleResourceRelationshipsStop) -> VehicleResourceRelationships {
    self.stop = Some(stop);
    self
  }

  pub fn stop(&self) -> Option<&::models::VehicleResourceRelationshipsStop> {
    self.stop.as_ref()
  }

  pub fn reset_stop(&mut self) {
    self.stop = None;
  }

  pub fn set_route(&mut self, route: ::models::ShapeResourceRelationshipsRoute) {
    self.route = Some(route);
  }

  pub fn with_route(mut self, route: ::models::ShapeResourceRelationshipsRoute) -> VehicleResourceRelationships {
    self.route = Some(route);
    self
  }

  pub fn route(&self) -> Option<&::models::ShapeResourceRelationshipsRoute> {
    self.route.as_ref()
  }

  pub fn reset_route(&mut self) {
    self.route = None;
  }

}



