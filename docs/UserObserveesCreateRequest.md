# UserObserveesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | Option<**String**> | The access token for the user to observe.  Required if <tt>observee[unique_id]</tt> or <tt>observee[password]</tt> are omitted. | [optional]
**pairing_code** | Option<**String**> | A generated pairing code for the user to observe. Required if the Observer pairing code feature flag is enabled | [optional]
**root_account_id** | Option<**i32**> | The ID for the root account to associate with the observation link. Defaults to the current domain account. If 'all' is specified, a link will be created for each root account associated to both the observer and observee. | [optional]
**observee** | Option<[**models::UserObserveesCreateRequestObservee**](UserObserveesCreateRequestObservee.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


