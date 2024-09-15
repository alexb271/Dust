use crate::builtin::TYPEID_DYN;
use crate::error::{Context, Error, ErrorKind};
use crate::expression::Expression;
use crate::function::{BuiltinFunction, Function};
use crate::session::{ExecSession, ParseSession};
use crate::variable::{AnnotatedIdentifier, Value, Variable};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    is_public: bool,
    id: AnnotatedIdentifier,
    init_expression: Expression,

    context: Context,
    assign_pos: usize,
}

impl PropertyDefinition {
    #[inline]
    pub fn new(
        is_public: bool,
        id: AnnotatedIdentifier,
        init_expression: Expression,
        context: Context,
        assign_pos: usize,
    ) -> PropertyDefinition {
        PropertyDefinition {
            is_public,
            id,
            init_expression,
            context,
            assign_pos,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassFunction {
    function: Function,
    uses_self: bool,
    is_public: bool,
}

impl ClassFunction {
    #[inline]
    pub fn new(function: Function, uses_self: bool, is_public: bool) -> ClassFunction {
        ClassFunction {
            function,
            uses_self,
            is_public,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassDefinition {
    property_definitions: Vec<PropertyDefinition>,
    function_definitions: HashMap<String, ClassFunction>,
    typeid: usize,

    context: Context,
}

impl ClassDefinition {
    #[inline]
    pub fn new(
        constructor_parameters: Vec<AnnotatedIdentifier>,
        property_definitions: Vec<PropertyDefinition>,
        mut function_definitions: HashMap<String, ClassFunction>,
        typeid: usize,
        context: Context,
    ) -> ClassDefinition {
        function_definitions.insert(
            "new".to_string(),
            ClassFunction::new(
                Function::BuiltinFunction(BuiltinFunction::new(
                    constructor_parameters,
                    class_constructor,
                )),
                false,
                true,
            ),
        );

        ClassDefinition {
            property_definitions,
            function_definitions,
            typeid,
            context,
        }
    }

    #[inline]
    pub fn new_without_constructor(
        function_definitions: HashMap<String, ClassFunction>,
        typeid: usize,
    ) -> ClassDefinition {
        ClassDefinition {
            property_definitions: Vec::new(),
            function_definitions,
            typeid,
            context: Context { start: 0, end: 0 },
        }
    }

    #[inline]
    pub fn property_definitions(&self) -> &Vec<PropertyDefinition> {
        &self.property_definitions
    }

    #[inline]
    pub fn get_function(
        &self,
        name: &str,
        is_member: bool,
        private_access: bool,
    ) -> Result<&Function, ErrorKind> {
        match self.function_definitions.get(name) {
            Some(classfunc) => match is_member {
                true => {
                    if classfunc.uses_self {
                        if !classfunc.is_public && !private_access {
                            Err(ErrorKind::MemberFunctionIsPrivate(name.to_string()))
                        } else {
                            Ok(&classfunc.function)
                        }
                    } else {
                        Err(ErrorKind::FunctionNotFound)
                    }
                }
                false => {
                    if classfunc.uses_self {
                        Err(ErrorKind::FunctionNotFound)
                    } else {
                        if !classfunc.is_public && !private_access {
                            Err(ErrorKind::MemberFunctionIsPrivate(name.to_string()))
                        } else {
                            Ok(&classfunc.function)
                        }
                    }
                }
            },
            None => Err(ErrorKind::FunctionNotFound),
        }
    }

    #[inline]
    pub fn typeid(&self) -> usize {
        self.typeid
    }

    #[inline]
    pub fn context(&self) -> Context {
        self.context
    }
}

#[derive(Debug, Clone)]
struct Property {
    var: Variable,
    is_public: bool,
}

impl Property {
    #[inline]
    fn new(var: Variable, is_public: bool) -> Property {
        Property { var, is_public }
    }
}

#[derive(Debug, Clone)]
pub struct ClassInstance {
    typeid: usize,
    properties: HashMap<String, Property>,
}

impl ClassInstance {
    #[inline]
    pub fn new(typeid: usize) -> ClassInstance {
        ClassInstance {
            typeid,
            properties: HashMap::new(),
        }
    }

    #[inline]
    pub fn typeid(&self) -> usize {
        self.typeid
    }

    #[inline]
    pub fn add_property(&mut self, name: &str, var: Variable, is_public: bool) {
        self.properties
            .insert(name.to_string(), Property::new(var, is_public));
    }

    #[inline]
    pub fn get_property(
        &self,
        name: &str,
        private_access: bool,
        parse_session: &ParseSession,
    ) -> Result<&Variable, ErrorKind> {
        match self.properties.get(name) {
            Some(prop) => {
                if !prop.is_public && !private_access {
                    Err(ErrorKind::MemberIsPrivate(name.to_string()))
                } else {
                    Ok(&prop.var)
                }
            }
            None => Err(ErrorKind::HasNoMember(
                parse_session.get_typename(self.typeid()),
                name.to_string(),
            )),
        }
    }

    #[inline]
    pub fn set_property(
        &mut self,
        name: &str,
        private_access: bool,
        value: Value,
        session: &ParseSession,
    ) -> Result<(), ErrorKind> {
        match self.properties.get_mut(name) {
            Some(prop) => {
                if !prop.is_public && !private_access {
                    return Err(ErrorKind::MemberIsPrivate(name.to_string()));
                }
                if prop.var.is_dynamic() || prop.var.get_value().typeid() == value.typeid() {
                    prop.var.set_value(value);
                    Ok(())
                } else {
                    Err(ErrorKind::InvalidAssignment(
                        session.get_typename(prop.var.get_value().typeid()),
                        session.get_typename(value.typeid()),
                    ))
                }
            }
            None => Err(ErrorKind::HasNoMember(
                session.get_typename(self.typeid),
                name.to_string(),
            )),
        }
    }
}

fn class_constructor(
    exec_session: &mut ExecSession,
    parse_session: &ParseSession,
    _: Context,
    _: usize,
) -> Result<Value, Error> {
    let typeid = match exec_session.get_variable("#").unwrap().get_value_clone() {
        Value::Int(id) => id as usize,
        _ => panic!("Invalid value in built-in function"),
    };

    let class_definition = parse_session.get_class_definition(typeid).unwrap();
    let mut class_instance = ClassInstance::new(typeid);

    for item in class_definition.property_definitions() {
        let value = item
            .init_expression
            .exec(exec_session, parse_session)?
            .expect("Expressions should always return a value on success");

        if item.id.typeid() == TYPEID_DYN {
            class_instance.add_property(item.id.name(), Variable::new(value, true), item.is_public);
        } else if item.id.typeid() == value.typeid() {
            class_instance.add_property(
                item.id.name(),
                Variable::new(value, false),
                item.is_public,
            );
        } else {
            return Err(Error::new(
                item.context,
                item.assign_pos,
                ErrorKind::InvalidAssignment(
                    parse_session.get_typename(item.id.typeid()),
                    parse_session.get_typename(value.typeid()),
                ),
            ));
        }
    }

    Ok(Value::new_class_instance(class_instance))
}
