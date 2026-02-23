# DataServiceSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | Option<**String**> | The id of the context for the subscription. | [optional]
**context_type** | Option<**String**> | The type of context for the subscription. Must be 'assignment', or 'root_account' | [optional]
**event_types** | Option<**Vec<String>**> | Array of strings representing the event types for the subscription. | [optional]
**format** | Option<**String**> | Format to deliver the live events. Must be 'live-event' or 'caliper'. | [optional]
**transport_metadata** | Option<**String**> | An object with a single key: 'Url'. | [optional]
**transport_type** | Option<**String**> | The type of transport for the event. Must be either 'sqs' or 'https'. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


