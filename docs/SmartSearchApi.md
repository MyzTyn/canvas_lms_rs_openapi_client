# \SmartSearchApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**smart_search_search**](SmartSearchApi.md#smart_search_search) | **GET** /courses/{course_id}/smartsearch | Search course content



## smart_search_search

> models::SearchResult smart_search_search(course_id, q, filter, include, page, per_page)
Search course content

Find course content using a meaning-based search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**q** | **String** | The search query | [required] |
**filter** | Option<[**Vec<String>**](String.md)> | [String, optional] Types of objects to search. By default, all supported types are searched. Supported types include +pages+, +assignments+, +announcements+, and +discussion_topics+. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"status\"|\"modules\"] Optional information to include with each search result: modules:: An array of module objects that the search result belongs to. status:: The published status for all results and the due_date for all assignments. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

