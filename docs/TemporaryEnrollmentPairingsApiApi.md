# \TemporaryEnrollmentPairingsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**temporary_enrollment_pairings_api_create**](TemporaryEnrollmentPairingsApiApi.md#temporary_enrollment_pairings_api_create) | **POST** /accounts/{account_id}/temporary_enrollment_pairings | Create Temporary Enrollment Pairing
[**temporary_enrollment_pairings_api_destroy**](TemporaryEnrollmentPairingsApiApi.md#temporary_enrollment_pairings_api_destroy) | **DELETE** /accounts/{account_id}/temporary_enrollment_pairings/{id} | Delete Temporary Enrollment Pairing
[**temporary_enrollment_pairings_api_index**](TemporaryEnrollmentPairingsApiApi.md#temporary_enrollment_pairings_api_index) | **GET** /accounts/{account_id}/temporary_enrollment_pairings | List temporary enrollment pairings
[**temporary_enrollment_pairings_api_new**](TemporaryEnrollmentPairingsApiApi.md#temporary_enrollment_pairings_api_new) | **GET** /accounts/{account_id}/temporary_enrollment_pairings/new | New TemporaryEnrollmentPairing
[**temporary_enrollment_pairings_api_show**](TemporaryEnrollmentPairingsApiApi.md#temporary_enrollment_pairings_api_show) | **GET** /accounts/{account_id}/temporary_enrollment_pairings/{id} | Get a single temporary enrollment pairing



## temporary_enrollment_pairings_api_create

> String temporary_enrollment_pairings_api_create(account_id, temporary_enrollment_pairings_api_create_request)
Create Temporary Enrollment Pairing

Create a Temporary Enrollment Pairing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**temporary_enrollment_pairings_api_create_request** | Option<[**TemporaryEnrollmentPairingsApiCreateRequest**](TemporaryEnrollmentPairingsApiCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## temporary_enrollment_pairings_api_destroy

> String temporary_enrollment_pairings_api_destroy(account_id, id)
Delete Temporary Enrollment Pairing

Delete a temporary enrollment pairing

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


## temporary_enrollment_pairings_api_index

> models::TemporaryEnrollmentPairing temporary_enrollment_pairings_api_index(account_id)
List temporary enrollment pairings

Returns the list of temporary enrollment pairings for a root account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

[**models::TemporaryEnrollmentPairing**](TemporaryEnrollmentPairing.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## temporary_enrollment_pairings_api_new

> String temporary_enrollment_pairings_api_new(account_id)
New TemporaryEnrollmentPairing

Initialize an unsaved Temporary Enrollment Pairing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## temporary_enrollment_pairings_api_show

> String temporary_enrollment_pairings_api_show(account_id, id)
Get a single temporary enrollment pairing

Returns the temporary enrollment pairing with the given id.

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

