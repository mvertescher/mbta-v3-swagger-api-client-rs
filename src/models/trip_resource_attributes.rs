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
pub struct TripResourceAttributes {
  /// Indicator of wheelchair accessibility: `0`, `1`, `2`  Wheelchair accessibility (`*_/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |  
  #[serde(rename = "wheelchair_accessible")]
  wheelchair_accessible: Option<i32>,
  /// The text that appears in schedules and sign boards to identify the trip to passengers, for example, to identify train numbers for commuter rail trips. See [GTFS `trips.txt` `trip_short_name`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt) 
  #[serde(rename = "name")]
  name: Option<String>,
  /// The text that appears on a sign that identifies the trip's destination to passengers. See [GTFS `trips.txt` `trip_headsign`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt). 
  #[serde(rename = "headsign")]
  headsign: Option<String>,
  /// Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  
  #[serde(rename = "direction_id")]
  direction_id: Option<i32>,
  /// ID used to group sequential trips with the same vehicle for a given service_id. See [GTFS `trips.txt` `block_id`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt) 
  #[serde(rename = "block_id")]
  block_id: Option<String>,
  /// Indicator of whether or not bikes are allowed on this trip: `0`, `1`, `2`  Bikes allowed (`*_/attributes/bikes_allowed`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                                                         | |-------|---------------------------------------------------------------------------------| | `0`   | No information                                                                  | | `1`   | Vehicle being used on this particular trip can accommodate at least one bicycle | | `2`   | No bicycles are allowed on this trip                                            |  
  #[serde(rename = "bikes_allowed")]
  bikes_allowed: Option<i32>
}

impl TripResourceAttributes {
  pub fn new() -> TripResourceAttributes {
    TripResourceAttributes {
      wheelchair_accessible: None,
      name: None,
      headsign: None,
      direction_id: None,
      block_id: None,
      bikes_allowed: None
    }
  }

  pub fn set_wheelchair_accessible(&mut self, wheelchair_accessible: i32) {
    self.wheelchair_accessible = Some(wheelchair_accessible);
  }

  pub fn with_wheelchair_accessible(mut self, wheelchair_accessible: i32) -> TripResourceAttributes {
    self.wheelchair_accessible = Some(wheelchair_accessible);
    self
  }

  pub fn wheelchair_accessible(&self) -> Option<&i32> {
    self.wheelchair_accessible.as_ref()
  }

  pub fn reset_wheelchair_accessible(&mut self) {
    self.wheelchair_accessible = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> TripResourceAttributes {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_headsign(&mut self, headsign: String) {
    self.headsign = Some(headsign);
  }

  pub fn with_headsign(mut self, headsign: String) -> TripResourceAttributes {
    self.headsign = Some(headsign);
    self
  }

  pub fn headsign(&self) -> Option<&String> {
    self.headsign.as_ref()
  }

  pub fn reset_headsign(&mut self) {
    self.headsign = None;
  }

  pub fn set_direction_id(&mut self, direction_id: i32) {
    self.direction_id = Some(direction_id);
  }

  pub fn with_direction_id(mut self, direction_id: i32) -> TripResourceAttributes {
    self.direction_id = Some(direction_id);
    self
  }

  pub fn direction_id(&self) -> Option<&i32> {
    self.direction_id.as_ref()
  }

  pub fn reset_direction_id(&mut self) {
    self.direction_id = None;
  }

  pub fn set_block_id(&mut self, block_id: String) {
    self.block_id = Some(block_id);
  }

  pub fn with_block_id(mut self, block_id: String) -> TripResourceAttributes {
    self.block_id = Some(block_id);
    self
  }

  pub fn block_id(&self) -> Option<&String> {
    self.block_id.as_ref()
  }

  pub fn reset_block_id(&mut self) {
    self.block_id = None;
  }

  pub fn set_bikes_allowed(&mut self, bikes_allowed: i32) {
    self.bikes_allowed = Some(bikes_allowed);
  }

  pub fn with_bikes_allowed(mut self, bikes_allowed: i32) -> TripResourceAttributes {
    self.bikes_allowed = Some(bikes_allowed);
    self
  }

  pub fn bikes_allowed(&self) -> Option<&i32> {
    self.bikes_allowed.as_ref()
  }

  pub fn reset_bikes_allowed(&mut self) {
    self.bikes_allowed = None;
  }

}



