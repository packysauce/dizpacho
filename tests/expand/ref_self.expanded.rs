struct RefString(String);
impl RefString {
    fn nab_string(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for RefString {
    fn as_ref(&self) -> &str {
        <RefString>::nab_string(self)
    }
}
impl std::ops::Deref for RefString {
    type Target = str;
    fn deref(&self) -> &str {
        <RefString>::nab_string(self)
    }
}
