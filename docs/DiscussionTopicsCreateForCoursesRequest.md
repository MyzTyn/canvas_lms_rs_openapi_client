# DiscussionTopicsCreateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**discussion_type** | Option<**DiscussionType**> | The type of discussion. Defaults to side_comment or not_threaded if not value is given. Accepted values are 'side_comment', 'not_threaded' for discussions that only allow one level of nested comments, and 'threaded' for fully threaded discussions. (enum: side_comment, threaded, not_threaded) | [optional]
**published** | Option<**bool**> | Whether this topic is published (true) or draft state (false). Only teachers and TAs have the ability to create draft state topics. | [optional]
**delayed_post_at** | Option<**String**> | If a timestamp is given, the topic will not be published until that time. | [optional]
**allow_rating** | Option<**bool**> | Whether or not users can rate entries in this topic. | [optional]
**lock_at** | Option<**String**> | If a timestamp is given, the topic will be scheduled to lock at the provided timestamp. If the timestamp is in the past, the topic will be locked. | [optional]
**podcast_enabled** | Option<**bool**> | If true, the topic will have an associated podcast feed. | [optional]
**podcast_has_student_posts** | Option<**bool**> | If true, the podcast will include posts from students as well. Implies podcast_enabled. | [optional]
**require_initial_post** | Option<**bool**> | If true then a user may not respond to other replies until that user has made an initial reply. Defaults to false. | [optional]
**is_announcement** | Option<**bool**> | If true, this topic is an announcement. It will appear in the announcement's section rather than the discussions section. This requires announcment-posting permissions. | [optional]
**pinned** | Option<**bool**> | If true, this topic will be listed in the \"Pinned Discussion\" section | [optional]
**position_after** | Option<**String**> | By default, discussions are sorted chronologically by creation date, you can pass the id of another topic to have this one show up after the other when they are listed. | [optional]
**group_category_id** | Option<**i32**> | If present, the topic will become a group discussion assigned to the group. | [optional]
**only_graders_can_rate** | Option<**bool**> | If true, only graders will be allowed to rate entries. | [optional]
**sort_order** | Option<**SortOrder**> | Default sort order of the discussion. Accepted values are \"asc\", \"desc\". (enum: asc, desc) | [optional]
**sort_order_locked** | Option<**bool**> | If true, users cannot choose their prefered sort order | [optional]
**expanded** | Option<**bool**> | If true, thread will be expanded by default | [optional]
**expanded_locked** | Option<**bool**> | If true, users cannot choose their prefered thread expansion setting | [optional]
**sort_by_rating** | Option<**bool**> | (DEPRECATED) If true, entries will be sorted by rating. | [optional]
**specific_sections** | Option<**String**> | A comma-separated list of sections ids to which the discussion topic should be made specific to.  If it is not desired to make the discussion topic specific to sections, then this parameter may be omitted or set to \"all\".  Can only be present only on announcements and only those that are for a course (as opposed to a group). | [optional]
**lock_comment** | Option<**bool**> | If is_announcement and lock_comment are true, ‘Allow Participants to Comment’ setting is disabled. | [optional]
**assignment** | Option<[**models::DiscussionTopicsUpdateForCoursesRequestAssignment**](DiscussionTopicsUpdateForCoursesRequestAssignment.md)> |  | [optional]
**attachment** | Option<[**models::DiscussionTopicsCreateForCoursesRequestAttachment**](DiscussionTopicsCreateForCoursesRequestAttachment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


