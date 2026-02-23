# Report

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the report. | [optional]
**file_url** | Option<**String**> | The url to the report download. | [optional]
**attachment** | Option<[**models::File**](File.md)> |  | [optional]
**status** | Option<**String**> | The status of the report | [optional]
**created_at** | Option<**String**> | The date and time the report was created. | [optional]
**started_at** | Option<**String**> | The date and time the report started processing. | [optional]
**ended_at** | Option<**String**> | The date and time the report finished processing. | [optional]
**parameters** | Option<**serde_json::Value**> | The parameters returned will vary for each report. | [optional]
**progress** | Option<**i32**> | The progress of the report | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


