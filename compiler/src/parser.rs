use crate::tokenizer::Token;

pub struct TokenReader {
    pub tokens: Vec<Token>,
}

trait Parsable {
    fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
    where
        Self: Sized;
}

pub mod elements {

    use super::{Parsable, TokenReader};
    use crate::tokenizer::{KeywordType, Token};

    pub struct Keyword(pub KeywordType);
    impl Parsable for Keyword {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::Keyword(keyword) => Some((Keyword(keyword), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct Symbol(pub char);
    impl Parsable for Symbol {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::Symbol(symbol) => Some((Symbol(symbol), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct IntegerConstant(pub i64);
    impl Parsable for IntegerConstant {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::IntConst(integer) => Some((IntegerConstant(integer), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct StringConstant(String);
    impl Parsable for StringConstant {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::StringConst(string) => Some((StringConstant(string.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct Identifier(String);
    impl Parsable for Identifier {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((Identifier(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    pub fn try_parse_symbol(reader: &TokenReader, idx: usize, symbol: char) -> Option<usize> {
        if let Token::Symbol(s) = &reader.tokens[idx] {
            if *s == symbol {
                return Some(idx + 1);
            }
        }
        None
    }
}

mod structures {
    use crate::tokenizer::{KeywordType, Token};

    use super::{
        elements::{self, try_parse_symbol},
        statements, Parsable, TokenReader,
    };

    pub struct Class {
        pub class_name: ClassName,
        pub class_var_dec: Vec<ClassVarDec>,
        pub subroutine_dec: Vec<SubroutineDec>,
    }
    impl Parsable for Class {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let mut p = idx;
            let _class_keyword = elements::Keyword::try_parse(reader, p)?;
            if _class_keyword.0 .0 != KeywordType::CLASS {
                return None;
            }
            p = _class_keyword.1;
            let _class_name = ClassName::try_parse(reader, p)?;
            p = _class_name.1;
            p = try_parse_symbol(reader, p, '{')?;
            let mut class_var_dec = vec![];
            let mut subroutine_dec = vec![];
            loop {
                let parse_class_var_dec = ClassVarDec::try_parse(reader, p);
                if parse_class_var_dec.is_none() {
                    break;
                }
                let parse_class_var_dec = parse_class_var_dec.unwrap();
                p = parse_class_var_dec.1;
                class_var_dec.push(parse_class_var_dec.0);
            }
            loop {
                let parse_subroutine_dec = SubroutineDec::try_parse(reader, p);
                if parse_subroutine_dec.is_none() {
                    break;
                }
                let parse_subroutine_dec = parse_subroutine_dec.unwrap();
                p = parse_subroutine_dec.1;
                subroutine_dec.push(parse_subroutine_dec.0);
            }
            p = try_parse_symbol(reader, p, '}')?;
            Some((
                Class {
                    class_name: _class_name.0,
                    class_var_dec: class_var_dec,
                    subroutine_dec: subroutine_dec,
                },
                p,
            ))
        }
    }

    pub enum ClassVarDecType {
        STATIC,
        FIELD,
    }
    impl Parsable for ClassVarDecType {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let keyword = elements::Keyword::try_parse(reader, idx)?;
            let in_keyword = keyword.0 .0;
            match in_keyword {
                KeywordType::STATIC => Some((ClassVarDecType::STATIC, keyword.1)),
                KeywordType::FIELD => Some((ClassVarDecType::FIELD, keyword.1)),
                _ => None,
            }
        }
    }

    pub enum VarTypeEnum {
        INT,
        CHAR,
        BOOLEAN,
        CLASSNAME(ClassName),
    }
    pub struct VarType(pub VarTypeEnum);
    impl Parsable for VarType {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Keyword(keyword) => match keyword {
                    KeywordType::INT => Some((VarType(VarTypeEnum::INT), idx + 1)),
                    KeywordType::CHAR => Some((VarType(VarTypeEnum::CHAR), idx + 1)),
                    KeywordType::BOOLEAN => Some((VarType(VarTypeEnum::BOOLEAN), idx + 1)),
                    _ => None,
                },
                Token::Identifier(_) => {
                    ClassName::try_parse(reader, idx).map(|(class_name, new_idx)| {
                        (VarType(VarTypeEnum::CLASSNAME(class_name)), new_idx)
                    })
                }
                _ => None,
            }
        }
    }

    pub struct ClassName(pub String);
    impl Parsable for ClassName {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((ClassName(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct SubroutineName(pub String);
    impl Parsable for SubroutineName {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => {
                    Some((SubroutineName(identifier.clone()), idx + 1))
                }
                _ => None,
            }
        }
    }

    pub struct VarName(pub String);
    impl Parsable for VarName {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((VarName(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    pub struct VarDec {
        pub var_type: VarType,
        pub var_names: Vec<VarName>,
    }
    impl Parsable for VarDec {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let _var_keyword = elements::Keyword::try_parse(reader, p)?;
            if _var_keyword.0 .0 != KeywordType::VAR {
                return None;
            }
            p = _var_keyword.1;
            let _var_type = VarType::try_parse(reader, p)?;
            p = _var_type.1;
            let _var_name = VarName::try_parse(reader, p)?;
            p = _var_name.1;
            let mut var_names = vec![_var_name.0];
            loop {
                let sep = try_parse_symbol(reader, p, ',');
                if sep.is_none() {
                    break;
                }
                p = sep.unwrap();
                let var_name = VarName::try_parse(reader, p)?;
                var_names.push(var_name.0);
                p = var_name.1;
            }
            p = try_parse_symbol(reader, p, ';')?;
            Some((
                VarDec {
                    var_type: _var_type.0,
                    var_names: var_names,
                },
                p,
            ))
        }
    }

    pub struct ClassVarDec {
        pub var_dec_type: ClassVarDecType,
        pub var_type: VarType,
        pub var_names: Vec<VarName>,
    }
    impl Parsable for ClassVarDec {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let _var_dec_type = ClassVarDecType::try_parse(reader, p)?;
            p = _var_dec_type.1;
            let _var_type = VarType::try_parse(reader, p)?;
            p = _var_type.1;
            let _var_name = VarName::try_parse(reader, p)?;
            p = _var_name.1;
            let mut var_names = vec![_var_name.0];
            loop {
                let separator = try_parse_symbol(reader, p, ',');
                if separator.is_none() {
                    break;
                }
                let separator = separator.unwrap();
                p = separator;
                let var_name = VarName::try_parse(reader, p)?;
                var_names.push(var_name.0);
                p = var_name.1;
            }
            p = try_parse_symbol(reader, p, ';')?;
            Some((
                ClassVarDec {
                    var_dec_type: _var_dec_type.0,
                    var_type: _var_type.0,
                    var_names: var_names,
                },
                p,
            ))
        }
    }

    pub struct ParameterList {
        pub parameters: Vec<(VarType, VarName)>,
    }
    impl Parsable for ParameterList {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            p = try_parse_symbol(reader, p, '(')?;
            let try_right_parentness = try_parse_symbol(reader, p, ')');
            if try_right_parentness.is_some() {
                return Some((
                    ParameterList { parameters: vec![] },
                    try_right_parentness.unwrap(),
                ));
            }
            let mut parameters = vec![];
            let _var_type = VarType::try_parse(reader, p)?;
            p = _var_type.1;
            let _var_name = VarName::try_parse(reader, p)?;
            p = _var_name.1;
            parameters.push((_var_type.0, _var_name.0));
            loop {
                let separator = try_parse_symbol(reader, p, ',');
                if separator.is_none() {
                    break;
                }
                p = separator.unwrap();
                let _var_type = VarType::try_parse(reader, p)?;
                p = _var_type.1;
                let _var_name = VarName::try_parse(reader, p)?;
                p = _var_name.1;
                parameters.push((_var_type.0, _var_name.0));
            }
            try_parse_symbol(reader, p, ')')?;
            Some((
                ParameterList {
                    parameters: parameters,
                },
                p,
            ))
        }
    }

    pub struct SubroutineBody {
        pub var_decs: Vec<VarDec>,
        pub statements: statements::Statements,
    }
    impl Parsable for SubroutineBody {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut var_decs = vec![];
            let mut p = idx;
            loop {
                let parse_var_dec = VarDec::try_parse(reader, p);
                if parse_var_dec.is_none() {
                    break;
                }
                let parse_var_dec = parse_var_dec.unwrap();
                p = parse_var_dec.1;
                var_decs.push(parse_var_dec.0);
            }
            let _statements = statements::Statements::try_parse(reader, p)?;
            p = _statements.1;
            Some((
                SubroutineBody {
                    var_decs: var_decs,
                    statements: _statements.0,
                },
                p,
            ))
        }
    }

    pub enum SubroutineType {
        CONSTRUCTOR,
        FUNCTION,
        METHOD,
    }
    impl Parsable for SubroutineType {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let keyword = elements::Keyword::try_parse(reader, idx)?;
            let in_keyword = keyword.0 .0;
            match in_keyword {
                KeywordType::CONSTRUCTOR => Some((SubroutineType::CONSTRUCTOR, keyword.1)),
                KeywordType::FUNCTION => Some((SubroutineType::FUNCTION, keyword.1)),
                KeywordType::METHOD => Some((SubroutineType::METHOD, keyword.1)),
                _ => None,
            }
        }
    }

    pub enum ReturnType {
        VOID,
        TYPE(VarType),
    }
    impl Parsable for ReturnType {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let keyword = elements::Keyword::try_parse(reader, idx)?;
            let in_keyword = keyword.0 .0;
            match in_keyword {
                KeywordType::VOID => Some((ReturnType::VOID, keyword.1)),
                _ => {
                    let var_type = VarType::try_parse(reader, idx)?;
                    Some((ReturnType::TYPE(var_type.0), var_type.1))
                }
            }
        }
    }

    pub struct SubroutineDec {
        pub subroutine_type: SubroutineType,
        pub return_type: ReturnType,
        pub subroutine_name: SubroutineName,
        pub parameter_list: ParameterList,
        pub subroutine_body: SubroutineBody,
    }
    impl Parsable for SubroutineDec {
        fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;

            let _subroutine_type = SubroutineType::try_parse(reader, p)?;
            p = _subroutine_type.1;

            let _return_type = ReturnType::try_parse(reader, p)?;
            p = _return_type.1;

            let _subroutine_name = SubroutineName::try_parse(reader, p)?;
            p = _subroutine_name.1;

            p = try_parse_symbol(reader, p, '(')?;

            let _parameter_list = ParameterList::try_parse(reader, p)?;
            p = _parameter_list.1;

            p = try_parse_symbol(reader, p, ')')?;

            let _subroutine_body = SubroutineBody::try_parse(reader, p)?;
            p = _subroutine_body.1;

            Some((
                SubroutineDec {
                    subroutine_type: _subroutine_type.0,
                    return_type: _return_type.0,
                    subroutine_name: _subroutine_name.0,
                    parameter_list: _parameter_list.0,
                    subroutine_body: _subroutine_body.0,
                },
                p,
            ))
        }
    }
}

pub mod statements {
    use super::{elements::try_parse_symbol, expressions, structures, Parsable};

    pub enum Statement {
        LetStatement(LetStatement),
        IfStatement(IfStatement),
        WhileStatement(WhileStatement),
        DoStatement(DoStatement),
        ReturnStatement(ReturnStatement),
    }
    impl Parsable for Statement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            LetStatement::try_parse(reader, idx)
                .map(|(s, i)| (Statement::LetStatement(s), i))
                .or_else(|| {
                    IfStatement::try_parse(reader, idx).map(|(s, i)| (Statement::IfStatement(s), i))
                })
                .or_else(|| {
                    WhileStatement::try_parse(reader, idx)
                        .map(|(s, i)| (Statement::WhileStatement(s), i))
                })
                .or_else(|| {
                    DoStatement::try_parse(reader, idx).map(|(s, i)| (Statement::DoStatement(s), i))
                })
                .or_else(|| {
                    ReturnStatement::try_parse(reader, idx)
                        .map(|(s, i)| (Statement::ReturnStatement(s), i))
                })
        }
    }

    pub struct Statements(Vec<Statement>);
    impl Parsable for Statements {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let mut statements = vec![];
            loop {
                let parse_statement = Statement::try_parse(reader, p);
                if parse_statement.is_none() {
                    break;
                }
                let parse_statement = parse_statement.unwrap();
                p = parse_statement.1;
                statements.push(parse_statement.0);
            }
            Some((Statements(statements), p))
        }
    }

    pub struct LetStatement {
        pub let_lhs: LetLHS,
        pub let_rhs: expressions::Expression,
    }
    impl Parsable for LetStatement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;

            let _keyword = super::elements::Keyword::try_parse(reader, p)?;
            if _keyword.0 .0 != crate::tokenizer::KeywordType::LET {
                return None;
            }
            p = _keyword.1;

            let _lhs = LetLHS::try_parse(reader, p)?;
            p = _lhs.1;

            p = try_parse_symbol(reader, p, '=')?;

            let _rhs = super::expressions::Expression::try_parse(reader, p)?;
            p = _rhs.1;

            p = try_parse_symbol(reader, p, ';')?;

            Some((
                LetStatement {
                    let_lhs: _lhs.0,
                    let_rhs: _rhs.0,
                },
                p,
            ))
        }
    }

    pub enum LetLHS {
        VarName(structures::VarName),
        ArrayTerm(expressions::ArrayTerm),
    }
    impl Parsable for LetLHS {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let _array_term = expressions::ArrayTerm::try_parse(reader, idx);
            if _array_term.is_some() {
                let _array_term = _array_term.unwrap();
                return Some((LetLHS::ArrayTerm(_array_term.0), _array_term.1));
            }
            let v = structures::VarName::try_parse(reader, idx)?;
            Some((LetLHS::VarName(v.0), v.1))
        }
    }

    pub struct IfStatement {
        pub condition: expressions::Expression,
        pub true_statements: Statements,
        pub false_statements: Option<Statements>,
    }
    impl Parsable for IfStatement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;

            let _keyword = super::elements::Keyword::try_parse(reader, p)?;
            if _keyword.0 .0 != crate::tokenizer::KeywordType::IF {
                return None;
            }
            p = _keyword.1;

            p = try_parse_symbol(reader, p, '(')?;

            let _condition = expressions::Expression::try_parse(reader, p)?;
            p = _condition.1;

            p = try_parse_symbol(reader, p, ')')?;
            p = try_parse_symbol(reader, p, '{')?;
            let _true_statements = Statements::try_parse(reader, p)?;
            p = _true_statements.1;
            p = try_parse_symbol(reader, p, '}')?;

            let _else = super::elements::Keyword::try_parse(reader, p);
            if _else.is_none() {
                return Some((
                    IfStatement {
                        condition: _condition.0,
                        true_statements: _true_statements.0,
                        false_statements: None,
                    },
                    p,
                ));
            }
            let _else = _else.unwrap();
            if _else.0 .0 != crate::tokenizer::KeywordType::ELSE {
                return None;
            }
            p = _else.1;

            p = try_parse_symbol(reader, p, '{')?;
            let _false_statements = Statements::try_parse(reader, p)?;
            p = _false_statements.1;
            p = try_parse_symbol(reader, p, '}')?;

            Some((
                IfStatement {
                    condition: _condition.0,
                    true_statements: _true_statements.0,
                    false_statements: Some(_false_statements.0),
                },
                p
            ))
        }
    }

    pub struct WhileStatement {
        pub condition: expressions::Expression,
        pub statements: Statements,
    }
    impl Parsable for WhileStatement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let _keyword = super::elements::Keyword::try_parse(reader, p)?;
            if _keyword.0 .0 != crate::tokenizer::KeywordType::WHILE {
                return None;
            }
            p = _keyword.1;

            p = try_parse_symbol(reader, p, '(')?;
            let _condition = expressions::Expression::try_parse(reader, p)?;
            p = _condition.1;
            p = try_parse_symbol(reader, p, ')')?;
            p = try_parse_symbol(reader, p, '{')?;
            let _statements = Statements::try_parse(reader, p)?;
            p = _statements.1;
            p = try_parse_symbol(reader, p, '}')?;
            Some((
                WhileStatement {
                    condition: _condition.0,
                    statements: _statements.0,
                },
                p,
            ))
        }
    }

    pub struct DoStatement {
        pub subroutine_call: expressions::SubroutineCall,
    }
    impl Parsable for DoStatement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let _keyword = super::elements::Keyword::try_parse(reader, p)?;
            if _keyword.0 .0 != crate::tokenizer::KeywordType::DO {
                return None;
            }
            p = _keyword.1;
            let _subroutine_call = super::expressions::SubroutineCall::try_parse(reader, p)?;
            p = _subroutine_call.1;
            p = super::elements::try_parse_symbol(reader, p, ';')?;
            Some((
                DoStatement {
                    subroutine_call: _subroutine_call.0,
                },
                p,
            ))
        }
    }

    pub struct ReturnStatement {
        pub expression: Option<expressions::Expression>,
    }
    impl Parsable for ReturnStatement {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let _keyword = super::elements::Keyword::try_parse(reader, p)?;
            if _keyword.0 .0 != crate::tokenizer::KeywordType::RETURN {
                return None;
            }
            p = _keyword.1;
            let _expression = expressions::Expression::try_parse(reader, p);
            if _expression.is_none() {
                p = super::elements::try_parse_symbol(reader, p, ';')?;
                return Some((ReturnStatement { expression: None }, p));
            }
            let _expression = _expression.unwrap();
            p = _expression.1;
            p = super::elements::try_parse_symbol(reader, p, ';')?;
            Some((
                ReturnStatement {
                    expression: Some(_expression.0),
                },
                p,
            ))
        }
    }
}

pub mod expressions {
    use crate::tokenizer::KeywordType;

    use super::{
        elements::{self, try_parse_symbol},
        structures, Parsable,
    };

    pub struct Expression {
        pub term: Box<Term>,
        pub op_term: Vec<(Op, Term)>,
    }
    impl Parsable for Expression {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let _term = Term::try_parse(reader, idx)?;
            let mut p = _term.1;
            let mut op_term = vec![];
            loop {
                let _op = Op::try_parse(reader, p);
                if _op.is_none() {
                    break;
                }
                let _op = _op.unwrap();
                p = _op.1;
                let _term = Term::try_parse(reader, p);
                if _term.is_none() {
                    break;
                }
                let _term = _term.unwrap();
                p = _term.1;
                op_term.push((_op.0, _term.0));
            }
            Some((
                Expression {
                    term: Box::new(_term.0),
                    op_term: op_term,
                },
                p,
            ))
        }
    }

    pub enum Term {
        IntegerConstant(elements::IntegerConstant),
        StringConstant(elements::StringConstant),
        KeywordConstant(KeywordConstant),
        VarName(structures::VarName),
        ArrayTerm(ArrayTerm),
        WrappedExpression(WrappedExpression),
        SubroutineCall(SubroutineCall),
    }
    impl Parsable for Term {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            elements::IntegerConstant::try_parse(reader, idx)
                .map(|i| (Term::IntegerConstant(i.0), i.1))
                .or_else(|| {
                    elements::StringConstant::try_parse(reader, idx)
                        .map(|i| (Term::StringConstant(i.0), i.1))
                })
                .or_else(|| {
                    KeywordConstant::try_parse(reader, idx)
                        .map(|i| (Term::KeywordConstant(i.0), i.1))
                })
                .or_else(|| {
                    structures::VarName::try_parse(reader, idx).map(|i| (Term::VarName(i.0), i.1))
                })
                .or_else(|| ArrayTerm::try_parse(reader, idx).map(|i| (Term::ArrayTerm(i.0), i.1)))
                .or_else(|| {
                    WrappedExpression::try_parse(reader, idx)
                        .map(|i| (Term::WrappedExpression(i.0), i.1))
                })
                .or_else(|| {
                    SubroutineCall::try_parse(reader, idx).map(|i| (Term::SubroutineCall(i.0), i.1))
                })
        }
    }

    pub struct ArrayTerm {
        pub var_name: structures::VarName,
        pub expression: Expression,
    }
    impl Parsable for ArrayTerm {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let mut p = idx;

            let _var_name = structures::VarName::try_parse(reader, p)?;
            p = _var_name.1;

            p = try_parse_symbol(reader, p, '[')?;

            let _expression = Expression::try_parse(reader, p)?;
            p = _expression.1;

            p = try_parse_symbol(reader, p, ']')?;
            Some((
                ArrayTerm {
                    var_name: _var_name.0,
                    expression: _expression.0,
                },
                p,
            ))
        }
    }
    pub struct WrappedExpression(Expression);
    impl Parsable for WrappedExpression {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;

            p = try_parse_symbol(reader, p, '(')?;

            let _expression = Expression::try_parse(reader, p)?;
            p = _expression.1;

            p = try_parse_symbol(reader, p, ')')?;

            Some((WrappedExpression(_expression.0), p))
        }
    }

    pub struct UnaryTerm {
        pub unary_op: UnaryOp,
        pub term: Box<Term>,
    }
    impl Parsable for UnaryTerm {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let _unary_op = UnaryOp::try_parse(reader, idx)?;
            let _term = Term::try_parse(reader, _unary_op.1)?;
            Some((
                UnaryTerm {
                    unary_op: _unary_op.0,
                    term: Box::new(_term.0),
                },
                _term.1,
            ))
        }
    }

    pub struct ExpressionList(pub Vec<Expression>);
    impl Parsable for ExpressionList {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let mut _expression_list = vec![];
            let mut p = idx;
            let _first_expression = Expression::try_parse(reader, p);
            if _first_expression.is_none() {
                return Some((ExpressionList(_expression_list), p));
            }
            let _first_expression = _first_expression.unwrap();
            _expression_list.push(_first_expression.0);
            p = _first_expression.1;
            loop {
                let _comma = try_parse_symbol(reader, p, ',');
                if _comma.is_none() {
                    break;
                }
                p = _comma.unwrap();
                let _expression = Expression::try_parse(reader, p)?;
                _expression_list.push(_expression.0);
                p = _expression.1;
            }
            Some((ExpressionList(_expression_list), p))
        }
    }

    pub struct SubroutineCall {
        pub bind_this: Option<structures::VarName>, // TODO: classname or varname
        pub subroutine_name: structures::SubroutineName,
        pub expression_list: ExpressionList,
    }
    impl Parsable for SubroutineCall {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let mut p = idx;
            let _dot = elements::Symbol::try_parse(reader, p + 1);
            let _subroutine_name;
            let mut _bind_this = None;
            if _dot.is_none() {
                _subroutine_name = structures::SubroutineName::try_parse(reader, p)?;
                p = _subroutine_name.1;
            } else {
                let _t_bind_this = structures::VarName::try_parse(reader, p)?;
                p = _t_bind_this.1;
                _bind_this = Some(_t_bind_this.0);
                p += 1; // skip dop
                _subroutine_name = structures::SubroutineName::try_parse(reader, p)?;
                p = _subroutine_name.1;
            }
            p = try_parse_symbol(reader, p, '(')?;

            let _expression_list = ExpressionList::try_parse(reader, p)?;
            p = _expression_list.1;

            p = try_parse_symbol(reader, p, ')')?;
            Some((
                SubroutineCall {
                    bind_this: _bind_this,
                    subroutine_name: _subroutine_name.0,
                    expression_list: _expression_list.0,
                },
                p,
            ))
        }
    }

    pub struct Op(elements::Symbol);
    impl Parsable for Op {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let symbol = elements::Symbol::try_parse(reader, idx)?;
            match symbol.0 .0 {
                '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => {
                    Some((Op(symbol.0), symbol.1))
                }
                _ => None,
            }
        }
    }

    pub struct UnaryOp(elements::Symbol);
    impl Parsable for UnaryOp {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let symbol = elements::Symbol::try_parse(reader, idx)?;
            match symbol.0 .0 {
                '-' | '~' => Some((UnaryOp(symbol.0), symbol.1)),
                _ => None,
            }
        }
    }

    pub enum KeywordConstant {
        TRUE,
        FALSE,
        NULL,
        THIS,
    }
    impl Parsable for KeywordConstant {
        fn try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let keyword = elements::Keyword::try_parse(reader, idx)?;
            match keyword.0 .0 {
                KeywordType::TRUE => Some((KeywordConstant::TRUE, keyword.1)),
                KeywordType::FALSE => Some((KeywordConstant::FALSE, keyword.1)),
                KeywordType::NULL => Some((KeywordConstant::NULL, keyword.1)),
                KeywordType::THIS => Some((KeywordConstant::THIS, keyword.1)),
                _ => None,
            }
        }
    }
}
