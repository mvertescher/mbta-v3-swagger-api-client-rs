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
pub struct TripResourceRelationships {
  #[serde(rename = "shape")]
  shape: Option<::models::TripResourceRelationshipsShape>,
  #[serde(rename = "service")]
  service: Option<::models::TripResourceRelationshipsService>,
  #[serde(rename = "route_pattern")]
  route_pattern: Option<::models::TripResourceRelationshipsRoutePattern>,
  #[serde(rename = "route")]
  route: Option<::models::ShapeResourceRelationshipsRoute>
}

impl TripResourceRelationships {
  pub fn new() -> TripResourceRelationships {
    TripResourceRelationships {
      shape: None,
      service: None,
      route_pattern: None,
      route: None
    }
  }

  pub fn set_shape(&mut self, shape: ::models::TripResourceRelationshipsShape) {
    self.shape = Some(shape);
  }

  pub fn with_shape(mut self, shape: ::models::TripResourceRelationshipsShape) -> TripResourceRelationships {
    self.shape = Some(shape);
    self
  }

  pub fn shape(&self) -> Option<&::models::TripResourceRelationshipsShape> {
    self.shape.as_ref()
  }

  pub fn reset_shape(&mut self) {
    self.shape = None;
  }

  pub fn set_service(&mut self, service: ::models::TripResourceRelationshipsService) {
    self.service = Some(service);
  }

  pub fn with_service(mut self, service: ::models::TripResourceRelationshipsService) -> TripResourceRelationships {
    self.service = Some(service);
    self
  }

  pub fn service(&self) -> Option<&::models::TripResourceRelationshipsService> {
    self.service.as_ref()
  }

  pub fn reset_service(&mut self) {
    self.service = None;
  }

  pub fn set_route_pattern(&mut self, route_pattern: ::models::TripResourceRelationshipsRoutePattern) {
    self.route_pattern = Some(route_pattern);
  }

  pub fn with_route_pattern(mut self, route_pattern: ::models::TripResourceRelationshipsRoutePattern) -> TripResourceRelationships {
    self.route_pattern = Some(route_pattern);
    self
  }

  pub fn route_pattern(&self) -> Option<&::models::TripResourceRelationshipsRoutePattern> {
    self.route_pattern.as_ref()
  }

  pub fn reset_route_pattern(&mut self) {
    self.route_pattern = None;
  }

  pub fn set_route(&mut self, route: ::models::ShapeResourceRelationshipsRoute) {
    self.route = Some(route);
  }

  pub fn with_route(mut self, route: ::models::ShapeResourceRelationshipsRoute) -> TripResourceRelationships {
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



