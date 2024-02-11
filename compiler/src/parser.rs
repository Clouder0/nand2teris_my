use crate::tokenizer::Token;

pub struct TokenReader {
    pub tokens: Vec<Token>,
}

pub trait Parsable {
    fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
    where
        Self: Sized;

    fn try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
    where
        Self: Sized,
    {
        if idx >= reader.tokens.len() {
            return None;
        }
        Parsable::_try_parse(reader, idx)
    }
}

pub enum Node {
    Keyword(elements::Keyword),
    Symbol(elements::Symbol),
    IntegerConstant(elements::IntegerConstant),
    StringConstant(elements::StringConstant),
    Identifier(elements::Identifier),
    Class(structures::Class),
    ClassVarDec(structures::ClassVarDec),
    VarDec(structures::VarDec),
    SubroutineDec(structures::SubroutineDec),
    ParameterList(structures::ParameterList),
    SubroutineBody(structures::SubroutineBody),
    Statements(statements::Statements),
    LetStatement(statements::LetStatement),
    IfStatement(statements::IfStatement),
    WhileStatement(statements::WhileStatement),
    DoStatement(statements::DoStatement),
    ReturnStatement(statements::ReturnStatement),
    Expression(expressions::Expression),
    Term(expressions::Term),
    ArrayTerm(expressions::ArrayTerm),
    WrappedExpression(expressions::WrappedExpression),
    SubroutineCall(expressions::SubroutineCall),
    Op(expressions::Op),
    UnaryOp(expressions::UnaryOp),
    KeywordConstant(expressions::KeywordConstant),
    VarName(structures::VarName),
    ClassName(structures::ClassName),
    SubroutineName(structures::SubroutineName),
    VarType(structures::VarType),
    ClassVarDecType(structures::ClassVarDecType),
    SubroutineType(structures::SubroutineType),
    ReturnType(structures::ReturnType),
    LetLHS(statements::LetLHS),
    UnaryTerm(expressions::UnaryTerm),
    ExpressionList(expressions::ExpressionList),
}

pub mod elements {

    use super::{Parsable, TokenReader};
    use crate::tokenizer::{KeywordType, Token};

    pub struct Keyword(pub KeywordType);
    impl Parsable for Keyword {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::Keyword(keyword) => Some((Keyword(keyword), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Symbol(pub char);
    impl Parsable for Symbol {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::Symbol(symbol) => Some((Symbol(symbol), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct IntegerConstant(pub i64);
    impl Parsable for IntegerConstant {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match reader.tokens[idx] {
                Token::IntConst(integer) => Some((IntegerConstant(integer), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct StringConstant(pub String);
    impl Parsable for StringConstant {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::StringConst(string) => Some((StringConstant(string.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Identifier(pub String);
    impl Parsable for Identifier {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((Identifier(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    pub fn try_parse_symbol(reader: &TokenReader, idx: usize, symbol: char) -> Option<usize> {
        if idx >= reader.tokens.len() {
            return None;
        }
        if let Token::Symbol(s) = &reader.tokens[idx] {
            if *s == symbol {
                return Some(idx + 1);
            }
        }
        None
    }
}

pub mod structures {
    use crate::tokenizer::{KeywordType, Token};

    use super::{
        elements::{self, try_parse_symbol},
        statements, Parsable, TokenReader,
    };

    #[derive(Debug)]
    pub struct Class {
        pub class_name: ClassName,
        pub class_var_dec: Vec<ClassVarDec>,
        pub subroutine_dec: Vec<SubroutineDec>,
    }
    impl Parsable for Class {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(PartialEq, Debug)]
    pub enum ClassVarDecType {
        STATIC,
        FIELD,
    }
    impl Parsable for ClassVarDecType {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub enum VarTypeEnum {
        INT,
        CHAR,
        BOOLEAN,
        CLASSNAME(ClassName),
    }

    #[derive(Debug, PartialEq)]
    pub struct VarType(pub VarTypeEnum);
    impl Parsable for VarType {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct ClassName(pub String);
    impl Parsable for ClassName {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((ClassName(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct SubroutineName(pub String);
    impl Parsable for SubroutineName {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => {
                    Some((SubroutineName(identifier.clone()), idx + 1))
                }
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct VarName(pub String);
    impl Parsable for VarName {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            match &reader.tokens[idx] {
                Token::Identifier(identifier) => Some((VarName(identifier.clone()), idx + 1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct VarDec {
        pub var_type: VarType,
        pub var_names: Vec<VarName>,
    }
    impl Parsable for VarDec {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug)]
    pub struct ClassVarDec {
        pub var_dec_type: ClassVarDecType,
        pub var_type: VarType,
        pub var_names: Vec<VarName>,
    }
    impl Parsable for ClassVarDec {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug)]
    pub struct ParameterList {
        pub parameters: Vec<(VarType, VarName)>,
    }
    impl Parsable for ParameterList {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;
            let mut parameters = vec![];
            let _var_type = VarType::try_parse(reader, p);
            if _var_type.is_none() {
                return Some((
                    ParameterList {
                        parameters: parameters,
                    },
                    p,
                ));
            }
            let _var_type = _var_type.unwrap();
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
            Some((
                ParameterList {
                    parameters: parameters,
                },
                p,
            ))
        }
    }

    #[derive(Debug)]
    pub struct SubroutineBody {
        pub var_decs: Vec<VarDec>,
        pub statements: statements::Statements,
    }
    impl Parsable for SubroutineBody {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut var_decs = vec![];
            let mut p = idx;
            p = try_parse_symbol(reader, p, '{')?;
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
            p = try_parse_symbol(reader, p, '}')?;
            Some((
                SubroutineBody {
                    var_decs: var_decs,
                    statements: _statements.0,
                },
                p,
            ))
        }
    }

    #[derive(Debug)]
    pub enum SubroutineType {
        CONSTRUCTOR,
        FUNCTION,
        METHOD,
    }
    impl Parsable for SubroutineType {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug)]
    pub enum ReturnType {
        VOID,
        VARTYPE(VarType),
    }
    impl Parsable for ReturnType {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let keyword = elements::Keyword::try_parse(reader, idx);
            if keyword.is_some() {
                let keyword = keyword.unwrap();
                return match keyword.0 .0 {
                    KeywordType::VOID => Some((ReturnType::VOID, keyword.1)),
                    _ => {
                        let var_type = VarType::try_parse(reader, idx)?;
                        Some((ReturnType::VARTYPE(var_type.0), var_type.1))
                    }
                }
            }
            let var_type = VarType::try_parse(reader, idx)?;
            Some((ReturnType::VARTYPE(var_type.0), var_type.1))
        }
    }

    #[derive(Debug)]
    pub struct SubroutineDec {
        pub subroutine_type: SubroutineType,
        pub return_type: ReturnType,
        pub subroutine_name: SubroutineName,
        pub parameter_list: ParameterList,
        pub subroutine_body: SubroutineBody,
    }
    impl Parsable for SubroutineDec {
        fn _try_parse(reader: &TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub enum Statement {
        LetStatement(LetStatement),
        IfStatement(IfStatement),
        WhileStatement(WhileStatement),
        DoStatement(DoStatement),
        ReturnStatement(ReturnStatement),
    }
    impl Parsable for Statement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct Statements(pub Vec<Statement>);
    impl Parsable for Statements {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct LetStatement {
        pub let_lhs: LetLHS,
        pub let_rhs: expressions::Expression,
    }
    impl Parsable for LetStatement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub enum LetLHS {
        VarName(structures::VarName),
        ArrayTerm(expressions::ArrayTerm),
    }
    impl Parsable for LetLHS {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub struct IfStatement {
        pub condition: expressions::Expression,
        pub true_statements: Statements,
        pub false_statements: Option<Statements>,
    }
    impl Parsable for IfStatement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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
                return Some((
                    IfStatement {
                        condition: _condition.0,
                        true_statements: _true_statements.0,
                        false_statements: None,
                    },
                    p,
                ));
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
                p,
            ))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct WhileStatement {
        pub condition: expressions::Expression,
        pub statements: Statements,
    }
    impl Parsable for WhileStatement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct DoStatement {
        pub subroutine_call: expressions::SubroutineCall,
    }
    impl Parsable for DoStatement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct ReturnStatement {
        pub expression: Option<expressions::Expression>,
    }
    impl Parsable for ReturnStatement {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

    #[derive(Debug, PartialEq)]
    pub struct Expression {
        pub term: Box<Term>,
        pub op_term: Vec<(Op, Term)>,
    }
    impl Parsable for Expression {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub enum Term {
        IntegerConstant(elements::IntegerConstant),
        StringConstant(elements::StringConstant),
        KeywordConstant(KeywordConstant),
        VarName(structures::VarName),
        ArrayTerm(ArrayTerm),
        UnaryTerm(UnaryTerm),
        WrappedExpression(WrappedExpression),
        SubroutineCall(SubroutineCall),
    }
    impl Parsable for Term {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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
                    UnaryTerm::_try_parse(reader, idx)
                        .map(|i| (Term::UnaryTerm(i.0), i.1))
                })
                .or_else(|| {
                    KeywordConstant::try_parse(reader, idx)
                        .map(|i| (Term::KeywordConstant(i.0), i.1))
                })
                .or_else(|| {
                    SubroutineCall::try_parse(reader, idx).map(|i| (Term::SubroutineCall(i.0), i.1))
                })
                .or_else(|| ArrayTerm::try_parse(reader, idx).map(|i| (Term::ArrayTerm(i.0), i.1)))
                .or_else(|| {
                    structures::VarName::try_parse(reader, idx).map(|i| (Term::VarName(i.0), i.1))
                })
                .or_else(|| {
                    WrappedExpression::try_parse(reader, idx)
                        .map(|i| (Term::WrappedExpression(i.0), i.1))
                })
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct ArrayTerm {
        pub var_name: structures::VarName,
        pub expression: Expression,
    }
    impl Parsable for ArrayTerm {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub struct WrappedExpression(pub Expression);
    impl Parsable for WrappedExpression {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let mut p = idx;

            p = try_parse_symbol(reader, p, '(')?;

            let _expression = Expression::try_parse(reader, p)?;
            p = _expression.1;

            p = try_parse_symbol(reader, p, ')')?;

            Some((WrappedExpression(_expression.0), p))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct UnaryTerm {
        pub unary_op: UnaryOp,
        pub term: Box<Term>,
    }
    impl Parsable for UnaryTerm {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub struct ExpressionList(pub Vec<Expression>);
    impl Parsable for ExpressionList {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
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

    #[derive(Debug, PartialEq)]
    pub struct SubroutineCall {
        pub bind_this: Option<structures::VarName>, // TODO: classname or varname
        pub subroutine_name: structures::SubroutineName,
        pub expression_list: ExpressionList,
    }
    impl Parsable for SubroutineCall {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)>
        where
            Self: Sized,
        {
            let mut p = idx;
            let _dot = try_parse_symbol(reader, p + 1, '.');
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

    #[derive(Debug, PartialEq)]
    pub struct Op(pub elements::Symbol);
    impl Parsable for Op {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let symbol = elements::Symbol::try_parse(reader, idx)?;
            match symbol.0 .0 {
                '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => {
                    Some((Op(symbol.0), symbol.1))
                }
                _ => None,
            }
        }
    }

    #[derive(Debug,PartialEq)]
    pub struct UnaryOp(pub elements::Symbol);
    impl Parsable for UnaryOp {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
            let symbol = elements::Symbol::try_parse(reader, idx)?;
            match symbol.0 .0 {
                '-' | '~' => Some((UnaryOp(symbol.0), symbol.1)),
                _ => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum KeywordConstant {
        TRUE,
        FALSE,
        NULL,
        THIS,
    }
    impl Parsable for KeywordConstant {
        fn _try_parse(reader: &super::TokenReader, idx: usize) -> Option<(Self, usize)> {
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

#[cfg(test)]
pub mod tests {
    use crate::{parser::*, tokenizer::KeywordType};

    #[test]
    fn test_parse_keyword() {
        let tokens = vec![Token::Keyword(KeywordType::CLASS)];
        let reader = TokenReader { tokens: tokens };
        let (keyword, new_idx) = elements::Keyword::try_parse(&reader, 0).unwrap();
        assert_eq!(keyword.0, KeywordType::CLASS);
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_parse_symbol() {
        let tokens = vec![Token::Symbol('{')];
        let reader = TokenReader { tokens: tokens };
        let new_idx = elements::try_parse_symbol(&reader, 0, '{').unwrap();
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_parse_int() {
        let tokens = vec![Token::IntConst(123)];
        let reader = TokenReader { tokens: tokens };
        let (int_const, new_idx) = elements::IntegerConstant::try_parse(&reader, 0).unwrap();
        assert_eq!(int_const.0, 123);
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::StringConst("abc".to_string())];
        let reader = TokenReader { tokens: tokens };
        let (string_const, new_idx) = elements::StringConstant::try_parse(&reader, 0).unwrap();
        assert_eq!(string_const.0, "abc".to_string());
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![Token::Identifier("abc".to_string())];
        let reader = TokenReader { tokens: tokens };
        let (identifier, new_idx) = elements::Identifier::try_parse(&reader, 0).unwrap();
        assert_eq!(identifier.0, "abc".to_string());
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_class_vardec_type() {
        let tokens = vec![Token::Keyword(KeywordType::STATIC)];
        let reader = TokenReader { tokens: tokens };
        let (class_vardec_type, new_idx) =
            structures::ClassVarDecType::try_parse(&reader, 0).unwrap();
        assert_eq!(class_vardec_type, structures::ClassVarDecType::STATIC);
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_var_type() {
        let tokens = vec![Token::Keyword(KeywordType::INT)];
        let reader = TokenReader { tokens: tokens };
        let (var_type, new_idx) = structures::VarType::try_parse(&reader, 0).unwrap();
        assert_eq!(var_type.0, structures::VarTypeEnum::INT);
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_class_name() {
        let tokens = vec![Token::Identifier("abc".to_string())];
        let reader = TokenReader { tokens: tokens };
        let (class_name, new_idx) = structures::ClassName::try_parse(&reader, 0).unwrap();
        assert_eq!(class_name.0, "abc".to_string());
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_subroutine_name() {
        let tokens = vec![Token::Identifier("abc".to_string())];
        let reader = TokenReader { tokens: tokens };
        let (subroutine_name, new_idx) = structures::SubroutineName::try_parse(&reader, 0).unwrap();
        assert_eq!(subroutine_name.0, "abc".to_string());
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_vardecs() {
        let tokens = vec![
            Token::Keyword(KeywordType::VAR),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (vardec, new_idx) = structures::VarDec::try_parse(&reader, 0).unwrap();
        assert_eq!(vardec.var_type.0, structures::VarTypeEnum::INT);
        assert_eq!(vardec.var_names[0].0, "abc".to_string());
        assert_eq!(new_idx, 4);
    }

    #[test]
    fn test_multiple_vardecs() {
        let tokens = vec![
            Token::Keyword(KeywordType::VAR),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(','),
            Token::Identifier("def".to_string()),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (vardec, new_idx) = structures::VarDec::try_parse(&reader, 0).unwrap();
        assert_eq!(vardec.var_type.0, structures::VarTypeEnum::INT);
        assert_eq!(vardec.var_names[0].0, "abc".to_string());
        assert_eq!(vardec.var_names[1].0, "def".to_string());
        assert_eq!(new_idx, 6);
    }

    #[test]
    fn test_class_vardec() {
        let tokens = vec![
            Token::Keyword(KeywordType::STATIC),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (class_vardec, new_idx) = structures::ClassVarDec::try_parse(&reader, 0).unwrap();
        assert_eq!(
            class_vardec.var_dec_type,
            structures::ClassVarDecType::STATIC
        );
        assert_eq!(class_vardec.var_type.0, structures::VarTypeEnum::INT);
        assert_eq!(class_vardec.var_names[0].0, "abc".to_string());
        assert_eq!(new_idx, 4);
    }

    #[test]
    fn test_multiple_class_vardec() {
        let tokens = vec![
            Token::Keyword(KeywordType::STATIC),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(','),
            Token::Identifier("def".to_string()),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (class_vardec, new_idx) = structures::ClassVarDec::try_parse(&reader, 0).unwrap();
        assert_eq!(
            class_vardec.var_dec_type,
            structures::ClassVarDecType::STATIC
        );
        assert_eq!(class_vardec.var_type.0, structures::VarTypeEnum::INT);
        assert_eq!(class_vardec.var_names[0].0, "abc".to_string());
        assert_eq!(class_vardec.var_names[1].0, "def".to_string());
        assert_eq!(new_idx, 6);
    }

    #[test]
    fn test_parameter_empty() {
        let tokens = vec![Token::Symbol('(')];
        let reader = TokenReader { tokens: tokens };
        let (parameter_list, new_idx) = structures::ParameterList::try_parse(&reader, 0).unwrap();
        assert_eq!(parameter_list.parameters.len(), 0);
        assert_eq!(new_idx, 0);
    }

    #[test]
    fn test_parameter_list() {
        let tokens = vec![
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(','),
            Token::Keyword(KeywordType::CHAR),
            Token::Identifier("def".to_string()),
            Token::Symbol(','),
            Token::Keyword(KeywordType::BOOLEAN),
            Token::Identifier("ghi".to_string()),
        ];
        let reader = TokenReader { tokens: tokens };
        let (parameter_list, new_idx) = structures::ParameterList::try_parse(&reader, 0).unwrap();
        assert_eq!(parameter_list.parameters.len(), 3);
        assert_eq!(
            parameter_list.parameters[0].0 .0,
            structures::VarTypeEnum::INT
        );
        assert_eq!(parameter_list.parameters[0].1 .0, "abc".to_string());
        assert_eq!(
            parameter_list.parameters[1].0 .0,
            structures::VarTypeEnum::CHAR
        );
        assert_eq!(parameter_list.parameters[1].1 .0, "def".to_string());
        assert_eq!(
            parameter_list.parameters[2].0 .0,
            structures::VarTypeEnum::BOOLEAN
        );
        assert_eq!(parameter_list.parameters[2].1 .0, "ghi".to_string());
        assert_eq!(new_idx, 8);
    }

    #[test]
    fn test_letstatement() {
        let tokens = vec![
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (let_statement, new_idx) = statements::LetStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(
            let_statement.let_lhs,
            statements::LetLHS::VarName(structures::VarName("abc".to_string()))
        );
        assert_eq!(
            *let_statement.let_rhs.term,
            expressions::Term::IntegerConstant(elements::IntegerConstant(123))
        );
        assert_eq!(new_idx, 5);
    }

    #[test]
    fn test_ifstatement() {
        let tokens = vec![
            Token::Keyword(KeywordType::IF),
            Token::Symbol('('),
            Token::Keyword(KeywordType::TRUE),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (if_statement, new_idx) = statements::IfStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(
            if_statement.condition,
            expressions::Expression {
                term: Box::new(expressions::Term::KeywordConstant(
                    expressions::KeywordConstant::TRUE
                )),
                op_term: vec![]
            }
        );
        assert_eq!(if_statement.true_statements.0.len(), 1);
        assert_eq!(new_idx, 11);
    }

    #[test]
    fn test_whilestatement() {
        let tokens = vec![
            Token::Keyword(KeywordType::WHILE),
            Token::Symbol('('),
            Token::Keyword(KeywordType::TRUE),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (while_statement, new_idx) = statements::WhileStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(
            while_statement.condition,
            expressions::Expression {
                term: Box::new(expressions::Term::KeywordConstant(
                    expressions::KeywordConstant::TRUE
                )),
                op_term: vec![]
            }
        );
        assert_eq!(while_statement.statements.0.len(), 1);
        assert_eq!(new_idx, 11);
    }

    #[test]
    fn test_do_statement_bind() {
        let tokens = vec![
            Token::Keyword(KeywordType::DO),
            Token::Identifier("abc".to_string()),
            Token::Symbol('.'),
            Token::Identifier("def".to_string()),
            Token::Symbol('('),
            Token::Symbol(')'),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (do_statement, new_idx) = statements::DoStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(
            do_statement.subroutine_call.bind_this.unwrap().0,
            "abc".to_string()
        );
        assert_eq!(
            do_statement.subroutine_call.subroutine_name.0,
            "def".to_string()
        );
        assert_eq!(do_statement.subroutine_call.expression_list.0.len(), 0);
        assert_eq!(new_idx, 7);
    }

    #[test]
    fn test_do_statement_nobind() {
        let tokens = vec![
            Token::Keyword(KeywordType::DO),
            Token::Identifier("def".to_string()),
            Token::Symbol('('),
            Token::Symbol(')'),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (do_statement, new_idx) = statements::DoStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(do_statement.subroutine_call.bind_this, None);
        assert_eq!(
            do_statement.subroutine_call.subroutine_name.0,
            "def".to_string()
        );
        assert_eq!(do_statement.subroutine_call.expression_list.0.len(), 0);
        assert_eq!(new_idx, 5);
    }

    #[test]
    fn test_return_statement() {
        let tokens = vec![
            Token::Keyword(KeywordType::RETURN),
            Token::IntConst(123),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (return_statement, new_idx) =
            statements::ReturnStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(
            *return_statement.expression.unwrap().term,
            expressions::Term::IntegerConstant(elements::IntegerConstant(123))
        );
        assert_eq!(new_idx, 3);
    }

    #[test]
    fn test_return_void() {
        let tokens = vec![Token::Keyword(KeywordType::RETURN), Token::Symbol(';')];
        let reader = TokenReader { tokens: tokens };
        let (return_statement, new_idx) =
            statements::ReturnStatement::try_parse(&reader, 0).unwrap();
        assert_eq!(return_statement.expression, None);
        assert_eq!(new_idx, 2);
    }

    #[test]
    fn test_op() {
        let tokens = vec![Token::Symbol('+')];
        let reader = TokenReader { tokens: tokens };
        let (op, new_idx) = expressions::Op::try_parse(&reader, 0).unwrap();
        assert_eq!(op.0 .0, '+');
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_unary() {
        let tokens = vec![Token::Symbol('-')];
        let reader = TokenReader { tokens: tokens };
        let (unary, new_idx) = expressions::UnaryOp::try_parse(&reader, 0).unwrap();
        assert_eq!(unary.0 .0, '-');
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_keyword_const() {
        let tokens = vec![Token::Keyword(KeywordType::TRUE)];
        let reader = TokenReader { tokens: tokens };
        let (keyword_const, new_idx) = expressions::KeywordConstant::try_parse(&reader, 0).unwrap();
        assert_eq!(keyword_const, expressions::KeywordConstant::TRUE);
        assert_eq!(new_idx, 1);
    }

    #[test]
    fn test_subroutine_call() {
        let tokens = vec![
            Token::Identifier("abc".to_string()),
            Token::Symbol('.'),
            Token::Identifier("def".to_string()),
            Token::Symbol('('),
            Token::Symbol(')'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (subroutine_call, new_idx) =
            expressions::SubroutineCall::try_parse(&reader, 0).unwrap();
        assert_eq!(subroutine_call.bind_this.unwrap().0, "abc".to_string());
        assert_eq!(subroutine_call.subroutine_name.0, "def".to_string());
        assert_eq!(subroutine_call.expression_list.0.len(), 0);
        assert_eq!(new_idx, 5);
    }

    #[test]
    fn test_subroutine_call_with_args() {
        let tokens = vec![
            Token::Identifier("abc".to_string()),
            Token::Symbol('.'),
            Token::Identifier("def".to_string()),
            Token::Symbol('('),
            Token::IntConst(123),
            Token::Symbol(','),
            Token::IntConst(456),
            Token::Symbol(')'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (subroutine_call, new_idx) =
            expressions::SubroutineCall::try_parse(&reader, 0).unwrap();
        assert_eq!(subroutine_call.bind_this.unwrap().0, "abc".to_string());
        assert_eq!(subroutine_call.subroutine_name.0, "def".to_string());
        assert_eq!(subroutine_call.expression_list.0.len(), 2);
        assert_eq!(
            subroutine_call.expression_list.0[0],
            expressions::Expression {
                term: Box::new(expressions::Term::IntegerConstant(
                    elements::IntegerConstant(123)
                )),
                op_term: vec![]
            }
        );
        assert_eq!(new_idx, 8);
    }

    #[test]
    fn test_wrapped_expression() {
        let tokens = vec![Token::Symbol('('), Token::IntConst(123), Token::Symbol(')')];
        let reader = TokenReader { tokens: tokens };
        let (wrapped_expression, new_idx) =
            expressions::WrappedExpression::try_parse(&reader, 0).unwrap();
        assert_eq!(
            *wrapped_expression.0.term,
            expressions::Term::IntegerConstant(elements::IntegerConstant(123))
        );
        assert_eq!(new_idx, 3);
    }

    #[test]
    fn test_math_express() {
        let tokens = vec![
            Token::IntConst(123),
            Token::Symbol('+'),
            Token::IntConst(456),
            Token::Symbol('*'),
            Token::IntConst(789),
        ];
        let reader = TokenReader { tokens: tokens };
        let (expression, new_idx) = expressions::Expression::try_parse(&reader, 0).unwrap();
        assert_eq!(
            *expression.term,
            expressions::Term::IntegerConstant(elements::IntegerConstant(123))
        );
        assert_eq!(expression.op_term.len(), 2);
        assert_eq!(new_idx, 5);
    }

    #[test]
    fn test_array_term() {
        let tokens = vec![
            Token::Identifier("abc".to_string()),
            Token::Symbol('['),
            Token::IntConst(123),
            Token::Symbol(']'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (array_term, new_idx) = expressions::ArrayTerm::try_parse(&reader, 0).unwrap();
        assert_eq!(array_term.var_name.0, "abc".to_string());
        assert_eq!(
            *array_term.expression.term,
            expressions::Term::IntegerConstant(elements::IntegerConstant(123))
        );
        assert_eq!(new_idx, 4);
    }

    #[test]
    fn test_subroutine_body() {
        let tokens = vec![
            Token::Symbol('{'),
            Token::Keyword(KeywordType::VAR),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(';'),
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (subroutine_body, new_idx) = structures::SubroutineBody::try_parse(&reader, 0).unwrap();
        assert_eq!(subroutine_body.var_decs.len(), 1);
        assert_eq!(subroutine_body.statements.0.len(), 1);
        assert_eq!(new_idx, 11);
    }

    #[test]
    fn test_multiple_statements() {
        let tokens = vec![
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
            Token::Keyword(KeywordType::LET),
            Token::Identifier("def".to_string()),
            Token::Symbol('='),
            Token::IntConst(456),
            Token::Symbol(';'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (statements, new_idx) = statements::Statements::try_parse(&reader, 0).unwrap();
        assert_eq!(statements.0.len(), 2);
        assert_eq!(new_idx, 10);
    }

    #[test]
    fn test_class() {
        let tokens = vec![
            Token::Keyword(KeywordType::CLASS),
            Token::Identifier("abc".to_string()),
            Token::Symbol('{'),
            Token::Keyword(KeywordType::STATIC),
            Token::Keyword(KeywordType::INT),
            Token::Identifier("abc".to_string()),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (class, new_idx) = structures::Class::try_parse(&reader, 0).unwrap();
        assert_eq!(class.class_name.0, "abc".to_string());
        assert_eq!(class.class_var_dec.len(), 1);
        assert_eq!(new_idx, 8);
    }

    #[test]
    fn test_let_call_return() {
        let tokens = vec![
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::Identifier("def".to_string()),
            Token::Symbol('.'),
            Token::Identifier("ghi".to_string()),
            Token::Symbol('('),
            Token::Symbol(')'),
            Token::Symbol(';'),
        ];
        let l = tokens.len();
        let reader = TokenReader { tokens: tokens };
        let (let_statement, new_idx) = statements::LetStatement::_try_parse(&reader, 0).unwrap();
        assert_eq!(
            let_statement.let_lhs,
            statements::LetLHS::VarName(structures::VarName("abc".to_string()))
        );
        assert_eq!(
            *let_statement.let_rhs.term,
            expressions::Term::SubroutineCall(expressions::SubroutineCall {
                bind_this: Some(structures::VarName("def".to_string())),
                subroutine_name: structures::SubroutineName("ghi".to_string()),
                expression_list: expressions::ExpressionList(vec![])
            })
        );
        assert_eq!(new_idx, l);
    }
    
    #[test]
    fn test_if_expr() {
        let tokens = vec![
            Token::Keyword(KeywordType::IF),
            Token::Symbol('('),
            Token::Identifier("x".to_string()),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Keyword(KeywordType::LET),
            Token::Identifier("abc".to_string()),
            Token::Symbol('='),
            Token::IntConst(123),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];
        let reader = TokenReader { tokens: tokens };
        let (if_statement, _new_idx) = statements::IfStatement::_try_parse(&reader, 0).unwrap();
        assert_eq!(
            if_statement.condition,
            expressions::Expression {
                term: Box::new(expressions::Term::VarName(structures::VarName("x".to_string()))),
                op_term: vec![]
            }
        );
    }
}
