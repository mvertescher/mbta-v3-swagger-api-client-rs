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
pub struct FacilityResourceAttributes {
  /// The type of the facility.
  #[serde(rename = "type")]
  _type: Option<String>,
  /// Short name of the facility
  #[serde(rename = "short_name")]
  short_name: Option<String>,
  /// A list of name/value pairs that apply to the facility. See [MBTA's facility documentation](https://www.mbta.com/developers/gtfs/f#facilities_properties_definitions) for more information on the possible names and values.
  #[serde(rename = "properties")]
  properties: Option<Vec<::models::FacilityProperty>>,
  /// Name of the facility
  #[serde(rename = "name")]
  name: Option<String>,
  /// Longitude of the facility. Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lon`] 
  #[serde(rename = "longitude")]
  longitude: Option<f32>,
  /// Latitude of the facility.  Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS `facilities.txt` `facility_lat`] 
  #[serde(rename = "latitude")]
  latitude: Option<f32>
}

impl FacilityResourceAttributes {
  pub fn new() -> FacilityResourceAttributes {
    FacilityResourceAttributes {
      _type: None,
      short_name: None,
      properties: None,
      name: None,
      longitude: None,
      latitude: None
    }
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> FacilityResourceAttributes {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_short_name(&mut self, short_name: String) {
    self.short_name = Some(short_name);
  }

  pub fn with_short_name(mut self, short_name: String) -> FacilityResourceAttributes {
    self.short_name = Some(short_name);
    self
  }

  pub fn short_name(&self) -> Option<&String> {
    self.short_name.as_ref()
  }

  pub fn reset_short_name(&mut self) {
    self.short_name = None;
  }

  pub fn set_properties(&mut self, properties: Vec<::models::FacilityProperty>) {
    self.properties = Some(properties);
  }

  pub fn with_properties(mut self, properties: Vec<::models::FacilityProperty>) -> FacilityResourceAttributes {
    self.properties = Some(properties);
    self
  }

  pub fn properties(&self) -> Option<&Vec<::models::FacilityProperty>> {
    self.properties.as_ref()
  }

  pub fn reset_properties(&mut self) {
    self.properties = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> FacilityResourceAttributes {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_longitude(&mut self, longitude: f32) {
    self.longitude = Some(longitude);
  }

  pub fn with_longitude(mut self, longitude: f32) -> FacilityResourceAttributes {
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

  pub fn with_latitude(mut self, latitude: f32) -> FacilityResourceAttributes {
    self.latitude = Some(latitude);
    self
  }

  pub fn latitude(&self) -> Option<&f32> {
    self.latitude.as_ref()
  }

  pub fn reset_latitude(&mut self) {
    self.latitude = None;
  }

}



