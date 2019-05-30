/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NotFoundErrors : A JSON-API error when a resource is not found

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotFoundErrors {
  /// A short, human-readable summary of the problem
  #[serde(rename = "title")]
  title: Option<String>,
  /// The HTTP status code applicable to the problem
  #[serde(rename = "status")]
  status: Option<String>,
  #[serde(rename = "source")]
  source: Option<::models::NotFoundSource>,
  /// An application-specific error code
  #[serde(rename = "code")]
  code: Option<String>
}

impl NotFoundErrors {
  /// A JSON-API error when a resource is not found
  pub fn new() -> NotFoundErrors {
    NotFoundErrors {
      title: None,
      status: None,
      source: None,
      code: None
    }
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> NotFoundErrors {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> NotFoundErrors {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_source(&mut self, source: ::models::NotFoundSource) {
    self.source = Some(source);
  }

  pub fn with_source(mut self, source: ::models::NotFoundSource) -> NotFoundErrors {
    self.source = Some(source);
    self
  }

  pub fn source(&self) -> Option<&::models::NotFoundSource> {
    self.source.as_ref()
  }

  pub fn reset_source(&mut self) {
    self.source = None;
  }

  pub fn set_code(&mut self, code: String) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: String) -> NotFoundErrors {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&String> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

}


