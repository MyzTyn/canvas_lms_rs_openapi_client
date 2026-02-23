# OutcomeGroupsApiImportOtherRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_outcome_group_id** | **i32** | The ID of the source outcome group. | 
**r#async** | Option<**bool**> | If true, perform action asynchronously.  In that case, this endpoint will return a Progress object instead of an OutcomeGroup. Use the {api:ProgressController#show progress endpoint} to query the status of the operation.  The imported outcome group id and url will be returned in the results of the Progress object as \"outcome_group_id\" and \"outcome_group_url\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


