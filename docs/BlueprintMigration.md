# BlueprintMigration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The ID of the migration. | [optional]
**template_id** | Option<**i64**> | The ID of the template the migration belongs to. Only present when querying a blueprint course. | [optional]
**subscription_id** | Option<**i64**> | The ID of the associated course's blueprint subscription. Only present when querying a course associated with a blueprint. | [optional]
**user_id** | Option<**i64**> | The ID of the user who queued the migration. | [optional]
**workflow_state** | Option<**String**> | Current state of the content migration: queued, exporting, imports_queued, completed, exports_failed, imports_failed | [optional]
**created_at** | Option<**String**> | Time when the migration was queued | [optional]
**exports_started_at** | Option<**String**> | Time when the exports begun | [optional]
**imports_queued_at** | Option<**String**> | Time when the exports were completed and imports were queued | [optional]
**imports_completed_at** | Option<**String**> | Time when the imports were completed | [optional]
**comment** | Option<**String**> | User-specified comment describing changes made in this operation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


