/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AlertResource : An effect (enumerated in `*_/attributes/effect` and human-readable in `*_/attributes/service_effect`) on a provided service (facility, route, route type, stop and/or trip in `/_*_/attributes/informed_entity`) described by a banner (`*_/attributes/banner`), short header (`*_/attributes/short_header`), header `*_/attributes/header`, and description (`*_/attributes/description`) that is active for one or more periods (`*_/attributes/active_period`) caused by a cause (`*_/attribute/cause`) that somewhere in its lifecycle (enumerated in `*_/attributes/lifecycle` and human-readable in `*_/attributes/timeframe`).  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  ## Descriptions  There are 5 descriptive attributes.  | JSON pointer                                | Usage                                                                           | |---------------------------------------------|---------------------------------------------------------------------------------| | `*_/attributes/banner`       | Display as alert across application/website                                     | | `*_/attributes/short_header` | When `*_/attributes/header` is too long to display               | | `*_/attributes/header`       | Used before showing and prepended to `*_/attributes/description` | | `*_/attributes/description`  | Used when user asks to expand alert.                                            |  ## Effect  | JSON pointer                                  |                | |-----------------------------------------------|----------------| | `*_/attributes/effect`         | Enumerated     | | `*_/attributes/service_effect` | Human-readable |  ## Timeline  There are 3 timeline related attributes  | JSON pointer                                 | Description                                                                              | |----------------------------------------------|------------------------------------------------------------------------------------------| | `*_/attributes/active_period` | Exact Date/Time ranges alert is active                                                   | | `*_/attributes/lifecycle`     | Enumerated, machine-readable description of `*_/attributes/active_period` | | `*_/attributes/timeframe`     | Human-readable description of `*_/attributes/active_period`               | 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertResource {
  /// The JSON-API resource type
  #[serde(rename = "type")]
  _type: Option<String>,
  #[serde(rename = "relationships")]
  relationships: Option<::models::AlertResourceRelationships>,
  #[serde(rename = "links")]
  links: Option<Value>,
  /// The JSON-API resource ID
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "attributes")]
  attributes: Option<::models::AlertResourceAttributes>
}

impl AlertResource {
  /// An effect (enumerated in `*_/attributes/effect` and human-readable in `*_/attributes/service_effect`) on a provided service (facility, route, route type, stop and/or trip in `/_*_/attributes/informed_entity`) described by a banner (`*_/attributes/banner`), short header (`*_/attributes/short_header`), header `*_/attributes/header`, and description (`*_/attributes/description`) that is active for one or more periods (`*_/attributes/active_period`) caused by a cause (`*_/attribute/cause`) that somewhere in its lifecycle (enumerated in `*_/attributes/lifecycle` and human-readable in `*_/attributes/timeframe`).  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  ## Descriptions  There are 5 descriptive attributes.  | JSON pointer                                | Usage                                                                           | |---------------------------------------------|---------------------------------------------------------------------------------| | `*_/attributes/banner`       | Display as alert across application/website                                     | | `*_/attributes/short_header` | When `*_/attributes/header` is too long to display               | | `*_/attributes/header`       | Used before showing and prepended to `*_/attributes/description` | | `*_/attributes/description`  | Used when user asks to expand alert.                                            |  ## Effect  | JSON pointer                                  |                | |-----------------------------------------------|----------------| | `*_/attributes/effect`         | Enumerated     | | `*_/attributes/service_effect` | Human-readable |  ## Timeline  There are 3 timeline related attributes  | JSON pointer                                 | Description                                                                              | |----------------------------------------------|------------------------------------------------------------------------------------------| | `*_/attributes/active_period` | Exact Date/Time ranges alert is active                                                   | | `*_/attributes/lifecycle`     | Enumerated, machine-readable description of `*_/attributes/active_period` | | `*_/attributes/timeframe`     | Human-readable description of `*_/attributes/active_period`               | 
  pub fn new() -> AlertResource {
    AlertResource {
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

  pub fn with__type(mut self, _type: String) -> AlertResource {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_relationships(&mut self, relationships: ::models::AlertResourceRelationships) {
    self.relationships = Some(relationships);
  }

  pub fn with_relationships(mut self, relationships: ::models::AlertResourceRelationships) -> AlertResource {
    self.relationships = Some(relationships);
    self
  }

  pub fn relationships(&self) -> Option<&::models::AlertResourceRelationships> {
    self.relationships.as_ref()
  }

  pub fn reset_relationships(&mut self) {
    self.relationships = None;
  }

  pub fn set_links(&mut self, links: Value) {
    self.links = Some(links);
  }

  pub fn with_links(mut self, links: Value) -> AlertResource {
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

  pub fn with_id(mut self, id: String) -> AlertResource {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_attributes(&mut self, attributes: ::models::AlertResourceAttributes) {
    self.attributes = Some(attributes);
  }

  pub fn with_attributes(mut self, attributes: ::models::AlertResourceAttributes) -> AlertResource {
    self.attributes = Some(attributes);
    self
  }

  pub fn attributes(&self) -> Option<&::models::AlertResourceAttributes> {
    self.attributes.as_ref()
  }

  pub fn reset_attributes(&mut self) {
    self.attributes = None;
  }

}



