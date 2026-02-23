# Profile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the user. | [optional]
**name** | Option<**String**> | Sample User | [optional]
**short_name** | Option<**String**> | Sample User | [optional]
**sortable_name** | Option<**String**> | user, sample | [optional]
**title** | Option<**String**> |  | [optional]
**bio** | Option<**String**> |  | [optional]
**pronunciation** | Option<**String**> | Name pronunciation | [optional]
**primary_email** | Option<**String**> | sample_user@example.com | [optional]
**login_id** | Option<**String**> | sample_user@example.com | [optional]
**sis_user_id** | Option<**String**> | sis1 | [optional]
**lti_user_id** | Option<**String**> |  | [optional]
**avatar_url** | Option<**String**> | The avatar_url can change over time, so we recommend not caching it for more than a few hours | [optional]
**calendar** | Option<[**models::CalendarLink**](CalendarLink.md)> |  | [optional]
**time_zone** | Option<**String**> | Optional: This field is only returned in certain API calls, and will return the IANA time zone name of the user's preferred timezone. | [optional]
**locale** | Option<**String**> | The users locale. | [optional]
**k5_user** | Option<**bool**> | Optional: Whether or not the user is a K5 user. This field is nil if the user settings are not for the user making the request. | [optional]
**use_classic_font_in_k5** | Option<**bool**> | Optional: Whether or not the user should see the classic font on the dashboard. Only applies if k5_user is true. This field is nil if the user settings are not for the user making the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


