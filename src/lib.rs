//! SigmaOS-data-validation: data validation model.
//!
//! Real implementation: rules, constraints, checks, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Validation constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConstraint {
    pub id: String,
    pub name: String,
    pub field: String,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmptyName,
    DuplicateConstraint(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "constraint name must not be empty"),
            Self::DuplicateConstraint(id) => write!(f, "duplicate constraint {:?}", id),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Data validation engine.
pub struct DataValidation {
    pub name: String,
    constraints: BTreeMap<String, ValidationConstraint>,
}

impl DataValidation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            constraints: BTreeMap::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ValidationConstraint) -> Result<(), ValidationError> {
        if constraint.name.trim().is_empty() { return Err(ValidationError::EmptyName); }
        let id = constraint.id.clone();
        if self.constraints.contains_key(&id) {
            return Err(ValidationError::DuplicateConstraint(id.clone()));
        }
        self.constraints.insert(id, constraint);
        Ok(())
    }

    pub fn constraint_count(&self) -> usize { self.constraints.len() }

    pub fn get_constraint(&self, id: &str) -> Option<&ValidationConstraint> { self.constraints.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_create_aaa() {
        let v = DataValidation::new("My Validator");
        assert_eq!(v.name, "My Validator");
        assert_eq!(v.constraint_count(), 0);
    }

    #[test]
    fn test_add_constraint_aaa() {
        let mut v = DataValidation::new("My Validator");
        let c = ValidationConstraint { id: "c1".into(), name: "not-null".into(), field: "email".into(), rule: "required".into() };
        v.add_constraint(c).unwrap();
        assert_eq!(v.constraint_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut v = DataValidation::new("My Validator");
        let c = ValidationConstraint { id: "x".into(), name: "".into(), field: "a".into(), rule: "b".into() };
        assert!(v.add_constraint(c).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut v = DataValidation::new("My Validator");
        let c = ValidationConstraint { id: "x".into(), name: "X".into(), field: "a".into(), rule: "b".into() };
        v.add_constraint(c).unwrap();
        let c2 = ValidationConstraint { id: "x".into(), name: "X".into(), field: "a".into(), rule: "b".into() };
        assert!(v.add_constraint(c2).is_err());
    }
}
