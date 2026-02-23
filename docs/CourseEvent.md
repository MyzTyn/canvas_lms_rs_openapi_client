# CourseEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the event. | [optional]
**created_at** | Option<**String**> | timestamp of the event | [optional]
**event_type** | Option<**String**> | Course event type The event type defines the type and schema of the event_data object. | [optional]
**event_data** | Option<**String**> | Course event data depending on the event type.  This will return an object containing the relevant event data.  An updated event type will return an UpdatedEventData object. | [optional]
**event_source** | Option<**String**> | Course event source depending on the event type.  This will return a string containing the source of the event. | [optional]
**links** | Option<[**models::CourseEventLink**](CourseEventLink.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


