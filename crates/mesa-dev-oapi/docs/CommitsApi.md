# \CommitsApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_org_by_repo_commits**](CommitsApi.md#get_by_org_by_repo_commits) | **GET** /{org}/{repo}/commits | List commits
[**get_by_org_by_repo_commits_by_sha**](CommitsApi.md#get_by_org_by_repo_commits_by_sha) | **GET** /{org}/{repo}/commits/{sha} | Get commit
[**post_by_org_by_repo_commits**](CommitsApi.md#post_by_org_by_repo_commits) | **POST** /{org}/{repo}/commits | Create commit



## get_by_org_by_repo_commits

> models::GetByOrgByRepoCommits200Response get_by_org_by_repo_commits(org, repo, cursor, limit, r#ref)
List commits

List commits for a repository from a specific ref

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**u8**> |  |  |
**r#ref** | Option<**String**> |  |  |

### Return type

[**models::GetByOrgByRepoCommits200Response**](getByOrgByRepoCommits_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo_commits_by_sha

> models::GetByOrgByRepoCommitsBySha200Response get_by_org_by_repo_commits_by_sha(org, repo, sha)
Get commit

Retrieve a specific commit by its SHA

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**sha** | Option<**String**> |  | [required] |

### Return type

[**models::GetByOrgByRepoCommitsBySha200Response**](getByOrgByRepoCommitsBySha_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_commits

> models::PostByOrgByRepoCommits201Response post_by_org_by_repo_commits(org, repo, post_by_org_by_repo_commits_request)
Create commit

Programmatically create a commit with file edits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**post_by_org_by_repo_commits_request** | Option<[**PostByOrgByRepoCommitsRequest**](PostByOrgByRepoCommitsRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoCommits201Response**](postByOrgByRepoCommits_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

