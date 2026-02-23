# ModuleAssignmentOverridesBulkUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**overrides** | **Vec<String>** | [Required, Array] List of overrides to apply to the module. Overrides that already exist should include an ID and will be updated if needed. New overrides will be created for overrides in the list without an ID. Overrides not included in the list will be deleted. Providing an empty list will delete all of the module's overrides. Keys for each override object can include: 'id', 'title', 'student_ids', and 'course_section_id'. 'group_id' is accepted if the Differentiation Tags account setting is enabled. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


