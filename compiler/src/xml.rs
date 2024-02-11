// xml backend for Jack language

use crate::parser::{self, expressions, structures::{ReturnType, VarTypeEnum}, Node};
use xml::writer::{EventWriter, XmlEvent};

pub fn convert_node(node: Node) -> String {
    let res = match node {
        Node::Class(ref class) => convert_class(class),
        _ => unimplemented!(),
    };
    let mut output: Vec<u8> = Vec::new();
    let mut writer =
        EventWriter::new_with_config(&mut output, xml::EmitterConfig::new().perform_indent(true));
    // set indentation
    for event in res {
        writer.write(event).unwrap();
    }
    String::from_utf8(output).unwrap()
}

fn wrap_xml<'a>(tag: &'a str, content: &'a str) -> Vec<XmlEvent<'a>> {
    let mut res = vec![];
    res.push(XmlEvent::start_element(tag).into());
    res.push(XmlEvent::characters(content).into());
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_class(class: &parser::structures::Class) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("class").into());

    res.extend(wrap_xml("keyword", "class"));

    res.extend(wrap_xml("identifier", &class.class_name.0));
    res.extend(wrap_xml("symbol", "{"));
    class.class_var_dec.iter().for_each(|var_dec| {
        res.extend(convert_class_var_dec(var_dec));
    });

    class.subroutine_dec.iter().for_each(|subroutine_dec| {
        res.extend(convert_subroutine_dec(subroutine_dec));
    });
    res.extend(wrap_xml("symbol", "}"));

    res.push(XmlEvent::end_element().into());
    res
}

fn convert_vartype(var_type: &VarTypeEnum) -> Vec<XmlEvent> {
    let mut res  = vec![];
    match var_type {
        VarTypeEnum::BOOLEAN => {
            res.extend(wrap_xml("keyword", "bool"));
        },
        VarTypeEnum::INT => {
            res.extend(wrap_xml("keyword", "int"));
        }
        VarTypeEnum::CHAR => {
            res.extend(wrap_xml("keyword", "char"));
        }
        VarTypeEnum::CLASSNAME(name) => {
            res.extend(wrap_xml("identifier", &name.0));
        }
    }
    res
}

fn convert_class_var_dec(class_var_dec: &parser::structures::ClassVarDec) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("classVarDec").into());
    res.extend(wrap_xml(
        "keyword",
        match class_var_dec.var_dec_type {
            parser::structures::ClassVarDecType::STATIC => "static",
            parser::structures::ClassVarDecType::FIELD => "field",
        },
    ));
    res.extend(convert_vartype(&class_var_dec.var_type.0));
    for i in 0..class_var_dec.var_names.len() {
        res.extend(wrap_xml("identifier", &class_var_dec.var_names[i].0));
        if i != class_var_dec.var_names.len() - 1 {
            res.extend(wrap_xml("symbol", ","));
        }
    }
    res.extend(wrap_xml("symbol", ";"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_subroutine_dec(subroutine_dec: &parser::structures::SubroutineDec) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("subroutineDec").into());
    res.extend(wrap_xml(
        "keyword",
        match subroutine_dec.subroutine_type {
            parser::structures::SubroutineType::CONSTRUCTOR => "constructor",
            parser::structures::SubroutineType::FUNCTION => "function",
            parser::structures::SubroutineType::METHOD => "method",
        },
    ));
    match &subroutine_dec.return_type {
        ReturnType::VOID => {
            res.extend(wrap_xml("keyword", "void"));
        }
        ReturnType::VARTYPE(vartype) => {
            res.extend(convert_vartype(&vartype.0));
        }
    }
    res.extend(wrap_xml("identifier", &subroutine_dec.subroutine_name.0));
    res.extend(wrap_xml("symbol", "("));
    res.extend(convert_parameter_list(&subroutine_dec.parameter_list));
    res.extend(wrap_xml("symbol", ")"));
    res.extend(convert_subroutine_body(&subroutine_dec.subroutine_body));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_parameter_list(param_list: &parser::structures::ParameterList) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("parameterList").into());
    for (i, param) in param_list.parameters.iter().enumerate() {
        res.extend(wrap_xml(
            "keyword",
            match param.0 .0 {
                VarTypeEnum::INT => "int",
                VarTypeEnum::BOOLEAN => "boolean",
                VarTypeEnum::CHAR => "char",
                VarTypeEnum::CLASSNAME(ref name) => &name.0,
            },
        ));
        res.extend(wrap_xml("identifier", &param.1 .0));
        if i != param_list.parameters.len() - 1 {
            res.extend(wrap_xml("symbol", ","));
        }
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_term<'a>(term: &'a expressions::Term) -> Vec<XmlEvent<'a>> {
    let mut res: Vec<XmlEvent<'a>> = vec![];
    res.push(XmlEvent::start_element("term").into());
    match term {
        expressions::Term::IntegerConstant(int) => {
            res.extend(wrap_xml("integerConstant", int.0.to_string().leak()));
        }
        expressions::Term::StringConstant(string) => {
            res.extend(wrap_xml("stringConstant", &string.0));
        }
        expressions::Term::KeywordConstant(keyword) => {
            res.extend(wrap_xml(
                "keyword",
                match keyword {
                    parser::expressions::KeywordConstant::TRUE => "true",
                    parser::expressions::KeywordConstant::FALSE => "false",
                    parser::expressions::KeywordConstant::NULL => "null",
                    parser::expressions::KeywordConstant::THIS => "this",
                },
            ));
        }
        expressions::Term::VarName(name) => {
            res.extend(wrap_xml("identifier", &name.0));
        }
        expressions::Term::ArrayTerm(ref array_term) => {
            let name = &array_term.var_name;
            let exp = &array_term.expression;
            res.extend(wrap_xml("identifier", &name.0));
            res.extend(wrap_xml("symbol", "["));
            res.extend(convert_expression(exp));
            res.extend(wrap_xml("symbol", "]"));
        }
        expressions::Term::SubroutineCall(subroutine_call) => {
            res.extend(convert_subroutine_call(subroutine_call));
        }
        expressions::Term::WrappedExpression(exp) => {
            res.extend(wrap_xml("symbol", "("));
            res.extend(convert_expression(&exp.0));
            res.extend(wrap_xml("symbol", ")"));
        }
        expressions::Term::UnaryTerm(uterm) => {
            let op = &uterm.unary_op;
            let term = &uterm.term;
            res.extend(wrap_xml("symbol", op.0.0.to_string().leak()));
            res.extend(convert_term(term));
        }
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_expression(exp: &expressions::Expression) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("expression").into());
    res.extend(convert_term(&exp.term));
    for i in 0..exp.op_term.len() {
        res.extend(wrap_xml(
            "symbol",
            exp.op_term[i].0 .0 .0.to_string().leak(),
        ));
        res.extend(convert_term(&exp.op_term[i].1));
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_subroutine_call(call: &expressions::SubroutineCall) -> Vec<XmlEvent> {
    let mut res = vec![];
    // res.push(XmlEvent::start_element("subroutineCall").into());
    if call.bind_this.is_some() {
        res.extend(wrap_xml("identifier", &call.bind_this.as_ref().unwrap().0));
        res.extend(wrap_xml("symbol", "."));
    }
    res.extend(wrap_xml("identifier", &call.subroutine_name.0));
    res.extend(wrap_xml("symbol", "("));
    res.extend(convert_expression_list(&call.expression_list));
    res.extend(wrap_xml("symbol", ")"));
    // res.push(XmlEvent::end_element().into());
    res
}

fn convert_expression_list(exp_list: &expressions::ExpressionList) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("expressionList").into());
    for (i, exp) in exp_list.0.iter().enumerate() {
        res.extend(convert_expression(exp));
        if i != exp_list.0.len() - 1 {
            res.extend(wrap_xml("symbol", ","));
        }
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_subroutine_body(body: &parser::structures::SubroutineBody) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("subroutineBody").into());
    res.extend(wrap_xml("symbol", "{"));
    res.extend(convert_var_decs(&body.var_decs));
    res.extend(convert_statements(&body.statements));
    res.extend(wrap_xml("symbol", "}"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_var_decs(var_decs: &Vec<parser::structures::VarDec>) -> Vec<XmlEvent> {
    let mut res = vec![];
    for var_dec in var_decs {
        res.extend(convert_var_dec(var_dec));
    }
    res
}

fn convert_var_dec(var_dec: &parser::structures::VarDec) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("varDec").into());
    res.extend(wrap_xml("keyword", "var"));
    match var_dec.var_type.0 {
        VarTypeEnum::INT => {res.extend(wrap_xml("keyword","int"));}
        VarTypeEnum::BOOLEAN => {res.extend(wrap_xml("keyword", "boolean"));}
        VarTypeEnum::CHAR => {
            res.extend(wrap_xml("keyword", "char"));
        }
        VarTypeEnum::CLASSNAME(ref name) => {
            res.extend(wrap_xml("identifier", &name.0));
        }
    }
    for i in 0..var_dec.var_names.len() {
        res.extend(wrap_xml("identifier", &var_dec.var_names[i].0));
        if i != var_dec.var_names.len() - 1 {
            res.extend(wrap_xml("symbol", ","));
        }
    }
    res.extend(wrap_xml("symbol", ";"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_statements(statements: &parser::statements::Statements) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("statements").into());
    for statement in &statements.0 {
        res.extend(match statement {
            parser::statements::Statement::LetStatement(let_statement) => {
                convert_let_statement(let_statement)
            }
            parser::statements::Statement::IfStatement(if_statement) => {
                convert_if_statement(if_statement)
            }
            parser::statements::Statement::WhileStatement(while_statement) => {
                convert_while_statement(while_statement)
            }
            parser::statements::Statement::DoStatement(do_statement) => {
                convert_do_statement(do_statement)
            }
            parser::statements::Statement::ReturnStatement(return_statement) => {
                convert_return_statement(return_statement)
            }
        });
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_let_statement(let_statement: &parser::statements::LetStatement) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("letStatement").into());
    res.extend(wrap_xml("keyword", "let"));
    match &let_statement.let_lhs {
        parser::statements::LetLHS::VarName(name) => {
            res.extend(wrap_xml("identifier", &name.0));
        }
        parser::statements::LetLHS::ArrayTerm(term) => {
            res.extend(wrap_xml("identifier", &term.var_name.0));
            res.extend(wrap_xml("symbol", "["));
            res.extend(convert_expression(&term.expression));
            res.extend(wrap_xml("symbol", "]"));
        }
    }
    res.extend(wrap_xml("symbol", "="));
    res.extend(convert_expression(&let_statement.let_rhs));
    res.extend(wrap_xml("symbol", ";"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_if_statement(if_statement: &parser::statements::IfStatement) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("ifStatement").into());
    res.extend(wrap_xml("keyword", "if"));
    res.extend(wrap_xml("symbol", "("));
    res.extend(convert_expression(&if_statement.condition));
    res.extend(wrap_xml("symbol", ")"));
    res.extend(wrap_xml("symbol", "{"));
    res.extend(convert_statements(&if_statement.true_statements));
    res.extend(wrap_xml("symbol", "}"));
    if let Some(else_statements) = &if_statement.false_statements {
        res.extend(wrap_xml("keyword", "else"));
        res.extend(wrap_xml("symbol", "{"));
        res.extend(convert_statements(else_statements));
        res.extend(wrap_xml("symbol", "}"));
    }
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_while_statement(while_statement: &parser::statements::WhileStatement) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("whileStatement").into());
    res.extend(wrap_xml("keyword", "while"));
    res.extend(wrap_xml("symbol", "("));
    res.extend(convert_expression(&while_statement.condition));
    res.extend(wrap_xml("symbol", ")"));
    res.extend(wrap_xml("symbol", "{"));
    res.extend(convert_statements(&while_statement.statements));
    res.extend(wrap_xml("symbol", "}"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_do_statement(do_statement: &parser::statements::DoStatement) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("doStatement").into());
    res.extend(wrap_xml("keyword", "do"));
    res.extend(convert_subroutine_call(&do_statement.subroutine_call));
    res.extend(wrap_xml("symbol", ";"));
    res.push(XmlEvent::end_element().into());
    res
}

fn convert_return_statement(
    return_statement: &parser::statements::ReturnStatement,
) -> Vec<XmlEvent> {
    let mut res = vec![];
    res.push(XmlEvent::start_element("returnStatement").into());
    res.extend(wrap_xml("keyword", "return"));
    if let Some(exp) = &return_statement.expression {
        res.extend(convert_expression(exp));
    }
    res.extend(wrap_xml("symbol", ";"));
    res.push(XmlEvent::end_element().into());
    res
}
