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
pub struct StopResourceRelationshipsParentStationLinks {
  /// Relationship link for parent_station
  #[serde(rename = "self")]
  _self: Option<String>,
  /// Related parent_station link
  #[serde(rename = "related")]
  related: Option<String>
}

impl StopResourceRelationshipsParentStationLinks {
  pub fn new() -> StopResourceRelationshipsParentStationLinks {
    StopResourceRelationshipsParentStationLinks {
      _self: None,
      related: None
    }
  }

  pub fn set__self(&mut self, _self: String) {
    self._self = Some(_self);
  }

  pub fn with__self(mut self, _self: String) -> StopResourceRelationshipsParentStationLinks {
    self._self = Some(_self);
    self
  }

  pub fn _self(&self) -> Option<&String> {
    self._self.as_ref()
  }

  pub fn reset__self(&mut self) {
    self._self = None;
  }

  pub fn set_related(&mut self, related: String) {
    self.related = Some(related);
  }

  pub fn with_related(mut self, related: String) -> StopResourceRelationshipsParentStationLinks {
    self.related = Some(related);
    self
  }

  pub fn related(&self) -> Option<&String> {
    self.related.as_ref()
  }

  pub fn reset_related(&mut self) {
    self.related = None;
  }

}



