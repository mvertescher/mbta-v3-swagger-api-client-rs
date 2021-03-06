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
pub struct StopResourceRelationshipsParentStation {
  #[serde(rename = "links")]
  links: Option<::models::StopResourceRelationshipsParentStationLinks>,
  #[serde(rename = "data")]
  data: Option<::models::StopResourceRelationshipsParentStationData>
}

impl StopResourceRelationshipsParentStation {
  pub fn new() -> StopResourceRelationshipsParentStation {
    StopResourceRelationshipsParentStation {
      links: None,
      data: None
    }
  }

  pub fn set_links(&mut self, links: ::models::StopResourceRelationshipsParentStationLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::StopResourceRelationshipsParentStationLinks) -> StopResourceRelationshipsParentStation {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::StopResourceRelationshipsParentStationLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: ::models::StopResourceRelationshipsParentStationData) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::StopResourceRelationshipsParentStationData) -> StopResourceRelationshipsParentStation {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::StopResourceRelationshipsParentStationData> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



