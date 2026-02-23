# ContentMigrationsCreateForAccountsRequestSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_url** | Option<**String**> | [string] A URL to download the file from. Must not require authentication. | [optional]
**content_export_id** | Option<**String**> | [String] The id of a ContentExport to import. This allows you to import content previously exported from Canvas without needing to download and re-upload it. | [optional]
**source_course_id** | **String** | [String] The course to copy from for a course copy migration. (required if doing course copy) | 
**folder_id** | Option<**String**> | [String] The folder to unzip the .zip file into for a zip_file_import. | [optional]
**overwrite_quizzes** | Option<**bool**> | [Boolean] Whether to overwrite quizzes with the same identifiers between content packages. | [optional]
**question_bank_id** | Option<**i32**> | [Integer] The existing question bank ID to import questions into if not specified in the content package. | [optional]
**question_bank_name** | Option<**String**> | [String] The question bank to import questions into if not specified in the content package, if both bank id and name are set, id will take precedence. | [optional]
**insert_into_module_id** | Option<**i32**> | [Integer] The id of a module in the target course. This will add all imported items (that can be added to a module) to the given module. | [optional]
**insert_into_module_type** | Option<**InsertIntoModuleType**> | [String,\"assignment\"|\"discussion_topic\"|\"file\"|\"page\"|\"quiz\"] If provided (and +insert_into_module_id+ is supplied), only add objects of the specified type to the module. (enum: assignment, discussion_topic, file, page, quiz) | [optional]
**insert_into_module_position** | Option<**i32**> | [Integer] The (1-based) position to insert the imported items into the course (if +insert_into_module_id+ is supplied). If this parameter is omitted, items will be added to the end of the module. | [optional]
**move_to_assignment_group_id** | Option<**i32**> | [Integer] The id of an assignment group in the target course. If provided, all imported assignments will be moved to the given assignment group. | [optional]
**importer_skips** | Option<**ImporterSkips**> | [Optional,Array,\"all_course_settings\"|\"visibility_settings\"] Set of importers to skip, even if otherwise selected by migration settings. (enum: all_course_settings, visibility_settings) | [optional]
**import_blueprint_settings** | Option<**bool**> | [Boolean] Import the \"use as blueprint course\" setting as well as the list of locked items from the source course or package. The destination course must not be associated with an existing blueprint course and cannot have any student or observer enrollments. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


