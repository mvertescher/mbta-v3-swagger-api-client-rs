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
pub struct ScheduleResourceRelationshipsPrediction {
  #[serde(rename = "links")]
  links: Option<::models::ScheduleResourceRelationshipsPredictionLinks>,
  #[serde(rename = "data")]
  data: Option<::models::ScheduleResourceRelationshipsPredictionData>
}

impl ScheduleResourceRelationshipsPrediction {
  pub fn new() -> ScheduleResourceRelationshipsPrediction {
    ScheduleResourceRelationshipsPrediction {
      links: None,
      data: None
    }
  }

  pub fn set_links(&mut self, links: ::models::ScheduleResourceRelationshipsPredictionLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::ScheduleResourceRelationshipsPredictionLinks) -> ScheduleResourceRelationshipsPrediction {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::ScheduleResourceRelationshipsPredictionLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: ::models::ScheduleResourceRelationshipsPredictionData) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::ScheduleResourceRelationshipsPredictionData) -> ScheduleResourceRelationshipsPrediction {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::ScheduleResourceRelationshipsPredictionData> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



