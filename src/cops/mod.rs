//! All cop implementations organized by category.

pub mod layout;
pub mod layout_extra;
pub mod style;
pub mod style_extra1;
pub mod style_extra2;
pub mod style_extra3;
pub mod lint;
pub mod lint_extra;
pub mod naming;
pub mod naming_extra;
pub mod metrics;

use crate::cop::Cop;

/// Returns all implemented cops as trait objects.
pub fn all_cops() -> Vec<Box<dyn Cop>> {
    let mut cops: Vec<Box<dyn Cop>> = vec![
        // ==================== Layout (original 10) ====================
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
        // ==================== Layout extra (90) ====================
        Box::new(layout_extra::LineLength),
        Box::new(layout_extra::EmptyComment),
        Box::new(layout_extra::EmptyLines),
        Box::new(layout_extra::LeadingCommentSpace),
        Box::new(layout_extra::ExtraSpacing),
        Box::new(layout_extra::SpaceAfterColon),
        Box::new(layout_extra::SpaceAfterMethodName),
        Box::new(layout_extra::SpaceAfterNot),
        Box::new(layout_extra::SpaceAfterSemicolon),
        Box::new(layout_extra::SpaceBeforeComma),
        Box::new(layout_extra::SpaceBeforeComment),
        Box::new(layout_extra::SpaceBeforeSemicolon),
        Box::new(layout_extra::SpaceInsideArrayLiteralBrackets),
        Box::new(layout_extra::SpaceInsideHashLiteralBraces),
        Box::new(layout_extra::SpaceInsideRangeLiteral),
        Box::new(layout_extra::SpaceInsideReferenceBrackets),
        Box::new(layout_extra::SpaceInsideStringInterpolation),
        Box::new(layout_extra::SpaceAroundEqualsInParameterDefault),
        Box::new(layout_extra::SpaceAroundKeyword),
        Box::new(layout_extra::SpaceBeforeBlockBraces),
        Box::new(layout_extra::SpaceInsideBlockBraces),
        Box::new(layout_extra::SpaceBeforeFirstArg),
        Box::new(layout_extra::SpaceBeforeBrackets),
        Box::new(layout_extra::SpaceInLambdaLiteral),
        Box::new(layout_extra::InitialIndentation),
        Box::new(layout_extra::CommentIndentation),
        Box::new(layout_extra::IndentationConsistency),
        Box::new(layout_extra::DotPosition),
        Box::new(layout_extra::ConditionPosition),
        Box::new(layout_extra::EmptyLineAfterMagicComment),
        Box::new(layout_extra::EmptyLineAfterGuardClause),
        Box::new(layout_extra::EmptyLinesAroundClassBody),
        Box::new(layout_extra::EmptyLinesAroundModuleBody),
        Box::new(layout_extra::EmptyLinesAroundMethodBody),
        Box::new(layout_extra::EmptyLinesAroundBlockBody),
        Box::new(layout_extra::EmptyLinesAroundAccessModifier),
        Box::new(layout_extra::SingleLineBlockChain),
        Box::new(layout_extra::BlockEndNewline),
        Box::new(layout_extra::CaseIndentation),
        Box::new(layout_extra::EndAlignment),
        Box::new(layout_extra::DefEndAlignment),
        Box::new(layout_extra::ElseAlignment),
        Box::new(layout_extra::RescueEnsureAlignment),
        Box::new(layout_extra::AccessModifierIndentation),
        Box::new(layout_extra::ClosingParenthesisIndentation),
        Box::new(layout_extra::MultilineMethodCallIndentation),
        Box::new(layout_extra::MultilineOperationIndentation),
        Box::new(layout_extra::ArgumentAlignment),
        Box::new(layout_extra::ArrayAlignment),
        Box::new(layout_extra::HashAlignment),
        Box::new(layout_extra::ParameterAlignment),
        Box::new(layout_extra::AssignmentIndentation),
        Box::new(layout_extra::FirstArgumentIndentation),
        Box::new(layout_extra::FirstArrayElementIndentation),
        Box::new(layout_extra::FirstHashElementIndentation),
        Box::new(layout_extra::HeredocIndentation),
        Box::new(layout_extra::LineContinuationSpacing),
        Box::new(layout_extra::LineContinuationLeadingSpace),
        Box::new(layout_extra::MultilineArrayBraceLayout),
        Box::new(layout_extra::MultilineHashBraceLayout),
        Box::new(layout_extra::MultilineBlockLayout),
        Box::new(layout_extra::MultilineMethodCallBraceLayout),
        Box::new(layout_extra::MultilineArrayLineBreaks),
        Box::new(layout_extra::MultilineHashKeyLineBreaks),
        Box::new(layout_extra::MultilineMethodArgumentLineBreaks),
        Box::new(layout_extra::MultilineMethodParameterLineBreaks),
        Box::new(layout_extra::FirstMethodArgumentLineBreak),
        Box::new(layout_extra::FirstMethodParameterLineBreak),
        Box::new(layout_extra::FirstArrayElementLineBreak),
        Box::new(layout_extra::FirstHashElementLineBreak),
        Box::new(layout_extra::MultilineAssignmentLayout),
        Box::new(layout_extra::MultilineMethodDefinitionBraceLayout),
        Box::new(layout_extra::RedundantLineBreak),
        Box::new(layout_extra::ClassStructure),
        Box::new(layout_extra::LineEndStringConcatenationIndentation),
        Box::new(layout_extra::ClosingHeredocIndentation),
        Box::new(layout_extra::BeginEndAlignment),
        Box::new(layout_extra::BlockAlignment),
        Box::new(layout_extra::EmptyLineAfterMultilineCondition),
        Box::new(layout_extra::EmptyLinesAfterModuleInclusion),
        Box::new(layout_extra::EmptyLinesAroundAttributeAccessor),
        Box::new(layout_extra::EmptyLinesAroundBeginBody),
        Box::new(layout_extra::EmptyLinesAroundExceptionHandlingKeywords),
        Box::new(layout_extra::EmptyLinesAroundArguments),
        Box::new(layout_extra::SpaceAroundMethodCallOperator),
        Box::new(layout_extra::SpaceInsideArrayPercentLiteral),
        Box::new(layout_extra::SpaceInsidePercentLiteralDelimiters),
        Box::new(layout_extra::HeredocArgumentClosingParenthesis),
        Box::new(layout_extra::FirstParameterIndentation),
        Box::new(layout_extra::ErrorA),
        // ==================== Style (original 5) ====================
        Box::new(style::FrozenStringLiteralComment),
        Box::new(style::StringLiterals),
        Box::new(style::NegatedIf),
        Box::new(style::RedundantReturn),
        Box::new(style::EmptyMethod),
        // ==================== Lint (original 3) ====================
        Box::new(lint::Debugger::new()),
        Box::new(lint::LiteralInCondition::new()),
        Box::new(lint::DuplicateMethods::new()),
        // ==================== Naming (original 3) ====================
        Box::new(naming::MethodName),
        Box::new(naming::VariableName),
        Box::new(naming::ConstantName),
    ];

    // Add Style extra cops
    cops.append(&mut style_extra1::all_style_extra1_cops());
    cops.append(&mut style_extra2::all_style_extra2_cops());
    cops.append(&mut style_extra3::all_style_extra3_cops());
    // Add Lint extra cops
    cops.append(&mut lint_extra::all_lint_extra_cops());
    // Add Naming extra cops
    cops.append(&mut naming_extra::all_naming_extra_cops());
    // Add Metrics + Security cops
    cops.append(&mut metrics::all_metrics_cops());

    cops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_cops_returns_many() {
        let cops = all_cops();
        assert!(cops.len() > 100, "Expected 100+ cops, got {}", cops.len());
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
        for cop in &cops {
            let name = cop.name();
            assert!(
                name.contains('/'),
                "Cop name '{}' should contain category separator '/'",
                name
            );
        }
    }
}
