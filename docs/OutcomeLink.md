# OutcomeLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | the URL for fetching/updating the outcome link. should be treated as opaque | [optional]
**context_id** | Option<**i32**> | the context owning the outcome link. will match the context owning the outcome group containing the outcome link; included for convenience. may be null for links in global outcome groups. | [optional]
**context_type** | Option<**String**> |  | [optional]
**outcome_group** | Option<[**models::OutcomeGroup**](OutcomeGroup.md)> |  | [optional]
**outcome** | Option<[**models::Outcome**](Outcome.md)> |  | [optional]
**assessed** | Option<**bool**> | whether this outcome has been used to assess a student in the context of this outcome link.  In other words, this will be set to true if the context is a course, and a student has been assessed with this outcome in that course. | [optional]
**can_unlink** | Option<**bool**> | whether this outcome link is manageable and is not the last link to an aligned outcome | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


