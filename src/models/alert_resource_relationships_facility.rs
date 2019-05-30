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
pub struct AlertResourceRelationshipsFacility {
  #[serde(rename = "links")]
  links: Option<::models::AlertResourceRelationshipsFacilityLinks>,
  #[serde(rename = "data")]
  data: Option<::models::AlertResourceRelationshipsFacilityData>
}

impl AlertResourceRelationshipsFacility {
  pub fn new() -> AlertResourceRelationshipsFacility {
    AlertResourceRelationshipsFacility {
      links: None,
      data: None
    }
  }

  pub fn set_links(&mut self, links: ::models::AlertResourceRelationshipsFacilityLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::AlertResourceRelationshipsFacilityLinks) -> AlertResourceRelationshipsFacility {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::AlertResourceRelationshipsFacilityLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: ::models::AlertResourceRelationshipsFacilityData) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::AlertResourceRelationshipsFacilityData) -> AlertResourceRelationshipsFacility {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::AlertResourceRelationshipsFacilityData> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



