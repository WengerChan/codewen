const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum CodewenAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_codewen_access_token(access_token: &str) -> CodewenAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        CodewenAccessToken::PersonalAccessToken(access_token)
    } else {
        CodewenAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
