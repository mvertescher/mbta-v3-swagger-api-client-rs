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
pub struct VehicleResourceAttributes {
  /// Time at which vehicle information was last updated. Format is ISO8601.
  #[serde(rename = "updated_at")]
  updated_at: Option<String>,
  /// Speed that the vehicle is traveling in meters per second. See [GTFS-realtime Position speed](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
  #[serde(rename = "speed")]
  speed: Option<f32>,
  /// Longitude of the vehicle's current position.  Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS-realtime Position longitude](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
  #[serde(rename = "longitude")]
  longitude: Option<f32>,
  /// Latitude of the vehicle's current position. Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS-realtime Position latitude](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
  #[serde(rename = "latitude")]
  latitude: Option<f32>,
  /// User visible label, such as the one of on the signage on the vehicle.  See [GTFS-realtime VehicleDescriptor label](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-vehicledescriptor).
  #[serde(rename = "label")]
  label: Option<String>,
  /// Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  
  #[serde(rename = "direction_id")]
  direction_id: Option<i32>,
  /// Index of current stop along trip. See [GTFS-realtime VehiclePosition current_stop_sequence](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-vehicleposition)
  #[serde(rename = "current_stop_sequence")]
  current_stop_sequence: Option<i32>,
  /// Status of vehicle relative to the stops. See [GTFS-realtime VehicleStopStatus](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#enum-vehiclestopstatus).  | _**Value**_       | _**Description**_                                                                                          | |-------------------|------------------------------------------------------------------------------------------------------------| | **INCOMING_AT**   | The vehicle is just about to arrive at the stop (on a stop display, the vehicle symbol typically flashes). | | **STOPPED_AT**    | The vehicle is standing at the stop.                                                                       | | **IN_TRANSIT_TO** | The vehicle has departed the previous stop and is in transit.                                              |  
  #[serde(rename = "current_status")]
  current_status: Option<String>,
  /// Bearing, in degrees, clockwise from True North, i.e., 0 is North and 90 is East. This can be the compass bearing, or the direction towards the next stop or intermediate location. See [GTFS-realtime Position bearing](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-position).
  #[serde(rename = "bearing")]
  bearing: Option<i32>
}

impl VehicleResourceAttributes {
  pub fn new() -> VehicleResourceAttributes {
    VehicleResourceAttributes {
      updated_at: None,
      speed: None,
      longitude: None,
      latitude: None,
      label: None,
      direction_id: None,
      current_stop_sequence: None,
      current_status: None,
      bearing: None
    }
  }

  pub fn set_updated_at(&mut self, updated_at: String) {
    self.updated_at = Some(updated_at);
  }

  pub fn with_updated_at(mut self, updated_at: String) -> VehicleResourceAttributes {
    self.updated_at = Some(updated_at);
    self
  }

  pub fn updated_at(&self) -> Option<&String> {
    self.updated_at.as_ref()
  }

  pub fn reset_updated_at(&mut self) {
    self.updated_at = None;
  }

  pub fn set_speed(&mut self, speed: f32) {
    self.speed = Some(speed);
  }

  pub fn with_speed(mut self, speed: f32) -> VehicleResourceAttributes {
    self.speed = Some(speed);
    self
  }

  pub fn speed(&self) -> Option<&f32> {
    self.speed.as_ref()
  }

  pub fn reset_speed(&mut self) {
    self.speed = None;
  }

  pub fn set_longitude(&mut self, longitude: f32) {
    self.longitude = Some(longitude);
  }

  pub fn with_longitude(mut self, longitude: f32) -> VehicleResourceAttributes {
    self.longitude = Some(longitude);
    self
  }

  pub fn longitude(&self) -> Option<&f32> {
    self.longitude.as_ref()
  }

  pub fn reset_longitude(&mut self) {
    self.longitude = None;
  }

  pub fn set_latitude(&mut self, latitude: f32) {
    self.latitude = Some(latitude);
  }

  pub fn with_latitude(mut self, latitude: f32) -> VehicleResourceAttributes {
    self.latitude = Some(latitude);
    self
  }

  pub fn latitude(&self) -> Option<&f32> {
    self.latitude.as_ref()
  }

  pub fn reset_latitude(&mut self) {
    self.latitude = None;
  }

  pub fn set_label(&mut self, label: String) {
    self.label = Some(label);
  }

  pub fn with_label(mut self, label: String) -> VehicleResourceAttributes {
    self.label = Some(label);
    self
  }

  pub fn label(&self) -> Option<&String> {
    self.label.as_ref()
  }

  pub fn reset_label(&mut self) {
    self.label = None;
  }

  pub fn set_direction_id(&mut self, direction_id: i32) {
    self.direction_id = Some(direction_id);
  }

  pub fn with_direction_id(mut self, direction_id: i32) -> VehicleResourceAttributes {
    self.direction_id = Some(direction_id);
    self
  }

  pub fn direction_id(&self) -> Option<&i32> {
    self.direction_id.as_ref()
  }

  pub fn reset_direction_id(&mut self) {
    self.direction_id = None;
  }

  pub fn set_current_stop_sequence(&mut self, current_stop_sequence: i32) {
    self.current_stop_sequence = Some(current_stop_sequence);
  }

  pub fn with_current_stop_sequence(mut self, current_stop_sequence: i32) -> VehicleResourceAttributes {
    self.current_stop_sequence = Some(current_stop_sequence);
    self
  }

  pub fn current_stop_sequence(&self) -> Option<&i32> {
    self.current_stop_sequence.as_ref()
  }

  pub fn reset_current_stop_sequence(&mut self) {
    self.current_stop_sequence = None;
  }

  pub fn set_current_status(&mut self, current_status: String) {
    self.current_status = Some(current_status);
  }

  pub fn with_current_status(mut self, current_status: String) -> VehicleResourceAttributes {
    self.current_status = Some(current_status);
    self
  }

  pub fn current_status(&self) -> Option<&String> {
    self.current_status.as_ref()
  }

  pub fn reset_current_status(&mut self) {
    self.current_status = None;
  }

  pub fn set_bearing(&mut self, bearing: i32) {
    self.bearing = Some(bearing);
  }

  pub fn with_bearing(mut self, bearing: i32) -> VehicleResourceAttributes {
    self.bearing = Some(bearing);
    self
  }

  pub fn bearing(&self) -> Option<&i32> {
    self.bearing.as_ref()
  }

  pub fn reset_bearing(&mut self) {
    self.bearing = None;
  }

}



