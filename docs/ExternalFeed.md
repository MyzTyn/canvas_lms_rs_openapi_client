# ExternalFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the feed | [optional]
**display_name** | Option<**String**> | The title of the feed, pulled from the feed itself. If the feed hasn't yet been pulled, a temporary name will be synthesized based on the URL | [optional]
**url** | Option<**String**> | The HTTP/HTTPS URL to the feed | [optional]
**header_match** | Option<**String**> | If not null, only feed entries whose title contains this string will trigger new posts in Canvas | [optional]
**created_at** | Option<**String**> | When this external feed was added to Canvas | [optional]
**verbosity** | Option<**Verbosity**> | The verbosity setting determines how much of the feed's content is imported into Canvas as part of the posting. 'link_only' means that only the title and a link to the item. 'truncate' means that a summary of the first portion of the item body will be used. 'full' means that the full item body will be used. (enum: link_only, truncate, full) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


