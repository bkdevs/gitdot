use async_trait::async_trait;

use crate::{
    client::{ImageClient, ImageClientImpl, R2Client, R2ClientImpl},
    dto::{
        AddMemberRequest, CreateOrganizationRequest, GetOrganizationRequest, ListMembersRequest,
        ListOrganizationRepositoriesRequest, OrganizationMemberResponse, OrganizationResponse,
        RepositoryResponse, UpdateOrganizationImageRequest, UpdateOrganizationMemberRequest,
        UpdateOrganizationRequest,
    },
    error::{ConflictError, NotFoundError, OptionNotFoundExt, OrganizationError},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
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
    ) -> Result<Vec<RepositoryResponse>, OrganizationError>;

    async fn list_organizations(&self) -> Result<Vec<OrganizationResponse>, OrganizationError>;

    async fn list_members(
        &self,
        request: ListMembersRequest,
    ) -> Result<Vec<OrganizationMemberResponse>, OrganizationError>;
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

#[crate::instrument_all]
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
        let org = self
            .org_repo
            .get(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;
        let webp_bytes = self.image_client.convert_to_webp(request.bytes).await?;
        let key = format!("orgs/{}.webp", org.id);
        self.r2_client.upload_object(&key, webp_bytes).await?;
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

    async fn list_organizations(&self) -> Result<Vec<OrganizationResponse>, OrganizationError> {
        let orgs = self.org_repo.list().await?;
        Ok(orgs.into_iter().map(|o| o.into()).collect())
    }

    async fn list_repositories(
        &self,
        request: ListOrganizationRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, OrganizationError> {
        let org_name = request.org_name.to_string();
        let org = self
            .org_repo
            .get(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;

        let repositories = self.repo_repo.list_by_owner(&org_name).await?;

        let is_member = match request.viewer_id {
            Some(viewer_id) => self.org_repo.is_member(org.id, viewer_id).await?,
            None => false,
        };
        let repositories = if is_member {
            repositories
        } else {
            repositories.into_iter().filter(|r| r.is_public()).collect()
        };

        Ok(repositories.into_iter().map(|r| r.into()).collect())
    }

    async fn list_members(
        &self,
        request: ListMembersRequest,
    ) -> Result<Vec<OrganizationMemberResponse>, OrganizationError> {
        let org_name = request.org_name.to_string();
        self.org_repo
            .get(&org_name)
            .await?
            .or_not_found("organization", &org_name)?;

        let members = self.org_repo.list_members(&org_name, request.role).await?;
        Ok(members.into_iter().map(|m| m.into()).collect())
    }
}
