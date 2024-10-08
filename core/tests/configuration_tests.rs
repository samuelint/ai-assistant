use app_test_utils::create_app_container;
use serial_test::serial;

mod app_test_utils;

#[tokio::test]
#[serial]
async fn test_configuration_create_read() {
    let app = create_app_container().await;
    let service = app.configuration_module.get_configuration_service();

    let find_result = service.find("aa").await.unwrap();
    assert!(find_result.is_none());

    let upsert_result = service.upsert("aa", "AAA").await.unwrap();

    assert_eq!(upsert_result.value, "AAA");
    assert_eq!(upsert_result.key, "aa");

    let find_result = service
        .find("aa")
        .await
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(
        find_result.value, "AAA",
        "Value does not exist after being find"
    );
}

#[tokio::test]
#[serial]
async fn test_configuration_upsert() {
    let app = create_app_container().await;
    let service = app.configuration_module.get_configuration_service();

    service.upsert("bb", "BB").await.unwrap();

    let upsert_result = service.upsert("bb", "Hello").await.unwrap();
    assert_eq!(upsert_result.value, "Hello", "Value should be updated");

    let find_result = service
        .find("bb")
        .await
        .unwrap()
        .expect("Configuration item is supposed to exist");

    assert_eq!(find_result.value, "Hello", "Value should be updated");
}
