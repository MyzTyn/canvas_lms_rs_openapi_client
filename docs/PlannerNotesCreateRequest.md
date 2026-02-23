# PlannerNotesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the planner note. | [optional]
**details** | Option<**String**> | Text of the planner note. | [optional]
**todo_date** | Option<**String**> | The date where this planner note should appear in the planner. The value should be formatted as: yyyy-mm-dd. | [optional]
**course_id** | Option<**i32**> | The ID of the course to associate with the planner note. The caller must be able to view the course in order to associate it with a planner note. | [optional]
**linked_object_type** | Option<**String**> | The type of a learning object to link to this planner note. Must be used in conjunction wtih linked_object_id and course_id. Valid linked_object_type values are: 'announcement', 'assignment', 'discussion_topic', 'wiki_page', 'quiz' | [optional]
**linked_object_id** | Option<**i32**> | The id of a learning object to link to this planner note. Must be used in conjunction with linked_object_type and course_id. The object must be in the same course as specified by course_id. If the title argument is not provided, the planner note will use the learning object's title as its title. Only one planner note may be linked to a specific learning object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


