# \StopApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_stop_controller_index**](StopApi.md#api_web_stop_controller_index) | **Get** /stops | 
[**api_web_stop_controller_show**](StopApi.md#api_web_stop_controller_show) | **Get** /stops/{id} | 


# **api_web_stop_controller_index**
> ::models::Stops api_web_stop_controller_index(ctx, ctx, optional)


List stops.  ## Accessibility  Wheelchair boarding (`/data/{index}/attributes/wheelchair_boarding`) corresponds to [GTFS wheelchair_boarding](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt). The MBTA handles parent station inheritance itself, so value can be treated simply:  | Value | Meaning                                       | |-------|-----------------------------------------------| | `0`   | No Information                                | | `1`   | Accessible (if trip is wheelchair accessible) | | `2`   | Inaccessible                                  |   ## Location  ### World  Use `/data/{index}/attributes/latitude` and `/data/{index}/attributes/longitude` to get the location of a stop.  ### Entrance  The stop may be inside a station.  If `/data/{index}/relationships/parent_station/data/id` is present, you should look up the parent station (`/stops/{parent_id}`) and use its location to give direction first to the parent station and then route from there to the stop.    ### Nearby  The `filter[latitude]` and `filter[longitude]` can be used together to find any stops near that latitude and longitude.  The distance is in degrees as if latitude and longitude were on a flat 2D plane and normal Pythagorean distance was calculated.  Over the region MBTA serves, `0.02` degrees is approximately `1` mile. How close is considered nearby, is controlled by `filter[radius]`, which default to `0.01` degrees (approximately a half mile). 

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Sorting by distance requires &#x60;filter[latitude]&#x60; and &#x60;filter[longitude]&#x60; to be set. Assumes ascending; may be prefixed with &#39;-&#39; for descending.  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/address&#x60; | ascending | &#x60;address&#x60; | | &#x60;/data/{index}/attributes/address&#x60; | descending | &#x60;-address&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | ascending | &#x60;description&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | descending | &#x60;-description&#x60; | | &#x60;/data/{index}/attributes/latitude&#x60; | ascending | &#x60;latitude&#x60; | | &#x60;/data/{index}/attributes/latitude&#x60; | descending | &#x60;-latitude&#x60; | | &#x60;/data/{index}/attributes/location_type&#x60; | ascending | &#x60;location_type&#x60; | | &#x60;/data/{index}/attributes/location_type&#x60; | descending | &#x60;-location_type&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | ascending | &#x60;longitude&#x60; | | &#x60;/data/{index}/attributes/longitude&#x60; | descending | &#x60;-longitude&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | ascending | &#x60;name&#x60; | | &#x60;/data/{index}/attributes/name&#x60; | descending | &#x60;-name&#x60; | | &#x60;/data/{index}/attributes/platform_code&#x60; | ascending | &#x60;platform_code&#x60; | | &#x60;/data/{index}/attributes/platform_code&#x60; | descending | &#x60;-platform_code&#x60; | | &#x60;/data/{index}/attributes/platform_name&#x60; | ascending | &#x60;platform_name&#x60; | | &#x60;/data/{index}/attributes/platform_name&#x60; | descending | &#x60;-platform_name&#x60; | | &#x60;/data/{index}/attributes/wheelchair_boarding&#x60; | ascending | &#x60;wheelchair_boarding&#x60; | | &#x60;/data/{index}/attributes/wheelchair_boarding&#x60; | descending | &#x60;-wheelchair_boarding&#x60; |  | Distance to (&#x60;/data/{index}/attributes/latitude&#x60;, &#x60;/data/{index}/attributes/longitude&#x60;) | ascending | &#x60;distance&#x60; | | Distance to (&#x60;/data/{index}/attributes/latitude&#x60;, &#x60;/data/{index}/attributes/longitude&#x60;) | descending | &#x60;-distance&#x60; |   | 
 **fields_stop** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;parent_station&#x60; * &#x60;child_stops&#x60; * &#x60;recommended_transfers&#x60; * &#x60;facilities&#x60; * &#x60;route&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  Note that &#x60;route&#x60; can only be included if &#x60;filter[route]&#x60; is present and has exactly one &#x60;/data/{index}/relationships/route/data/id&#x60;.  | 
 **filter_date** | **String**| Filter by date when stop is in use The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.     | 
 **filter_latitude** | **String**| Latitude in degrees North in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system to search &#x60;filter[radius]&#x60; degrees around with &#x60;filter[longitude]&#x60;.  | 
 **filter_longitude** | **String**| Longitude in degrees East in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system to search &#x60;filter[radius]&#x60; degrees around with &#x60;filter[latitude]&#x60;.  | 
 **filter_radius** | **f32**| The distance is in degrees as if latitude and longitude were on a flat 2D plane and normal Pythagorean distance was calculated.  Over the region MBTA serves, &#x60;0.02&#x60; degrees is approximately &#x60;1&#x60; mile. Defaults to &#x60;0.01&#x60; degrees (approximately a half mile).  | 
 **filter_id** | **String**| Filter by &#x60;/data/{index}/id&#x60; (the stop ID). Multiple &#x60;/data/{index}/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_route_type** | **String**| Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple &#x60;route_type&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_route** | **String**| Filter by &#x60;/data/{index}/relationships/route/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/route/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_location_type** | **String**| Filter by location_type https://github.com/mbta/gtfs-documentation/blob/master/reference/gtfs.md#stopstxt. Multiple location_type **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list  | 

### Return type

[**::models::Stops**](Stops.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_stop_controller_show**
> ::models::Stop api_web_stop_controller_show(ctx, ctx, id, optional)


Detail for a specific stop.  ## Accessibility  Wheelchair boarding (`/data/attributes/wheelchair_boarding`) corresponds to [GTFS wheelchair_boarding](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stopstxt). The MBTA handles parent station inheritance itself, so value can be treated simply:  | Value | Meaning                                       | |-------|-----------------------------------------------| | `0`   | No Information                                | | `1`   | Accessible (if trip is wheelchair accessible) | | `2`   | Inaccessible                                  |   ## Location  ### World  Use `/data/attributes/latitude` and `/data/attributes/longitude` to get the location of a stop.  ### Entrance  The stop may be inside a station.  If `/data/relationships/parent_station/data/id` is present, you should look up the parent station (`/stops/{parent_id}`) and use its location to give direction first to the parent station and then route from there to the stop.   

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for stop | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for stop | 
 **fields_stop** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;parent_station&#x60; * &#x60;child_stops&#x60; * &#x60;recommended_transfers&#x60; * &#x60;facilities&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 

### Return type

[**::models::Stop**](Stop.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

