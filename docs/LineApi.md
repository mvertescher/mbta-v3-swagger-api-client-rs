# \LineApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_line_controller_index**](LineApi.md#api_web_line_controller_index) | **Get** /lines | 
[**api_web_line_controller_show**](LineApi.md#api_web_line_controller_show) | **Get** /lines/{id} | 


# **api_web_line_controller_index**
> ::models::Lines api_web_line_controller_index(ctx, ctx, optional)


List of lines. A line is a combination of routes. This concept can be used to group similar routes when displaying them to customers, such as for routes which serve the same trunk corridor or bus terminal. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page_offset** | **i32**| Offset (0-based) of first element in the page | 
 **page_limit** | **i32**| Max number of elements to return | 
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/color&#x60; | ascending | &#x60;color&#x60; | | &#x60;/data/{index}/attributes/color&#x60; | descending | &#x60;-color&#x60; | | &#x60;/data/{index}/attributes/long_name&#x60; | ascending | &#x60;long_name&#x60; | | &#x60;/data/{index}/attributes/long_name&#x60; | descending | &#x60;-long_name&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | ascending | &#x60;short_name&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | descending | &#x60;-short_name&#x60; | | &#x60;/data/{index}/attributes/sort_order&#x60; | ascending | &#x60;sort_order&#x60; | | &#x60;/data/{index}/attributes/sort_order&#x60; | descending | &#x60;-sort_order&#x60; | | &#x60;/data/{index}/attributes/text_color&#x60; | ascending | &#x60;text_color&#x60; | | &#x60;/data/{index}/attributes/text_color&#x60; | descending | &#x60;-text_color&#x60; |   | 
 **fields_line** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;routes&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 
 **filter_id** | **String**| Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Lines**](Lines.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_line_controller_show**
> ::models::Lines api_web_line_controller_show(ctx, ctx, id, optional)


Single line, which represents a combination of routes. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for a line | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for a line | 
 **fields_line** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;routes&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 

### Return type

[**::models::Lines**](Lines.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

