# \RouteApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_route_controller_index**](RouteApi.md#api_web_route_controller_index) | **Get** /routes | 
[**api_web_route_controller_show**](RouteApi.md#api_web_route_controller_show) | **Get** /routes/{id} | 


# **api_web_route_controller_index**
> ::models::Routes api_web_route_controller_index(ctx, ctx, optional)


List of routes.  ## Names and Descriptions  There are 3 attributes with increasing details for naming and describing the route.  1. `/data/{index}/attributes/short_name` 2. `/data/{index}/attributes/long_name` 3. `/data/{index}/attributes/description`  ## Directions  `/data/{index}/attributes/direction_names` is the only place to convert the `direction_id` used throughout the rest of the API to human-readable names.  ## Type  `/data/{index}/attributes/type` corresponds to [GTFS `routes.txt` `route_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |   

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/color&#x60; | ascending | &#x60;color&#x60; | | &#x60;/data/{index}/attributes/color&#x60; | descending | &#x60;-color&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | ascending | &#x60;description&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | descending | &#x60;-description&#x60; | | &#x60;/data/{index}/attributes/direction_destinations&#x60; | ascending | &#x60;direction_destinations&#x60; | | &#x60;/data/{index}/attributes/direction_destinations&#x60; | descending | &#x60;-direction_destinations&#x60; | | &#x60;/data/{index}/attributes/direction_names&#x60; | ascending | &#x60;direction_names&#x60; | | &#x60;/data/{index}/attributes/direction_names&#x60; | descending | &#x60;-direction_names&#x60; | | &#x60;/data/{index}/attributes/fare_class&#x60; | ascending | &#x60;fare_class&#x60; | | &#x60;/data/{index}/attributes/fare_class&#x60; | descending | &#x60;-fare_class&#x60; | | &#x60;/data/{index}/attributes/long_name&#x60; | ascending | &#x60;long_name&#x60; | | &#x60;/data/{index}/attributes/long_name&#x60; | descending | &#x60;-long_name&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | ascending | &#x60;short_name&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | descending | &#x60;-short_name&#x60; | | &#x60;/data/{index}/attributes/sort_order&#x60; | ascending | &#x60;sort_order&#x60; | | &#x60;/data/{index}/attributes/sort_order&#x60; | descending | &#x60;-sort_order&#x60; | | &#x60;/data/{index}/attributes/text_color&#x60; | ascending | &#x60;text_color&#x60; | | &#x60;/data/{index}/attributes/text_color&#x60; | descending | &#x60;-text_color&#x60; | | &#x60;/data/{index}/attributes/type&#x60; | ascending | &#x60;type&#x60; | | &#x60;/data/{index}/attributes/type&#x60; | descending | &#x60;-type&#x60; |   | 
 **fields_route** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stop&#x60; * &#x60;line&#x60; * &#x60;route_patterns&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  include&#x3D;stop only works when &#x60;filter[stop]&#x60; is also used  | 
 **filter_stop** | **String**| Filter by &#x60;/data/{index}/relationships/stop/data/id&#x60;. Must filter by stop in order to include stop with response | 
 **filter_type** | **String**| | Value | Name          | Example    | |-------|---------------|------------| | &#x60;0&#x60;   | Light Rail    | Green Line | | &#x60;1&#x60;   | Heavy Rail    | Red Line   | | &#x60;2&#x60;   | Commuter Rail |            | | &#x60;3&#x60;   | Bus           |            | | &#x60;4&#x60;   | Ferry         |            |   Multiple &#x60;route_type&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.   When combined with stop_id, filters by routes which stop at that stop when traveling in a particular direction  | 
 **filter_date** | **String**| Filter by date that route is active The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. | 
 **filter_id** | **String**| Filter by multiple IDs. Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Routes**](Routes.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_route_controller_show**
> ::models::Route api_web_route_controller_show(ctx, ctx, id, optional)


Show a particular route by the route's id.  ## Names and Descriptions  There are 3 attributes with increasing details for naming and describing the route.  1. `/data/attributes/short_name` 2. `/data/attributes/long_name` 3. `/data/attributes/description`  ## Directions  `/data/attributes/direction_names` is the only place to convert the `direction_id` used throughout the rest of the API to human-readable names.  ## Type  `/data/attributes/type` corresponds to [GTFS `routes.txt` `route_type`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | Value | Name          | Example    | |-------|---------------|------------| | `0`   | Light Rail    | Green Line | | `1`   | Heavy Rail    | Red Line   | | `2`   | Commuter Rail |            | | `3`   | Bus           |            | | `4`   | Ferry         |            |   

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for route | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for route | 
 **fields_route** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;line&#x60; * &#x60;route_patterns&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 

### Return type

[**::models::Route**](Route.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

