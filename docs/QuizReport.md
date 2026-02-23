# QuizReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the quiz report | [optional]
**quiz_id** | Option<**i32**> | the ID of the quiz | [optional]
**report_type** | Option<**ReportType**> | which type of report this is possible values: 'student_analysis', 'item_analysis' (enum: student_analysis, item_analysis) | [optional]
**readable_type** | Option<**String**> | a human-readable (and localized) version of the report_type | [optional]
**includes_all_versions** | Option<**bool**> | boolean indicating whether the report represents all submissions or only the most recent ones for each student | [optional]
**anonymous** | Option<**bool**> | boolean indicating whether the report is for an anonymous survey. if true, no student names will be included in the csv | [optional]
**generatable** | Option<**bool**> | boolean indicating whether the report can be generated, which is true unless the quiz is a survey one | [optional]
**created_at** | Option<**String**> | when the report was created | [optional]
**updated_at** | Option<**String**> | when the report was last updated | [optional]
**url** | Option<**String**> | the API endpoint for this report | [optional]
**file** | Option<[**models::File**](File.md)> |  | [optional]
**progress_url** | Option<**String**> | if the report has not yet finished generating, a URL where information about its progress can be retrieved. refer to the Progress API for more information (Note: not available in JSON-API format) | [optional]
**progress** | Option<[**models::Progress**](Progress.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


