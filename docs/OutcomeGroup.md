# OutcomeGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the outcome group | [optional]
**url** | Option<**String**> | the URL for fetching/updating the outcome group. should be treated as opaque | [optional]
**parent_outcome_group** | Option<[**models::OutcomeGroup**](OutcomeGroup.md)> |  | [optional]
**context_id** | Option<**i32**> | the context owning the outcome group. may be null for global outcome groups. omitted in the abbreviated form. | [optional]
**context_type** | Option<**String**> |  | [optional]
**title** | Option<**String**> | title of the outcome group | [optional]
**description** | Option<**String**> | description of the outcome group. omitted in the abbreviated form. | [optional]
**vendor_guid** | Option<**String**> | A custom GUID for the learning standard. | [optional]
**subgroups_url** | Option<**String**> | the URL for listing/creating subgroups under the outcome group. should be treated as opaque | [optional]
**outcomes_url** | Option<**String**> | the URL for listing/creating outcome links under the outcome group. should be treated as opaque | [optional]
**import_url** | Option<**String**> | the URL for importing another group into this outcome group. should be treated as opaque. omitted in the abbreviated form. | [optional]
**can_edit** | Option<**bool**> | whether the current user can update the outcome group | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


