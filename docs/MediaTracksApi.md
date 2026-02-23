# \MediaTracksApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**media_tracks_index_other**](MediaTracksApi.md#media_tracks_index_other) | **GET** /media_objects/{media_object_id}/media_tracks | List media tracks for a Media Object or Attachment
[**media_tracks_index_other2**](MediaTracksApi.md#media_tracks_index_other2) | **GET** /media_attachments/{attachment_id}/media_tracks | List media tracks for a Media Object or Attachment
[**media_tracks_update_other**](MediaTracksApi.md#media_tracks_update_other) | **PUT** /media_objects/{media_object_id}/media_tracks | Update Media Tracks
[**media_tracks_update_other2**](MediaTracksApi.md#media_tracks_update_other2) | **PUT** /media_attachments/{attachment_id}/media_tracks | Update Media Tracks



## media_tracks_index_other

> serde_json::Value media_tracks_index_other(media_object_id, include)
List media tracks for a Media Object or Attachment

List the media tracks associated with a media object or attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_object_id** | **String** | Scope response to media_object_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"content\"|\"webvtt_content\"|\"updated_at\"|\"created_at\"] By default, index returns id, locale, kind, media_object_id, and user_id for each of the result MediaTracks. Use include[] to add additional fields. For example include[]=content |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_tracks_index_other2

> serde_json::Value media_tracks_index_other2(attachment_id, include)
List media tracks for a Media Object or Attachment

List the media tracks associated with a media object or attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of the attachment | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"content\"|\"webvtt_content\"|\"updated_at\"|\"created_at\"] By default, index returns id, locale, kind, media_object_id, and user_id for each of the result MediaTracks. Use include[] to add additional fields. For example include[]=content |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_tracks_update_other

> serde_json::Value media_tracks_update_other(media_object_id, media_tracks_update_other_request)
Update Media Tracks

Replace the media tracks associated with a media object or attachment with the array of tracks provided in the body. Update will delete any existing tracks not listed, leave untouched any tracks with no content field, and update or create tracks with a content field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_object_id** | **String** | Scope response to media_object_id | [required] |
**media_tracks_update_other_request** | Option<[**MediaTracksUpdateOtherRequest**](MediaTracksUpdateOtherRequest.md)> | Request body parameters |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_tracks_update_other2

> serde_json::Value media_tracks_update_other2(attachment_id, media_tracks_update_other_request)
Update Media Tracks

Replace the media tracks associated with a media object or attachment with the array of tracks provided in the body. Update will delete any existing tracks not listed, leave untouched any tracks with no content field, and update or create tracks with a content field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of the attachment | [required] |
**media_tracks_update_other_request** | Option<[**MediaTracksUpdateOtherRequest**](MediaTracksUpdateOtherRequest.md)> | Request body parameters |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

