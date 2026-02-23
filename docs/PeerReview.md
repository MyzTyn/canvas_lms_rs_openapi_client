# PeerReview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assessor_id** | Option<**i32**> | The assessors user id | [optional]
**asset_id** | Option<**i32**> | The id for the asset associated with this Peer Review | [optional]
**asset_type** | Option<**String**> | The type of the asset | [optional]
**id** | Option<**i32**> | The id of the Peer Review | [optional]
**user_id** | Option<**i32**> | The user id for the owner of the asset | [optional]
**workflow_state** | Option<**String**> | The state of the Peer Review, either 'assigned' or 'completed' | [optional]
**user** | Option<**String**> | the User object for the owner of the asset if the user include parameter is provided (see user API) (optional) | [optional]
**assessor** | Option<**String**> | The User object for the assessor if the user include parameter is provided (see user API) (optional) | [optional]
**submission_comments** | Option<**String**> | The submission comments associated with this Peer Review if the submission_comment include parameter is provided (see submissions API) (optional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


