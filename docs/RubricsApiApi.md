# \RubricsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rubrics_api_index_for_accounts**](RubricsApiApi.md#rubrics_api_index_for_accounts) | **GET** /accounts/{account_id}/rubrics | List rubrics
[**rubrics_api_index_for_courses**](RubricsApiApi.md#rubrics_api_index_for_courses) | **GET** /courses/{course_id}/rubrics | List rubrics
[**rubrics_api_show_for_accounts**](RubricsApiApi.md#rubrics_api_show_for_accounts) | **GET** /accounts/{account_id}/rubrics/{id} | Get a single rubric
[**rubrics_api_show_for_courses**](RubricsApiApi.md#rubrics_api_show_for_courses) | **GET** /courses/{course_id}/rubrics/{id} | Get a single rubric
[**rubrics_api_upload_for_accounts**](RubricsApiApi.md#rubrics_api_upload_for_accounts) | **POST** /accounts/{account_id}/rubrics/upload | Creates a rubric using a CSV file
[**rubrics_api_upload_for_courses**](RubricsApiApi.md#rubrics_api_upload_for_courses) | **POST** /courses/{course_id}/rubrics/upload | Creates a rubric using a CSV file
[**rubrics_api_upload_status_for_accounts**](RubricsApiApi.md#rubrics_api_upload_status_for_accounts) | **GET** /accounts/{account_id}/rubrics/upload/{id} | Get the status of a rubric import
[**rubrics_api_upload_status_for_courses**](RubricsApiApi.md#rubrics_api_upload_status_for_courses) | **GET** /courses/{course_id}/rubrics/upload/{id} | Get the status of a rubric import
[**rubrics_api_upload_template**](RubricsApiApi.md#rubrics_api_upload_template) | **GET** /rubrics/upload_template | Templated file for importing a rubric
[**rubrics_api_used_locations_for_accounts**](RubricsApiApi.md#rubrics_api_used_locations_for_accounts) | **GET** /accounts/{account_id}/rubrics/{id}/used_locations | Get the courses and assignments for a rubric
[**rubrics_api_used_locations_for_courses**](RubricsApiApi.md#rubrics_api_used_locations_for_courses) | **GET** /courses/{course_id}/rubrics/{id}/used_locations | Get the courses and assignments for a rubric



## rubrics_api_index_for_accounts

> rubrics_api_index_for_accounts(account_id, page, per_page)
List rubrics

Returns the paginated list of active rubrics for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_index_for_courses

> rubrics_api_index_for_courses(course_id, page, per_page)
List rubrics

Returns the paginated list of active rubrics for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_show_for_accounts

> String rubrics_api_show_for_accounts(account_id, id, include, style)
Get a single rubric

Returns the rubric with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"assessments\"|\"graded_assessments\"|\"peer_assessments\"|\"associations\"|\"assignment_associations\"|\"course_associations\"|\"account_associations\"] Related records to include in the response. |  |
**style** | Option<**String**> | Applicable only if assessments are being returned. If included, returns either all criteria data associated with the assessment, or just the comments. If not included, both data and comments are omitted. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_show_for_courses

> String rubrics_api_show_for_courses(course_id, id, include, style)
Get a single rubric

Returns the rubric with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"assessments\"|\"graded_assessments\"|\"peer_assessments\"|\"associations\"|\"assignment_associations\"|\"course_associations\"|\"account_associations\"] Related records to include in the response. |  |
**style** | Option<**String**> | Applicable only if assessments are being returned. If included, returns either all criteria data associated with the assessment, or just the comments. If not included, both data and comments are omitted. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_upload_for_accounts

> String rubrics_api_upload_for_accounts(account_id)
Creates a rubric using a CSV file

Returns the rubric import object that was created

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_upload_for_courses

> String rubrics_api_upload_for_courses(course_id)
Creates a rubric using a CSV file

Returns the rubric import object that was created

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_upload_status_for_accounts

> String rubrics_api_upload_status_for_accounts(account_id, id)
Get the status of a rubric import

Can return the latest rubric import for an account or course, or a specific import by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_upload_status_for_courses

> String rubrics_api_upload_status_for_courses(course_id, id)
Get the status of a rubric import

Can return the latest rubric import for an account or course, or a specific import by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_upload_template

> String rubrics_api_upload_template()
Templated file for importing a rubric

Returns a CSV template file that can be used to import rubrics into Canvas.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_used_locations_for_accounts

> String rubrics_api_used_locations_for_accounts(account_id, id, page, per_page)
Get the courses and assignments for a rubric

Returns the courses and assignments where a rubric is being used

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_api_used_locations_for_courses

> String rubrics_api_used_locations_for_courses(course_id, id, page, per_page)
Get the courses and assignments for a rubric

Returns the courses and assignments where a rubric is being used

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

