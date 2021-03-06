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
pub struct LineResourceAttributes {
  /// This field can be used to specify a legible color to use for text drawn against a background of line_color. The color must be provided as a six-character hexadecimal number, for example, `FFD700`. 
  #[serde(rename = "text_color")]
  text_color: Option<String>,
  /// Lines sort in ascending order
  #[serde(rename = "sort_order")]
  sort_order: Option<i32>,
  /// Short, public-facing name for the group of routes represented in this line 
  #[serde(rename = "short_name")]
  short_name: Option<String>,
  /// Lengthier, public-facing name for the group of routes represented in this line 
  #[serde(rename = "long_name")]
  long_name: Option<String>,
  /// In systems that have colors assigned to lines, the route_color field defines a color that corresponds to a line. The color must be provided as a six-character hexadecimal number, for example, `00FFFF`. 
  #[serde(rename = "color")]
  color: Option<String>
}

impl LineResourceAttributes {
  pub fn new() -> LineResourceAttributes {
    LineResourceAttributes {
      text_color: None,
      sort_order: None,
      short_name: None,
      long_name: None,
      color: None
    }
  }

  pub fn set_text_color(&mut self, text_color: String) {
    self.text_color = Some(text_color);
  }

  pub fn with_text_color(mut self, text_color: String) -> LineResourceAttributes {
    self.text_color = Some(text_color);
    self
  }

  pub fn text_color(&self) -> Option<&String> {
    self.text_color.as_ref()
  }

  pub fn reset_text_color(&mut self) {
    self.text_color = None;
  }

  pub fn set_sort_order(&mut self, sort_order: i32) {
    self.sort_order = Some(sort_order);
  }

  pub fn with_sort_order(mut self, sort_order: i32) -> LineResourceAttributes {
    self.sort_order = Some(sort_order);
    self
  }

  pub fn sort_order(&self) -> Option<&i32> {
    self.sort_order.as_ref()
  }

  pub fn reset_sort_order(&mut self) {
    self.sort_order = None;
  }

  pub fn set_short_name(&mut self, short_name: String) {
    self.short_name = Some(short_name);
  }

  pub fn with_short_name(mut self, short_name: String) -> LineResourceAttributes {
    self.short_name = Some(short_name);
    self
  }

  pub fn short_name(&self) -> Option<&String> {
    self.short_name.as_ref()
  }

  pub fn reset_short_name(&mut self) {
    self.short_name = None;
  }

  pub fn set_long_name(&mut self, long_name: String) {
    self.long_name = Some(long_name);
  }

  pub fn with_long_name(mut self, long_name: String) -> LineResourceAttributes {
    self.long_name = Some(long_name);
    self
  }

  pub fn long_name(&self) -> Option<&String> {
    self.long_name.as_ref()
  }

  pub fn reset_long_name(&mut self) {
    self.long_name = None;
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> LineResourceAttributes {
    self.color = Some(color);
    self
  }

  pub fn color(&self) -> Option<&String> {
    self.color.as_ref()
  }

  pub fn reset_color(&mut self) {
    self.color = None;
  }

}



