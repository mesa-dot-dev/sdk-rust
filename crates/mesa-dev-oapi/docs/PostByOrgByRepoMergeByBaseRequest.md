# PostByOrgByRepoMergeByBaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**head** | **String** | The branch to merge from | 
**message** | Option<**String**> | Custom merge commit message. Defaults to \"Merge branch 'head' into 'base'\" | [optional]
**author** | [**models::PostByOrgByRepoMergeByBaseRequestAuthor**](PostByOrgByRepoMergeByBaseRequestAuthor.md) |  | 
**delete_branch** | Option<**bool**> | Whether to delete the head branch after a successful merge | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


