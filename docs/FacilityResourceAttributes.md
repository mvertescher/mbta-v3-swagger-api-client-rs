# FacilityResourceAttributes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of the facility. | [optional] [default to null]
**short_name** | **String** | Short name of the facility | [optional] [default to null]
**properties** | [**Vec<::models::FacilityProperty>**](FacilityProperty.md) | A list of name/value pairs that apply to the facility. See [MBTA&#39;s facility documentation](https://www.mbta.com/developers/gtfs/f#facilities_properties_definitions) for more information on the possible names and values. | [optional] [default to null]
**name** | **String** | Name of the facility | [optional] [default to null]
**longitude** | **f32** | Longitude of the facility. Degrees East, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#Longitudes_on_WGS.C2.A084) coordinate system. See [GTFS &#x60;facilities.txt&#x60; &#x60;facility_lon&#x60;]  | [optional] [default to null]
**latitude** | **f32** | Latitude of the facility.  Degrees North, in the [WGS-84](https://en.wikipedia.org/wiki/World_Geodetic_System#A_new_World_Geodetic_System:_WGS.C2.A084) coordinate system. See [GTFS &#x60;facilities.txt&#x60; &#x60;facility_lat&#x60;]  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


