pub fn validate_property<T, F>(property: T, validator: F, error_message: &str) -> Result<(), String>
where
    F: Fn(&T) -> bool, // El validador es una closure/función: (T) -> bool
{
    if !validator(&property) {
        // En Rust no solemos hacer "throw", devolvemos un Result::Err
        return Err(error_message.to_string());
    }
    Ok(())
}
