/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FacilityResource : Amenities at a station stop (`*_/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE. 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FacilityResource {
  /// The JSON-API resource type
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "relationships")]
  relationships: Option<::models::FacilityResourceRelationships>,
  #[serde(rename = "links")]
  links: Option<Value>,
  /// The JSON-API resource ID
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "attributes")]
  attributes: Option<::models::FacilityResourceAttributes>
}

impl FacilityResource {
  /// Amenities at a station stop (`*_/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE. 
  pub fn new() -> FacilityResource {
    FacilityResource {
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

  pub fn with__type(mut self, _type: String) -> FacilityResource {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_relationships(&mut self, relationships: ::models::FacilityResourceRelationships) {
    self.relationships = Some(relationships);
  }

  pub fn with_relationships(mut self, relationships: ::models::FacilityResourceRelationships) -> FacilityResource {
    self.relationships = Some(relationships);
    self
  }

  pub fn relationships(&self) -> Option<&::models::FacilityResourceRelationships> {
    self.relationships.as_ref()
  }

  pub fn reset_relationships(&mut self) {
    self.relationships = None;
  }

  pub fn set_links(&mut self, links: Value) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: Value) -> FacilityResource {
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

  pub fn with_id(mut self, id: String) -> FacilityResource {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_attributes(&mut self, attributes: ::models::FacilityResourceAttributes) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: ::models::FacilityResourceAttributes) -> FacilityResource {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&::models::FacilityResourceAttributes> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

}



