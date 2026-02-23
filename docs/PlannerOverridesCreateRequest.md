# PlannerOverridesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plannable_type** | **PlannableType** | Type of the item that you are overriding in the planner (enum: announcement, assignment, discussion_topic, quiz, wiki_page, planner_note, calendar_event, assessment_request, sub_assignment) | 
**plannable_id** | **i32** | ID of the item that you are overriding in the planner | 
**marked_complete** | Option<**bool**> | If this is true, the item will show in the planner as completed | [optional]
**dismissed** | Option<**bool**> | If this is true, the item will not show in the opportunities list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


