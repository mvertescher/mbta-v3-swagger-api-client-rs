# \FacilityApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_facility_controller_index**](FacilityApi.md#api_web_facility_controller_index) | **Get** /facilities | 
[**api_web_facility_controller_show**](FacilityApi.md#api_web_facility_controller_show) | **Get** /facilities/{id} | 


# **api_web_facility_controller_index**
> ::models::Facilities api_web_facility_controller_index(ctx, ctx, optional)


List Escalators and Elevators  Amenities at a station stop (`/data/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE.  

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/latitude&#x60; | ascending | &#x60;latitude&#x60; | | &#x60;/data/{index}/attributes/latitude&#x60; | descending | &#x60;-latitude&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | ascending | &#x60;longitude&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | descending | &#x60;-longitude&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | ascending | &#x60;name&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | descending | &#x60;-name&#x60; | | &#x60;/data/{index}/attributes/properties&#x60; | ascending | &#x60;properties&#x60; | | &#x60;/data/{index}/attributes/properties&#x60; | descending | &#x60;-properties&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | ascending | &#x60;short_name&#x60; | | &#x60;/data/{index}/attributes/short_name&#x60; | descending | &#x60;-short_name&#x60; | | &#x60;/data/{index}/attributes/type&#x60; | ascending | &#x60;type&#x60; | | &#x60;/data/{index}/attributes/type&#x60; | descending | &#x60;-type&#x60; |   | 
 **fields_facility** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stop&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 
 **filter_stop** | **String**| Filter by &#x60;/data/{index}/relationships/stop/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/stop/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_type** | **String**| Filter by multiple types.  Multiple types **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Facilities**](Facilities.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_facility_controller_show**
> ::models::Facility api_web_facility_controller_show(ctx, ctx, id, optional)


Specific Escalator or Elevator  Amenities at a station stop (`/data/{index}/relationships/stop`) such as elevators, escalators, parking lots, and bike storage.  An [MBTA extension](https://groups.google.com/forum/#!topic/gtfs-changes/EzC5m9k45pA).  This spec is not yet finalized.  ## Accessibility  Riders with limited mobility can search any facility, either `ELEVATOR` or `ESCALATOR`, while riders that need wheelchair access can search for `ELEVATOR` only.  The lack of an `ELEVATOR` MAY NOT make a stop wheelchair inaccessible.  Riders should check `/stops/{id}` `/data/attributes/wheelchair_boarding` is `1` to guarantee a path is available from the station entrance to the stop or `0` if it MAY be accessible.  Completely avoid `2` as that is guaranteed to be INACCESSIBLE.  

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for facility | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for facility | 
 **fields_facility** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stop&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 

### Return type

[**::models::Facility**](Facility.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

