struct TokenReader {
    tokens: Vec<Token>,
    index: usize
}

trait parsable {
    fn try_parse(reader: &mut TokenReader) -> Option<Self>;
}

trait element: parsable {}

trait structure: parsable {}

trait statement: parsable {}

trait expression: parsable {}


mod elements {
    enum Keywordtype {
        Class,
        Constructor,
        Function,
        Method,
        Field,
        Static,
        Var,
        Int,
        Char,
        Boolean,
        Void,
        True,
        False,
        Null,
        This,
        Let,
        Do,
        If,
        Else,
        While,
        Return
    }
    struct Keyword(KeywordType);
    impl element for Keyword {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Keyword(keyword) => Some(Keyword(keyword)),
                _ => None
            }
        }
    }
    
    struct Symbol(char);
    impl element for Symbol {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Symbol(symbol) => Some(Symbol(symbol)),
                _ => None
            }
        }
    }

    struct IntegerConstant(i64);
    impl element for IntegerConstant {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::IntegerConstant(integer) => Some(IntegerConstant(integer)),
                _ => None
            }
        }
    }
    
    struct StringConstant(String);
    impl element for StringConstant {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::StringConstant(string) => Some(StringConstant(string)),
                _ => None
            }
        }
    }

    struct Identifier(String);
    impl element for Identifier {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Identifier(identifier) => Some(Identifier(identifier)),
                _ => None
            }
        }
    }
}

mod structures {
    struct Class{
        class_name: String,
    }
    
    enum ClassVarDecType{
        STATIC,
        FIELD
    }
    struct ClassVarDec{
        var_dec_type: ClassVarDecType,
        var_name: String,
        var_type: ClassVarType,
    }
    
    enum ClassVarTypeEnum {
        INT,
        CHAR,
        BOOLEAN,
        CLASSNAME(ClassName)
    }
    struct ClassVarType(ClassVarTypeEnum);
    impl structure for ClassVarType {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Keyword(keyword) => match keyword {
                    tokenizer::KeywordType::Int => Some(ClassVarType(ClassVarTypeEnum::INT)),
                    tokenizer::KeywordType::Char => Some(ClassVarType(ClassVarTypeEnum::CHAR)),
                    tokenizer::KeywordType::Boolean => Some(ClassVarType(ClassVarTypeEnum::BOOLEAN)),
                    _ => None
                }
                Token::Identifier(identifier) => ClassName::try_parse(reader).map(|class_name| ClassVarType(ClassVarTypeEnum::CLASSNAME(class_name))),
                _ => None
            }
        }
    }
    
    struct ClassName(String);
    impl structure for ClassName {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Identifier(identifier) => Some(ClassName(identifier)),
                _ => None
            }
        }
    }
    
    struct SubroutineName(String);
    impl structure for SubroutineName {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Identifier(identifier) => Some(SubroutineName(identifier)),
                _ => None
            }
        }
    }

    struct VarName(String);
    impl structure for VarName {
        fn try_parse(reader: &mut TokenReader) -> Option<Self> {
            match reader.tokens[reader.index] {
                Token::Identifier(identifier) => Some(VarName(identifier)),
                _ => None
            }
        }
    }
}

mod statements {

}

mod expressions {

}