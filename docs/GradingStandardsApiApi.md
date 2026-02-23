# \GradingStandardsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grading_standards_api_context_index_for_accounts**](GradingStandardsApiApi.md#grading_standards_api_context_index_for_accounts) | **GET** /accounts/{account_id}/grading_standards | List the grading standards available in a context.
[**grading_standards_api_context_index_for_courses**](GradingStandardsApiApi.md#grading_standards_api_context_index_for_courses) | **GET** /courses/{course_id}/grading_standards | List the grading standards available in a context.
[**grading_standards_api_context_show_for_accounts**](GradingStandardsApiApi.md#grading_standards_api_context_show_for_accounts) | **GET** /accounts/{account_id}/grading_standards/{grading_standard_id} | Get a single grading standard in a context.
[**grading_standards_api_context_show_for_courses**](GradingStandardsApiApi.md#grading_standards_api_context_show_for_courses) | **GET** /courses/{course_id}/grading_standards/{grading_standard_id} | Get a single grading standard in a context.
[**grading_standards_api_create_for_accounts**](GradingStandardsApiApi.md#grading_standards_api_create_for_accounts) | **POST** /accounts/{account_id}/grading_standards | Create a new grading standard
[**grading_standards_api_create_for_courses**](GradingStandardsApiApi.md#grading_standards_api_create_for_courses) | **POST** /courses/{course_id}/grading_standards | Create a new grading standard
[**grading_standards_api_destroy_for_accounts**](GradingStandardsApiApi.md#grading_standards_api_destroy_for_accounts) | **DELETE** /accounts/{account_id}/grading_standards/{grading_standard_id} | Delete a grading standard
[**grading_standards_api_destroy_for_courses**](GradingStandardsApiApi.md#grading_standards_api_destroy_for_courses) | **DELETE** /courses/{course_id}/grading_standards/{grading_standard_id} | Delete a grading standard
[**grading_standards_api_update_for_accounts**](GradingStandardsApiApi.md#grading_standards_api_update_for_accounts) | **PUT** /accounts/{account_id}/grading_standards/{grading_standard_id} | Update a grading standard
[**grading_standards_api_update_for_courses**](GradingStandardsApiApi.md#grading_standards_api_update_for_courses) | **PUT** /courses/{course_id}/grading_standards/{grading_standard_id} | Update a grading standard



## grading_standards_api_context_index_for_accounts

> serde_json::Value grading_standards_api_context_index_for_accounts(account_id)
List the grading standards available in a context.

Returns the paginated list of grading standards for the given context that are visible to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_context_index_for_courses

> serde_json::Value grading_standards_api_context_index_for_courses(course_id)
List the grading standards available in a context.

Returns the paginated list of grading standards for the given context that are visible to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_context_show_for_accounts

> String grading_standards_api_context_show_for_accounts(account_id, grading_standard_id)
Get a single grading standard in a context.

Returns a grading standard for the given context that is visible to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_context_show_for_courses

> String grading_standards_api_context_show_for_courses(course_id, grading_standard_id)
Get a single grading standard in a context.

Returns a grading standard for the given context that is visible to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_create_for_accounts

> String grading_standards_api_create_for_accounts(account_id, grading_standards_api_create_for_accounts_request)
Create a new grading standard

Create a new grading standard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**grading_standards_api_create_for_accounts_request** | Option<[**GradingStandardsApiCreateForAccountsRequest**](GradingStandardsApiCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_create_for_courses

> String grading_standards_api_create_for_courses(course_id, grading_standards_api_create_for_accounts_request)
Create a new grading standard

Create a new grading standard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**grading_standards_api_create_for_accounts_request** | Option<[**GradingStandardsApiCreateForAccountsRequest**](GradingStandardsApiCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_destroy_for_accounts

> String grading_standards_api_destroy_for_accounts(account_id, grading_standard_id)
Delete a grading standard

Deletes the grading standard with the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_destroy_for_courses

> String grading_standards_api_destroy_for_courses(course_id, grading_standard_id)
Delete a grading standard

Deletes the grading standard with the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_update_for_accounts

> String grading_standards_api_update_for_accounts(account_id, grading_standard_id, grading_standards_api_update_for_courses_request)
Update a grading standard

Updates the grading standard with the given id  If the grading standard has been used for grading, only the title can be updated. The data, points_based, and scaling_factor cannot be modified once the grading standard has been used to grade assignments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |
**grading_standards_api_update_for_courses_request** | Option<[**GradingStandardsApiUpdateForCoursesRequest**](GradingStandardsApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_standards_api_update_for_courses

> String grading_standards_api_update_for_courses(course_id, grading_standard_id, grading_standards_api_update_for_courses_request)
Update a grading standard

Updates the grading standard with the given id  If the grading standard has been used for grading, only the title can be updated. The data, points_based, and scaling_factor cannot be modified once the grading standard has been used to grade assignments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**grading_standard_id** | **String** | Scope response to grading_standard_id | [required] |
**grading_standards_api_update_for_courses_request** | Option<[**GradingStandardsApiUpdateForCoursesRequest**](GradingStandardsApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

