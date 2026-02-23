# LearningObjectDatesUpdateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**due_at** | Option<**String**> | The learning object's due date. Not applicable for ungraded discussions, pages, and files. | [optional]
**unlock_at** | Option<**String**> | The learning object's unlock date. Must be before the due date if there is one. | [optional]
**lock_at** | Option<**String**> | The learning object's lock date. Must be after the due date if there is one. | [optional]
**only_visible_to_overrides** | Option<**bool**> | Whether the learning object is only assigned to students who are targeted by an override. | [optional]
**assignment_overrides** | Option<**Vec<String>**> | [Array] List of overrides to apply to the learning object. Overrides that already exist should include an ID and will be updated if needed. New overrides will be created for overrides in the list without an ID. Overrides not included in the list will be deleted. Providing an empty list will delete all of the object's overrides. Keys for each override object can include: 'id', 'title', 'due_at', 'unlock_at', 'lock_at', 'student_ids', and 'course_section_id', 'course_id', 'noop_id', and 'unassign_item'. | [optional]
**peer_review** | Option<[**models::LearningObjectDatesUpdateForCoursesRequestPeerReview**](LearningObjectDatesUpdateForCoursesRequestPeerReview.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


