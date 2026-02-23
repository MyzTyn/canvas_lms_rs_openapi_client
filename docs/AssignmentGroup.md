# AssignmentGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the id of the Assignment Group | [optional]
**name** | Option<**String**> | the name of the Assignment Group | [optional]
**position** | Option<**i32**> | the position of the Assignment Group | [optional]
**group_weight** | Option<**i32**> | the weight of the Assignment Group | [optional]
**sis_source_id** | Option<**String**> | the sis source id of the Assignment Group | [optional]
**integration_data** | Option<**serde_json::Value**> | the integration data of the Assignment Group | [optional]
**assignments** | Option<**Vec<i32>**> | the assignments in this Assignment Group (see the Assignment API for a detailed list of fields) | [optional]
**rules** | Option<[**models::GradingRules**](GradingRules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


