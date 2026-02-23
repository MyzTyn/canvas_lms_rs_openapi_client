# ErrorsCreateRequestError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | **String** | [Required, String] The summary of the problem | 
**url** | Option<**String**> | [Optional, String] URL from which the report was issued | [optional]
**email** | Option<**String**> | [Optional, String] Email address for the reporting user | [optional]
**comments** | Option<**String**> | [Optional, String] The long version of the story from the user one what they experienced | [optional]
**http_env** | Option<**String**> | [Optional, SerializedHash] A collection of metadata about the users' environment.  If not provided, canvas will collect it based on information found in the request. (Doesn't have to be HTTPENV info, could be anything JSON object that can be serialized as a hash, a mobile app might include relevant metadata for itself) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


