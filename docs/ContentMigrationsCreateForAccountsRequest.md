# ContentMigrationsCreateForAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**migration_type** | **String** | The type of the migration. Use the {api:ContentMigrationsController#available_migrators Migrator} endpoint to see all available migrators. Default allowed values: canvas_cartridge_importer, common_cartridge_importer, course_copy_importer, zip_file_importer, qti_converter, moodle_converter | 
**selective_import** | Option<**bool**> | If set, perform a selective import instead of importing all content. The migration will identify the contents of the package and then stop in the +waiting_for_select+ workflow state. At this point, use the {api:ContentMigrationsController#content_list List items endpoint} to enumerate the contents of the package, identifying the copy parameters for the desired content. Then call the {api:ContentMigrationsController#update Update endpoint} and provide these copy parameters to start the import. | [optional]
**select** | Option<**Select**> | For +course_copy_importer+ migrations, this parameter allows you to select the objects to copy without using the +selective_import+ argument and +waiting_for_select+ state as is required for uploaded imports (though that workflow is also supported for course copy migrations). The keys are object types like 'files', 'folders', 'pages', etc. The value for each key is a list of object ids. An id can be an integer or a string. Multiple object types can be selected in the same call. (enum: folders, files, attachments, quizzes, assignments, announcements, calendar_events, discussion_topics, modules, module_items, pages, rubrics) | [optional]
**pre_attachment** | [**models::ContentMigrationsCreateForAccountsRequestPreAttachment**](ContentMigrationsCreateForAccountsRequestPreAttachment.md) |  | 
**settings** | [**models::ContentMigrationsCreateForAccountsRequestSettings**](ContentMigrationsCreateForAccountsRequestSettings.md) |  | 
**date_shift_options** | Option<[**models::ContentMigrationsCreateForAccountsRequestDateShiftOptions**](ContentMigrationsCreateForAccountsRequestDateShiftOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


