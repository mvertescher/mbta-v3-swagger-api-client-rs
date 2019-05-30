/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Shapes : A page of [ShapeResource](#shaperesource) results

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shapes {
  #[serde(rename = "links")]
  links: Option<::models::VehiclesLinks>,
  /// Content with [ShapeResource](#shaperesource) objects
  #[serde(rename = "data")]
  data: Vec<::models::ShapeResource>
}

impl Shapes {
  /// A page of [ShapeResource](#shaperesource) results
  pub fn new(data: Vec<::models::ShapeResource>) -> Shapes {
    Shapes {
      links: None,
      data: data
    }
  }

  pub fn set_links(&mut self, links: ::models::VehiclesLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::VehiclesLinks) -> Shapes {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::VehiclesLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: Vec<::models::ShapeResource>) {
    self.data = data;
  }

  pub fn with_data(mut self, data: Vec<::models::ShapeResource>) -> Shapes {
    self.data = data;
    self
  }

  pub fn data(&self) -> &Vec<::models::ShapeResource> {
    &self.data
  }


}



