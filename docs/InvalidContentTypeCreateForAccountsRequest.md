# InvalidContentTypeCreateForAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**import_type** | Option<**String**> | Choose the data format for reading outcome data. With a standard Canvas install, this option can only be 'instructure_csv', and if unprovided, will be assumed to be so. Can be part of the query string. | [optional]
**attachment** | **String** | There are two ways to post outcome import data - either via a multipart/form-data form-field-style attachment, or via a non-multipart raw post request.  'attachment' is required for multipart/form-data style posts. Assumed to be outcome data from a file upload form field named 'attachment'.  Examples:   curl -F attachment=@<filename> -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports?import_type=instructure_csv'   curl -F attachment=@<filename> -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/courses/<course_id>/outcome_imports?import_type=instructure_csv'  If you decide to do a raw post, you can skip the 'attachment' argument, but you will then be required to provide a suitable Content-Type header. You are encouraged to also provide the 'extension' argument.  Examples:   curl -H 'Content-Type: text/csv' --data-binary @<filename>.csv \\       -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports?import_type=instructure_csv'    curl -H 'Content-Type: text/csv' --data-binary @<filename>.csv \\       -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/courses/<course_id>/outcome_imports?import_type=instructure_csv' | 
**extension** | Option<**String**> | Recommended for raw post request style imports. This field will be used to distinguish between csv and other file format extensions that would usually be provided with the filename in the multipart post request scenario. If not provided, this value will be inferred from the Content-Type, falling back to csv-file format if all else fails. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


