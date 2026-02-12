# \MergeApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_by_org_by_repo_merge_by_base**](MergeApi.md#post_by_org_by_repo_merge_by_base) | **POST** /{org}/{repo}/merge/{base} | Merge branches



## post_by_org_by_repo_merge_by_base

> models::PostByOrgByRepoMergeByBase200Response post_by_org_by_repo_merge_by_base(org, repo, base, post_by_org_by_repo_merge_by_base_request)
Merge branches

Merge a head branch into a base branch. Performs a fast-forward merge when possible, otherwise creates a merge commit. Returns 409 if there are merge conflicts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**base** | Option<**String**> |  | [required] |
**post_by_org_by_repo_merge_by_base_request** | Option<[**PostByOrgByRepoMergeByBaseRequest**](PostByOrgByRepoMergeByBaseRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoMergeByBase200Response**](postByOrgByRepoMergeByBase_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

