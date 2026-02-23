# PageRevision

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**revision_id** | Option<**i32**> | an identifier for this revision of the page | [optional]
**updated_at** | Option<**String**> | the time when this revision was saved | [optional]
**latest** | Option<**bool**> | whether this is the latest revision or not | [optional]
**edited_by** | Option<[**models::User**](User.md)> |  | [optional]
**url** | Option<**String**> | the following fields are not included in the index action and may be omitted from the show action via summary=1 the historic url of the page | [optional]
**title** | Option<**String**> | the historic page title | [optional]
**body** | Option<**String**> | the historic page contents | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


