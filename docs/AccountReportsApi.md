# \AccountReportsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_reports_abort**](AccountReportsApi.md#account_reports_abort) | **PUT** /accounts/{account_id}/reports/{report}/{id}/abort | Abort a Report
[**account_reports_available_reports**](AccountReportsApi.md#account_reports_available_reports) | **GET** /accounts/{account_id}/reports | List Available Reports
[**account_reports_create**](AccountReportsApi.md#account_reports_create) | **POST** /accounts/{account_id}/reports/{report} | Start a Report
[**account_reports_destroy**](AccountReportsApi.md#account_reports_destroy) | **DELETE** /accounts/{account_id}/reports/{report}/{id} | Delete a Report
[**account_reports_index**](AccountReportsApi.md#account_reports_index) | **GET** /accounts/{account_id}/reports/{report} | Index of Reports
[**account_reports_show**](AccountReportsApi.md#account_reports_show) | **GET** /accounts/{account_id}/reports/{report}/{id} | Status of a Report



## account_reports_abort

> String account_reports_abort(account_id, id, report)
Abort a Report

Abort a report in progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**report** | **String** | Scope response to report | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_reports_available_reports

> account_reports_available_reports(account_id, include)
List Available Reports

Returns a paginated list of reports for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"description_html\"|\"params_html\"] Array of additional information to include.  \"description_html\":: an HTML description of the report, with example output \"parameters_html\":: an HTML form for the report parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_reports_create

> String account_reports_create(account_id, report, account_reports_create_request)
Start a Report

Generates a report instance for the account. Note that \"report\" in the request must match one of the available report names. To fetch a list of available report names and parameters for each report (including whether or not those parameters are required), see {api:AccountReportsController#available_reports List Available Reports}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**report** | **String** | Scope response to report | [required] |
**account_reports_create_request** | Option<[**AccountReportsCreateRequest**](AccountReportsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST \\      https://<canvas>/api/v1/accounts/1/reports/provisioning_csv \\      -H 'Authorization: Bearer <token>' \\      -H 'Content-Type: multipart/form-data' \\      -F 'parameters[users]=true' \\      -F 'parameters[courses]=true' \\      -F 'parameters[enrollments]=true' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_reports_destroy

> String account_reports_destroy(account_id, id, report)
Delete a Report

Deletes a generated report instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**report** | **String** | Scope response to report | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_reports_index

> models::Report account_reports_index(account_id, report, page, per_page)
Index of Reports

Shows all reports that have been run for the account of a specific type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**report** | **String** | Scope response to report | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Report**](Report.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_reports_show

> String account_reports_show(account_id, id, report)
Status of a Report

Returns the status of a report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**report** | **String** | Scope response to report | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

