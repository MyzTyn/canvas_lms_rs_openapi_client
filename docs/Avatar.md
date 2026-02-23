# Avatar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | ['gravatar'|'attachment'|'no_pic'] The type of avatar record, for categorization purposes. | 
**url** | **String** | The url of the avatar | 
**token** | **String** | A unique representation of the avatar record which can be used to set the avatar with the user update endpoint. Note: this is an internal representation and is subject to change without notice. It should be consumed with this api endpoint and used in the user update endpoint, and should not be constructed by the client. | 
**display_name** | **String** | A textual description of the avatar record. | 
**id** | Option<**i32**> | ['attachment' type only] the internal id of the attachment | [optional]
**content_type** | Option<**String**> | ['attachment' type only] the content-type of the attachment. | [optional]
**filename** | Option<**String**> | ['attachment' type only] the filename of the attachment | [optional]
**size** | Option<**i32**> | ['attachment' type only] the size of the attachment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


