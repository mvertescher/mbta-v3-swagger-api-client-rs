/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Services : A page of [ServiceResource](#serviceresource) results

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
  #[serde(rename = "links")]
  links: Option<::models::VehiclesLinks>,
  /// Content with [ServiceResource](#serviceresource) objects
  #[serde(rename = "data")]
  data: Vec<::models::ServiceResource>
}

impl Services {
  /// A page of [ServiceResource](#serviceresource) results
  pub fn new(data: Vec<::models::ServiceResource>) -> Services {
    Services {
      links: None,
      data: data
    }
  }

  pub fn set_links(&mut self, links: ::models::VehiclesLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::VehiclesLinks) -> Services {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::VehiclesLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: Vec<::models::ServiceResource>) {
    self.data = data;
  }

  pub fn with_data(mut self, data: Vec<::models::ServiceResource>) -> Services {
    self.data = data;
    self
  }

  pub fn data(&self) -> &Vec<::models::ServiceResource> {
    &self.data
  }


}



