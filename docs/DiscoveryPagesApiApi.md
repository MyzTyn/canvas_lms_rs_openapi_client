# \DiscoveryPagesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**discovery_pages_api_show**](DiscoveryPagesApiApi.md#discovery_pages_api_show) | **GET** /discovery_pages | Get Discovery Page
[**discovery_pages_api_upsert**](DiscoveryPagesApiApi.md#discovery_pages_api_upsert) | **PUT** /discovery_pages | Update Discovery Page



## discovery_pages_api_show

> String discovery_pages_api_show()
Get Discovery Page

Get the discovery page configuration for the domain root account.

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


## discovery_pages_api_upsert

> String discovery_pages_api_upsert(discovery_pages_api_upsert_request)
Update Discovery Page

Update or create the discovery page configuration for the domain root account. This is a full replacement - provide the complete configuration including primary, secondary, and active fields. Any fields omitted will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discovery_pages_api_upsert_request** | Option<[**DiscoveryPagesApiUpsertRequest**](DiscoveryPagesApiUpsertRequest.md)> | Request body parameters  **Example Request:** ``` curl -X PUT 'https://<canvas>/api/v1/discovery_pages' \\   -H 'Authorization: Bearer <token>' \\   -H 'Content-Type: application/json' \\   -d '{     \"discovery_page\": {       \"primary\": [         {           \"authentication_provider_id\": 1,           \"label\": \"Students\",           \"icon\": \"google\"         },         {           \"authentication_provider_id\": 2,           \"label\": \"Faculty\",           \"icon\": \"okta\"         }       ],       \"secondary\": [         {           \"authentication_provider_id\": 3,           \"label\": \"Admins\"         }       ],       \"active\": true     }   }' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

