use crate::entity;
use crate::types::Account;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_account_from_model() {
    let model = entity::account::Model {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "testpassword".to_string().into_bytes(),
        date_of_joining: Utc::now().naive_utc(),
        money: 100.0,
        diamonds: 50,
    };

    let account: Account = model.into();
    assert_eq!(account.username(), "testuser");
    assert_eq!(account.email(), "test@example.com");
    assert_eq!(account.password(), vec![36, 50, 98, 36, 49, 50, 36, 68, 121, 46, 107, 88, 114, 52, 54, 68, 66, 77, 110, 110, 108, 79, 48, 116, 76, 117, 69, 111, 117, 51, 51, 103, 104, 104, 70, 68, 118, 66, 121, 101, 117, 113, 86, 69, 107, 84, 115, 113, 86, 109, 57, 72, 119, 77, 83, 72, 116, 80, 114, 113]);
    assert_eq!(account.money(), 100.0);
    assert_eq!(account.diamonds(), 50);
}

#[test]
fn test_account_to_active_model() {
    let account = Account::new(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "testpassword".to_string(),
    );

    let active_model: entity::account::ActiveModel = account.into();
    assert_eq!(active_model.username.unwrap(), "testuser");
    assert_eq!(active_model.email.unwrap(), "test@example.com");
    assert_eq!(active_model.password.unwrap(), vec![36, 50, 98, 36, 49, 50, 36, 90, 111, 116, 113, 48, 121, 99, 107, 104, 48, 52, 116, 48, 80, 65, 121, 56, 119, 76, 50, 114, 46, 86, 102, 46, 116, 108, 77, 73, 117, 52, 100, 70, 101, 78, 52, 84, 116, 75, 109, 108, 81, 117, 105, 78, 116, 70, 99, 52, 79, 69, 116, 101]);
    assert_eq!(active_model.money.unwrap(), 100.0);
    assert_eq!(active_model.diamonds.unwrap(), 50);
}
