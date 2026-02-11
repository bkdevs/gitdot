cfg_modules!("main", {
    mod git_http;
    pub mod legacy_repository;
    mod oauth;
    mod organization;
    mod question;
    mod repository;
    mod user;
    pub use git_http::*;
    pub use oauth::*;
    pub use organization::*;
    pub use question::*;
    pub use repository::*;
    pub use user::*;
});

cfg_modules!("ci", {
    mod dag;
    mod runner;
    mod task;
    pub use dag::*;
    pub use runner::*;
    pub use task::*;
});
