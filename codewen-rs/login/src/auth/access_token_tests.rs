use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_codewen_access_token("at-example"),
        CodewenAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_codewen_access_token("header.payload.signature"),
        CodewenAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
