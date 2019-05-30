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
pub struct RoutePatternResourceRelationshipsRepresentativeTrip {
  #[serde(rename = "links")]
  links: Option<::models::RoutePatternResourceRelationshipsRepresentativeTripLinks>,
  #[serde(rename = "data")]
  data: Option<::models::RoutePatternResourceRelationshipsRepresentativeTripData>
}

impl RoutePatternResourceRelationshipsRepresentativeTrip {
  pub fn new() -> RoutePatternResourceRelationshipsRepresentativeTrip {
    RoutePatternResourceRelationshipsRepresentativeTrip {
      links: None,
      data: None
    }
  }

  pub fn set_links(&mut self, links: ::models::RoutePatternResourceRelationshipsRepresentativeTripLinks) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: ::models::RoutePatternResourceRelationshipsRepresentativeTripLinks) -> RoutePatternResourceRelationshipsRepresentativeTrip {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&::models::RoutePatternResourceRelationshipsRepresentativeTripLinks> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_data(&mut self, data: ::models::RoutePatternResourceRelationshipsRepresentativeTripData) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::RoutePatternResourceRelationshipsRepresentativeTripData) -> RoutePatternResourceRelationshipsRepresentativeTrip {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::RoutePatternResourceRelationshipsRepresentativeTripData> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}


