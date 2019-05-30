# \TripApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_trip_controller_index**](TripApi.md#api_web_trip_controller_index) | **Get** /trips | 
[**api_web_trip_controller_show**](TripApi.md#api_web_trip_controller_show) | **Get** /trips/{id} | 


# **api_web_trip_controller_index**
> ::models::Trips api_web_trip_controller_index(ctx, ctx, optional)


**NOTE:** A filter **MUST** be present for any trips to be returned.  List of trips, the journies of a particular vehicle through a set of stops on a primary `route` and zero or more alternative `route`s that can be filtered on.  ## Accessibility  Wheelchair accessibility (`/data/{index}/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |   ## Grouping  Multiple trips **may** be grouped together using `/data/{index}/attributes/block_id`. A block represents a series of trips scheduled to be operated by the same vehicle.  ## Naming  There are 3 names associated with a trip.  | API Field                   | GTFS              | Show users? | |-----------------------------|-------------------|-------------| | `/data/attributes/headsign` | `trip_headsign`   | Yes         | | `/data/attributes/name`     | `trip_short_name` | Yes         | | `/data/id`                  | `trip_id`         | No          |   

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/bikes_allowed&#x60; | ascending | &#x60;bikes_allowed&#x60; | | &#x60;/data/{index}/attributes/bikes_allowed&#x60; | descending | &#x60;-bikes_allowed&#x60; | | &#x60;/data/{index}/attributes/block_id&#x60; | ascending | &#x60;block_id&#x60; | | &#x60;/data/{index}/attributes/block_id&#x60; | descending | &#x60;-block_id&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | ascending | &#x60;direction_id&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | descending | &#x60;-direction_id&#x60; | | &#x60;/data/{index}/attributes/headsign&#x60; | ascending | &#x60;headsign&#x60; | | &#x60;/data/{index}/attributes/headsign&#x60; | descending | &#x60;-headsign&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | ascending | &#x60;name&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | descending | &#x60;-name&#x60; | | &#x60;/data/{index}/attributes/wheelchair_accessible&#x60; | ascending | &#x60;wheelchair_accessible&#x60; | | &#x60;/data/{index}/attributes/wheelchair_accessible&#x60; | descending | &#x60;-wheelchair_accessible&#x60; |   | 
 **fields_trip** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;route&#x60; * &#x60;vehicle&#x60; * &#x60;service&#x60; * &#x60;shape&#x60; * &#x60;predictions&#x60; * &#x60;route_pattern&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  | include         | Description | |-----------------|-------------| | &#x60;route&#x60;         | The *primary* route for the trip. | | &#x60;vehicle&#x60;       | The vehicle on this trip. | | &#x60;service&#x60;       | The service controlling when this trip is active. | | &#x60;shape&#x60;         | The shape of the trip. | | &#x60;route_pattern&#x60; | The route pattern for the trip. | | &#x60;predictions&#x60;   | Predictions of when the &#x60;vehicle&#x60; on this &#x60;trip&#x60; will arrive at or depart from each stop on the route(s) on the &#x60;trip&#x60;. |   | 
 **filter_date** | **String**| Filter by trips on a particular date The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.     | 
 **filter_route** | **String**| Filter by &#x60;/data/{index}/relationships/route/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/route/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_route_pattern** | **String**| Filter by route patern IDs **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_id** | **String**| Filter by multiple IDs. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_name** | **String**| Filter by multiple names. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Trips**](Trips.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_trip_controller_show**
> ::models::Trip api_web_trip_controller_show(ctx, ctx, id, optional)


Single trip - the journey of a particular vehicle through a set of stops  ## Accessibility  Wheelchair accessibility (`/data/attributes/wheelchair_accessible`) [as defined in GTFS](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#tripstxt):  | Value | Meaning                                            | |-------|----------------------------------------------------| | `0`   | No information                                     | | `1`   | Accessible (at stops allowing wheelchair_boarding) | | `2`   | Inaccessible                                       |   ## Grouping  Multiple trips **may** be grouped together using `/data/attributes/block_id`. A block represents a series of trips scheduled to be operated by the same vehicle.  ## Naming  There are 3 names associated with a trip.  | API Field                   | GTFS              | Show users? | |-----------------------------|-------------------|-------------| | `/data/attributes/headsign` | `trip_headsign`   | Yes         | | `/data/attributes/name`     | `trip_short_name` | Yes         | | `/data/id`                  | `trip_id`         | No          |   

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for a trip | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for a trip | 
 **fields_trip** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;route&#x60; * &#x60;vehicle&#x60; * &#x60;service&#x60; * &#x60;shape&#x60; * &#x60;predictions&#x60; * &#x60;route_pattern&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  | include         | Description | |-----------------|-------------| | &#x60;route&#x60;         | The *primary* route for the trip. | | &#x60;vehicle&#x60;       | The vehicle on this trip. | | &#x60;service&#x60;       | The service controlling when this trip is active. | | &#x60;shape&#x60;         | The shape of the trip. | | &#x60;route_pattern&#x60; | The route pattern for the trip. | | &#x60;predictions&#x60;   | Predictions of when the &#x60;vehicle&#x60; on this &#x60;trip&#x60; will arrive at or depart from each stop on the route(s) on the &#x60;trip&#x60;. |   | 

### Return type

[**::models::Trip**](Trip.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

