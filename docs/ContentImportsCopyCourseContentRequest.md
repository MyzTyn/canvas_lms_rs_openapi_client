# ContentImportsCopyCourseContentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_course** | Option<**String**> | ID or SIS-ID of the course to copy the content from | [optional]
**except** | Option<**Vec<String>**> | [String, \"course_settings\"|\"assignments\"|\"external_tools\"|\"files\"|\"topics\"|\"calendar_events\"|\"quizzes\"|\"wiki_pages\"|\"modules\"|\"outcomes\"] A list of the course content types to exclude, all areas not listed will be copied. | [optional]
**only** | Option<**Vec<String>**> | [String, \"course_settings\"|\"assignments\"|\"external_tools\"|\"files\"|\"topics\"|\"calendar_events\"|\"quizzes\"|\"wiki_pages\"|\"modules\"|\"outcomes\"] A list of the course content types to copy, all areas not listed will not be copied. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


