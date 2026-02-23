# \SisImportErrorsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sis_import_errors_api_index_for_accounts**](SisImportErrorsApiApi.md#sis_import_errors_api_index_for_accounts) | **GET** /accounts/{account_id}/sis_imports/{id}/errors | Get SIS import error list
[**sis_import_errors_api_index_for_accounts2**](SisImportErrorsApiApi.md#sis_import_errors_api_index_for_accounts2) | **GET** /accounts/{account_id}/sis_import_errors | Get SIS import error list



## sis_import_errors_api_index_for_accounts

> serde_json::Value sis_import_errors_api_index_for_accounts(account_id, id, failure, page, per_page)
Get SIS import error list

Returns the list of SIS import errors for an account or a SIS import. Import errors are only stored for 30 days.  Example:   curl 'https://<canvas>/api/v1/accounts/<account_id>/sis_imports/<id>/sis_import_errors' \\     -H \"Authorization: Bearer <token>\"  Example:   curl 'https://<canvas>/api/v1/accounts/<account_id>/sis_import_errors' \\     -H \"Authorization: Bearer <token>\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**failure** | Option<**bool**> | If set, only shows errors on a sis import that would cause a failure. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_import_errors_api_index_for_accounts2

> serde_json::Value sis_import_errors_api_index_for_accounts2(account_id, failure, page, per_page)
Get SIS import error list

Returns the list of SIS import errors for an account or a SIS import. Import errors are only stored for 30 days.  Example:   curl 'https://<canvas>/api/v1/accounts/<account_id>/sis_imports/<id>/sis_import_errors' \\     -H \"Authorization: Bearer <token>\"  Example:   curl 'https://<canvas>/api/v1/accounts/<account_id>/sis_import_errors' \\     -H \"Authorization: Bearer <token>\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**failure** | Option<**bool**> | If set, only shows errors on a sis import that would cause a failure. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

