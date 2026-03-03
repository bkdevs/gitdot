mod common;

use std::time::Duration;

use assert_matches::assert_matches;
use common::{S2Basin, SharedS2Basin, unique_stream_name, uuid};
use futures::StreamExt;
use s2_sdk::types::*;
use test_context::test_context;

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn create_list_and_delete_stream(basin: &S2Basin) -> Result<(), S2Error> {
    let stream_name = unique_stream_name();

    let stream_info = basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    assert_eq!(stream_info.name, stream_name);

    let page = basin.list_streams(ListStreamsInput::new()).await?;

    assert_eq!(page.values, vec![stream_info]);
    assert!(!page.has_more);

    basin
        .delete_stream(DeleteStreamInput::new(stream_name))
        .await?;

    let page = basin.list_streams(ListStreamsInput::new()).await?;

    assert_matches!(
        page.values.as_slice(),
        [] | [StreamInfo {
            deleted_at: Some(_),
            ..
        }]
    );

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_limit(basin: &S2Basin) -> Result<(), S2Error> {
    let stream_name_1 = unique_stream_name();
    let stream_name_2 = unique_stream_name();
    let stream_name_3 = unique_stream_name();

    let stream_info_1 = basin
        .create_stream(CreateStreamInput::new(stream_name_1.clone()))
        .await?;

    let _stream_info_2 = basin
        .create_stream(CreateStreamInput::new(stream_name_2.clone()))
        .await?;
    let _stream_info_3 = basin
        .create_stream(CreateStreamInput::new(stream_name_3.clone()))
        .await?;

    let page = basin
        .list_streams(ListStreamsInput::new().with_limit(1))
        .await?;

    assert_eq!(page.values, vec![stream_info_1]);
    assert!(page.has_more);

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_prefix(basin: &S2Basin) -> Result<(), S2Error> {
    let stream_name_1: StreamName = "users/eu/0001".parse().expect("valid stream name");
    let stream_name_2: StreamName = "users/ca/0001".parse().expect("valid stream name");
    let stream_name_3: StreamName = "users/ca/0002".parse().expect("valid stream name");

    let _stream_info_1 = basin
        .create_stream(CreateStreamInput::new(stream_name_1.clone()))
        .await?;
    let stream_info_2 = basin
        .create_stream(CreateStreamInput::new(stream_name_2.clone()))
        .await?;
    let stream_info_3 = basin
        .create_stream(CreateStreamInput::new(stream_name_3.clone()))
        .await?;

    let page = basin
        .list_streams(
            ListStreamsInput::new().with_prefix("users/ca/".parse().expect("valid prefix")),
        )
        .await?;

    assert_eq!(page.values, vec![stream_info_2, stream_info_3]);
    assert!(!page.has_more);

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_start_after(basin: &S2Basin) -> Result<(), S2Error> {
    let stream_name_1 = unique_stream_name();
    let stream_name_2 = unique_stream_name();

    let _stream_info_1 = basin
        .create_stream(CreateStreamInput::new(stream_name_1.clone()))
        .await?;
    let stream_info_2 = basin
        .create_stream(CreateStreamInput::new(stream_name_2.clone()))
        .await?;

    let page = basin
        .list_streams(
            ListStreamsInput::new()
                .with_start_after(stream_name_1.parse().expect("valid start after")),
        )
        .await?;

    assert_eq!(page.values, vec![stream_info_2]);
    assert!(!page.has_more);

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_start_after_returns_empty_page(basin: &S2Basin) -> Result<(), S2Error> {
    let stream_name_1 = unique_stream_name();
    let stream_name_2 = unique_stream_name();

    basin
        .create_stream(CreateStreamInput::new(stream_name_1.clone()))
        .await?;
    basin
        .create_stream(CreateStreamInput::new(stream_name_2.clone()))
        .await?;

    let page = basin
        .list_streams(
            ListStreamsInput::new()
                .with_start_after(stream_name_2.parse().expect("valid start after")),
        )
        .await?;

    assert_eq!(page.values.len(), 0);
    assert!(!page.has_more);

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_start_after_less_than_prefix_errors(
    basin: &S2Basin,
) -> Result<(), S2Error> {
    let prefix = uuid();
    let stream_name_1: StreamName = format!("{}-a-a", prefix)
        .parse()
        .expect("valid stream name");
    let stream_name_2: StreamName = format!("{}-a-b", prefix)
        .parse()
        .expect("valid stream name");
    let stream_name_3: StreamName = format!("{}-b-a", prefix)
        .parse()
        .expect("valid stream name");

    basin
        .create_stream(CreateStreamInput::new(stream_name_1.clone()))
        .await?;
    basin
        .create_stream(CreateStreamInput::new(stream_name_2.clone()))
        .await?;
    basin
        .create_stream(CreateStreamInput::new(stream_name_3.clone()))
        .await?;

    let result = basin
        .list_streams(
            ListStreamsInput::new()
                .with_prefix(format!("{}-b", prefix).parse().expect("valid prefix"))
                .with_start_after(format!("{}-a", prefix).parse().expect("valid start after")),
        )
        .await;

    assert_matches!(
        result,
        Err(S2Error::Server(ErrorResponse { code, message, .. })) => {
            assert_eq!(code, "invalid");
            assert_eq!(message, "`start_after` must be greater than or equal to the `prefix`");
        }
    );

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn delete_nonexistent_stream_errors(basin: &S2Basin) -> Result<(), S2Error> {
    let result = basin
        .delete_stream(DeleteStreamInput::new(unique_stream_name()))
        .await;

    assert_matches!(
        result,
        Err(S2Error::Server(ErrorResponse {code, message: _, ..})) => {
            assert_eq!(code, "stream_not_found")
        }
    );

    Ok(())
}

#[test_context(S2Basin)]
#[tokio_shared_rt::test(shared)]
async fn delete_nonexistent_stream_with_ignore(basin: &S2Basin) -> Result<(), S2Error> {
    let result = basin
        .delete_stream(DeleteStreamInput::new(unique_stream_name()).with_ignore_not_found(true))
        .await;

    assert_matches!(result, Ok(()));

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_limit_zero(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("limit0-{}", uuid());
    let stream_name: StreamName = format!("{}-0001", prefix)
        .parse()
        .expect("valid stream name");

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    let page = basin
        .list_streams(
            ListStreamsInput::new()
                .with_prefix(prefix.parse().expect("valid prefix"))
                .with_limit(0),
        )
        .await?;

    assert!(page.values.iter().any(|info| info.name == stream_name));
    assert!(page.values.len() <= 1000);

    basin
        .delete_stream(DeleteStreamInput::new(stream_name))
        .await?;

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_limit_over_max(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("limitmax-{}", uuid());
    let stream_name: StreamName = format!("{}-0001", prefix)
        .parse()
        .expect("valid stream name");

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    let page = basin
        .list_streams(
            ListStreamsInput::new()
                .with_prefix(prefix.parse().expect("valid prefix"))
                .with_limit(1500),
        )
        .await?;

    assert!(page.values.iter().any(|info| info.name == stream_name));
    assert!(page.values.len() <= 1000);

    basin
        .delete_stream(DeleteStreamInput::new(stream_name))
        .await?;

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_with_pagination(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("page-{}", uuid());
    let stream_names: Vec<StreamName> = (0..3)
        .map(|idx| {
            format!("{}-{:04}", prefix, idx)
                .parse()
                .expect("valid stream name")
        })
        .collect();

    for name in &stream_names {
        basin
            .create_stream(CreateStreamInput::new(name.clone()))
            .await?;
    }

    let page_1 = basin
        .list_streams(
            ListStreamsInput::new()
                .with_prefix(prefix.parse().expect("valid prefix"))
                .with_limit(2),
        )
        .await?;

    assert!(!page_1.values.is_empty());

    let last_name = page_1
        .values
        .last()
        .expect("page should have value")
        .name
        .clone();

    let page_2 = basin
        .list_streams(
            ListStreamsInput::new()
                .with_prefix(prefix.parse().expect("valid prefix"))
                .with_start_after(last_name.clone().into())
                .with_limit(2),
        )
        .await?;

    assert!(
        page_2
            .values
            .iter()
            .all(|info| info.name.as_ref() > last_name.as_ref())
    );

    let mut listed: Vec<String> = page_1
        .values
        .into_iter()
        .chain(page_2.values.into_iter())
        .map(|info| info.name.to_string())
        .collect();
    listed.sort();
    let mut expected: Vec<String> = stream_names.iter().map(|name| name.to_string()).collect();
    expected.sort();
    assert_eq!(listed, expected);

    for name in stream_names {
        let _ = basin.delete_stream(DeleteStreamInput::new(name)).await;
    }

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_streams_returns_lexicographic_order(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("order-{}", uuid());
    let stream_names: Vec<StreamName> = (1..=3)
        .map(|idx| {
            format!("{}-{:04}", prefix, idx)
                .parse()
                .expect("valid stream name")
        })
        .collect();

    for name in &stream_names {
        basin
            .create_stream(CreateStreamInput::new(name.clone()))
            .await?;
    }

    let page = basin
        .list_streams(ListStreamsInput::new().with_prefix(prefix.parse().expect("valid prefix")))
        .await?;

    let listed: Vec<StreamName> = page.values.into_iter().map(|info| info.name).collect();
    assert_eq!(listed, stream_names);

    for name in stream_names {
        let _ = basin.delete_stream(DeleteStreamInput::new(name)).await;
    }

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_all_streams_iterates_with_prefix(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("iter-{}", uuid());
    let stream_names: Vec<StreamName> = (1..=3)
        .map(|idx| {
            format!("{}-{:04}", prefix, idx)
                .parse()
                .expect("valid stream name")
        })
        .collect();

    for name in &stream_names {
        basin
            .create_stream(CreateStreamInput::new(name.clone()))
            .await?;
    }

    let mut listed = Vec::new();
    let mut stream = basin.list_all_streams(
        ListAllStreamsInput::new().with_prefix(prefix.parse().expect("valid prefix")),
    );
    while let Some(info) = stream.next().await {
        listed.push(info?.name);
    }

    assert_eq!(listed, stream_names);

    for name in stream_names {
        let _ = basin.delete_stream(DeleteStreamInput::new(name)).await;
    }

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn list_all_streams_include_deleted(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let prefix = format!("iter-del-{}", uuid());
    let stream_name: StreamName = format!("{}-0001", prefix)
        .parse()
        .expect("valid stream name");

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;
    basin
        .delete_stream(DeleteStreamInput::new(stream_name.clone()))
        .await?;

    let mut stream = basin.list_all_streams(
        ListAllStreamsInput::new()
            .with_prefix(prefix.parse().expect("valid prefix"))
            .with_include_deleted(true),
    );

    let mut found = None;
    while let Some(info) = stream.next().await {
        let info = info?;
        if info.name == stream_name {
            found = Some(info);
            break;
        }
    }

    if let Some(info) = found {
        assert!(info.deleted_at.is_some());
    }

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn create_stream_invalid_retention_age_zero(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let stream_name = unique_stream_name();
    let config = StreamConfig::new().with_retention_policy(RetentionPolicy::Age(0));

    let result = basin
        .create_stream(CreateStreamInput::new(stream_name).with_config(config))
        .await;

    assert_matches!(
        result,
        Err(S2Error::Server(ErrorResponse { code, .. })) => {
            assert_eq!(code, "invalid");
        }
    );

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn create_stream_duplicate_name_errors(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let stream_name = unique_stream_name();

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    let result = basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await;

    assert_matches!(
        result,
        Err(S2Error::Server(ErrorResponse { code, .. })) => {
            assert_eq!(code, "resource_already_exists");
        }
    );

    basin
        .delete_stream(DeleteStreamInput::new(stream_name))
        .await?;

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn delete_stream_already_deleting_is_idempotent(
    basin: &SharedS2Basin,
) -> Result<(), S2Error> {
    let stream_name = unique_stream_name();

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    basin
        .delete_stream(DeleteStreamInput::new(stream_name.clone()))
        .await?;

    let result = basin
        .delete_stream(DeleteStreamInput::new(stream_name.clone()))
        .await;

    match result {
        Ok(()) => {}
        Err(S2Error::Server(ErrorResponse { code, .. })) if code == "stream_not_found" => {}
        Err(err) => return Err(err),
    }

    Ok(())
}

#[test_context(SharedS2Basin)]
#[tokio_shared_rt::test(shared)]
async fn deleted_stream_has_deleted_at_when_listed(basin: &SharedS2Basin) -> Result<(), S2Error> {
    let stream_name = unique_stream_name();

    basin
        .create_stream(CreateStreamInput::new(stream_name.clone()))
        .await?;

    basin
        .delete_stream(DeleteStreamInput::new(stream_name.clone()))
        .await?;

    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let page = basin
            .list_streams(ListStreamsInput::new().with_prefix(stream_name.clone().into()))
            .await?;

        let mut found = false;
        for info in page.values {
            if info.name == stream_name {
                found = true;
                if info.deleted_at.is_some() {
                    return Ok(());
                }
            }
        }

        if !found {
            return Ok(());
        }
        if std::time::Instant::now() >= deadline {
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    panic!("deleted stream still listed without deleted_at after timeout");
}
