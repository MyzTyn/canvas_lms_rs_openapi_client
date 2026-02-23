# LtiOverlayVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**root_account_id** | Option<**i32**> | The Canvas id of the root account | [optional]
**created_at** | Option<**String**> | Timestamp of the version's creation | [optional]
**updated_at** | Option<**String**> | Timestamp of the version's last update | [optional]
**caused_by_reset** | Option<**bool**> | Whether or not this change was caused by a reset of the tool's configuration | [optional]
**created_by** | Option<[**models::User**](User.md)> |  | [optional]
**diff** | Option<[**Vec<Vec<serde_json::Value>>**](Vec.md)> | A list of changes made in this version compared to the previous version | [optional]
**lti_overlay_id** | Option<**i32**> | The id of the overlay this version is for | [optional]
**account_id** | Option<**i32**> | The id of the account this version is for | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


