# RouteResourceAttributes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **i32** | | Value | Name          | Example    | |-------|---------------|------------| | &#x60;0&#x60;   | Light Rail    | Green Line | | &#x60;1&#x60;   | Heavy Rail    | Red Line   | | &#x60;2&#x60;   | Commuter Rail |            | | &#x60;3&#x60;   | Bus           |            | | &#x60;4&#x60;   | Ferry         |            |  | [optional] [default to null]
**text_color** | **String** | A legible color to use for text drawn against a background of the route&#39;s &#x60;color&#x60; attribute. See [GTFS &#x60;routes.txt&#x60; &#x60;route_text_color&#x60;](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional] [default to null]
**sort_order** | **i32** | Routes sort in ascending order | [optional] [default to null]
**short_name** | **String** | This will often be a short, abstract identifier like \&quot;32\&quot;, \&quot;100X\&quot;, or \&quot;Green\&quot; that riders use to identify a route, but which doesn&#39;t give any indication of what places the route serves. See [GTFS &#x60;routes.txt&#x60; &#x60;route_short_name&#x60;](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional] [default to null]
**long_name** | **String** | The full name of a route. This name is generally more descriptive than the &#x60;short_name&#x60; and will often include the route&#39;s destination or stop. See [GTFS &#x60;routes.txt&#x60; &#x60;route_long_name&#x60;](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional] [default to null]
**fare_class** | **String** | Specifies the fare type of the route, which can differ from the service category.  | [optional] [default to null]
**direction_names** | **Vec<String>** |  | [optional] [default to null]
**direction_destinations** | **Vec<String>** |  | [optional] [default to null]
**description** | **String** | Details about stops, schedule, and/or service.  See [GTFS &#x60;routes.txt&#x60; &#x60;route_desc&#x60;](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional] [default to null]
**color** | **String** | A color that corresponds to the route, such as the line color on a map.\&quot; See [GTFS &#x60;routes.txt&#x60; &#x60;route_color&#x60;](https://github.com/google/transit/blob/master/gtfs/spec/en/reference.md#routestxt).  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


