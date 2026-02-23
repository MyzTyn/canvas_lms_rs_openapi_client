# EPortfolio

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The database ID of the ePortfolio | [optional]
**user_id** | Option<**i32**> | The user ID to which the ePortfolio belongs | [optional]
**name** | Option<**String**> | The name of the ePortfolio | [optional]
**public** | Option<**bool**> | Whether or not the ePortfolio is visible without authentication | [optional]
**created_at** | Option<**String**> | The creation timestamp for the ePortfolio | [optional]
**updated_at** | Option<**String**> | The timestamp of the last time any of the ePortfolio attributes changed | [optional]
**workflow_state** | Option<**String**> | The state of the ePortfolio. Either 'active' or 'deleted' | [optional]
**deleted_at** | Option<**String**> | The timestamp when the ePortfolio was deleted, or else null | [optional]
**spam_status** | Option<**String**> | A flag indicating whether the ePortfolio has been       flagged or moderated as spam. One of 'flagged_as_possible_spam',       'marked_as_safe', 'marked_as_spam', or null | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


