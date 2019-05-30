# ServiceResourceAttributes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid_days** | **Vec<f32>** |  | [optional] [default to null]
**start_date** | [***String**](string.md) | Earliest date which is valid for this service. Format is ISO8601. | [optional] [default to null]
**schedule_typicality** | **i32** | Describes how well this schedule represents typical service for the listed &#x60;schedule_type&#x60;  | Value | Description                                                                 | |-------|-----------------------------------------------------------------------------| | &#x60;0&#x60;   | Not defined.                                                                | | &#x60;1&#x60;   | Typical service with perhaps minor modifications                            | | &#x60;2&#x60;   | Extra service supplements typical schedules                                 | | &#x60;3&#x60;   | Reduced holiday service is provided by typical Saturday or Sunday schedule  | | &#x60;4&#x60;   | Major changes in service due to a planned disruption, such as construction  | | &#x60;5&#x60;   | Major reductions in service for weather events or other atypical situations |  | [optional] [default to null]
**schedule_type** | **String** | Description of the schedule type the service_id can be applied. For example, on a holiday, the schedule_type value may be \&quot;Saturday\&quot; or \&quot;Sunday\&quot;. Current valid values are \&quot;Weekday\&quot;, \&quot;Saturday\&quot;, \&quot;Sunday\&quot;, or \&quot;Other\&quot;  | [optional] [default to null]
**schedule_name** | **String** | Description of when the &#x60;service_id&#x60; is in effect. | [optional] [default to null]
**removed_dates_notes** | **Vec<String>** |  | [optional] [default to null]
**removed_dates** | [**Vec<String>**](string.md) |  | [optional] [default to null]
**end_date** | [***String**](string.md) | Latest date which is valid for this service. Format is ISO8601. | [optional] [default to null]
**description** | **String** | Human-readable description of the service, as it should appear on public-facing websites and applications. | [optional] [default to null]
**added_dates_notes** | **Vec<String>** |  | [optional] [default to null]
**added_dates** | [**Vec<String>**](string.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


