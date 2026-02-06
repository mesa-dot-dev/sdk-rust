# \ReposApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_by_org_by_repo**](ReposApi.md#delete_by_org_by_repo) | **DELETE** /{org}/{repo} | Delete repository
[**get_by_org_by_repo**](ReposApi.md#get_by_org_by_repo) | **GET** /{org}/{repo} | Get repository
[**get_by_org_by_repo_sync**](ReposApi.md#get_by_org_by_repo_sync) | **GET** /{org}/{repo}/sync | Get sync status
[**get_by_org_repos**](ReposApi.md#get_by_org_repos) | **GET** /{org}/repos | List repositories
[**patch_by_org_by_repo**](ReposApi.md#patch_by_org_by_repo) | **PATCH** /{org}/{repo} | Update repository
[**post_by_org_by_repo_sync**](ReposApi.md#post_by_org_by_repo_sync) | **POST** /{org}/{repo}/sync | Sync repository
[**post_by_org_repos**](ReposApi.md#post_by_org_repos) | **POST** /{org}/repos | Create repository



## delete_by_org_by_repo

> models::DeleteByOrgApiKeysById200Response delete_by_org_by_repo(org, repo)
Delete repository

Permanently delete a repository and all its data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::DeleteByOrgApiKeysById200Response**](deleteByOrgApiKeysById_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo

> models::PostByOrgRepos201Response get_by_org_by_repo(org, repo)
Get repository

Get metadata for a specific repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::PostByOrgRepos201Response**](postByOrgRepos_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo_sync

> models::GetByOrgByRepoSync200Response get_by_org_by_repo_sync(org, repo)
Get sync status

Get the sync status for a repository with upstream configured

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::GetByOrgByRepoSync200Response**](getByOrgByRepoSync_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_repos

> models::GetByOrgRepos200Response get_by_org_repos(org, cursor, limit)
List repositories

List all repositories in the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**u8**> |  |  |

### Return type

[**models::GetByOrgRepos200Response**](getByOrgRepos_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_by_org_by_repo

> models::PostByOrgRepos201Response patch_by_org_by_repo(org, repo, patch_by_org_by_repo_request)
Update repository

Update repository name or upstream configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**patch_by_org_by_repo_request** | Option<[**PatchByOrgByRepoRequest**](PatchByOrgByRepoRequest.md)> |  |  |

### Return type

[**models::PostByOrgRepos201Response**](postByOrgRepos_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_sync

> models::DeleteByOrgApiKeysById200Response post_by_org_by_repo_sync(org, repo)
Sync repository

Trigger a sync from the upstream repository. Waits for sync to complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::DeleteByOrgApiKeysById200Response**](deleteByOrgApiKeysById_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_repos

> models::PostByOrgRepos201Response post_by_org_repos(org, post_by_org_repos_request)
Create repository

Create a new repository in the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**post_by_org_repos_request** | Option<[**PostByOrgReposRequest**](PostByOrgReposRequest.md)> |  |  |

### Return type

[**models::PostByOrgRepos201Response**](postByOrgRepos_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

