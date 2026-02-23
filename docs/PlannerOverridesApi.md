# \PlannerOverridesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**planner_overrides_create**](PlannerOverridesApi.md#planner_overrides_create) | **POST** /planner/overrides | Create a planner override
[**planner_overrides_destroy**](PlannerOverridesApi.md#planner_overrides_destroy) | **DELETE** /planner/overrides/{id} | Delete a planner override
[**planner_overrides_index**](PlannerOverridesApi.md#planner_overrides_index) | **GET** /planner/overrides | List planner overrides
[**planner_overrides_show**](PlannerOverridesApi.md#planner_overrides_show) | **GET** /planner/overrides/{id} | Show a planner override
[**planner_overrides_update**](PlannerOverridesApi.md#planner_overrides_update) | **PUT** /planner/overrides/{id} | Update a planner override



## planner_overrides_create

> String planner_overrides_create(planner_overrides_create_request)
Create a planner override

Create a planner override for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**planner_overrides_create_request** | Option<[**PlannerOverridesCreateRequest**](PlannerOverridesCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_overrides_destroy

> String planner_overrides_destroy(id)
Delete a planner override

Delete a planner override for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_overrides_index

> models::PlannerOverride planner_overrides_index(page, per_page)
List planner overrides

Retrieve a planner override for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::PlannerOverride**](PlannerOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_overrides_show

> String planner_overrides_show(id)
Show a planner override

Retrieve a planner override for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_overrides_update

> String planner_overrides_update(id, planner_overrides_update_request)
Update a planner override

Update a planner override's visibilty for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**planner_overrides_update_request** | Option<[**PlannerOverridesUpdateRequest**](PlannerOverridesUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

