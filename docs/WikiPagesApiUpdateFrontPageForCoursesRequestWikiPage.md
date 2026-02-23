# WikiPagesApiUpdateFrontPageForCoursesRequestWikiPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | [String] The title for the new page. NOTE: changing a page's title will change its url. The updated url will be returned in the result. | [optional]
**body** | Option<**String**> | [String] The content for the new page. | [optional]
**editing_roles** | Option<**EditingRoles**> | [String, \"teachers\"|\"students\"|\"members\"|\"public\"] Which user roles are allowed to edit this page. Any combination of these roles is allowed (separated by commas).  \"teachers\":: Allows editing by teachers in the course. \"students\":: Allows editing by students in the course. \"members\":: For group wikis, allows editing by members of the group. \"public\":: Allows editing by any user. (enum: teachers, students, members, public) | [optional]
**notify_of_update** | Option<**bool**> | [Boolean] Whether participants should be notified when this page changes. | [optional]
**published** | Option<**bool**> | [Boolean] Whether the page is published (true) or draft state (false). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


