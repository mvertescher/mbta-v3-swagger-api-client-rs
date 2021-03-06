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
pub struct ScheduleResourceAttributes {
  /// | Value   | `*_/attributes/arrival_time` and `*_/attributes/departure_time` | |---------|---------------------------------------------------------------| | `true`  | Exact                                                         | | `false` | Estimates                                                     |  See [GTFS `stop_times.txt` `timepoint`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) 
  #[serde(rename = "timepoint")]
  timepoint: Option<bool>,
  /// The sequence the `stop_id` is arrived at during the `trip_id`.  The stop sequence is monotonically increasing along the trip, but the `stop_sequence` along the `trip_id` are not necessarily consecutive.  See [GTFS `stop_times.txt` `stop_sequence`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) 
  #[serde(rename = "stop_sequence")]
  stop_sequence: Option<i32>,
  /// How the vehicle departs from `stop_id`.  | Value | Description                                   | |-------|-----------------------------------------------| | `0`   | Regularly scheduled pickup                    | | `1`   | No pickup available                           | | `2`   | Must phone agency to arrange pickup           | | `3`   | Must coordinate with driver to arrange pickup |  See [GTFS `stop_times.txt` `pickup_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) 
  #[serde(rename = "pickup_type")]
  pickup_type: Option<i32>,
  /// How the vehicle arrives at `stop_id`.  | Value | Description                                   | |-------|-----------------------------------------------| | `0`   | Regularly scheduled drop off                  | | `1`   | No drop off available                         | | `2`   | Must phone agency to arrange pickup           | | `3`   | Must coordinate with driver to arrange pickup |  See [GTFS `stop_times.txt` `drop_off_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) 
  #[serde(rename = "drop_off_type")]
  drop_off_type: Option<i32>,
  /// Direction in which trip is traveling: `0` or `1`.  The meaning of `direction_id` varies based on the route. You can programmatically get the direction names from `/routes` `/data/{index}/attributes/direction_names` or `/routes/{id}` `/data/attributes/direction_names`.  
  #[serde(rename = "direction_id")]
  direction_id: Option<i32>,
  /// Time when the trip departs the given stop. See [GTFS `stop_times.txt` `departure_time`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) Format is ISO8601. 
  #[serde(rename = "departure_time")]
  departure_time: Option<String>,
  /// Time when the trip arrives at the given stop. See [GTFS `stop_times.txt` `arrival_time`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) Format is ISO8601. 
  #[serde(rename = "arrival_time")]
  arrival_time: Option<String>
}

impl ScheduleResourceAttributes {
  pub fn new() -> ScheduleResourceAttributes {
    ScheduleResourceAttributes {
      timepoint: None,
      stop_sequence: None,
      pickup_type: None,
      drop_off_type: None,
      direction_id: None,
      departure_time: None,
      arrival_time: None
    }
  }

  pub fn set_timepoint(&mut self, timepoint: bool) {
    self.timepoint = Some(timepoint);
  }

  pub fn with_timepoint(mut self, timepoint: bool) -> ScheduleResourceAttributes {
    self.timepoint = Some(timepoint);
    self
  }

  pub fn timepoint(&self) -> Option<&bool> {
    self.timepoint.as_ref()
  }

  pub fn reset_timepoint(&mut self) {
    self.timepoint = None;
  }

  pub fn set_stop_sequence(&mut self, stop_sequence: i32) {
    self.stop_sequence = Some(stop_sequence);
  }

  pub fn with_stop_sequence(mut self, stop_sequence: i32) -> ScheduleResourceAttributes {
    self.stop_sequence = Some(stop_sequence);
    self
  }

  pub fn stop_sequence(&self) -> Option<&i32> {
    self.stop_sequence.as_ref()
  }

  pub fn reset_stop_sequence(&mut self) {
    self.stop_sequence = None;
  }

  pub fn set_pickup_type(&mut self, pickup_type: i32) {
    self.pickup_type = Some(pickup_type);
  }

  pub fn with_pickup_type(mut self, pickup_type: i32) -> ScheduleResourceAttributes {
    self.pickup_type = Some(pickup_type);
    self
  }

  pub fn pickup_type(&self) -> Option<&i32> {
    self.pickup_type.as_ref()
  }

  pub fn reset_pickup_type(&mut self) {
    self.pickup_type = None;
  }

  pub fn set_drop_off_type(&mut self, drop_off_type: i32) {
    self.drop_off_type = Some(drop_off_type);
  }

  pub fn with_drop_off_type(mut self, drop_off_type: i32) -> ScheduleResourceAttributes {
    self.drop_off_type = Some(drop_off_type);
    self
  }

  pub fn drop_off_type(&self) -> Option<&i32> {
    self.drop_off_type.as_ref()
  }

  pub fn reset_drop_off_type(&mut self) {
    self.drop_off_type = None;
  }

  pub fn set_direction_id(&mut self, direction_id: i32) {
    self.direction_id = Some(direction_id);
  }

  pub fn with_direction_id(mut self, direction_id: i32) -> ScheduleResourceAttributes {
    self.direction_id = Some(direction_id);
    self
  }

  pub fn direction_id(&self) -> Option<&i32> {
    self.direction_id.as_ref()
  }

  pub fn reset_direction_id(&mut self) {
    self.direction_id = None;
  }

  pub fn set_departure_time(&mut self, departure_time: String) {
    self.departure_time = Some(departure_time);
  }

  pub fn with_departure_time(mut self, departure_time: String) -> ScheduleResourceAttributes {
    self.departure_time = Some(departure_time);
    self
  }

  pub fn departure_time(&self) -> Option<&String> {
    self.departure_time.as_ref()
  }

  pub fn reset_departure_time(&mut self) {
    self.departure_time = None;
  }

  pub fn set_arrival_time(&mut self, arrival_time: String) {
    self.arrival_time = Some(arrival_time);
  }

  pub fn with_arrival_time(mut self, arrival_time: String) -> ScheduleResourceAttributes {
    self.arrival_time = Some(arrival_time);
    self
  }

  pub fn arrival_time(&self) -> Option<&String> {
    self.arrival_time.as_ref()
  }

  pub fn reset_arrival_time(&mut self) {
    self.arrival_time = None;
  }

}



