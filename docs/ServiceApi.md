# \ServiceApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_service_controller_index**](ServiceApi.md#api_web_service_controller_index) | **Get** /services | 
[**api_web_service_controller_show**](ServiceApi.md#api_web_service_controller_show) | **Get** /services/{id} | 


# **api_web_service_controller_index**
> ::models::Services api_web_service_controller_index(ctx, ctx, optional)


List of services. Service represents the days of the week, as well as extra days, that a trip is valid. 

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/added_dates&#x60; | ascending | &#x60;added_dates&#x60; | | &#x60;/data/{index}/attributes/added_dates&#x60; | descending | &#x60;-added_dates&#x60; | | &#x60;/data/{index}/attributes/added_dates_notes&#x60; | ascending | &#x60;added_dates_notes&#x60; | | &#x60;/data/{index}/attributes/added_dates_notes&#x60; | descending | &#x60;-added_dates_notes&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | ascending | &#x60;description&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | descending | &#x60;-description&#x60; | | &#x60;/data/{index}/attributes/end_date&#x60; | ascending | &#x60;end_date&#x60; | | &#x60;/data/{index}/attributes/end_date&#x60; | descending | &#x60;-end_date&#x60; | | &#x60;/data/{index}/attributes/removed_dates&#x60; | ascending | &#x60;removed_dates&#x60; | | &#x60;/data/{index}/attributes/removed_dates&#x60; | descending | &#x60;-removed_dates&#x60; | | &#x60;/data/{index}/attributes/removed_dates_notes&#x60; | ascending | &#x60;removed_dates_notes&#x60; | | &#x60;/data/{index}/attributes/removed_dates_notes&#x60; | descending | &#x60;-removed_dates_notes&#x60; | | &#x60;/data/{index}/attributes/schedule_name&#x60; | ascending | &#x60;schedule_name&#x60; | | &#x60;/data/{index}/attributes/schedule_name&#x60; | descending | &#x60;-schedule_name&#x60; | | &#x60;/data/{index}/attributes/schedule_type&#x60; | ascending | &#x60;schedule_type&#x60; | | &#x60;/data/{index}/attributes/schedule_type&#x60; | descending | &#x60;-schedule_type&#x60; | | &#x60;/data/{index}/attributes/schedule_typicality&#x60; | ascending | &#x60;schedule_typicality&#x60; | | &#x60;/data/{index}/attributes/schedule_typicality&#x60; | descending | &#x60;-schedule_typicality&#x60; | | &#x60;/data/{index}/attributes/start_date&#x60; | ascending | &#x60;start_date&#x60; | | &#x60;/data/{index}/attributes/start_date&#x60; | descending | &#x60;-start_date&#x60; | | &#x60;/data/{index}/attributes/valid_days&#x60; | ascending | &#x60;valid_days&#x60; | | &#x60;/data/{index}/attributes/valid_days&#x60; | descending | &#x60;-valid_days&#x60; |   | 
 **fields_service** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **filter_id** | **String**| Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_route** | **String**| Filter by route. Multiple &#x60;route&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Services**](Services.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_service_controller_show**
> ::models::Service api_web_service_controller_show(ctx, ctx, id, optional)


Single service, which represents the days of the week, as well as extra days, that a trip is valid. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for a service | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for a service | 
 **fields_service** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 

### Return type

[**::models::Service**](Service.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

