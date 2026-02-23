# \PageViewsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**page_views_batch_query**](PageViewsApi.md#page_views_batch_query) | **POST** /users/page_views/query | BETA - Initiate batch page views query
[**page_views_batch_query_results**](PageViewsApi.md#page_views_batch_query_results) | **GET** /users/page_views/query/{query_id}/results | BETA - Get batch query results
[**page_views_index**](PageViewsApi.md#page_views_index) | **GET** /users/{user_id}/page_views | List user page views
[**page_views_poll_batch_query**](PageViewsApi.md#page_views_poll_batch_query) | **GET** /users/page_views/query/{query_id} | BETA - Poll batch query status
[**page_views_poll_query**](PageViewsApi.md#page_views_poll_query) | **GET** /users/{user_id}/page_views/query/{query_id} | BETA - Poll query status
[**page_views_query**](PageViewsApi.md#page_views_query) | **POST** /users/{user_id}/page_views/query | BETA - Initiate page views query
[**page_views_query_results**](PageViewsApi.md#page_views_query_results) | **GET** /users/{user_id}/page_views/query/{query_id}/results | BETA - Get query results



## page_views_batch_query

> String page_views_batch_query(page_views_batch_query_request)
BETA - Initiate batch page views query

Initiates an asynchronous query for page views data across multiple users. This method enqueues a background job to process the batch page views query and returns a polling URL that can be used to check the query status and retrieve results when ready.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_views_batch_query_request** | Option<[**PageViewsBatchQueryRequest**](PageViewsBatchQueryRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/page_views/query \\   -X POST \\   -H 'Authorization: Bearer <token>' \\   -H 'Content-Type: application/json' \\   -d '{     \"user_ids\": [123, 456, 789],     \"start_date\": \"2023-01-01\",     \"end_date\": \"2023-02-01\",     \"results_format\": \"csv\"   }' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_batch_query_results

> String page_views_batch_query_results(query_id)
BETA - Get batch query results

Retrieves the results of a completed batch page views query. Returns the data in the format specified when the query was initiated (CSV or JSON). The response may be compressed with gzip encoding.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** | The UUID of the completed query to retrieve results for | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_index

> models::PageView page_views_index(user_id, start_time, end_time, page, per_page)
List user page views

Return a paginated list of the user's page view history in json format, similar to the available CSV download. Page views are returned in descending order, newest to oldest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want page views. |  |
**end_time** | Option<**String**> | The end of the time range from which you want page views. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::PageView**](PageView.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_poll_batch_query

> String page_views_poll_batch_query(query_id)
BETA - Poll batch query status

Checks the status of a previously initiated batch page views query. Returns the current processing status and provides a result URL when the query is complete.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** | The UUID of the query to check status for | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_poll_query

> String page_views_poll_query(query_id, user_id)
BETA - Poll query status

Checks the status of a previously initiated page views query. Returns the current processing status and provides a result URL when the query is complete.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** | The UUID of the query to check status for | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_query

> String page_views_query(user_id, page_views_query_request)
BETA - Initiate page views query

Initiates an asynchronous query for user page views data within a specified date range. This method enqueues a background job to process the page views query and returns a polling URL that can be used to check the query status and retrieve results when ready.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**page_views_query_request** | Option<[**PageViewsQueryRequest**](PageViewsQueryRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/:user_id/page_views/query \\   -X POST \\   -H 'Authorization: Bearer <token>' \\   -H 'Content-Type: application/json' \\   -d '{     \"start_date\": \"2023-01-01\",     \"end_date\": \"2023-02-01\",     \"results_format\": \"csv\"   }' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## page_views_query_results

> String page_views_query_results(query_id, user_id)
BETA - Get query results

Retrieves the results of a completed page views query. Returns the data in the format specified when the query was initiated (CSV or JSON). The response may be compressed with gzip encoding.  As this is a beta endpoint, it is subject to change or removal at any time without the standard notice periods outlined in the API policy.  Note: PageView payloads use two types of identifiers: globalId and localId. Global identifier is equal to (shardId*10000000000000)+localId. Please note our global identifiers might change if your Canvas instance goes through shard migration process, in this case your current shardId in the global identifier will change to a new shardId. Local identifiers do not change after shard migration and stay unique in the context of the Canvas account. The following fields in the PageView payload are global identifiers: `links_user`, `links_context`, `links_asset`, `links_real_user`, `links_account`, `developer_key_id`, `asset_user_access_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | **String** | The UUID of the completed query to retrieve results for | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

