//! All cop implementations organized by category.

pub mod layout;
pub mod lint;
pub mod naming;
pub mod style;

use crate::cop::Cop;

/// Returns all implemented cops as trait objects.
pub fn all_cops() -> Vec<Box<dyn Cop>> {
    vec![
        // Layout cops
        Box::new(layout::TrailingWhitespace),
        Box::new(layout::TrailingEmptyLines),
        Box::new(layout::LeadingEmptyLines),
        Box::new(layout::EndOfLine),
        Box::new(layout::IndentationStyle),
        Box::new(layout::IndentationWidth),
        Box::new(layout::SpaceAfterComma),
        Box::new(layout::SpaceAroundOperators),
        Box::new(layout::EmptyLineBetweenDefs),
        Box::new(layout::SpaceInsideParens),
        // Style cops
        Box::new(style::FrozenStringLiteralComment),
        Box::new(style::StringLiterals),
        Box::new(style::NegatedIf),
        Box::new(style::RedundantReturn),
        Box::new(style::EmptyMethod),
        // Lint cops
        Box::new(lint::Debugger::new()),
        Box::new(lint::LiteralInCondition::new()),
        Box::new(lint::DuplicateMethods::new()),
        // Naming cops
        Box::new(naming::MethodName),
        Box::new(naming::VariableName),
        Box::new(naming::ConstantName),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_cops_returns_instances() {
        let cops = all_cops();
        assert_eq!(cops.len(), 21); // 10 layout + 5 style + 3 lint + 3 naming
    }

    #[test]
    fn test_all_cops_have_unique_names() {
        let cops = all_cops();
        let names: Vec<&str> = cops.iter().map(|c| c.name()).collect();
        let mut unique_names = names.clone();
        unique_names.sort();
        unique_names.dedup();
        assert_eq!(names.len(), unique_names.len(), "Duplicate cop names found");
    }

    #[test]
    fn test_cop_names_follow_convention() {
        let cops = all_cops();
        for cop in cops {
            let name = cop.name();
            assert!(
                name.contains('/'),
                "Cop name '{}' should contain category separator '/'",
                name
            );
            let parts: Vec<&str> = name.split('/').collect();
            assert_eq!(
                parts.len(),
                2,
                "Cop name '{}' should have exactly one '/'",
                name
            );
        }
    }
}
