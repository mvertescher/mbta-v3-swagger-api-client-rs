/* 
 * MBTA
 *
 * MBTA service API. https://www.mbta.com Source code: https://github.com/mbta/api
 *
 * OpenAPI spec version: 3.0
 * Contact: developer@mbta.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ActivePeriod : Start and End dates for active alert

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivePeriod {
  /// Start Date. Format is ISO8601.
  #[serde(rename = "start")]
  start: Option<String>,
  /// End Date. Format is ISO8601.
  #[serde(rename = "end")]
  end: Option<String>
}

impl ActivePeriod {
  /// Start and End dates for active alert
  pub fn new() -> ActivePeriod {
    ActivePeriod {
      start: None,
      end: None
    }
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> ActivePeriod {
    self.start = Some(start);
    self
  }

  pub fn start(&self) -> Option<&String> {
    self.start.as_ref()
  }

  pub fn reset_start(&mut self) {
    self.start = None;
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> ActivePeriod {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

}


