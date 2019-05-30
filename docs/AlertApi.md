# \AlertApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_web_alert_controller_index**](AlertApi.md#api_web_alert_controller_index) | **Get** /alerts | 
[**api_web_alert_controller_show**](AlertApi.md#api_web_alert_controller_show) | **Get** /alerts/{id} | 


# **api_web_alert_controller_index**
> ::models::Alerts api_web_alert_controller_index(ctx, ctx, optional)


List active and upcoming system alerts  An effect (enumerated in `/data/{index}/attributes/effect` and human-readable in `/data/{index}/attributes/service_effect`) on a provided service (facility, route, route type, stop and/or trip in `//data/{index}/attributes/informed_entity`) described by a banner (`/data/{index}/attributes/banner`), short header (`/data/{index}/attributes/short_header`), header `/data/{index}/attributes/header`, and description (`/data/{index}/attributes/description`) that is active for one or more periods (`/data/{index}/attributes/active_period`) caused by a cause (`/data/{index}/attribute/cause`) that somewhere in its lifecycle (enumerated in `/data/{index}/attributes/lifecycle` and human-readable in `/data/{index}/attributes/timeframe`).  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  ## Descriptions  There are 5 descriptive attributes.  | JSON pointer                                | Usage                                                                           | |---------------------------------------------|---------------------------------------------------------------------------------| | `/data/{index}/attributes/banner`       | Display as alert across application/website                                     | | `/data/{index}/attributes/short_header` | When `/data/{index}/attributes/header` is too long to display               | | `/data/{index}/attributes/header`       | Used before showing and prepended to `/data/{index}/attributes/description` | | `/data/{index}/attributes/description`  | Used when user asks to expand alert.                                            |  ## Effect  | JSON pointer                                  |                | |-----------------------------------------------|----------------| | `/data/{index}/attributes/effect`         | Enumerated     | | `/data/{index}/attributes/service_effect` | Human-readable |  ## Timeline  There are 3 timeline related attributes  | JSON pointer                                 | Description                                                                              | |----------------------------------------------|------------------------------------------------------------------------------------------| | `/data/{index}/attributes/active_period` | Exact Date/Time ranges alert is active                                                   | | `/data/{index}/attributes/lifecycle`     | Enumerated, machine-readable description of `/data/{index}/attributes/active_period` | | `/data/{index}/attributes/timeframe`     | Human-readable description of `/data/{index}/attributes/active_period`               |   ## Activities  Alerts are by default filtered to those where `/data/{index}/attributes/informed_entity/{informed_entity_index}/activities/{activity_index}` in one of BOARDEXITRIDE, as these cover most riders.  If you want all alerts without filtering by activity, you should use the special value `\"ALL\"`: `filter[activity]=ALL`.  ### Accessibility  The default activities cover if boarding, exiting, or riding is generally affected for all riders by the alert. If ONLY wheelchair using riders are affected, such as if a ramp, lift, or safety system for wheelchairs is affected, only the `\"USING_WHEELCHAIR\"` activity will be set. To cover wheelchair using rider, filter on the defaults and `\"USING_WHEELCHAIR\"`: `filter[activity]=USING_WHEELCHAIR,BOARD,EXIT,RIDE`.  Similarly for riders with limited mobility that need escalators, `\"USING_ESCALATOR\"` should be added to the defaults: `filter[activity]=USING_ESCALATOR,BOARD,EXIT,RIDE`.  

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
 **sort** | **String**| Results can be [sorted](http://jsonapi.org/format/#fetching-sorting) by the id or any &#x60;/data/{index}/attributes&#x60; key. Assumes ascending; may be prefixed with &#39;-&#39; for descending  | JSON pointer | Direction | &#x60;sort&#x60;     | |--------------|-----------|------------| | &#x60;/data/{index}/attributes/active_period&#x60; | ascending | &#x60;active_period&#x60; | | &#x60;/data/{index}/attributes/active_period&#x60; | descending | &#x60;-active_period&#x60; | | &#x60;/data/{index}/attributes/banner&#x60; | ascending | &#x60;banner&#x60; | | &#x60;/data/{index}/attributes/banner&#x60; | descending | &#x60;-banner&#x60; | | &#x60;/data/{index}/attributes/cause&#x60; | ascending | &#x60;cause&#x60; | | &#x60;/data/{index}/attributes/cause&#x60; | descending | &#x60;-cause&#x60; | | &#x60;/data/{index}/attributes/created_at&#x60; | ascending | &#x60;created_at&#x60; | | &#x60;/data/{index}/attributes/created_at&#x60; | descending | &#x60;-created_at&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | ascending | &#x60;description&#x60; | | &#x60;/data/{index}/attributes/description&#x60; | descending | &#x60;-description&#x60; | | &#x60;/data/{index}/attributes/effect&#x60; | ascending | &#x60;effect&#x60; | | &#x60;/data/{index}/attributes/effect&#x60; | descending | &#x60;-effect&#x60; | | &#x60;/data/{index}/attributes/effect_name&#x60; | ascending | &#x60;effect_name&#x60; | | &#x60;/data/{index}/attributes/effect_name&#x60; | descending | &#x60;-effect_name&#x60; | | &#x60;/data/{index}/attributes/header&#x60; | ascending | &#x60;header&#x60; | | &#x60;/data/{index}/attributes/header&#x60; | descending | &#x60;-header&#x60; | | &#x60;/data/{index}/attributes/informed_entity&#x60; | ascending | &#x60;informed_entity&#x60; | | &#x60;/data/{index}/attributes/informed_entity&#x60; | descending | &#x60;-informed_entity&#x60; | | &#x60;/data/{index}/attributes/lifecycle&#x60; | ascending | &#x60;lifecycle&#x60; | | &#x60;/data/{index}/attributes/lifecycle&#x60; | descending | &#x60;-lifecycle&#x60; | | &#x60;/data/{index}/attributes/service_effect&#x60; | ascending | &#x60;service_effect&#x60; | | &#x60;/data/{index}/attributes/service_effect&#x60; | descending | &#x60;-service_effect&#x60; | | &#x60;/data/{index}/attributes/severity&#x60; | ascending | &#x60;severity&#x60; | | &#x60;/data/{index}/attributes/severity&#x60; | descending | &#x60;-severity&#x60; | | &#x60;/data/{index}/attributes/short_header&#x60; | ascending | &#x60;short_header&#x60; | | &#x60;/data/{index}/attributes/short_header&#x60; | descending | &#x60;-short_header&#x60; | | &#x60;/data/{index}/attributes/timeframe&#x60; | ascending | &#x60;timeframe&#x60; | | &#x60;/data/{index}/attributes/timeframe&#x60; | descending | &#x60;-timeframe&#x60; | | &#x60;/data/{index}/attributes/updated_at&#x60; | ascending | &#x60;updated_at&#x60; | | &#x60;/data/{index}/attributes/updated_at&#x60; | descending | &#x60;-updated_at&#x60; | | &#x60;/data/{index}/attributes/url&#x60; | ascending | &#x60;url&#x60; | | &#x60;/data/{index}/attributes/url&#x60; | descending | &#x60;-url&#x60; |   | 
 **fields_alert** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stops&#x60; * &#x60;routes&#x60; * &#x60;trips&#x60; * &#x60;facilities&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 
 **filter_activity** | **String**| Filter to alerts for only those activities (&#x60;/data/{index}/attributes/informed_entity/activities/{activity_index}&#x60;) matching.  Multiple activities **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  An activity affected by an alert.  | Value                | Description                                                                                                                                                                                                                                                                       | |----------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------| | &#x60;\&quot;BOARD\&quot;&#x60;            | Boarding a vehicle. Any passenger trip includes boarding a vehicle and exiting from a vehicle.                                                                                                                                                                                    | | &#x60;\&quot;BRINGING_BIKE\&quot;&#x60;    | Bringing a bicycle while boarding or exiting.                                                                                                                                                                                                                                     | | &#x60;\&quot;EXIT\&quot;&#x60;             | Exiting from a vehicle (disembarking). Any passenger trip includes boarding a vehicle and exiting a vehicle.                                                                                                                                                                      | | &#x60;\&quot;PARK_CAR\&quot;&#x60;         | Parking a car at a garage or lot in a station.                                                                                                                                                                                                                                    | | &#x60;\&quot;RIDE\&quot;&#x60;             | Riding through a stop without boarding or exiting.. Not every passenger trip will include this -- a passenger may board at one stop and exit at the next stop.                                                                                                                    | | &#x60;\&quot;STORE_BIKE\&quot;&#x60;       | Storing a bicycle at a station.                                                                                                                                                                                                                                                   | | &#x60;\&quot;USING_ESCALATOR\&quot;&#x60;  | Using an escalator while boarding or exiting (should only be used for customers who specifically want to avoid stairs.)                                                                                                                                                           | | &#x60;\&quot;USING_WHEELCHAIR\&quot;&#x60; | Using a wheelchair while boarding or exiting. Note that this applies to something that specifically affects customers who use a wheelchair to board or exit; a delay should not include this as an affected activity unless it specifically affects customers using wheelchairs.  |   ## Special Values  * If the filter is not given OR it is empty, then defaults to [\&quot;BOARD\&quot;, \&quot;EXIT\&quot;, \&quot;RIDE\&quot;]. * If the value &#x60;\&quot;ALL\&quot;&#x60; is used then all alerts will be returned, not just those with the default activities.  ## Accessibility  The default activities cover if boarding, exiting, or riding is generally affected for all riders by the alert. If ONLY wheelchair using riders are affected, such as if a ramp, lift, or safety system for wheelchairs is affected, only the &#x60;\&quot;USING_WHEELCHAIR\&quot;&#x60; activity will be set. To cover wheelchair using rider, filter on the defaults and &#x60;\&quot;USING_WHEELCHAIR\&quot;&#x60;: &#x60;filter[activity]&#x3D;USING_WHEELCHAIR,BOARD,EXIT,RIDE&#x60;.  Similarly for riders with limited mobility that need escalators, &#x60;\&quot;USING_ESCALATOR\&quot;&#x60; should be added to the defaults: &#x60;filter[activity]&#x3D;USING_ESCALATOR,BOARD,EXIT,RIDE&#x60;.   | [default to BOARD,EXIT,RIDE]
 **filter_route_type** | **String**| Filter by route_type: https://developers.google.com/transit/gtfs/reference/routes-file.  Multiple &#x60;route_type&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_direction_id** | **String**| Filter by direction of travel along the route.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.     | 
 **filter_route** | **String**| Filter by &#x60;/data/{index}/relationships/route/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/route/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_stop** | **String**| Filter by &#x60;/data/{index}/relationships/stop/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/stop/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_trip** | **String**| Filter by &#x60;/data/{index}/relationships/trip/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/trip/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_facility** | **String**| Filter by &#x60;/data/{index}/relationships/facility/data/id&#x60;. Multiple &#x60;/data/{index}/relationships/facility/data/id&#x60; **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_id** | **String**| Filter by multiple IDs. Multiple IDs **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list. | 
 **filter_banner** | **String**| When combined with other filters, filters by alerts with or without a banner. **MUST** be \&quot;true\&quot; or \&quot;false\&quot;.  | 
 **filter_datetime** | **String**| Filter to alerts that are active at a given time (ISO8601 format).  Additionally, the string \&quot;NOW\&quot; can be used to filter to alerts that are currently active.  | 
 **filter_lifecycle** | **String**| Filters by an alert&#39;s lifecycle. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  | 
 **filter_severity** | **String**| Filters alerts by list of severities. **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Example: filter[severity]&#x3D;3,4,10 returns alerts with severity levels 3, 4 and 10.  | 

### Return type

[**::models::Alerts**](Alerts.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api_web_alert_controller_show**
> ::models::Alert api_web_alert_controller_show(ctx, ctx, id, optional)


Show a particular alert by the alert's id  An effect (enumerated in `/data/attributes/effect` and human-readable in `/data/attributes/service_effect`) on a provided service (facility, route, route type, stop and/or trip in `//data/attributes/informed_entity`) described by a banner (`/data/attributes/banner`), short header (`/data/attributes/short_header`), header `/data/attributes/header`, and description (`/data/attributes/description`) that is active for one or more periods (`/data/attributes/active_period`) caused by a cause (`/data/attribute/cause`) that somewhere in its lifecycle (enumerated in `/data/attributes/lifecycle` and human-readable in `/data/attributes/timeframe`).  See [GTFS Realtime `FeedMessage` `FeedEntity` `Alert`](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/reference.md#message-alert)  ## Descriptions  There are 5 descriptive attributes.  | JSON pointer                                | Usage                                                                           | |---------------------------------------------|---------------------------------------------------------------------------------| | `/data/attributes/banner`       | Display as alert across application/website                                     | | `/data/attributes/short_header` | When `/data/attributes/header` is too long to display               | | `/data/attributes/header`       | Used before showing and prepended to `/data/attributes/description` | | `/data/attributes/description`  | Used when user asks to expand alert.                                            |  ## Effect  | JSON pointer                                  |                | |-----------------------------------------------|----------------| | `/data/attributes/effect`         | Enumerated     | | `/data/attributes/service_effect` | Human-readable |  ## Timeline  There are 3 timeline related attributes  | JSON pointer                                 | Description                                                                              | |----------------------------------------------|------------------------------------------------------------------------------------------| | `/data/attributes/active_period` | Exact Date/Time ranges alert is active                                                   | | `/data/attributes/lifecycle`     | Enumerated, machine-readable description of `/data/attributes/active_period` | | `/data/attributes/timeframe`     | Human-readable description of `/data/attributes/active_period`               |  

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Unique identifier for alert | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Unique identifier for alert | 
 **fields_alert** | **String**| Fields to include with the response. Multiple fields **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list.  Note that fields can also be selected for included data types: see the [V3 API Best Practices](https://www.mbta.com/developers/v3-api/best-practices) for an example.  | 
 **include** | **String**| Relationships to include.  * &#x60;stops&#x60; * &#x60;routes&#x60; * &#x60;trips&#x60; * &#x60;facilities&#x60;  The value of the include parameter **MUST** be a comma-separated (U+002C COMMA, \&quot;,\&quot;) list of relationship paths. A relationship path is a dot-separated (U+002E FULL-STOP, \&quot;.\&quot;) list of relationship names. [JSONAPI \&quot;include\&quot; behavior](http://jsonapi.org/format/#fetching-includes)    | 

### Return type

[**::models::Alert**](Alert.md)

### Authorization

[api_key_in_header](../README.md#api_key_in_header), [api_key_in_query](../README.md#api_key_in_query)

### HTTP request headers

 - **Content-Type**: application/vnd.api+json
 - **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
