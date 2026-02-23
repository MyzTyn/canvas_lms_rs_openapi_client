# \SisImportsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sis_imports_api_abort**](SisImportsApiApi.md#sis_imports_api_abort) | **PUT** /accounts/{account_id}/sis_imports/{id}/abort | Abort SIS import
[**sis_imports_api_abort_all_pending**](SisImportsApiApi.md#sis_imports_api_abort_all_pending) | **PUT** /accounts/{account_id}/sis_imports/abort_all_pending | Abort all pending SIS imports
[**sis_imports_api_create**](SisImportsApiApi.md#sis_imports_api_create) | **POST** /accounts/{account_id}/sis_imports | Import SIS data
[**sis_imports_api_importing**](SisImportsApiApi.md#sis_imports_api_importing) | **GET** /accounts/{account_id}/sis_imports/importing | Get the current importing SIS import
[**sis_imports_api_index**](SisImportsApiApi.md#sis_imports_api_index) | **GET** /accounts/{account_id}/sis_imports | Get SIS import list
[**sis_imports_api_restore_states**](SisImportsApiApi.md#sis_imports_api_restore_states) | **PUT** /accounts/{account_id}/sis_imports/{id}/restore_states | Restore workflow_states of SIS imported items
[**sis_imports_api_show**](SisImportsApiApi.md#sis_imports_api_show) | **GET** /accounts/{account_id}/sis_imports/{id} | Get SIS import status



## sis_imports_api_abort

> String sis_imports_api_abort(account_id, id)
Abort SIS import

Abort a SIS import that has not completed.  Aborting a sis batch that is running can take some time for every process to see the abort event. Subsequent sis batches begin to process 10 minutes after the abort to allow each process to clean up properly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_abort_all_pending

> String sis_imports_api_abort_all_pending(account_id)
Abort all pending SIS imports

Abort already created but not processed or processing SIS imports.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_create

> String sis_imports_api_create(account_id, sis_imports_api_create_request)
Import SIS data

Import SIS data into Canvas. Must be on a root account with SIS imports enabled.  For more information on the format that's expected here, please see the \"SIS CSV\" section in the API docs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**sis_imports_api_create_request** | Option<[**SisImportsApiCreateRequest**](SisImportsApiCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_importing

> String sis_imports_api_importing(account_id)
Get the current importing SIS import

Returns the SIS imports that are currently processing for an account. If no imports are running, will return an empty array.  Example:   curl https://<canvas>/api/v1/accounts/<account_id>/sis_imports/importing \\     -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_index

> models::SisImport sis_imports_api_index(account_id, created_since, created_before, workflow_state, page, per_page)
Get SIS import list

Returns the list of SIS imports for an account  Example:   curl https://<canvas>/api/v1/accounts/<account_id>/sis_imports \\     -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**created_since** | Option<**String**> | If set, only shows imports created after the specified date (use ISO8601 format) |  |
**created_before** | Option<**String**> | If set, only shows imports created before the specified date (use ISO8601 format) |  |
**workflow_state** | Option<[**Vec<String>**](String.md)> | [String, \"initializing\"|\"created\"|\"importing\"|\"cleanup_batch\"|\"imported\"|\"imported_with_messages\"|\"aborted\"|\"failed\"|\"failed_with_messages\"|\"restoring\"|\"partially_restored\"|\"restored\"] If set, only returns imports that are in the given state. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::SisImport**](SisImport.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_restore_states

> String sis_imports_api_restore_states(account_id, id, sis_imports_api_restore_states_request)
Restore workflow_states of SIS imported items

This will restore the the workflow_state for all the items that changed their workflow_state during the import being restored. This will restore states for items imported with the following importers: accounts.csv terms.csv courses.csv sections.csv group_categories.csv groups.csv users.csv admins.csv This also restores states for other items that changed during the import. An example would be if an enrollment was deleted from a sis import and the group_membership was also deleted as a result of the enrollment deletion, both items would be restored when the sis batch is restored.  Restore data is retained for 30 days post-import. This endpoint is unavailable after that time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**sis_imports_api_restore_states_request** | Option<[**SisImportsApiRestoreStatesRequest**](SisImportsApiRestoreStatesRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/<account_id>/sis_imports/<sis_import_id>/restore_states \\   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_imports_api_show

> String sis_imports_api_show(account_id, id)
Get SIS import status

Get the status of an already created SIS import.    Examples:     curl https://<canvas>/api/v1/accounts/<account_id>/sis_imports/<sis_import_id> \\         -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

