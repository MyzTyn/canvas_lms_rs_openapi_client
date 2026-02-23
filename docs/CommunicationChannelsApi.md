# \CommunicationChannelsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**communication_channels_create**](CommunicationChannelsApi.md#communication_channels_create) | **POST** /users/{user_id}/communication_channels | Create a communication channel
[**communication_channels_delete_push_token**](CommunicationChannelsApi.md#communication_channels_delete_push_token) | **DELETE** /users/self/communication_channels/push | Delete a push notification endpoint
[**communication_channels_destroy_for_users**](CommunicationChannelsApi.md#communication_channels_destroy_for_users) | **DELETE** /users/{user_id}/communication_channels/{id} | Delete a communication channel
[**communication_channels_destroy_for_users2**](CommunicationChannelsApi.md#communication_channels_destroy_for_users2) | **DELETE** /users/{user_id}/communication_channels/{type}/{address} | Delete a communication channel
[**communication_channels_index**](CommunicationChannelsApi.md#communication_channels_index) | **GET** /users/{user_id}/communication_channels | List user communication channels



## communication_channels_create

> String communication_channels_create(user_id, communication_channels_create_request)
Create a communication channel

Creates a new communication channel for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**communication_channels_create_request** | Option<[**CommunicationChannelsCreateRequest**](CommunicationChannelsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/1/communication_channels \\      -H 'Authorization: Bearer <token>' \\      -d 'communication_channel[address]=new@example.com' \\      -d 'communication_channel[type]=email' \\ ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## communication_channels_delete_push_token

> serde_json::Value communication_channels_delete_push_token()
Delete a push notification endpoint

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## communication_channels_destroy_for_users

> String communication_channels_destroy_for_users(id, user_id)
Delete a communication channel

Delete an existing communication channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## communication_channels_destroy_for_users2

> String communication_channels_destroy_for_users2(address, r#type, user_id)
Delete a communication channel

Delete an existing communication channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | ID of the address | [required] |
**r#type** | **String** | ID of the type | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## communication_channels_index

> models::CommunicationChannel communication_channels_index(user_id, page, per_page)
List user communication channels

Returns a paginated list of communication channels for the specified user, sorted by position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CommunicationChannel**](CommunicationChannel.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

