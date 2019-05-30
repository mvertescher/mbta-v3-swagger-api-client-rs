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
pub struct AlertResourceRelationships {
  #[serde(rename = "facility")]
  facility: Option<::models::AlertResourceRelationshipsFacility>
}

impl AlertResourceRelationships {
  pub fn new() -> AlertResourceRelationships {
    AlertResourceRelationships {
      facility: None
    }
  }

  pub fn set_facility(&mut self, facility: ::models::AlertResourceRelationshipsFacility) {
    self.facility = Some(facility);
  }

  pub fn with_facility(mut self, facility: ::models::AlertResourceRelationshipsFacility) -> AlertResourceRelationships {
    self.facility = Some(facility);
    self
  }

  pub fn facility(&self) -> Option<&::models::AlertResourceRelationshipsFacility> {
    self.facility.as_ref()
  }

  pub fn reset_facility(&mut self) {
    self.facility = None;
  }

}



