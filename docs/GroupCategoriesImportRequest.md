# GroupCategoriesImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachment** | **String** | There are two ways to post group category import data - either via a multipart/form-data form-field-style attachment, or via a non-multipart raw post request.  'attachment' is required for multipart/form-data style posts. Assumed to be outcome data from a file upload form field named 'attachment'.  Examples:   curl -F attachment=@<filename> -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/group_categories/<category_id>/import'  If you decide to do a raw post, you can skip the 'attachment' argument, but you will then be required to provide a suitable Content-Type header. You are encouraged to also provide the 'extension' argument.  Examples:   curl -H 'Content-Type: text/csv' --data-binary @<filename>.csv \\       -H \"Authorization: Bearer <token>\" \\       'https://<canvas>/api/v1/group_categories/<category_id>/import' | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


