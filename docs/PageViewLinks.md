# PageViewLinks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | Option<**i64**> | The ID of the user for this page view | [optional]
**context** | Option<**i64**> | The ID of the context for the request (course id if context_type is Course, etc) | [optional]
**asset** | Option<**i64**> | The ID of the asset for the request, if any. Not available in history CSV. | [optional]
**real_user** | Option<**i64**> | The ID of the actual user who made this request, if the request was made by a user who was masquerading | [optional]
**account** | Option<**i64**> | The ID of the account context for this page view | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


