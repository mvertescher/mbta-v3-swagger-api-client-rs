# \VehicleApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_vehicle_controller_index**](VehicleApi.md#api_web_vehicle_controller_index) | **Get** /vehicles | 
[**api_web_vehicle_controller_show**](VehicleApi.md#api_web_vehicle_controller_show) | **Get** /vehicles/{id} | 


# **api_web_vehicle_controller_index**
> ::models::Vehicles api_web_vehicle_controller_index(ctx, ctx, optional)


List of vehicles (buses, ferries, and trains)  ## Direction  ### World  To figure out which way the vehicle is pointing at the location, use `/data/{index}/attributes/bearing`.  This can be the compass bearing, or the direction towards the next stop or intermediate location.  ### Trip  To get the direction around the stops in the trip use `/data/{index}/attributes/direction_id`.  ## Location  ### World  Use `/data/{index}/attributes/latitude` and `/data/{index}/attributes/longitude` to get the location of a vehicle.  ### Trip  Use `/data/{index}/attributes/current_stop_sequence` to get the stop number along the trip.  Useful for linear stop indicators.  Position relative to the current stop is in `/data/{index}/attributes/current_status`.  ## Movement  ### World  Use `/data/{index}/attributes/speed` to get the speed of the vehicle in meters per second.  

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/bearing&#x60; | ascending | &#x60;bearing&#x60; | | &#x60;/data/{index}/attributes/bearing&#x60; | descending | &#x60;-bearing&#x60; | | &#x60;/data/{index}/attributes/current_status&#x60; | ascending | &#x60;current_status&#x60; | | &#x60;/data/{index}/attributes/current_status&#x60; | descending | &#x60;-current_status&#x60; | | &#x60;/data/{index}/attributes/current_stop_sequence&#x60; | ascending | &#x60;current_stop_sequence&#x60; | | &#x60;/data/{index}/attributes/current_stop_sequence&#x60; | descending | &#x60;-current_stop_sequence&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | ascending | &#x60;direction_id&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | descending | &#x60;-direction_id&#x60; | | &#x60;/data/{index}/attributes/label&#x60; | ascending | &#x60;label&#x60; | | &#x60;/data/{index}/attributes/label&#x60; | descending | &#x60;-label&#x60; | | &#x60;/data/{index}/attributes/latitude&#x60; | ascending | &#x60;latitude&#x60; | | &#x60;/data/{index}/attributes/latitude&#x60; | descending | &#x60;-latitude&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | ascending | &#x60;longitude&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | descending | &#x60;-longitude&#x60; | | &#x60;/data/{index}/attributes/speed&#x60; | ascending | &#x60;speed&#x60; | | &#x60;/data/{index}/attributes/speed&#x60; | descending | &#x60;-speed&#x60; | | &#x60;/data/{index}/attributes/updated_at&#x60; | ascending | &#x60;updated_at&#x60; | | &#x60;/data/{index}/attributes/updated_at&#x60; | descending | &#x60;-updated_at&#x60; |   | 
 **fields_vehicle** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;trip&#x60; * &#x60;stop&#x60; * &#x60;route&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description                                                                                                                                                                                                                                                                                                                                                  | |---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| | &#x60;trip&#x60;  | The trip which the vehicle is currently operating.                                                                                                                                                                                                                                                                                                           | | &#x60;stop&#x60;  | The vehicle&#39;s current (when &#x60;current_status&#x60; is **STOPPED_AT**) or *next* stop.                                                                                                                                                                                                                                                                              | | &#x60;route&#x60; | The one route that is designated for that trip, as in GTFS &#x60;trips.txt&#x60;.  A trip might also provide service on other routes, identified by the MBTA&#39;s &#x60;multi_route_trips.txt&#x60; GTFS extension. &#x60;filter[route]&#x60; does consider the multi_route_trips GTFS extension, so it is possible to filter for one route and get a different route included in the response. |   | 
 **filter_id** | **String**| Filter by multiple IDs. Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. Cannot be combined with any other filter. | 
 **filter_trip** | **String**| Filter by &#x60;/data/{index}/relationships/trip/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/trip/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. Cannot be combined with any other filter. | 
 **filter_label** | **String**| Filter by label. Multiple &#x60;label&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_route** | **String**| Filter by route. If the vehicle is on a [multi-route trip](https://groups.google.com/forum/#!msg/massdotdevelopers/1egrhNjT9eA/iy6NFymcCgAJ), it will be returned for any of the routes. Multiple &#x60;route_id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.   Only used if &#x60;filter[route]&#x60; is also present.  | 
 **filter_route_type** | **String**| Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple &#x60;route_type&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 

### Return type

[**::models::Vehicles**](Vehicles.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_vehicle_controller_show**
> ::models::Vehicle api_web_vehicle_controller_show(ctx, ctx, id, optional)


Single vehicle (bus, ferry, or train)  ## Direction  ### World  To figure out which way the vehicle is pointing at the location, use `/data/attributes/bearing`.  This can be the compass bearing, or the direction towards the next stop or intermediate location.  ### Trip  To get the direction around the stops in the trip use `/data/attributes/direction_id`.  ## Location  ### World  Use `/data/attributes/latitude` and `/data/attributes/longitude` to get the location of a vehicle.  ### Trip  Use `/data/attributes/current_stop_sequence` to get the stop number along the trip.  Useful for linear stop indicators.  Position relative to the current stop is in `/data/attributes/current_status`.  ## Movement  ### World  Use `/data/attributes/speed` to get the speed of the vehicle in meters per second.  

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for a vehicle | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for a vehicle | 
 **fields_vehicle** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;trip&#x60; * &#x60;stop&#x60; * &#x60;route&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  | include | Description                                                                                                                                                                                                                                                                                                                                                  | |---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| | &#x60;trip&#x60;  | The trip which the vehicle is currently operating.                                                                                                                                                                                                                                                                                                           | | &#x60;stop&#x60;  | The vehicle&#39;s current (when &#x60;current_status&#x60; is **STOPPED_AT**) or *next* stop.                                                                                                                                                                                                                                                                              | | &#x60;route&#x60; | The one route that is designated for that trip, as in GTFS &#x60;trips.txt&#x60;.  A trip might also provide service on other routes, identified by the MBTA&#39;s &#x60;multi_route_trips.txt&#x60; GTFS extension. &#x60;filter[route]&#x60; does consider the multi_route_trips GTFS extension, so it is possible to filter for one route and get a different route included in the response. |   | 

### Return type

[**::models::Vehicle**](Vehicle.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

