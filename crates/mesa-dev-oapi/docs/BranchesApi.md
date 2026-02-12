# \BranchesApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_by_org_by_repo_branches_by_branch**](BranchesApi.md#delete_by_org_by_repo_branches_by_branch) | **DELETE** /{org}/{repo}/branches/{branch} | Delete branch
[**get_by_org_by_repo_branches**](BranchesApi.md#get_by_org_by_repo_branches) | **GET** /{org}/{repo}/branches | List branches
[**post_by_org_by_repo_branches**](BranchesApi.md#post_by_org_by_repo_branches) | **POST** /{org}/{repo}/branches | Create branch



## delete_by_org_by_repo_branches_by_branch

> models::DeleteByOrgApiKeysById200Response delete_by_org_by_repo_branches_by_branch(org, repo, branch)
Delete branch

Delete a branch from a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**branch** | Option<**String**> |  | [required] |

### Return type

[**models::DeleteByOrgApiKeysById200Response**](deleteByOrgApiKeysById_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo_branches

> models::GetByOrgByRepoBranches200Response get_by_org_by_repo_branches(org, repo, cursor, limit)
List branches

List all branches in a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**u8**> |  |  |

### Return type

[**models::GetByOrgByRepoBranches200Response**](getByOrgByRepoBranches_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_branches

> models::PostByOrgByRepoBranches201Response post_by_org_by_repo_branches(org, repo, post_by_org_by_repo_branches_request)
Create branch

Create a new branch from an existing ref

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**post_by_org_by_repo_branches_request** | Option<[**PostByOrgByRepoBranchesRequest**](PostByOrgByRepoBranchesRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoBranches201Response**](postByOrgByRepoBranches_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

