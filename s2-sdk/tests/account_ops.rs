mod common;

use assert_matches::assert_matches;
use common::{s2, unique_basin_name};
use s2_sdk::types::*;

#[tokio::test]
async fn create_and_delete_basin() -> Result<(), S2Error> {
    let s2 = s2();
    let basin_name = unique_basin_name();

    let basin_info = s2
        .create_basin(CreateBasinInput::new(basin_name.clone()))
        .await?;

    assert_eq!(basin_info.name, basin_name);

    s2.delete_basin(DeleteBasinInput::new(basin_name)).await?;

    Ok(())
}

#[tokio::test]
async fn delete_nonexistent_basin_errors() -> Result<(), S2Error> {
    let s2 = s2();
    let result = s2
        .delete_basin(DeleteBasinInput::new(unique_basin_name()))
        .await;

    assert_matches!(
        result,
        Err(S2Error::Server(ErrorResponse { code, message: _, .. })) => {
            assert_eq!(code, "basin_not_found")
        }
    );

    Ok(())
}

#[tokio::test]
async fn delete_nonexistent_basin_with_ignore() -> Result<(), S2Error> {
    let s2 = s2();
    let result = s2
        .delete_basin(DeleteBasinInput::new(unique_basin_name()).with_ignore_not_found(true))
        .await;

    assert_matches!(result, Ok(()));

    Ok(())
}
