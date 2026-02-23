# PlannerNotesUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the planner note. | [optional]
**details** | Option<**String**> | Text of the planner note. | [optional]
**todo_date** | Option<**String**> | The date where this planner note should appear in the planner. The value should be formatted as: yyyy-mm-dd. | [optional]
**course_id** | Option<**i32**> | The ID of the course to associate with the planner note. The caller must be able to view the course in order to associate it with a planner note. Use a null or empty value to remove a planner note from a course. Note that if the planner note is linked to a learning object, its course_id cannot be changed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


