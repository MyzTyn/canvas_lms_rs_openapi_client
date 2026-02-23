# AccountCalendar

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the account associated with this calendar | [optional]
**name** | Option<**String**> | the name of the account associated with this calendar | [optional]
**parent_account_id** | Option<**i32**> | the account's parent ID, or null if this is the root account | [optional]
**root_account_id** | Option<**i32**> | the ID of the root account, or null if this is the root account | [optional]
**visible** | Option<**bool**> | whether this calendar is visible to users | [optional]
**auto_subscribe** | Option<**bool**> | whether users see this calendar's events without needing to manually add it | [optional]
**sub_account_count** | Option<**i32**> | number of this account's direct sub-accounts | [optional]
**asset_string** | Option<**String**> | Asset string of the account | [optional]
**r#type** | Option<**String**> | Object type | [optional]
**calendar_event_url** | Option<**String**> | url to get full detailed events | [optional]
**can_create_calendar_events** | Option<**bool**> | whether the user can create calendar events | [optional]
**create_calendar_event_url** | Option<**String**> | API path to create events for the account | [optional]
**new_calendar_event_url** | Option<**String**> | url to open the more options event editor | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


