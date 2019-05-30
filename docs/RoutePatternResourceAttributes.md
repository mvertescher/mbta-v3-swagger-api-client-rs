# RoutePatternResourceAttributes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**typicality** | **i32** | Explains how common the route pattern is. For the MBTA, this is within the context of the entire route. Current valid values are: | Value | Description | |-|-| | &#x60;0&#x60; | Not defined | | &#x60;1&#x60; | Typical. Pattern is common for the route. Most routes will have only one such pattern per direction. A few routes may have more than 1, such as the Red Line (with one branch to Ashmont and another to Braintree); routes with more than 2 are rare. | | &#x60;2&#x60; | Pattern is a deviation from the regular route. | | &#x60;3&#x60; | Pattern represents a highly atypical pattern for the route, such as a special routing which only runs a handful of times per day. | | &#x60;4&#x60; | Diversions from normal service, such as planned detours, bus shuttles, or snow routes. |  | [optional] [default to null]
**sort_order** | **i32** | Can be used to order the route patterns in a way which is ideal for presentation to customers. Route patterns with smaller sort_order values should be displayed before those with larger values.  | [optional] [default to null]
**name** | **String** | User-facing description of where trips on the route pattern serve. These names are published in the form Destination, Destination via Street or Landmark, Origin - Destination, or Origin - Destination via Street or Landmark. Note that names for bus and subway route patterns currently do not include the origin location, but will in the future.  | [optional] [default to null]
**direction_id** | **i32** | Direction in which trip is traveling: &#x60;0&#x60; or &#x60;1&#x60;.  The meaning of &#x60;direction_id&#x60; varies based on the route. You can programmatically get the direction names from &#x60;/routes&#x60; &#x60;/data/{index}/attributes/direction_names&#x60; or &#x60;/routes/{id}&#x60; &#x60;/data/attributes/direction_names&#x60;.   | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


