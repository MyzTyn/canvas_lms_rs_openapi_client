# \EportfoliosApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**eportfolios_api_delete**](EportfoliosApiApi.md#eportfolios_api_delete) | **DELETE** /eportfolios/{id} | Delete an ePortfolio
[**eportfolios_api_index**](EportfoliosApiApi.md#eportfolios_api_index) | **GET** /users/{user_id}/eportfolios | Get all ePortfolios for a User
[**eportfolios_api_moderate**](EportfoliosApiApi.md#eportfolios_api_moderate) | **PUT** /eportfolios/{eportfolio_id}/moderate | Moderate an ePortfolio
[**eportfolios_api_moderate_all**](EportfoliosApiApi.md#eportfolios_api_moderate_all) | **PUT** /users/{user_id}/eportfolios | Moderate all ePortfolios for a User
[**eportfolios_api_pages**](EportfoliosApiApi.md#eportfolios_api_pages) | **GET** /eportfolios/{eportfolio_id}/pages | Get ePortfolio Pages
[**eportfolios_api_restore**](EportfoliosApiApi.md#eportfolios_api_restore) | **PUT** /eportfolios/{eportfolio_id}/restore | Restore a deleted ePortfolio
[**eportfolios_api_show**](EportfoliosApiApi.md#eportfolios_api_show) | **GET** /eportfolios/{id} | Get an ePortfolio



## eportfolios_api_delete

> String eportfolios_api_delete(id)
Delete an ePortfolio

Mark an ePortfolio as deleted.

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


## eportfolios_api_index

> models::EPortfolio eportfolios_api_index(user_id, include, page, per_page)
Get all ePortfolios for a User

Get a list of all ePortfolios for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"deleted\"] deleted:: Include deleted ePortfolios. Only available to admins who can moderate_user_content. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::EPortfolio**](ePortfolio.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eportfolios_api_moderate

> String eportfolios_api_moderate(eportfolio_id, eportfolios_api_moderate_request)
Moderate an ePortfolio

Update the spam_status of an eportfolio. Only available to admins who can moderate_user_content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eportfolio_id** | **String** | Scope response to eportfolio_id | [required] |
**eportfolios_api_moderate_request** | Option<[**EportfoliosApiModerateRequest**](EportfoliosApiModerateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eportfolios_api_moderate_all

> eportfolios_api_moderate_all(user_id, eportfolios_api_moderate_all_request)
Moderate all ePortfolios for a User

Update the spam_status for all active eportfolios of a user. Only available to admins who can moderate_user_content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**eportfolios_api_moderate_all_request** | Option<[**EportfoliosApiModerateAllRequest**](EportfoliosApiModerateAllRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eportfolios_api_pages

> models::EPortfolioPage eportfolios_api_pages(eportfolio_id, page, per_page)
Get ePortfolio Pages

Get details for the pages of an ePortfolio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eportfolio_id** | **String** | Scope response to eportfolio_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::EPortfolioPage**](ePortfolioPage.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eportfolios_api_restore

> String eportfolios_api_restore(eportfolio_id)
Restore a deleted ePortfolio

Restore an ePortfolio back to active that was previously deleted. Only available to admins who can moderate_user_content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**eportfolio_id** | **String** | Scope response to eportfolio_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eportfolios_api_show

> String eportfolios_api_show(id)
Get an ePortfolio

Get details for a single ePortfolio.

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

