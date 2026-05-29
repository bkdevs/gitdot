use async_trait::async_trait;

use crate::{
    client::{ImageClient, ImageClientImpl, R2Client, R2ClientImpl},
    dto::{
        AddMemberRequest, CreateOrganizationRequest, GetOrganizationRequest,
        ListOrganizationRepositoriesRequest, ListOrganizationsRequest, OrganizationMemberResponse,
        OrganizationResponse, Page, RepositoryResponse, UpdateOrganizationImageRequest,
        UpdateOrganizationMemberRequest, UpdateOrganizationRequest,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, OrganizationError},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::cursor,
};

#[async_trait]
pub trait OrganizationService: Send + Sync + 'static {
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError>;

    async fn get_organization(
        &self,
        request: GetOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError>;

    async fn update_organization(
        &self,
        request: UpdateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError>;

    async fn update_organization_image(
        &self,
        request: UpdateOrganizationImageRequest,
    ) -> Result<(), OrganizationError>;

    async fn add_member(
        &self,
        request: AddMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError>;

    async fn update_member(
        &self,
        request: UpdateOrganizationMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError>;

    async fn list_repositories(
        &self,
        request: ListOrganizationRepositoriesRequest,
    ) -> Result<Page<RepositoryResponse>, OrganizationError>;

    async fn list_organizations(
        &self,
        request: ListOrganizationsRequest,
    ) -> Result<Page<OrganizationResponse>, OrganizationError>;
}

#[derive(Debug, Clone)]
pub struct OrganizationServiceImpl<O, U, R, I, R2>
where
    O: OrganizationRepository,
    U: UserRepository,
    R: RepositoryRepository,
    I: ImageClient,
    R2: R2Client,
{
    org_repo: O,
    user_repo: U,
    repo_repo: R,
    image_client: I,
    r2_client: R2,
}

impl
    OrganizationServiceImpl<
        OrganizationRepositoryImpl,
        UserRepositoryImpl,
        RepositoryRepositoryImpl,
        ImageClientImpl,
        R2ClientImpl,
    >
{
    pub fn new(
        org_repo: OrganizationRepositoryImpl,
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        image_client: ImageClientImpl,
        r2_client: R2ClientImpl,
    ) -> Self {
        Self {
            org_repo,
            user_repo,
            repo_repo,
            image_client,
            r2_client,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<O, U, R, I, R2> OrganizationService for OrganizationServiceImpl<O, U, R, I, R2>
where
    O: OrganizationRepository,
    U: UserRepository,
    R: RepositoryRepository,
    I: ImageClient,
    R2: R2Client,
{
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        if self.org_repo.get(&org_name).await?.is_some() {
            return Err(ConflictError::new("organization", &org_name).into());
        }
        if self.user_repo.get(&org_name).await?.is_some() {
            return Err(ConflictError::new("organization", &org_name).into());
        }

        let org = self
            .org_repo
            .create(&org_name, request.owner_id, request.readme)
            .await?;

        if let Ok(image_bytes) = self.image_client.generate_org_image(&org_name).await {
            self.r2_client
                .upload_object(&format!("orgs/{}.webp", org.id), image_bytes)
                .await
                .ok();
        }

        Ok(org.into())
    }

    async fn get_organization(
        &self,
        request: GetOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let org = self
            .org_repo
            .get(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;

        Ok(org.into())
    }

    async fn update_organization(
        &self,
        request: UpdateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let org = self
            .org_repo
            .update(
                &org_name,
                request.location,
                request.readme,
                request.links,
                request.display_name,
            )
            .await?
            .or_not_found("organization", &org_name)?;

        Ok(org.into())
    }

    async fn update_organization_image(
        &self,
        request: UpdateOrganizationImageRequest,
    ) -> Result<(), OrganizationError> {
        let org_name = request.org_name.to_string();
        let org_id = self
            .org_repo
            .get_id(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;

        let webp_bytes = self.image_client.convert_to_webp(request.bytes).await?;
        let key = format!("orgs/{org_id}.webp");
        self.r2_client.upload_object(&key, webp_bytes).await?;
        self.org_repo.touch_image(org_id).await?;

        Ok(())
    }

    async fn add_member(
        &self,
        request: AddMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let user_name = request.user_name.to_string();

        let member = self
            .org_repo
            .add_member(
                &org_name,
                &user_name,
                request.role,
                request.role_description,
            )
            .await?;

        match member {
            Some(m) => Ok(m.into()),
            None => {
                if self.org_repo.get(&org_name).await?.is_none() {
                    return Err(NotFoundError::new("organization", &org_name).into());
                }
                if self.user_repo.get(&user_name).await?.is_none() {
                    return Err(NotFoundError::new("user", &user_name).into());
                }
                Err(ConflictError::new("member", &user_name).into())
            }
        }
    }

    async fn update_member(
        &self,
        request: UpdateOrganizationMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let member_id = request.member_id;

        let member = self
            .org_repo
            .update_member(&org_name, member_id, request.role_description)
            .await?
            .or_not_found("member", &member_id.to_string())?;

        Ok(member.into())
    }

    async fn list_organizations(
        &self,
        request: ListOrganizationsRequest,
    ) -> Result<Page<OrganizationResponse>, OrganizationError> {
        let (orgs, next_cursor) = self
            .org_repo
            .list(request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: orgs.into_iter().map(|o| o.into()).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_repositories(
        &self,
        request: ListOrganizationRepositoriesRequest,
    ) -> Result<Page<RepositoryResponse>, OrganizationError> {
        let org_name = request.org_name.to_string();
        self.org_repo
            .get(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;

        // visibility is filtered in SQL based on viewer_id
        let (repositories, next_cursor) = self
            .repo_repo
            .list_by_owner(
                &org_name,
                request.viewer_id,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: repositories.into_iter().map(|r| r.into()).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use uuid::Uuid;

    use super::{OrganizationService, OrganizationServiceImpl};
    use crate::{
        dto::{
            AddMemberRequest, CreateOrganizationRequest, GetOrganizationRequest,
            ListOrganizationRepositoriesRequest, ListOrganizationsRequest,
            UpdateOrganizationImageRequest, UpdateOrganizationMemberRequest,
            UpdateOrganizationRequest,
        },
        error::OrganizationError,
        model::{OrganizationRole, RepositoryOwnerType, RepositoryVisibility},
        service::{
            test_client::{MockImageClient, MockR2Client},
            test_common::{create_member, create_organization, create_repository, create_user},
            test_repository::{
                MockOrganizationRepository, MockRepositoryRepository, MockUserRepository,
            },
        },
    };

    type Service = OrganizationServiceImpl<
        MockOrganizationRepository,
        MockUserRepository,
        MockRepositoryRepository,
        MockImageClient,
        MockR2Client,
    >;

    fn create_service() -> Service {
        OrganizationServiceImpl {
            org_repo: MockOrganizationRepository::new(),
            user_repo: MockUserRepository::new(),
            repo_repo: MockRepositoryRepository::new(),
            image_client: MockImageClient::new(),
            r2_client: MockR2Client::new(),
        }
    }

    #[tokio::test]
    async fn create_organization_succeeds() {
        let mut svc = create_service();
        // Neither an org nor a user already owns the name.
        svc.org_repo.expect_get().returning(|_| Ok(None));
        svc.user_repo.expect_get().returning(|_| Ok(None));
        svc.org_repo
            .expect_create()
            .returning(|_, _, _| Ok(create_organization("acme")));
        // Avatar generation + upload are best-effort but still invoked.
        svc.image_client
            .expect_generate_org_image()
            .returning(|_| Ok(Bytes::from_static(b"webp")));
        svc.r2_client
            .expect_upload_object()
            .returning(|_, _| Ok(()));

        let req = CreateOrganizationRequest::new("acme", Uuid::new_v4(), None).unwrap();
        let resp = svc.create_organization(req).await.unwrap();
        assert_eq!(resp.name, "acme");
    }

    #[tokio::test]
    async fn create_organization_conflicts_when_org_name_exists() {
        let mut svc = create_service();
        svc.org_repo
            .expect_get()
            .returning(|_| Ok(Some(create_organization("acme"))));

        let req = CreateOrganizationRequest::new("acme", Uuid::new_v4(), None).unwrap();
        let err = svc.create_organization(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::Conflict(_)));
    }

    #[tokio::test]
    async fn create_organization_conflicts_when_user_name_exists() {
        let mut svc = create_service();
        svc.org_repo.expect_get().returning(|_| Ok(None));
        svc.user_repo
            .expect_get()
            .returning(|_| Ok(Some(create_user("acme"))));

        let req = CreateOrganizationRequest::new("acme", Uuid::new_v4(), None).unwrap();
        let err = svc.create_organization(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::Conflict(_)));
    }

    #[tokio::test]
    async fn get_organization_returns_response() {
        let mut svc = create_service();
        svc.org_repo
            .expect_get()
            .returning(|_| Ok(Some(create_organization("acme"))));

        let resp = svc
            .get_organization(GetOrganizationRequest::new("acme").unwrap())
            .await
            .unwrap();
        assert_eq!(resp.name, "acme");
    }

    #[tokio::test]
    async fn get_organization_missing_is_not_found() {
        let mut svc = create_service();
        svc.org_repo.expect_get().returning(|_| Ok(None));

        let err = svc
            .get_organization(GetOrganizationRequest::new("ghost").unwrap())
            .await
            .unwrap_err();
        assert!(matches!(err, OrganizationError::NotFound(_)));
    }

    #[tokio::test]
    async fn update_organization_updates_fields() {
        let mut svc = create_service();
        svc.org_repo.expect_update().returning(|_, _, _, _, _| {
            let mut org = create_organization("acme");
            org.location = Some("Earth".to_string());
            Ok(Some(org))
        });

        let req =
            UpdateOrganizationRequest::new("acme", Some("Earth".to_string()), None, None, None)
                .unwrap();
        let resp = svc.update_organization(req).await.unwrap();
        assert_eq!(resp.location.as_deref(), Some("Earth"));
    }

    #[tokio::test]
    async fn update_organization_image_uploads_and_touches() {
        let mut svc = create_service();
        svc.org_repo
            .expect_get_id()
            .returning(|_| Ok(Some(Uuid::new_v4())));
        svc.image_client
            .expect_convert_to_webp()
            .returning(|_| Ok(Bytes::from_static(b"webp")));
        svc.r2_client
            .expect_upload_object()
            .returning(|_, _| Ok(()));
        svc.org_repo.expect_touch_image().returning(|_| Ok(()));

        let req = UpdateOrganizationImageRequest::new("acme", Bytes::from_static(b"png")).unwrap();
        svc.update_organization_image(req).await.unwrap();
    }

    #[tokio::test]
    async fn update_organization_image_missing_is_not_found() {
        let mut svc = create_service();
        svc.org_repo.expect_get_id().returning(|_| Ok(None));

        let req = UpdateOrganizationImageRequest::new("ghost", Bytes::from_static(b"png")).unwrap();
        let err = svc.update_organization_image(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::NotFound(_)));
    }

    #[tokio::test]
    async fn add_member_succeeds() {
        let mut svc = create_service();
        svc.org_repo
            .expect_add_member()
            .returning(|_, _, _, _| Ok(Some(create_member("bob", OrganizationRole::Member))));

        let req = AddMemberRequest::new("acme", "bob", "member", None).unwrap();
        let resp = svc.add_member(req).await.unwrap();
        assert_eq!(resp.user_name, "bob");
        assert_eq!(resp.role, OrganizationRole::Member);
    }

    #[tokio::test]
    async fn add_member_missing_org_is_not_found() {
        let mut svc = create_service();
        svc.org_repo
            .expect_add_member()
            .returning(|_, _, _, _| Ok(None));
        svc.org_repo.expect_get().returning(|_| Ok(None));

        let req = AddMemberRequest::new("ghost", "bob", "member", None).unwrap();
        let err = svc.add_member(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::NotFound(_)));
    }

    #[tokio::test]
    async fn add_member_missing_user_is_not_found() {
        let mut svc = create_service();
        svc.org_repo
            .expect_add_member()
            .returning(|_, _, _, _| Ok(None));
        svc.org_repo
            .expect_get()
            .returning(|_| Ok(Some(create_organization("acme"))));
        svc.user_repo.expect_get().returning(|_| Ok(None));

        let req = AddMemberRequest::new("acme", "ghost", "member", None).unwrap();
        let err = svc.add_member(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::NotFound(_)));
    }

    #[tokio::test]
    async fn add_member_existing_member_is_conflict() {
        let mut svc = create_service();
        // A null insert with both org and user present means the row already exists.
        svc.org_repo
            .expect_add_member()
            .returning(|_, _, _, _| Ok(None));
        svc.org_repo
            .expect_get()
            .returning(|_| Ok(Some(create_organization("acme"))));
        svc.user_repo
            .expect_get()
            .returning(|_| Ok(Some(create_user("bob"))));

        let req = AddMemberRequest::new("acme", "bob", "member", None).unwrap();
        let err = svc.add_member(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::Conflict(_)));
    }

    #[tokio::test]
    async fn update_member_updates() {
        let mut svc = create_service();
        svc.org_repo.expect_update_member().returning(|_, _, _| {
            let mut m = create_member("bob", OrganizationRole::Admin);
            m.role_description = Some("lead".to_string());
            Ok(Some(m))
        });

        let req =
            UpdateOrganizationMemberRequest::new("acme", Uuid::new_v4(), Some("lead".to_string()))
                .unwrap();
        let resp = svc.update_member(req).await.unwrap();
        assert_eq!(resp.role_description.as_deref(), Some("lead"));
    }

    #[tokio::test]
    async fn list_organizations_maps_page() {
        let mut svc = create_service();
        svc.org_repo.expect_list().returning(|_, _| {
            Ok((
                vec![create_organization("a"), create_organization("b")],
                None,
            ))
        });

        let page = svc
            .list_organizations(ListOrganizationsRequest::new(None, None).unwrap())
            .await
            .unwrap();
        assert_eq!(page.data.len(), 2);
        assert!(page.next_cursor.is_none());
    }

    #[tokio::test]
    async fn list_repositories_returns_repos_from_list_by_owner() {
        let mut svc = create_service();
        svc.org_repo
            .expect_get()
            .returning(|_| Ok(Some(create_organization("acme"))));
        svc.repo_repo
            .expect_list_by_owner()
            .returning(|_, _, _, _| {
                let owner = Uuid::new_v4();
                Ok((
                    vec![
                        create_repository(
                            owner,
                            RepositoryOwnerType::Organization,
                            RepositoryVisibility::Public,
                        ),
                        create_repository(
                            owner,
                            RepositoryOwnerType::Organization,
                            RepositoryVisibility::Private,
                        ),
                    ],
                    None,
                ))
            });

        let req =
            ListOrganizationRepositoriesRequest::new("acme", None, None, Some(Uuid::new_v4()))
                .unwrap();
        let page = svc.list_repositories(req).await.unwrap();
        assert_eq!(page.data.len(), 2);
    }

    #[tokio::test]
    async fn list_repositories_missing_org_is_not_found() {
        let mut svc = create_service();
        svc.org_repo.expect_get().returning(|_| Ok(None));

        let req = ListOrganizationRepositoriesRequest::new("ghost", None, None, None).unwrap();
        let err = svc.list_repositories(req).await.unwrap_err();
        assert!(matches!(err, OrganizationError::NotFound(_)));
    }
}
