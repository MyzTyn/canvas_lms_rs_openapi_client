# ContentExportsApiCreateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**export_type** | **ExportType** | \"common_cartridge\":: Export the contents of the course in the Common Cartridge (.imscc) format \"qti\":: Export quizzes from a course in the QTI format \"zip\":: Export files from a course, group, or user in a zip file (enum: common_cartridge, qti, zip) | 
**skip_notifications** | Option<**bool**> | Don't send the notifications about the export to the user. Default: false | [optional]
**select** | Option<**Select**> | The select parameter allows exporting specific data. The keys are object types like 'files', 'folders', 'pages', etc. The value for each key is a list of object ids. An id can be an integer or a string.  Multiple object types can be selected in the same call. However, not all object types are valid for every export_type. Common Cartridge supports all object types. Zip and QTI only support the object types as described below.  \"folders\":: Also supported for zip export_type. \"files\":: Also supported for zip export_type. \"quizzes\":: Also supported for qti export_type. (enum: folders, files, attachments, quizzes, assignments, announcements, calendar_events, discussion_topics, modules, module_items, pages, rubrics) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


