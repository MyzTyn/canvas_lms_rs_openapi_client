# LtiRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the Canvas ID of the Lti_Registration object | [optional]
**name** | Option<**String**> | Tool-provided registration name | [optional]
**admin_nickname** | Option<**String**> | Admin-configured friendly display name | [optional]
**icon_url** | Option<**String**> | Tool-provided URL to the tool's icon | [optional]
**vendor** | Option<**String**> | Tool-provided name of the tool vendor | [optional]
**account_id** | Option<**i32**> | The Canvas id of the account that owns this registration | [optional]
**internal_service** | Option<**bool**> | Flag indicating if registration is internally-owned | [optional]
**inherited** | Option<**bool**> | Flag indicating if registration is owned by this account, or inherited from Site Admin | [optional]
**lti_version** | Option<**String**> | LTI version of the registration, either 1.1 or 1.3 | [optional]
**dynamic_registration** | Option<**bool**> | Flag indicating if registration was created using LTI Dynamic Registration. Only present if lti_version is 1.3 | [optional]
**workflow_state** | Option<**String**> | The state of the registration | [optional]
**created_at** | Option<**String**> | Timestamp of the registration's creation | [optional]
**updated_at** | Option<**String**> | Timestamp of the registration's last update | [optional]
**created_by** | Option<[**models::User**](User.md)> |  | [optional]
**updated_by** | Option<[**models::User**](User.md)> |  | [optional]
**root_account_id** | Option<**i32**> | The Canvas id of the root account | [optional]
**account_binding** | Option<[**models::LtiRegistrationAccountBinding**](LtiRegistrationAccountBinding.md)> |  | [optional]
**configuration** | Option<[**models::LtiToolConfiguration**](LtiToolConfiguration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


