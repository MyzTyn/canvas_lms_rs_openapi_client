# \ProfileApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**profile_profile_pics**](ProfileApi.md#profile_profile_pics) | **GET** /users/{user_id}/avatars | List avatar options
[**profile_settings**](ProfileApi.md#profile_settings) | **GET** /users/{user_id}/profile | Get user profile



## profile_profile_pics

> models::Avatar profile_profile_pics(user_id)
List avatar options

A paginated list of the possible user avatar options that can be set with the user update endpoint. The response will be an array of avatar records. If the 'type' field is 'attachment', the record will include all the normal attachment json fields; otherwise it will include only the 'url' and 'display_name' fields. Additionally, all records will include a 'type' field and a 'token' field. The following explains each field in more detail type:: [\"gravatar\"|\"attachment\"|\"no_pic\"] The type of avatar record, for categorization purposes. url:: The url of the avatar token:: A unique representation of the avatar record which can be used to set the avatar with the user update endpoint. Note: this is an internal representation and is subject to change without notice. It should be consumed with this api endpoint and used in the user update endpoint, and should not be constructed by the client. display_name:: A textual description of the avatar record id:: ['attachment' type only] the internal id of the attachment content-type:: ['attachment' type only] the content-type of the attachment filename:: ['attachment' type only] the filename of the attachment size:: ['attachment' type only] the size of the attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

[**models::Avatar**](Avatar.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_settings

> String profile_settings(user_id, include)
Get user profile

Returns user profile data, including user id, name, and profile pic.  When requesting the profile for the user accessing the API, the user's calendar feed URL and LTI user id will be returned as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"links\"|\"user_services\"|\"uuid\"] Array of additional information to include.  \"links\":: include the user's profile links in the response           as an array of objects with +url+ and +title+ fields \"user_services\":: include names and links for the user's connected services \"uuid\":: include the user's uuid in the response |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

