# \InvalidContentTypeApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invalid_content_type_create_for_accounts**](InvalidContentTypeApi.md#invalid_content_type_create_for_accounts) | **POST** /accounts/{account_id}/outcome_imports(/group/{learning_outcome_group_id}) | Import Outcomes
[**invalid_content_type_create_for_courses**](InvalidContentTypeApi.md#invalid_content_type_create_for_courses) | **POST** /courses/{course_id}/outcome_imports(/group/{learning_outcome_group_id}) | Import Outcomes
[**invalid_content_type_created_group_ids_for_accounts**](InvalidContentTypeApi.md#invalid_content_type_created_group_ids_for_accounts) | **GET** /accounts/{account_id}/outcome_imports/{id}/created_group_ids | Get IDs of outcome groups created after successful import
[**invalid_content_type_created_group_ids_for_courses**](InvalidContentTypeApi.md#invalid_content_type_created_group_ids_for_courses) | **GET** /courses/{course_id}/outcome_imports/{id}/created_group_ids | Get IDs of outcome groups created after successful import
[**invalid_content_type_show_for_accounts**](InvalidContentTypeApi.md#invalid_content_type_show_for_accounts) | **GET** /accounts/{account_id}/outcome_imports/{id} | Get Outcome import status
[**invalid_content_type_show_for_courses**](InvalidContentTypeApi.md#invalid_content_type_show_for_courses) | **GET** /courses/{course_id}/outcome_imports/{id} | Get Outcome import status



## invalid_content_type_create_for_accounts

> String invalid_content_type_create_for_accounts(account_id, learning_outcome_group_id, invalid_content_type_create_for_accounts_request)
Import Outcomes

Import outcomes into Canvas.  For more information on the format that's expected here, please see the \"Outcomes CSV\" section in the API docs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**learning_outcome_group_id** | **String** | Scope response to learning_outcome_group_id | [required] |
**invalid_content_type_create_for_accounts_request** | Option<[**InvalidContentTypeCreateForAccountsRequest**](InvalidContentTypeCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalid_content_type_create_for_courses

> String invalid_content_type_create_for_courses(course_id, learning_outcome_group_id, invalid_content_type_create_for_accounts_request)
Import Outcomes

Import outcomes into Canvas.  For more information on the format that's expected here, please see the \"Outcomes CSV\" section in the API docs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**learning_outcome_group_id** | **String** | Scope response to learning_outcome_group_id | [required] |
**invalid_content_type_create_for_accounts_request** | Option<[**InvalidContentTypeCreateForAccountsRequest**](InvalidContentTypeCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalid_content_type_created_group_ids_for_accounts

> String invalid_content_type_created_group_ids_for_accounts(account_id, id)
Get IDs of outcome groups created after successful import

Get the IDs of the outcome groups created after a successful import. Pass 'latest' for the outcome import id for the latest import.    Examples:     curl 'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports/outcomes_group_ids/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"     curl 'https://<canvas>/api/v1/courses/<course_id>/outcome_imports/outcome_group_ids/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"

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


## invalid_content_type_created_group_ids_for_courses

> String invalid_content_type_created_group_ids_for_courses(course_id, id)
Get IDs of outcome groups created after successful import

Get the IDs of the outcome groups created after a successful import. Pass 'latest' for the outcome import id for the latest import.    Examples:     curl 'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports/outcomes_group_ids/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"     curl 'https://<canvas>/api/v1/courses/<course_id>/outcome_imports/outcome_group_ids/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invalid_content_type_show_for_accounts

> String invalid_content_type_show_for_accounts(account_id, id)
Get Outcome import status

Get the status of an already created Outcome import. Pass 'latest' for the outcome import id for the latest import.    Examples:     curl 'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"     curl 'https://<canvas>/api/v1/courses/<course_id>/outcome_imports/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"

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


## invalid_content_type_show_for_courses

> String invalid_content_type_show_for_courses(course_id, id)
Get Outcome import status

Get the status of an already created Outcome import. Pass 'latest' for the outcome import id for the latest import.    Examples:     curl 'https://<canvas>/api/v1/accounts/<account_id>/outcome_imports/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"     curl 'https://<canvas>/api/v1/courses/<course_id>/outcome_imports/<outcome_import_id>' \\         -H \"Authorization: Bearer <token>\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

