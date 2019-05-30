/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// TripResource : Representation of the journey of a particular vehicle through a given set of stops. See [GTFS `trips.txt`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt) 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TripResource {
  /// The JSON-API resource type
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "relationships")]
  relationships: Option<::models::TripResourceRelationships>,
  #[serde(rename = "links")]
  links: Option<Value>,
  /// The JSON-API resource ID
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "attributes")]
  attributes: Option<::models::TripResourceAttributes>
}

impl TripResource {
  /// Representation of the journey of a particular vehicle through a given set of stops. See [GTFS `trips.txt`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt) 
  pub fn new() -> TripResource {
    TripResource {
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

  pub fn with__type(mut self, _type: String) -> TripResource {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_relationships(&mut self, relationships: ::models::TripResourceRelationships) {
    self.relationships = Some(relationships);
  }

  pub fn with_relationships(mut self, relationships: ::models::TripResourceRelationships) -> TripResource {
    self.relationships = Some(relationships);
    self
  }

  pub fn relationships(&self) -> Option<&::models::TripResourceRelationships> {
    self.relationships.as_ref()
  }

  pub fn reset_relationships(&mut self) {
    self.relationships = None;
  }

  pub fn set_links(&mut self, links: Value) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: Value) -> TripResource {
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

  pub fn with_id(mut self, id: String) -> TripResource {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_attributes(&mut self, attributes: ::models::TripResourceAttributes) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: ::models::TripResourceAttributes) -> TripResource {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&::models::TripResourceAttributes> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

}



