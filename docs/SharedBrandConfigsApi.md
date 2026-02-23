# \SharedBrandConfigsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shared_brand_configs_create**](SharedBrandConfigsApi.md#shared_brand_configs_create) | **POST** /accounts/{account_id}/shared_brand_configs | Share a BrandConfig (Theme)
[**shared_brand_configs_destroy**](SharedBrandConfigsApi.md#shared_brand_configs_destroy) | **DELETE** /shared_brand_configs/{id} | Un-share a BrandConfig (Theme)
[**shared_brand_configs_update**](SharedBrandConfigsApi.md#shared_brand_configs_update) | **PUT** /accounts/{account_id}/shared_brand_configs/{id} | Update a shared theme



## shared_brand_configs_create

> String shared_brand_configs_create(account_id, shared_brand_configs_create_request)
Share a BrandConfig (Theme)

Create a SharedBrandConfig, which will give the given brand_config a name and make it available to other users of this account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**shared_brand_configs_create_request** | Option<[**SharedBrandConfigsCreateRequest**](SharedBrandConfigsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/accounts/<account_id>/shared_brand_configs' \\      -X POST \\      -F 'shared_brand_config[name]=Crimson and Gold Theme' \\      -F 'shared_brand_config[brand_config_md5]=a1f113321fa024e7a14cb0948597a2a4' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shared_brand_configs_destroy

> String shared_brand_configs_destroy(id)
Un-share a BrandConfig (Theme)

Delete a SharedBrandConfig, which will unshare it so you nor anyone else in your account will see it as an option to pick from.

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


## shared_brand_configs_update

> String shared_brand_configs_update(account_id, id)
Update a shared theme

Update the specified shared_brand_config with a new name or to point to a new brand_config. Uses same parameters as create.

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

