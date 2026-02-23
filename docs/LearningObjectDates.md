# LearningObjectDates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the learning object (not present for checkpoints) | [optional]
**due_at** | Option<**String**> | the due date for the learning object. returns null if not present or applicable. never applicable for ungraded discussions, pages, and files | [optional]
**lock_at** | Option<**String**> | the lock date (learning object is locked after this date). returns null if not present | [optional]
**reply_to_topic_due_at** | Option<**String**> | the reply_to_topic sub_assignment due_date. returns null if not present | [optional]
**required_replies_due_at** | Option<**String**> | the reply_to_entry sub_assignment due_date. returns null if not present | [optional]
**unlock_at** | Option<**String**> | the unlock date (learning object is unlocked after this date). returns null if not present | [optional]
**only_visible_to_overrides** | Option<**bool**> | whether the learning object is only visible to overrides | [optional]
**graded** | Option<**bool**> | whether the learning object is graded (and thus has a due date) | [optional]
**blueprint_date_locks** | Option<**Vec<String>**> | [exclusive to blueprint child content only] list of lock types | [optional]
**visible_to_everyone** | Option<**bool**> | whether the learning object is visible to everyone | [optional]
**overrides** | Option<[**Vec<models::AssignmentOverride>**](AssignmentOverride.md)> | paginated list of AssignmentOverride objects | [optional]
**checkpoints** | Option<[**Vec<models::LearningObjectDates>**](LearningObjectDates.md)> | list of Checkpoint objects, only present if a learning object has subAssignments | [optional]
**tag** | Option<**String**> | the tag identifying the type of checkpoint (only present for checkpoints) | [optional]
**peer_review_sub_assignment** | Option<**serde_json::Value**> | peer review sub assignment details, only present when include_peer_review=true is specified, assignment has peer reviews enabled, and peer_review_allocation_and_grading feature flag is enabled | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


