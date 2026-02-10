# GetByOrgByRepoContent200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** |  (enum: file, symlink, dir) | 
**name** | **String** |  | 
**path** | **String** |  | 
**sha** | **String** |  | 
**size** | **f64** |  | 
**encoding** | **Encoding** |  (enum: base64) | 
**content** | **String** |  | 
**executable** | **bool** |  | 
**last_commit_at** | **String** |  | 
**child_count** | **i32** |  | 
**entries** | [**Vec<models::GetByOrgByRepoContent200ResponseAnyOf2EntriesInner>**](GetByOrgByRepoContent200ResponseAnyOf2EntriesInner.md) |  | 
**next_cursor** | **String** |  | 
**has_more** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


