#[test]
fn test_parse_bearer_token() {
    let input: &'static str = "user1::regular::access_token::e034dff6-1e26-4e69-96cd-05c8f8193b08";
    let expect = crate::routes::token::helpers::UsernameTypeRoleUniq {
        username: String::from("user1"),
        role: String::from("regular"),
        token_type: String::from("access_token"),
        uniq: String::from("e034dff6-1e26-4e69-96cd-05c8f8193b08"),
    };
    assert_eq!(
        crate::routes::token::helpers::parse_bearer_token(input).unwrap(),
        expect
    )
}
