# LtiDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the Canvas ID of the Lti_Deployment object | [optional]
**registration_id** | Option<**i32**> | the Canvas ID of the associated Lti_Registration object | [optional]
**deployment_id** | Option<**String**> | The Deployment ID of this deployment which is shared with launched tools | [optional]
**context_id** | Option<**i32**> | The Canvas ID of the context this deployment is associated with | [optional]
**context_type** | Option<**String**> | The type of context this deployment is associated with | [optional]
**context_name** | Option<**String**> | The name of the context this deployment is associated with | [optional]
**workflow_state** | Option<**String**> | The workflow state of the deployment | [optional]
**context_controls** | Option<[**Vec<models::LtiContextControl>**](LtiContextControl.md)> | The context controls for this deployment. Only present in the LTI Context Controls - List All Context Controls endpoint. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


