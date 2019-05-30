# \ScheduleApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_schedule_controller_index**](ScheduleApi.md#api_web_schedule_controller_index) | **Get** /schedules | 


# **api_web_schedule_controller_index**
> ::models::Schedules api_web_schedule_controller_index(ctx, ctx, optional)


**NOTE:** `filter[route]`, `filter[stop]`, or `filter[trip]` **MUST** be present for any schedules to be returned.  List of schedules.  To get a realtime prediction instead of the scheduled times, use `/predictions`.  A schedule is the arrival drop off (`/data/{index}/attributes/drop_off_type`) time (`/data/{index}/attributes/arrival_time`) and departure pick up (`/data/{index}/attributes/pickup_type`) time (`/data/{index}/attributes/departure_time`) to/from a stop (`/data/{index}/relationships/stop/data/id`) at a given sequence (`/data/{index}/attributes/stop_sequence`) along a trip (`/data/{index}/relationships/trip/data/id`) going in a direction (`/data/{index}/attributes/direction_id`) on a route (`/data/{index}/relationships/route/data/id`) when the trip is following a service (`/data/{index}/relationships/service/data/id`) to determine when it is active.  See [GTFS `stop_times.txt`](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#stop_timestxt) for base specification.   ## When a vehicle is scheduled to be at a stop  `/schedules?filter[stop]=STOP_ID`  ## The schedule for one route  `/schedules?filter[route]=ROUTE_ID`  ### When a route is open  Query for the `first` and `last` stops on the route.  `/schedules?filter[route]=ROUTE_ID&filter[stop_sequence]=first,last`  ## The schedule for a whole trip  `/schedule?filter[trip]=TRIP_ID`  

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/arrival_time&#x60; | ascending | &#x60;arrival_time&#x60; | | &#x60;/data/{index}/attributes/arrival_time&#x60; | descending | &#x60;-arrival_time&#x60; | | &#x60;/data/{index}/attributes/departure_time&#x60; | ascending | &#x60;departure_time&#x60; | | &#x60;/data/{index}/attributes/departure_time&#x60; | descending | &#x60;-departure_time&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | ascending | &#x60;direction_id&#x60; | | &#x60;/data/{index}/attributes/direction_id&#x60; | descending | &#x60;-direction_id&#x60; | | &#x60;/data/{index}/attributes/drop_off_type&#x60; | ascending | &#x60;drop_off_type&#x60; | | &#x60;/data/{index}/attributes/drop_off_type&#x60; | descending | &#x60;-drop_off_type&#x60; | | &#x60;/data/{index}/attributes/pickup_type&#x60; | ascending | &#x60;pickup_type&#x60; | | &#x60;/data/{index}/attributes/pickup_type&#x60; | descending | &#x60;-pickup_type&#x60; | | &#x60;/data/{index}/attributes/stop_sequence&#x60; | ascending | &#x60;stop_sequence&#x60; | | &#x60;/data/{index}/attributes/stop_sequence&#x60; | descending | &#x60;-stop_sequence&#x60; | | &#x60;/data/{index}/attributes/timepoint&#x60; | ascending | &#x60;timepoint&#x60; | | &#x60;/data/{index}/attributes/timepoint&#x60; | descending | &#x60;-timepoint&#x60; |   | 
 **fields_schedule** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stop&#x60; * &#x60;trip&#x60; * &#x60;prediction&#x60; * &#x60;route&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 
 **filter_date** | **String**| Filter schedule by date that they are active. The active date is the service date. Trips that begin between midnight and 3am are considered part of the previous service day. The format is ISO8601 with the template of YYYY-MM-DD. | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.     | 
 **filter_min_time** | **String**| Time before which schedule should not be returned. To filter times after midnight use more than 24 hours. For example, min_time&#x3D;24:00 will return schedule information for the next calendar day, since that service is considered part of the current service day. Additionally, min_time&#x3D;00:00&amp;max_time&#x3D;02:00 will not return anything. The time format is HH:MM. | 
 **filter_max_time** | **String**| Time after which schedule should not be returned. To filter times after midnight use more than 24 hours. For example, min_time&#x3D;24:00 will return schedule information for the next calendar day, since that service is considered part of the current service day. Additionally, min_time&#x3D;00:00&amp;max_time&#x3D;02:00 will not return anything. The time format is HH:MM. | 
 **filter_route** | **String**| Filter by &#x60;/data/{index}/relationships/route/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/route/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_stop** | **String**| Filter by &#x60;/data/{index}/relationships/stop/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/stop/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_trip** | **String**| Filter by &#x60;/data/{index}/relationships/trip/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/trip/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_stop_sequence** | **String**| Filter by the index of the stop in the trip.  Symbolic values &#x60;first&#x60; and &#x60;last&#x60; can be used instead of numeric sequence number too.  | 

### Return type

[**::models::Schedules**](Schedules.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

