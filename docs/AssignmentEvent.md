# AssignmentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A synthetic ID for the assignment | [optional]
**title** | Option<**String**> | The title of the assignment | [optional]
**start_at** | Option<**String**> | The due_at timestamp of the assignment | [optional]
**end_at** | Option<**String**> | The due_at timestamp of the assignment | [optional]
**description** | Option<**String**> | The HTML description of the assignment | [optional]
**context_code** | Option<**String**> | the context code of the (course) calendar this assignment belongs to | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the assignment ('published' or 'deleted') (enum: published, deleted) | [optional]
**url** | Option<**String**> | URL for this assignment (note that updating/deleting should be done via the Assignments API) | [optional]
**html_url** | Option<**String**> | URL for a user to view this assignment | [optional]
**all_day_date** | Option<**String**> | The due date of this assignment | [optional]
**all_day** | Option<**bool**> | Boolean indicating whether this is an all-day event (e.g. assignment due at midnight) | [optional]
**created_at** | Option<**String**> | When the assignment was created | [optional]
**updated_at** | Option<**String**> | When the assignment was last updated | [optional]
**assignment** | Option<[**models::Assignment**](Assignment.md)> |  | [optional]
**assignment_overrides** | Option<[**models::AssignmentOverride**](AssignmentOverride.md)> |  | [optional]
**important_dates** | Option<**bool**> | Boolean indicating whether this has important dates. | [optional]
**rrule** | Option<**String**> | An iCalendar RRULE for defining how events in a recurring event series repeat. | [optional]
**series_head** | Option<**bool**> | Trueif this is the first event in the series of recurring events. | [optional]
**series_natural_language** | Option<**String**> | A natural language expression of how events occur in the series. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


