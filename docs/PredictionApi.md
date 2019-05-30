# \PredictionApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_prediction_controller_index**](PredictionApi.md#api_web_prediction_controller_index) | **Get** /predictions | 


# **api_web_prediction_controller_index**
> ::models::Predictions api_web_prediction_controller_index(ctx, ctx, optional)


**NOTE:** A filter **MUST** be present for any predictions to be returned.  List of predictions for trips.  To get the scheduled times instead of the predictions, use `/schedules`.  The predicted arrival time (`//data/{index}/attributes/arrival_time`) and departure time (`/data/{index}/attributes/departure_time`) to/from a stop (`/data/{index}/relationships/stop/data/id`) at a given sequence (`/data/{index}/attriutes/stop_sequence`) along a trip (`/data/{index}/relationships/trip/data/id`) going a direction (`/data/{index}/attributes/direction_id`) along a route (`/data/{index}/relationships/route/data/id`).  See [GTFS Realtime `FeedMesage` `FeedEntity` `TripUpdate` `TripDescriptor`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-tripdescriptor) See [GTFS Realtime `FeedMesage` `FeedEntity` `TripUpdate` `StopTimeUpdate`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-stoptimeupdate)   ## When a vehicle is predicted to be at a stop  `/predictions?filter[stop]=STOP_ID`  ## The predicted schedule for one route  `/predictions?filter[route]=ROUTE_ID`  ## The predicted schedule for a whole trip  `/predictions?filter[trip]=TRIP_ID`  

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/arrival_time&#x60; | ascending | &#x60;arrival_time&#x60; | | &#x60;/data/{index}/attributes/arrival_time&#x60; | descending | &#x60;-arrival_time&#x60; | | &#x60;/data/{index}/attributes/departure_time&#x60; | ascending | &#x60;departure_time&#x60; | | &#x60;/data/{index}/attributes/departure_time&#x60; | descending | &#x60;-departure_time&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | ascending | &#x60;direction_id&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | descending | &#x60;-direction_id&#x60; | | &#x60;/data/{index}/attributes/schedule_relationship&#x60; | ascending | &#x60;schedule_relationship&#x60; | | &#x60;/data/{index}/attributes/schedule_relationship&#x60; | descending | &#x60;-schedule_relationship&#x60; | | &#x60;/data/{index}/attributes/status&#x60; | ascending | &#x60;status&#x60; | | &#x60;/data/{index}/attributes/status&#x60; | descending | &#x60;-status&#x60; | | &#x60;/data/{index}/attributes/stop_sequence&#x60; | ascending | &#x60;stop_sequence&#x60; | | &#x60;/data/{index}/attributes/stop_sequence&#x60; | descending | &#x60;-stop_sequence&#x60; |   | 
 **fields_prediction** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;schedule&#x60; * &#x60;stop&#x60; * &#x60;route&#x60; * &#x60;trip&#x60; * &#x60;vehicle&#x60; * &#x60;alerts&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)  Here is an example: &#x60;https://api-v3.mbta.com/predictions?filter%5Bstop%5D&#x3D;place-sstat&amp;filter%5Bdirection_id%5D&#x3D;0&amp;include&#x3D;stop&#x60; returns predictions from South Station with direction_id&#x3D;0, below is a truncated response with only relevant fields displayed: &#x60;&#x60;&#x60;   {     \&quot;data\&quot;: [       {         \&quot;id\&quot;: \&quot;prediction-CR-Weekday-Fall-18-743-South Station-02-1\&quot;,         \&quot;relationships\&quot;: {           \&quot;stop\&quot;: {             \&quot;data\&quot;: {               \&quot;id\&quot;: \&quot;South Station-02\&quot;,               \&quot;type\&quot;: \&quot;stop\&quot;             }           },         },         \&quot;type\&quot;: \&quot;prediction\&quot;       }     ],     \&quot;included\&quot;: [       {         \&quot;attributes\&quot;: {           \&quot;platform_code\&quot;: \&quot;2\&quot;,         },         \&quot;id\&quot;: \&quot;South Station-02\&quot;,         \&quot;type\&quot;: \&quot;stop\&quot;       }     ],   } &#x60;&#x60;&#x60; Note the stop relationship; use it to cross-reference  stop-id with the included stops to retrieve the platform_code for the given prediction.   | 
 **filter_latitude** | **String**|  Latitude/Longitude must be both present or both absent. | 
 **filter_longitude** | **String**|  Latitude/Longitude must be both present or both absent. | 
 **filter_radius** | **String**|  Radius accepts a floating point number, and the default is 0.01.  For example, if you query for: latitude: 42,  longitude: -71,  radius: 0.05 then you will filter between latitudes 41.95 and 42.05, and longitudes -70.95 and -71.05. | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.     | 
 **filter_route_type** | **String**| Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple &#x60;route_type&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_stop** | **String**| Filter by &#x60;/data/{index}/relationships/stop/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/stop/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_route** | **String**| Filter by &#x60;/data/{index}/relationships/route/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/route/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_trip** | **String**| Filter by &#x60;/data/{index}/relationships/trip/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/trip/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 

### Return type

[**::models::Predictions**](Predictions.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

