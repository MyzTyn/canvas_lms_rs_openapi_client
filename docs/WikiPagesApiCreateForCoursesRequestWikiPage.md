# WikiPagesApiCreateForCoursesRequestWikiPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | [Required, String] The title for the new page. | 
**body** | Option<**String**> | [String] The content for the new page. | [optional]
**editing_roles** | Option<**EditingRoles**> | [String, \"teachers\"|\"students\"|\"members\"|\"public\"] Which user roles are allowed to edit this page. Any combination of these roles is allowed (separated by commas).  \"teachers\":: Allows editing by teachers in the course. \"students\":: Allows editing by students in the course. \"members\":: For group wikis, allows editing by members of the group. \"public\":: Allows editing by any user. (enum: teachers, students, members, public) | [optional]
**notify_of_update** | Option<**bool**> | [Boolean] Whether participants should be notified when this page changes. | [optional]
**published** | Option<**bool**> | [Boolean] Whether the page is published (true) or draft state (false). | [optional]
**front_page** | Option<**bool**> | [Boolean] Set an unhidden page as the front page (if true) | [optional]
**publish_at** | Option<**String**> | [Optional, DateTime] Schedule a future date/time to publish the page. This will have no effect unless the \"Scheduled Page Publication\" feature is enabled in the account. If a future date is supplied, the page will be unpublished and +wiki_page[published]+ will be ignored. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


