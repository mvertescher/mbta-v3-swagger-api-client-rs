/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RoutePatternResource : Information about the different variations of service that may be run within a single route_id, including when and how often they are operated. See [GTFS `route_patterns.txt](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#route_patternstxt) for the base specification. 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutePatternResource {
  /// The JSON-API resource type
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "relationships")]
  relationships: Option<::models::RoutePatternResourceRelationships>,
  #[serde(rename = "links")]
  links: Option<Value>,
  /// The JSON-API resource ID
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "attributes")]
  attributes: Option<::models::RoutePatternResourceAttributes>
}

impl RoutePatternResource {
  /// Information about the different variations of service that may be run within a single route_id, including when and how often they are operated. See [GTFS `route_patterns.txt](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#route_patternstxt) for the base specification. 
  pub fn new() -> RoutePatternResource {
    RoutePatternResource {
      _type: None,
      relationships: None,
      links: None,
      id: None,
      attributes: None
    }
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> RoutePatternResource {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_relationships(&mut self, relationships: ::models::RoutePatternResourceRelationships) {
    self.relationships = Some(relationships);
  }

  pub fn with_relationships(mut self, relationships: ::models::RoutePatternResourceRelationships) -> RoutePatternResource {
    self.relationships = Some(relationships);
    self
  }

  pub fn relationships(&self) -> Option<&::models::RoutePatternResourceRelationships> {
    self.relationships.as_ref()
  }

  pub fn reset_relationships(&mut self) {
    self.relationships = None;
  }

  pub fn set_links(&mut self, links: Value) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: Value) -> RoutePatternResource {
    self.links = Some(links);
    self
  }

  pub fn links(&self) -> Option<&Value> {
    self.links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self.links = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> RoutePatternResource {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_attributes(&mut self, attributes: ::models::RoutePatternResourceAttributes) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: ::models::RoutePatternResourceAttributes) -> RoutePatternResource {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&::models::RoutePatternResourceAttributes> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

}



