use crate::{endpoint::Endpoint, resource::migration::MigrationResource};

pub struct ListMigrations;

impl Endpoint for ListMigrations {
    const PATH: &'static str = "/migrations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ();
    type Response = ListMigrationsResponse;
}

pub type ListMigrationsResponse = Vec<MigrationResource>;
