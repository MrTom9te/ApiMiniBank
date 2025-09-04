use std::sync::{OnceLock, mpsc::RecvTimeoutError};

use actix_web::cookie::time::format_description::parse_strftime_owned;
use regex::Regex;
use serde::Serialize;

use crate::models::error::UserError;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::<Regex>::new();

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap())
}

pub struct UserValidator;

impl UserValidator {
    pub fn validate_name(name: &str) -> Result<String, UserError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(UserError::InvalidName);
        }

        let name_parts: Vec<&str> = trimmed.split_whitespace().collect();
        if name_parts.len() < 2 {
            return Err(UserError::InvalidName);
        }

        if name_parts.iter().any(|part| part.len() < 2) {
            return Err(UserError::InvalidName);
        };

        Ok(trimmed.to_string())
    }

    pub fn validate_email(email: &str) -> Result<String, UserError> {
        let trimmed = email.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err(UserError::InvalidEmail("Email can't be  empty".into()));
        }

        if !get_email_regex().is_match(&trimmed) {
            return Err(UserError::InvalidEmail("Not a valid email".into()));
        }

        Ok(trimmed)
    }
    pub fn validate_password(password: &str) -> Result<(), UserError> {
        Self::validade_passoword_with_criteria(password, PasswordCriteria::default())
    }

    pub fn validade_passoword_with_criteria(
        password: &str,
        criteria: PasswordCriteria,
    ) -> Result<(), UserError> {
        if password.len() < criteria.min_length {
            return Err(UserError::WeakPassword);
        }

        if criteria.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(UserError::WeakPassword);
        }

        if criteria.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(UserError::WeakPassword);
        }

        if criteria.require_number && !password.chars().any(|c| c.is_numeric()) {
            return Err(UserError::WeakPassword);
        }

        if criteria.require_special_char {
            let has_espacial = password
                .chars()
                .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
            if !has_espacial {
                return Err(UserError::WeakPassword);
            }
        }

        if password
            .chars()
            .any(|c| criteria.forbidden_chars.contains(&c))
        {
            return Err(UserError::WeakPassword);
        }

        Ok(())
    }

    pub fn validade_user_data(
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<ValidatedUserData, UserError> {
        let valideted_name = Self::validate_name(name)?;
        let validated_email = Self::validate_email(email)?;

        Self::validate_password(password)?;
        Ok(ValidatedUserData {
            name: valideted_name,
            email: validated_email,
            password: password.into(),
        })
    }
}

pub struct ValidatedUserData {
    pub name: String,
    pub email: String,
    pub password: String,
}

// Critérios de senha personalizáveis
#[derive(Debug, Clone)]
pub struct PasswordCriteria {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_special_char: bool,
    pub forbidden_chars: Vec<char>,
}

impl Default for PasswordCriteria {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: false,
            require_number: true,
            require_special_char: false,
            forbidden_chars: vec![' ', '\t', '\n'], //sem espaços em branco
        }
    }
}

pub struct LoginValidator;
impl LoginValidator {
    pub fn validate_login_data(
        email: &str,
        password: &str,
    ) -> Result<ValidatedLoginData, UserError> {
        let validated_email = UserValidator::validate_email(email)?;
        if password.is_empty() {
            return Err(UserError::InvalidCredentials);
        }

        Ok(ValidatedLoginData {
            email: validated_email,
            password: password.into(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ValidatedLoginData {
    pub email: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        assert!(UserValidator::validate_name("João Silva").is_ok());
        assert!(UserValidator::validate_name("Maria José Santos").is_ok());
        assert!(UserValidator::validate_name("João").is_err());
        assert!(UserValidator::validate_name("  ").is_err());
        assert!(UserValidator::validate_name("A B").is_err()); // Nomes muito curtos
    }

    #[test]
    fn test_validate_email() {
        assert!(UserValidator::validate_email("test@example.com").is_ok());
        assert!(UserValidator::validate_email("  TEST@EXAMPLE.COM  ").is_ok());
        assert!(UserValidator::validate_email("invalid-email").is_err());
        assert!(UserValidator::validate_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(UserValidator::validate_password("Password123").is_ok());
        assert!(UserValidator::validate_password("weak").is_err());
        assert!(UserValidator::validate_password("NoNumber").is_err());
        assert!(UserValidator::validate_password("nonumber123").is_err());
    }
}
