# OutcomeResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique identifier for this result | [optional]
**score** | Option<**i32**> | The student's score | [optional]
**submitted_or_assessed_at** | Option<**String**> | The datetime the resulting OutcomeResult was submitted at, or absent that, when it was assessed. | [optional]
**links** | Option<**serde_json::Value**> | Unique identifiers of objects associated with this result | [optional]
**percent** | Option<**f64**> | score's percent of maximum points possible for outcome, scaled to reflect any custom mastery levels that differ from the learning outcome | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


