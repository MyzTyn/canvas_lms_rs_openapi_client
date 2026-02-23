# CourseEventLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**course** | Option<**i32**> | ID of the course for the event. | [optional]
**user** | Option<**i32**> | ID of the user for the event (who made the change). | [optional]
**page_view** | Option<**String**> | ID of the page view during the event if it exists. | [optional]
**copied_from** | Option<**i32**> | ID of the course that this course was copied from. This is only included if the event_type is copied_from. | [optional]
**copied_to** | Option<**i32**> | ID of the course that this course was copied to. This is only included if the event_type is copied_to. | [optional]
**sis_batch** | Option<**i32**> | ID of the SIS batch that triggered the event. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


