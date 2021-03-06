/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// InformedEntity : Object representing a particular part of the system affected by an alert

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InformedEntity {
  /// Unique id of a trip
  #[serde(rename = "trip")]
  trip: Option<String>,
  /// Unique id of a stop
  #[serde(rename = "stop")]
  stop: Option<String>,
  /// | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            | 
  #[serde(rename = "route_type")]
  route_type: Option<i32>,
  /// Unique id of a route
  #[serde(rename = "route")]
  route: Option<String>,
  /// Unique id of a facility
  #[serde(rename = "facility")]
  facility: Option<String>,
  /// Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  
  #[serde(rename = "direction_id")]
  direction_id: Option<i32>,
  /// Activities affected by this alert.  If an entity is a station platform, and the alert only impacts those boarding at that platform and no one else, and the activity `\"BOARD\"` represents customers boarding at the informed entity, then the entity includes `activities` `[\"BOARD\"]`. If the alert affected customers exiting at the platform too, then `activities` is `[\"BOARD\", \"EXIT\"]`.  It should be noted that the `activities` array includes activities that are specifically affected. Thus if there were activities `\"BOARD\"`, `\"EXIT\"`, and `\"USING_WHEELCHAIR\"` [to board or exit], and a station were closed, then the `activities` array would include `\"BOARD\"` and `\"EXIT\"` but it would not be necessary to include the activity `\"USING_WHEELCHAIR\"`. Any rider entering the station who is `\"USING_WHEELCHAIR\"` is also a rider who `\"BOARD\"`s. Using a wheelchair to board is not specifically affected. 
  #[serde(rename = "activities")]
  activities: Option<Vec<::models::Activity>>
}

impl InformedEntity {
  /// Object representing a particular part of the system affected by an alert
  pub fn new() -> InformedEntity {
    InformedEntity {
      trip: None,
      stop: None,
      route_type: None,
      route: None,
      facility: None,
      direction_id: None,
      activities: None
    }
  }

  pub fn set_trip(&mut self, trip: String) {
    self.trip = Some(trip);
  }

  pub fn with_trip(mut self, trip: String) -> InformedEntity {
    self.trip = Some(trip);
    self
  }

  pub fn trip(&self) -> Option<&String> {
    self.trip.as_ref()
  }

  pub fn reset_trip(&mut self) {
    self.trip = None;
  }

  pub fn set_stop(&mut self, stop: String) {
    self.stop = Some(stop);
  }

  pub fn with_stop(mut self, stop: String) -> InformedEntity {
    self.stop = Some(stop);
    self
  }

  pub fn stop(&self) -> Option<&String> {
    self.stop.as_ref()
  }

  pub fn reset_stop(&mut self) {
    self.stop = None;
  }

  pub fn set_route_type(&mut self, route_type: i32) {
    self.route_type = Some(route_type);
  }

  pub fn with_route_type(mut self, route_type: i32) -> InformedEntity {
    self.route_type = Some(route_type);
    self
  }

  pub fn route_type(&self) -> Option<&i32> {
    self.route_type.as_ref()
  }

  pub fn reset_route_type(&mut self) {
    self.route_type = None;
  }

  pub fn set_route(&mut self, route: String) {
    self.route = Some(route);
  }

  pub fn with_route(mut self, route: String) -> InformedEntity {
    self.route = Some(route);
    self
  }

  pub fn route(&self) -> Option<&String> {
    self.route.as_ref()
  }

  pub fn reset_route(&mut self) {
    self.route = None;
  }

  pub fn set_facility(&mut self, facility: String) {
    self.facility = Some(facility);
  }

  pub fn with_facility(mut self, facility: String) -> InformedEntity {
    self.facility = Some(facility);
    self
  }

  pub fn facility(&self) -> Option<&String> {
    self.facility.as_ref()
  }

  pub fn reset_facility(&mut self) {
    self.facility = None;
  }

  pub fn set_direction_id(&mut self, direction_id: i32) {
    self.direction_id = Some(direction_id);
  }

  pub fn with_direction_id(mut self, direction_id: i32) -> InformedEntity {
    self.direction_id = Some(direction_id);
    self
  }

  pub fn direction_id(&self) -> Option<&i32> {
    self.direction_id.as_ref()
  }

  pub fn reset_direction_id(&mut self) {
    self.direction_id = None;
  }

  pub fn set_activities(&mut self, activities: Vec<::models::Activity>) {
    self.activities = Some(activities);
  }

  pub fn with_activities(mut self, activities: Vec<::models::Activity>) -> InformedEntity {
    self.activities = Some(activities);
    self
  }

  pub fn activities(&self) -> Option<&Vec<::models::Activity>> {
    self.activities.as_ref()
  }

  pub fn reset_activities(&mut self) {
    self.activities = None;
  }

}



