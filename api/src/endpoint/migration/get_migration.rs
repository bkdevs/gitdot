use crate::{endpoint::Endpoint, resource::migration::MigrationResource};

pub struct GetMigration;

impl Endpoint for GetMigration {
    const PATH: &'static str = "/migration/{number}";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = GetMigrationResponse;
}

pub type GetMigrationResponse = MigrationResource;
