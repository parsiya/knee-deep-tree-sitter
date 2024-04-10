#[doc = "Typed node `_expression`\n\nThis node type is a supertype of:\n- `binary_expression` ([BinaryExpression])\n- `call_expression` ([CallExpression])\n- `composite_literal` ([CompositeLiteral])\n- `false` ([False])\n- `float_literal` ([FloatLiteral])\n- `func_literal` ([FuncLiteral])\n- `identifier` ([Identifier])\n- `imaginary_literal` ([ImaginaryLiteral])\n- `index_expression` ([IndexExpression])\n- `int_literal` ([IntLiteral])\n- `interpreted_string_literal` ([InterpretedStringLiteral])\n- `iota` ([Iota])\n- `nil` ([Nil])\n- `parenthesized_expression` ([ParenthesizedExpression])\n- `raw_string_literal` ([RawStringLiteral])\n- `rune_literal` ([RuneLiteral])\n- `selector_expression` ([SelectorExpression])\n- `slice_expression` ([SliceExpression])\n- `true` ([True])\n- `type_assertion_expression` ([TypeAssertionExpression])\n- `type_conversion_expression` ([TypeConversionExpression])\n- `type_instantiation_expression` ([TypeInstantiationExpression])\n- `unary_expression` ([UnaryExpression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Expression<'tree> {
    BinaryExpression(BinaryExpression<'tree>),
    CallExpression(CallExpression<'tree>),
    CompositeLiteral(CompositeLiteral<'tree>),
    False(False<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    FuncLiteral(FuncLiteral<'tree>),
    Identifier(Identifier<'tree>),
    ImaginaryLiteral(ImaginaryLiteral<'tree>),
    IndexExpression(IndexExpression<'tree>),
    IntLiteral(IntLiteral<'tree>),
    InterpretedStringLiteral(InterpretedStringLiteral<'tree>),
    Iota(Iota<'tree>),
    Nil(Nil<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    RuneLiteral(RuneLiteral<'tree>),
    SelectorExpression(SelectorExpression<'tree>),
    SliceExpression(SliceExpression<'tree>),
    True(True<'tree>),
    TypeAssertionExpression(TypeAssertionExpression<'tree>),
    TypeConversionExpression(TypeConversionExpression<'tree>),
    TypeInstantiationExpression(TypeInstantiationExpression<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
}
#[automatically_derived]
impl<'tree> Expression<'tree> {
    #[doc = "Returns the node if it is of kind `binary_expression` ([BinaryExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn binary_expression(self) -> Option<BinaryExpression<'tree>> {
        match self {
            Self::BinaryExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `call_expression` ([CallExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn call_expression(self) -> Option<CallExpression<'tree>> {
        match self {
            Self::CallExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `composite_literal` ([CompositeLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn composite_literal(self) -> Option<CompositeLiteral<'tree>> {
        match self {
            Self::CompositeLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `false` ([False]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn r#false(self) -> Option<False<'tree>> {
        match self {
            Self::False(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `float_literal` ([FloatLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
        match self {
            Self::FloatLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `func_literal` ([FuncLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn func_literal(self) -> Option<FuncLiteral<'tree>> {
        match self {
            Self::FuncLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        match self {
            Self::Identifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `imaginary_literal` ([ImaginaryLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn imaginary_literal(self) -> Option<ImaginaryLiteral<'tree>> {
        match self {
            Self::ImaginaryLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `index_expression` ([IndexExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn index_expression(self) -> Option<IndexExpression<'tree>> {
        match self {
            Self::IndexExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `int_literal` ([IntLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn int_literal(self) -> Option<IntLiteral<'tree>> {
        match self {
            Self::IntLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `interpreted_string_literal` ([InterpretedStringLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn interpreted_string_literal(self) -> Option<InterpretedStringLiteral<'tree>> {
        match self {
            Self::InterpretedStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `iota` ([Iota]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn iota(self) -> Option<Iota<'tree>> {
        match self {
            Self::Iota(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `nil` ([Nil]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn nil(self) -> Option<Nil<'tree>> {
        match self {
            Self::Nil(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `parenthesized_expression` ([ParenthesizedExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn parenthesized_expression(self) -> Option<ParenthesizedExpression<'tree>> {
        match self {
            Self::ParenthesizedExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `raw_string_literal` ([RawStringLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
        match self {
            Self::RawStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `rune_literal` ([RuneLiteral]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn rune_literal(self) -> Option<RuneLiteral<'tree>> {
        match self {
            Self::RuneLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `selector_expression` ([SelectorExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn selector_expression(self) -> Option<SelectorExpression<'tree>> {
        match self {
            Self::SelectorExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `slice_expression` ([SliceExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn slice_expression(self) -> Option<SliceExpression<'tree>> {
        match self {
            Self::SliceExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `true` ([True]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn r#true(self) -> Option<True<'tree>> {
        match self {
            Self::True(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_assertion_expression` ([TypeAssertionExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_assertion_expression(self) -> Option<TypeAssertionExpression<'tree>> {
        match self {
            Self::TypeAssertionExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_conversion_expression` ([TypeConversionExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_conversion_expression(self) -> Option<TypeConversionExpression<'tree>> {
        match self {
            Self::TypeConversionExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_instantiation_expression` ([TypeInstantiationExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_instantiation_expression(self) -> Option<TypeInstantiationExpression<'tree>> {
        match self {
            Self::TypeInstantiationExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `unary_expression` ([UnaryExpression]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unary_expression(self) -> Option<UnaryExpression<'tree>> {
        match self {
            Self::UnaryExpression(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Expression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "binary_expression" => {
                Ok(unsafe {
                    Self :: BinaryExpression (< BinaryExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "call_expression" => Ok(unsafe {
                Self::CallExpression(<CallExpression<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "composite_literal" => {
                Ok(unsafe {
                    Self :: CompositeLiteral (< CompositeLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "false" => Ok(unsafe {
                Self::False(
                    <False<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node),
                )
            }),
            "float_literal" => {
                Ok(unsafe {
                    Self :: FloatLiteral (< FloatLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "func_literal" => {
                Ok(unsafe {
                    Self :: FuncLiteral (< FuncLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "identifier" => {
                Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "imaginary_literal" => {
                Ok(unsafe {
                    Self :: ImaginaryLiteral (< ImaginaryLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "index_expression" => Ok(unsafe {
                Self::IndexExpression(<IndexExpression<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "int_literal" => {
                Ok(unsafe {
                    Self :: IntLiteral (< IntLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "interpreted_string_literal" => Ok(unsafe {
                Self :: InterpretedStringLiteral (< InterpretedStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "iota" => Ok(unsafe {
                Self::Iota(
                    <Iota<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node),
                )
            }),
            "nil" => Ok(unsafe {
                Self::Nil(
                    <Nil<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node),
                )
            }),
            "parenthesized_expression" => Ok(unsafe {
                Self :: ParenthesizedExpression (< ParenthesizedExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "raw_string_literal" => {
                Ok(unsafe {
                    Self :: RawStringLiteral (< RawStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "rune_literal" => {
                Ok(unsafe {
                    Self :: RuneLiteral (< RuneLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "selector_expression" => Ok(unsafe {
                Self :: SelectorExpression (< SelectorExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "slice_expression" => Ok(unsafe {
                Self::SliceExpression(<SliceExpression<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "true" => Ok(unsafe {
                Self::True(
                    <True<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node),
                )
            }),
            "type_assertion_expression" => Ok(unsafe {
                Self :: TypeAssertionExpression (< TypeAssertionExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "type_conversion_expression" => Ok(unsafe {
                Self :: TypeConversionExpression (< TypeConversionExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "type_instantiation_expression" => Ok(unsafe {
                Self :: TypeInstantiationExpression (< TypeInstantiationExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "unary_expression" => Ok(unsafe {
                Self::UnaryExpression(<UnaryExpression<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression<'tree> {
    const KIND: &'static str = "_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        match self {
            Self::BinaryExpression(x) => x.node(),
            Self::CallExpression(x) => x.node(),
            Self::CompositeLiteral(x) => x.node(),
            Self::False(x) => x.node(),
            Self::FloatLiteral(x) => x.node(),
            Self::FuncLiteral(x) => x.node(),
            Self::Identifier(x) => x.node(),
            Self::ImaginaryLiteral(x) => x.node(),
            Self::IndexExpression(x) => x.node(),
            Self::IntLiteral(x) => x.node(),
            Self::InterpretedStringLiteral(x) => x.node(),
            Self::Iota(x) => x.node(),
            Self::Nil(x) => x.node(),
            Self::ParenthesizedExpression(x) => x.node(),
            Self::RawStringLiteral(x) => x.node(),
            Self::RuneLiteral(x) => x.node(),
            Self::SelectorExpression(x) => x.node(),
            Self::SliceExpression(x) => x.node(),
            Self::True(x) => x.node(),
            Self::TypeAssertionExpression(x) => x.node(),
            Self::TypeConversionExpression(x) => x.node(),
            Self::TypeInstantiationExpression(x) => x.node(),
            Self::UnaryExpression(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        match self {
            Self::BinaryExpression(x) => x.node_mut(),
            Self::CallExpression(x) => x.node_mut(),
            Self::CompositeLiteral(x) => x.node_mut(),
            Self::False(x) => x.node_mut(),
            Self::FloatLiteral(x) => x.node_mut(),
            Self::FuncLiteral(x) => x.node_mut(),
            Self::Identifier(x) => x.node_mut(),
            Self::ImaginaryLiteral(x) => x.node_mut(),
            Self::IndexExpression(x) => x.node_mut(),
            Self::IntLiteral(x) => x.node_mut(),
            Self::InterpretedStringLiteral(x) => x.node_mut(),
            Self::Iota(x) => x.node_mut(),
            Self::Nil(x) => x.node_mut(),
            Self::ParenthesizedExpression(x) => x.node_mut(),
            Self::RawStringLiteral(x) => x.node_mut(),
            Self::RuneLiteral(x) => x.node_mut(),
            Self::SelectorExpression(x) => x.node_mut(),
            Self::SliceExpression(x) => x.node_mut(),
            Self::True(x) => x.node_mut(),
            Self::TypeAssertionExpression(x) => x.node_mut(),
            Self::TypeConversionExpression(x) => x.node_mut(),
            Self::TypeInstantiationExpression(x) => x.node_mut(),
            Self::UnaryExpression(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        match self {
            Self::BinaryExpression(x) => x.into_node(),
            Self::CallExpression(x) => x.into_node(),
            Self::CompositeLiteral(x) => x.into_node(),
            Self::False(x) => x.into_node(),
            Self::FloatLiteral(x) => x.into_node(),
            Self::FuncLiteral(x) => x.into_node(),
            Self::Identifier(x) => x.into_node(),
            Self::ImaginaryLiteral(x) => x.into_node(),
            Self::IndexExpression(x) => x.into_node(),
            Self::IntLiteral(x) => x.into_node(),
            Self::InterpretedStringLiteral(x) => x.into_node(),
            Self::Iota(x) => x.into_node(),
            Self::Nil(x) => x.into_node(),
            Self::ParenthesizedExpression(x) => x.into_node(),
            Self::RawStringLiteral(x) => x.into_node(),
            Self::RuneLiteral(x) => x.into_node(),
            Self::SelectorExpression(x) => x.into_node(),
            Self::SliceExpression(x) => x.into_node(),
            Self::True(x) => x.into_node(),
            Self::TypeAssertionExpression(x) => x.into_node(),
            Self::TypeConversionExpression(x) => x.into_node(),
            Self::TypeInstantiationExpression(x) => x.into_node(),
            Self::UnaryExpression(x) => x.into_node(),
        }
    }
}
#[doc = "Typed node `_simple_statement`\n\nThis node type is a supertype of:\n- `assignment_statement` ([AssignmentStatement])\n- `dec_statement` ([DecStatement])\n- `expression_statement` ([ExpressionStatement])\n- `inc_statement` ([IncStatement])\n- `send_statement` ([SendStatement])\n- `short_var_declaration` ([ShortVarDeclaration])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum SimpleStatement<'tree> {
    AssignmentStatement(AssignmentStatement<'tree>),
    DecStatement(DecStatement<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    IncStatement(IncStatement<'tree>),
    SendStatement(SendStatement<'tree>),
    ShortVarDeclaration(ShortVarDeclaration<'tree>),
}
#[automatically_derived]
impl<'tree> SimpleStatement<'tree> {
    #[doc = "Returns the node if it is of kind `assignment_statement` ([AssignmentStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn assignment_statement(self) -> Option<AssignmentStatement<'tree>> {
        match self {
            Self::AssignmentStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `dec_statement` ([DecStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn dec_statement(self) -> Option<DecStatement<'tree>> {
        match self {
            Self::DecStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `expression_statement` ([ExpressionStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn expression_statement(self) -> Option<ExpressionStatement<'tree>> {
        match self {
            Self::ExpressionStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `inc_statement` ([IncStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn inc_statement(self) -> Option<IncStatement<'tree>> {
        match self {
            Self::IncStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `send_statement` ([SendStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn send_statement(self) -> Option<SendStatement<'tree>> {
        match self {
            Self::SendStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `short_var_declaration` ([ShortVarDeclaration]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn short_var_declaration(self) -> Option<ShortVarDeclaration<'tree>> {
        match self {
            Self::ShortVarDeclaration(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SimpleStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "assignment_statement" => Ok(unsafe {
                Self :: AssignmentStatement (< AssignmentStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "dec_statement" => {
                Ok(unsafe {
                    Self :: DecStatement (< DecStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "expression_statement" => Ok(unsafe {
                Self :: ExpressionStatement (< ExpressionStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            "inc_statement" => {
                Ok(unsafe {
                    Self :: IncStatement (< IncStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "send_statement" => Ok(unsafe {
                Self::SendStatement(<SendStatement<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "short_var_declaration" => Ok(unsafe {
                Self :: ShortVarDeclaration (< ShortVarDeclaration < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SimpleStatement<'tree> {
    const KIND: &'static str = "_simple_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        match self {
            Self::AssignmentStatement(x) => x.node(),
            Self::DecStatement(x) => x.node(),
            Self::ExpressionStatement(x) => x.node(),
            Self::IncStatement(x) => x.node(),
            Self::SendStatement(x) => x.node(),
            Self::ShortVarDeclaration(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        match self {
            Self::AssignmentStatement(x) => x.node_mut(),
            Self::DecStatement(x) => x.node_mut(),
            Self::ExpressionStatement(x) => x.node_mut(),
            Self::IncStatement(x) => x.node_mut(),
            Self::SendStatement(x) => x.node_mut(),
            Self::ShortVarDeclaration(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        match self {
            Self::AssignmentStatement(x) => x.into_node(),
            Self::DecStatement(x) => x.into_node(),
            Self::ExpressionStatement(x) => x.into_node(),
            Self::IncStatement(x) => x.into_node(),
            Self::SendStatement(x) => x.into_node(),
            Self::ShortVarDeclaration(x) => x.into_node(),
        }
    }
}
#[doc = "Typed node `_simple_type`\n\nThis node type is a supertype of:\n- `array_type` ([ArrayType])\n- `channel_type` ([ChannelType])\n- `function_type` ([FunctionType])\n- `generic_type` ([GenericType])\n- `interface_type` ([InterfaceType])\n- `map_type` ([MapType])\n- `negated_type` ([NegatedType])\n- `pointer_type` ([PointerType])\n- `qualified_type` ([QualifiedType])\n- `slice_type` ([SliceType])\n- `struct_type` ([StructType])\n- `type_identifier` ([TypeIdentifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum SimpleType<'tree> {
    ArrayType(ArrayType<'tree>),
    ChannelType(ChannelType<'tree>),
    FunctionType(FunctionType<'tree>),
    GenericType(GenericType<'tree>),
    InterfaceType(InterfaceType<'tree>),
    MapType(MapType<'tree>),
    NegatedType(NegatedType<'tree>),
    PointerType(PointerType<'tree>),
    QualifiedType(QualifiedType<'tree>),
    SliceType(SliceType<'tree>),
    StructType(StructType<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
}
#[automatically_derived]
impl<'tree> SimpleType<'tree> {
    #[doc = "Returns the node if it is of kind `array_type` ([ArrayType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array_type(self) -> Option<ArrayType<'tree>> {
        match self {
            Self::ArrayType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `channel_type` ([ChannelType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn channel_type(self) -> Option<ChannelType<'tree>> {
        match self {
            Self::ChannelType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `function_type` ([FunctionType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_type(self) -> Option<FunctionType<'tree>> {
        match self {
            Self::FunctionType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `generic_type` ([GenericType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn generic_type(self) -> Option<GenericType<'tree>> {
        match self {
            Self::GenericType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `interface_type` ([InterfaceType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn interface_type(self) -> Option<InterfaceType<'tree>> {
        match self {
            Self::InterfaceType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `map_type` ([MapType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn map_type(self) -> Option<MapType<'tree>> {
        match self {
            Self::MapType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `negated_type` ([NegatedType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn negated_type(self) -> Option<NegatedType<'tree>> {
        match self {
            Self::NegatedType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `pointer_type` ([PointerType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn pointer_type(self) -> Option<PointerType<'tree>> {
        match self {
            Self::PointerType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `qualified_type` ([QualifiedType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn qualified_type(self) -> Option<QualifiedType<'tree>> {
        match self {
            Self::QualifiedType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `slice_type` ([SliceType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn slice_type(self) -> Option<SliceType<'tree>> {
        match self {
            Self::SliceType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `struct_type` ([StructType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_type(self) -> Option<StructType<'tree>> {
        match self {
            Self::StructType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
        match self {
            Self::TypeIdentifier(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SimpleType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "array_type" => {
                Ok(unsafe {
                    Self :: ArrayType (< ArrayType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "channel_type" => {
                Ok(unsafe {
                    Self :: ChannelType (< ChannelType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "function_type" => {
                Ok(unsafe {
                    Self :: FunctionType (< FunctionType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "generic_type" => {
                Ok(unsafe {
                    Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "interface_type" => Ok(unsafe {
                Self::InterfaceType(<InterfaceType<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "map_type" => Ok(unsafe {
                Self::MapType(
                    <MapType<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                        node,
                    ),
                )
            }),
            "negated_type" => {
                Ok(unsafe {
                    Self :: NegatedType (< NegatedType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "pointer_type" => {
                Ok(unsafe {
                    Self :: PointerType (< PointerType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "qualified_type" => Ok(unsafe {
                Self::QualifiedType(<QualifiedType<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            "slice_type" => {
                Ok(unsafe {
                    Self :: SliceType (< SliceType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "struct_type" => {
                Ok(unsafe {
                    Self :: StructType (< StructType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                })
            }
            "type_identifier" => Ok(unsafe {
                Self::TypeIdentifier(<TypeIdentifier<'tree> as type_sitter_lib::TypedNode<
                    'tree,
                >>::from_node_unchecked(node))
            }),
            _ => Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            }),
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SimpleType<'tree> {
    const KIND: &'static str = "_simple_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        match self {
            Self::ArrayType(x) => x.node(),
            Self::ChannelType(x) => x.node(),
            Self::FunctionType(x) => x.node(),
            Self::GenericType(x) => x.node(),
            Self::InterfaceType(x) => x.node(),
            Self::MapType(x) => x.node(),
            Self::NegatedType(x) => x.node(),
            Self::PointerType(x) => x.node(),
            Self::QualifiedType(x) => x.node(),
            Self::SliceType(x) => x.node(),
            Self::StructType(x) => x.node(),
            Self::TypeIdentifier(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        match self {
            Self::ArrayType(x) => x.node_mut(),
            Self::ChannelType(x) => x.node_mut(),
            Self::FunctionType(x) => x.node_mut(),
            Self::GenericType(x) => x.node_mut(),
            Self::InterfaceType(x) => x.node_mut(),
            Self::MapType(x) => x.node_mut(),
            Self::NegatedType(x) => x.node_mut(),
            Self::PointerType(x) => x.node_mut(),
            Self::QualifiedType(x) => x.node_mut(),
            Self::SliceType(x) => x.node_mut(),
            Self::StructType(x) => x.node_mut(),
            Self::TypeIdentifier(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        match self {
            Self::ArrayType(x) => x.into_node(),
            Self::ChannelType(x) => x.into_node(),
            Self::FunctionType(x) => x.into_node(),
            Self::GenericType(x) => x.into_node(),
            Self::InterfaceType(x) => x.into_node(),
            Self::MapType(x) => x.into_node(),
            Self::NegatedType(x) => x.into_node(),
            Self::PointerType(x) => x.into_node(),
            Self::QualifiedType(x) => x.into_node(),
            Self::SliceType(x) => x.into_node(),
            Self::StructType(x) => x.into_node(),
            Self::TypeIdentifier(x) => x.into_node(),
        }
    }
}
#[doc = "Typed node `_statement`\n\nThis node type is a supertype of:\n- `_simple_statement` ([SimpleStatement])\n- `block` ([Block])\n- `break_statement` ([BreakStatement])\n- `const_declaration` ([ConstDeclaration])\n- `continue_statement` ([ContinueStatement])\n- `defer_statement` ([DeferStatement])\n- `empty_statement` ([EmptyStatement])\n- `expression_switch_statement` ([ExpressionSwitchStatement])\n- `fallthrough_statement` ([FallthroughStatement])\n- `for_statement` ([ForStatement])\n- `go_statement` ([GoStatement])\n- `goto_statement` ([GotoStatement])\n- `if_statement` ([IfStatement])\n- `labeled_statement` ([LabeledStatement])\n- `return_statement` ([ReturnStatement])\n- `select_statement` ([SelectStatement])\n- `type_declaration` ([TypeDeclaration])\n- `type_switch_statement` ([TypeSwitchStatement])\n- `var_declaration` ([VarDeclaration])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Statement<'tree> {
    SimpleStatement(SimpleStatement<'tree>),
    Block(Block<'tree>),
    BreakStatement(BreakStatement<'tree>),
    ConstDeclaration(ConstDeclaration<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DeferStatement(DeferStatement<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    ExpressionSwitchStatement(ExpressionSwitchStatement<'tree>),
    FallthroughStatement(FallthroughStatement<'tree>),
    ForStatement(ForStatement<'tree>),
    GoStatement(GoStatement<'tree>),
    GotoStatement(GotoStatement<'tree>),
    IfStatement(IfStatement<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    SelectStatement(SelectStatement<'tree>),
    TypeDeclaration(TypeDeclaration<'tree>),
    TypeSwitchStatement(TypeSwitchStatement<'tree>),
    VarDeclaration(VarDeclaration<'tree>),
}
#[automatically_derived]
impl<'tree> Statement<'tree> {
    #[doc = "Returns the node if it is of kind `_simple_statement` ([SimpleStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn simple_statement(self) -> Option<SimpleStatement<'tree>> {
        match self {
            Self::SimpleStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `block` ([Block]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn block(self) -> Option<Block<'tree>> {
        match self {
            Self::Block(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `break_statement` ([BreakStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn break_statement(self) -> Option<BreakStatement<'tree>> {
        match self {
            Self::BreakStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `const_declaration` ([ConstDeclaration]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_declaration(self) -> Option<ConstDeclaration<'tree>> {
        match self {
            Self::ConstDeclaration(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `continue_statement` ([ContinueStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn continue_statement(self) -> Option<ContinueStatement<'tree>> {
        match self {
            Self::ContinueStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `defer_statement` ([DeferStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn defer_statement(self) -> Option<DeferStatement<'tree>> {
        match self {
            Self::DeferStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `empty_statement` ([EmptyStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn empty_statement(self) -> Option<EmptyStatement<'tree>> {
        match self {
            Self::EmptyStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `expression_switch_statement` ([ExpressionSwitchStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn expression_switch_statement(self) -> Option<ExpressionSwitchStatement<'tree>> {
        match self {
            Self::ExpressionSwitchStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `fallthrough_statement` ([FallthroughStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn fallthrough_statement(self) -> Option<FallthroughStatement<'tree>> {
        match self {
            Self::FallthroughStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `for_statement` ([ForStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn for_statement(self) -> Option<ForStatement<'tree>> {
        match self {
            Self::ForStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `go_statement` ([GoStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn go_statement(self) -> Option<GoStatement<'tree>> {
        match self {
            Self::GoStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `goto_statement` ([GotoStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn goto_statement(self) -> Option<GotoStatement<'tree>> {
        match self {
            Self::GotoStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `if_statement` ([IfStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn if_statement(self) -> Option<IfStatement<'tree>> {
        match self {
            Self::IfStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `labeled_statement` ([LabeledStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn labeled_statement(self) -> Option<LabeledStatement<'tree>> {
        match self {
            Self::LabeledStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `return_statement` ([ReturnStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn return_statement(self) -> Option<ReturnStatement<'tree>> {
        match self {
            Self::ReturnStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `select_statement` ([SelectStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn select_statement(self) -> Option<SelectStatement<'tree>> {
        match self {
            Self::SelectStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_declaration` ([TypeDeclaration]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_declaration(self) -> Option<TypeDeclaration<'tree>> {
        match self {
            Self::TypeDeclaration(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `type_switch_statement` ([TypeSwitchStatement]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_switch_statement(self) -> Option<TypeSwitchStatement<'tree>> {
        match self {
            Self::TypeSwitchStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `var_declaration` ([VarDeclaration]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn var_declaration(self) -> Option<VarDeclaration<'tree>> {
        match self {
            Self::VarDeclaration(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Statement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if let Ok(this) = <SimpleStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::SimpleStatement(this));
        }
        if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::Block(this));
        }
        if let Ok(this) = <BreakStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::BreakStatement(this));
        }
        if let Ok(this) = <ConstDeclaration<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ConstDeclaration(this));
        }
        if let Ok(this) = <ContinueStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ContinueStatement(this));
        }
        if let Ok(this) = <DeferStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::DeferStatement(this));
        }
        if let Ok(this) = <EmptyStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::EmptyStatement(this));
        }
        if let Ok(this) = <ExpressionSwitchStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ExpressionSwitchStatement(this));
        }
        if let Ok(this) = <FallthroughStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::FallthroughStatement(this));
        }
        if let Ok(this) = <ForStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ForStatement(this));
        }
        if let Ok(this) = <GoStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::GoStatement(this));
        }
        if let Ok(this) = <GotoStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::GotoStatement(this));
        }
        if let Ok(this) = <IfStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::IfStatement(this));
        }
        if let Ok(this) = <LabeledStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::LabeledStatement(this));
        }
        if let Ok(this) = <ReturnStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ReturnStatement(this));
        }
        if let Ok(this) = <SelectStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::SelectStatement(this));
        }
        if let Ok(this) = <TypeDeclaration<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TypeDeclaration(this));
        }
        if let Ok(this) = <TypeSwitchStatement<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::TypeSwitchStatement(this));
        }
        if let Ok(this) = <VarDeclaration<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::VarDeclaration(this));
        }
        Err(type_sitter_lib::IncorrectKind {
            node,
            kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
        })
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Statement<'tree> {
    const KIND: &'static str = "_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        match self {
            Self::SimpleStatement(x) => x.node(),
            Self::Block(x) => x.node(),
            Self::BreakStatement(x) => x.node(),
            Self::ConstDeclaration(x) => x.node(),
            Self::ContinueStatement(x) => x.node(),
            Self::DeferStatement(x) => x.node(),
            Self::EmptyStatement(x) => x.node(),
            Self::ExpressionSwitchStatement(x) => x.node(),
            Self::FallthroughStatement(x) => x.node(),
            Self::ForStatement(x) => x.node(),
            Self::GoStatement(x) => x.node(),
            Self::GotoStatement(x) => x.node(),
            Self::IfStatement(x) => x.node(),
            Self::LabeledStatement(x) => x.node(),
            Self::ReturnStatement(x) => x.node(),
            Self::SelectStatement(x) => x.node(),
            Self::TypeDeclaration(x) => x.node(),
            Self::TypeSwitchStatement(x) => x.node(),
            Self::VarDeclaration(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        match self {
            Self::SimpleStatement(x) => x.node_mut(),
            Self::Block(x) => x.node_mut(),
            Self::BreakStatement(x) => x.node_mut(),
            Self::ConstDeclaration(x) => x.node_mut(),
            Self::ContinueStatement(x) => x.node_mut(),
            Self::DeferStatement(x) => x.node_mut(),
            Self::EmptyStatement(x) => x.node_mut(),
            Self::ExpressionSwitchStatement(x) => x.node_mut(),
            Self::FallthroughStatement(x) => x.node_mut(),
            Self::ForStatement(x) => x.node_mut(),
            Self::GoStatement(x) => x.node_mut(),
            Self::GotoStatement(x) => x.node_mut(),
            Self::IfStatement(x) => x.node_mut(),
            Self::LabeledStatement(x) => x.node_mut(),
            Self::ReturnStatement(x) => x.node_mut(),
            Self::SelectStatement(x) => x.node_mut(),
            Self::TypeDeclaration(x) => x.node_mut(),
            Self::TypeSwitchStatement(x) => x.node_mut(),
            Self::VarDeclaration(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        match self {
            Self::SimpleStatement(x) => x.into_node(),
            Self::Block(x) => x.into_node(),
            Self::BreakStatement(x) => x.into_node(),
            Self::ConstDeclaration(x) => x.into_node(),
            Self::ContinueStatement(x) => x.into_node(),
            Self::DeferStatement(x) => x.into_node(),
            Self::EmptyStatement(x) => x.into_node(),
            Self::ExpressionSwitchStatement(x) => x.into_node(),
            Self::FallthroughStatement(x) => x.into_node(),
            Self::ForStatement(x) => x.into_node(),
            Self::GoStatement(x) => x.into_node(),
            Self::GotoStatement(x) => x.into_node(),
            Self::IfStatement(x) => x.into_node(),
            Self::LabeledStatement(x) => x.into_node(),
            Self::ReturnStatement(x) => x.into_node(),
            Self::SelectStatement(x) => x.into_node(),
            Self::TypeDeclaration(x) => x.into_node(),
            Self::TypeSwitchStatement(x) => x.into_node(),
            Self::VarDeclaration(x) => x.into_node(),
        }
    }
}
#[doc = "Typed node `_type`\n\nThis node type is a supertype of:\n- `_simple_type` ([SimpleType])\n- `parenthesized_type` ([ParenthesizedType])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Type<'tree> {
    SimpleType(SimpleType<'tree>),
    ParenthesizedType(ParenthesizedType<'tree>),
}
#[automatically_derived]
impl<'tree> Type<'tree> {
    #[doc = "Returns the node if it is of kind `_simple_type` ([SimpleType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn simple_type(self) -> Option<SimpleType<'tree>> {
        match self {
            Self::SimpleType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = "Returns the node if it is of kind `parenthesized_type` ([ParenthesizedType]), otherwise returns None"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn parenthesized_type(self) -> Option<ParenthesizedType<'tree>> {
        match self {
            Self::ParenthesizedType(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Type<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if let Ok(this) = <SimpleType<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::SimpleType(this));
        }
        if let Ok(this) = <ParenthesizedType<'tree> as TryFrom<_>>::try_from(node) {
            return Ok(Self::ParenthesizedType(this));
        }
        Err(type_sitter_lib::IncorrectKind {
            node,
            kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
        })
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
    const KIND: &'static str = "_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        match self {
            Self::SimpleType(x) => x.node(),
            Self::ParenthesizedType(x) => x.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        match self {
            Self::SimpleType(x) => x.node_mut(),
            Self::ParenthesizedType(x) => x.node_mut(),
        }
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        match self {
            Self::SimpleType(x) => x.into_node(),
            Self::ParenthesizedType(x) => x.into_node(),
        }
    }
}
#[doc = "Typed node `argument_list`\n\nThis node has children: `{_expression | _type | variadic_argument}*`:\n- [Expression]\n- [Type]\n- [VariadicArgument]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArgumentList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArgumentList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_Type_VariadicArgument<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_Type_VariadicArgument < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Expression_Type_VariadicArgument<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Expression_Type_VariadicArgument < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ArgumentList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "argument_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArgumentList<'tree> {
    const KIND: &'static str = "argument_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `array_type`\n\nThis node has these fields:\n- `element`: `_type` ([Type])\n- `length`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArrayType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArrayType<'tree> {
    #[doc = "Get the field `element` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn element(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("element") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `length` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn length(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("length") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ArrayType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArrayType<'tree> {
    const KIND: &'static str = "array_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `assignment_statement`\n\nThis node has these fields:\n- `left`: `expression_list` ([ExpressionList])\n- `operator`: `{%= | &= | &^= | *= | += | -= | /= | <<= | = | >>= | ^= | |=}` ([anon_unions::ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq])\n- `right`: `expression_list` ([ExpressionList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AssignmentStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AssignmentStatement<'tree> {
    #[doc = "Get the field `left` which has kind `expression_list` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< ExpressionList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `operator` which has kind `{%= | &= | &^= | *= | += | -= | /= | <<= | = | >>= | ^= | |=}` ([anon_unions::ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operator(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq<
            'tree,
        >,
    > {
        self . 0 . child_by_field_name ("operator") . map (< anon_unions :: ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `right` which has kind `expression_list` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< ExpressionList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AssignmentStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "assignment_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AssignmentStatement<'tree> {
    const KIND: &'static str = "assignment_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `binary_expression`\n\nThis node has these fields:\n- `left`: `_expression` ([Expression])\n- `operator`: `{!= | % | & | && | &^ | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}` ([anon_unions::NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr])\n- `right`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BinaryExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BinaryExpression<'tree> {
    #[doc = "Get the field `left` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `operator` which has kind `{!= | % | & | && | &^ | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}` ([anon_unions::NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr])"]
    #[allow(dead_code)]
    #[inline]    pub fn operator (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr < 'tree > >{
        self . 0 . child_by_field_name ("operator") . map (< anon_unions :: NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `right` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BinaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BinaryExpression<'tree> {
    const KIND: &'static str = "binary_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `block`\n\nThis node has children: `_statement*` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Block<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Block<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Statement<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Statement<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Statement<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Statement<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Block<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Block<'tree> {
    const KIND: &'static str = "block";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `break_statement`\n\nThis node has an (optional) child: `label_name?` ([LabelName])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BreakStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BreakStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LabelName<'tree>>> {
        self.0
            .named_child(0)
            .map(<LabelName<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BreakStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BreakStatement<'tree> {
    const KIND: &'static str = "break_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `call_expression`\n\nThis node has these fields:\n- `arguments`: `argument_list` ([ArgumentList])\n- `function`: `_expression` ([Expression])\n- `type_arguments`: `type_arguments?` ([TypeArguments])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CallExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CallExpression<'tree> {
    #[doc = "Get the field `arguments` which has kind `argument_list` ([ArgumentList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn arguments(&self) -> type_sitter_lib::NodeResult<'tree, ArgumentList<'tree>> {
        self . 0 . child_by_field_name ("arguments") . map (< ArgumentList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `function` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn function(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("function") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type_arguments` which has kind `type_arguments?` ([TypeArguments])"]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for CallExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "call_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CallExpression<'tree> {
    const KIND: &'static str = "call_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `channel_type`\n\nThis node has these fields:\n- `value`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ChannelType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ChannelType<'tree> {
    #[doc = "Get the field `value` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ChannelType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "channel_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ChannelType<'tree> {
    const KIND: &'static str = "channel_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `communication_case`\n\nThis node has these fields:\n- `communication`: `{receive_statement | send_statement}` ([anon_unions::ReceiveStatement_SendStatement])\n\nAnd additional children: `_statement*` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CommunicationCase<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CommunicationCase<'tree> {
    #[doc = "Get the field `communication` which has kind `{receive_statement | send_statement}` ([anon_unions::ReceiveStatement_SendStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn communication(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::ReceiveStatement_SendStatement<'tree>>
    {
        self . 0 . child_by_field_name ("communication") . map (< anon_unions :: ReceiveStatement_SendStatement < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Statement_ReceiveStatement_SendStatement<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Statement_ReceiveStatement_SendStatement<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Statement_ReceiveStatement_SendStatement<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Statement_ReceiveStatement_SendStatement<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for CommunicationCase<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "communication_case" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CommunicationCase<'tree> {
    const KIND: &'static str = "communication_case";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `composite_literal`\n\nThis node has these fields:\n- `body`: `literal_value` ([LiteralValue])\n- `type`: `{array_type | generic_type | implicit_length_array_type | map_type | qualified_type | slice_type | struct_type | type_identifier}` ([anon_unions::ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CompositeLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CompositeLiteral<'tree> {
    #[doc = "Get the field `body` which has kind `literal_value` ([LiteralValue])"]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, LiteralValue<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< LiteralValue < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `{array_type | generic_type | implicit_length_array_type | map_type | qualified_type | slice_type | struct_type | type_identifier}` ([anon_unions::ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]    pub fn r#type (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier < 'tree > >{
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for CompositeLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "composite_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CompositeLiteral<'tree> {
    const KIND: &'static str = "composite_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `const_declaration`\n\nThis node has children: `const_spec*` ([ConstSpec])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstDeclaration<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ConstSpec<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, ConstSpec<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, ConstSpec<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, ConstSpec<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ConstDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstDeclaration<'tree> {
    const KIND: &'static str = "const_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `const_spec`\n\nThis node has these fields:\n- `name`: `{, | identifier}+` ([anon_unions::Comma_Identifier])\n- `type`: `_type?` ([Type])\n- `value`: `expression_list?` ([ExpressionList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstSpec<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstSpec<'tree> {
    #[doc = "Get the field `name` which has kind `{, | identifier}+` ([anon_unions::Comma_Identifier])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Comma_Identifier<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("name" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Comma_Identifier < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `type` which has kind `_type?` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `value` which has kind `expression_list?` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ConstSpec<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_spec" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstSpec<'tree> {
    const KIND: &'static str = "const_spec";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `continue_statement`\n\nThis node has an (optional) child: `label_name?` ([LabelName])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ContinueStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ContinueStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LabelName<'tree>>> {
        self.0
            .named_child(0)
            .map(<LabelName<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ContinueStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ContinueStatement<'tree> {
    const KIND: &'static str = "continue_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `dec_statement`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DecStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DecStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DecStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dec_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DecStatement<'tree> {
    const KIND: &'static str = "dec_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `default_case`\n\nThis node has children: `_statement*` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DefaultCase<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DefaultCase<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Statement<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Statement<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Statement<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Statement<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DefaultCase<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "default_case" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DefaultCase<'tree> {
    const KIND: &'static str = "default_case";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `defer_statement`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DeferStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DeferStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DeferStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "defer_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DeferStatement<'tree> {
    const KIND: &'static str = "defer_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `dot`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Dot<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Dot<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Dot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dot" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Dot<'tree> {
    const KIND: &'static str = "dot";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `empty_statement`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EmptyStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EmptyStatement<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for EmptyStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EmptyStatement<'tree> {
    const KIND: &'static str = "empty_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `expression_case`\n\nThis node has these fields:\n- `value`: `expression_list` ([ExpressionList])\n\nAnd additional children: `_statement*` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionCase<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionCase<'tree> {
    #[doc = "Get the field `value` which has kind `expression_list` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< ExpressionList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_ExpressionList<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_ExpressionList < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_ExpressionList<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_ExpressionList < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ExpressionCase<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_case" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionCase<'tree> {
    const KIND: &'static str = "expression_case";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `expression_list`\n\nThis node has children: `_expression+` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionList<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Expression<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Expression<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ExpressionList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionList<'tree> {
    const KIND: &'static str = "expression_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `expression_statement`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ExpressionStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionStatement<'tree> {
    const KIND: &'static str = "expression_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `expression_switch_statement`\n\nThis node has these fields:\n- `initializer`: `_simple_statement?` ([SimpleStatement])\n- `value`: `_expression?` ([Expression])\n\nAnd additional children: `{default_case | expression_case}*`:\n- [DefaultCase]\n- [ExpressionCase]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionSwitchStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionSwitchStatement<'tree> {
    #[doc = "Get the field `initializer` which has kind `_simple_statement?` ([SimpleStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn initializer(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, SimpleStatement<'tree>>> {
        self.0
            .child_by_field_name("initializer")
            .map(<SimpleStatement<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `value` which has kind `_expression?` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ExpressionSwitchStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_switch_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionSwitchStatement<'tree> {
    const KIND: &'static str = "expression_switch_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `fallthrough_statement`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FallthroughStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FallthroughStatement<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FallthroughStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fallthrough_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FallthroughStatement<'tree> {
    const KIND: &'static str = "fallthrough_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `field_declaration`\n\nThis node has these fields:\n- `name`: `field_identifier*` ([FieldIdentifier])\n- `tag`: `{interpreted_string_literal | raw_string_literal}?` ([anon_unions::InterpretedStringLiteral_RawStringLiteral])\n- `type`: `{_type | generic_type | qualified_type | type_identifier}` ([anon_unions::Type_GenericType_QualifiedType_TypeIdentifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclaration<'tree> {
    #[doc = "Get the field `name` which has kind `field_identifier*` ([FieldIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, FieldIdentifier<'tree>>,
        >,
    > + 'a {
        self.0.children_by_field_name("name", c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, FieldIdentifier<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the field `tag` which has kind `{interpreted_string_literal | raw_string_literal}?` ([anon_unions::InterpretedStringLiteral_RawStringLiteral])"]
    #[allow(dead_code)]
    #[inline]
    pub fn tag(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::InterpretedStringLiteral_RawStringLiteral<'tree>,
        >,
    > {
        self.0.child_by_field_name("tag").map(
            <anon_unions::InterpretedStringLiteral_RawStringLiteral<'tree> as TryFrom<_>>::try_from,
        )
    }
    #[doc = "Get the field `type` which has kind `{_type | generic_type | qualified_type | type_identifier}` ([anon_unions::Type_GenericType_QualifiedType_TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::Type_GenericType_QualifiedType_TypeIdentifier<'tree>,
    > {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: Type_GenericType_QualifiedType_TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FieldDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclaration<'tree> {
    const KIND: &'static str = "field_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `field_declaration_list`\n\nThis node has children: `field_declaration*` ([FieldDeclaration])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclarationList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclarationList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, FieldDeclaration<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, FieldDeclaration<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, FieldDeclaration<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, FieldDeclaration<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FieldDeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclarationList<'tree> {
    const KIND: &'static str = "field_declaration_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `for_clause`\n\nThis node has these fields:\n- `condition`: `_expression?` ([Expression])\n- `initializer`: `_simple_statement?` ([SimpleStatement])\n- `update`: `_simple_statement?` ([SimpleStatement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForClause<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForClause<'tree> {
    #[doc = "Get the field `condition` which has kind `_expression?` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("condition")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `initializer` which has kind `_simple_statement?` ([SimpleStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn initializer(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, SimpleStatement<'tree>>> {
        self.0
            .child_by_field_name("initializer")
            .map(<SimpleStatement<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `update` which has kind `_simple_statement?` ([SimpleStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn update(&self) -> Option<type_sitter_lib::NodeResult<'tree, SimpleStatement<'tree>>> {
        self.0
            .child_by_field_name("update")
            .map(<SimpleStatement<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ForClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForClause<'tree> {
    const KIND: &'static str = "for_clause";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `for_statement`\n\nThis node has these fields:\n- `body`: `block` ([Block])\n\nAnd an additional (optional) child: `{_expression | for_clause | range_clause}?`:\n- [Expression]\n- [ForClause]\n- [RangeClause]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForStatement<'tree> {
    #[doc = "Get the field `body` which has kind `block` ([Block])"]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Expression_ForClause_RangeClause_Block<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Expression_ForClause_RangeClause_Block<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Expression_ForClause_RangeClause_Block<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Expression_ForClause_RangeClause_Block<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ForStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForStatement<'tree> {
    const KIND: &'static str = "for_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `func_literal`\n\nThis node has these fields:\n- `body`: `block` ([Block])\n- `parameters`: `parameter_list` ([ParameterList])\n- `result`: `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FuncLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FuncLiteral<'tree> {
    #[doc = "Get the field `body` which has kind `block` ([Block])"]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("body") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `parameters` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `result` which has kind `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn result(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::SimpleType_ParameterList<'tree>>>
    {
        self.0
            .child_by_field_name("result")
            .map(<anon_unions::SimpleType_ParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FuncLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "func_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FuncLiteral<'tree> {
    const KIND: &'static str = "func_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `function_declaration`\n\nThis node has these fields:\n- `body`: `block?` ([Block])\n- `name`: `identifier` ([Identifier])\n- `parameters`: `parameter_list` ([ParameterList])\n- `result`: `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])\n- `type_parameters`: `type_parameter_list?` ([TypeParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionDeclaration<'tree> {
    #[doc = "Get the field `body` which has kind `block?` ([Block])"]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> Option<type_sitter_lib::NodeResult<'tree, Block<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `parameters` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `result` which has kind `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn result(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::SimpleType_ParameterList<'tree>>>
    {
        self.0
            .child_by_field_name("result")
            .map(<anon_unions::SimpleType_ParameterList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `type_parameters` which has kind `type_parameter_list?` ([TypeParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameterList<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FunctionDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionDeclaration<'tree> {
    const KIND: &'static str = "function_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `function_type`\n\nThis node has these fields:\n- `parameters`: `parameter_list` ([ParameterList])\n- `result`: `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionType<'tree> {
    #[doc = "Get the field `parameters` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `result` which has kind `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn result(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::SimpleType_ParameterList<'tree>>>
    {
        self.0
            .child_by_field_name("result")
            .map(<anon_unions::SimpleType_ParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FunctionType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionType<'tree> {
    const KIND: &'static str = "function_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `generic_type`\n\nThis node has these fields:\n- `type`: `{negated_type | qualified_type | type_identifier}` ([anon_unions::NegatedType_QualifiedType_TypeIdentifier])\n- `type_arguments`: `type_arguments` ([TypeArguments])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericType<'tree> {
    #[doc = "Get the field `type` which has kind `{negated_type | qualified_type | type_identifier}` ([anon_unions::NegatedType_QualifiedType_TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::NegatedType_QualifiedType_TypeIdentifier<'tree>,
    > {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: NegatedType_QualifiedType_TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type_arguments` which has kind `type_arguments` ([TypeArguments])"]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(&self) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self . 0 . child_by_field_name ("type_arguments") . map (< TypeArguments < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GenericType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericType<'tree> {
    const KIND: &'static str = "generic_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `go_statement`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GoStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GoStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GoStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "go_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GoStatement<'tree> {
    const KIND: &'static str = "go_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `goto_statement`\n\nThis node has a child: `label_name` ([LabelName])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GotoStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GotoStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, LabelName<'tree>> {
        self . 0 . named_child (0) . map (< LabelName < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GotoStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "goto_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GotoStatement<'tree> {
    const KIND: &'static str = "goto_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `if_statement`\n\nThis node has these fields:\n- `alternative`: `{block | if_statement}?` ([anon_unions::Block_IfStatement])\n- `condition`: `_expression` ([Expression])\n- `consequence`: `block` ([Block])\n- `initializer`: `_simple_statement?` ([SimpleStatement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IfStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IfStatement<'tree> {
    #[doc = "Get the field `alternative` which has kind `{block | if_statement}?` ([anon_unions::Block_IfStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn alternative(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::Block_IfStatement<'tree>>> {
        self.0
            .child_by_field_name("alternative")
            .map(<anon_unions::Block_IfStatement<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `condition` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("condition") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `consequence` which has kind `block` ([Block])"]
    #[allow(dead_code)]
    #[inline]
    pub fn consequence(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self . 0 . child_by_field_name ("consequence") . map (< Block < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `initializer` which has kind `_simple_statement?` ([SimpleStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn initializer(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, SimpleStatement<'tree>>> {
        self.0
            .child_by_field_name("initializer")
            .map(<SimpleStatement<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for IfStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IfStatement<'tree> {
    const KIND: &'static str = "if_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `implicit_length_array_type`\n\nThis node has these fields:\n- `element`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImplicitLengthArrayType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImplicitLengthArrayType<'tree> {
    #[doc = "Get the field `element` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn element(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("element") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImplicitLengthArrayType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "implicit_length_array_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImplicitLengthArrayType<'tree> {
    const KIND: &'static str = "implicit_length_array_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `import_declaration`\n\nThis node has a child: `{import_spec | import_spec_list}`:\n- [ImportSpec]\n- [ImportSpecList]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImportDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImportDeclaration<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::ImportSpec_ImportSpecList<'tree>> {
        self . 0 . named_child (0) . map (< anon_unions :: ImportSpec_ImportSpecList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImportDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "import_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImportDeclaration<'tree> {
    const KIND: &'static str = "import_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `import_spec`\n\nThis node has these fields:\n- `name`: `{blank_identifier | dot | package_identifier}?` ([anon_unions::BlankIdentifier_Dot_PackageIdentifier])\n- `path`: `{interpreted_string_literal | raw_string_literal}` ([anon_unions::InterpretedStringLiteral_RawStringLiteral])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImportSpec<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImportSpec<'tree> {
    #[doc = "Get the field `name` which has kind `{blank_identifier | dot | package_identifier}?` ([anon_unions::BlankIdentifier_Dot_PackageIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            anon_unions::BlankIdentifier_Dot_PackageIdentifier<'tree>,
        >,
    > {
        self.0.child_by_field_name("name").map(
            <anon_unions::BlankIdentifier_Dot_PackageIdentifier<'tree> as TryFrom<_>>::try_from,
        )
    }
    #[doc = "Get the field `path` which has kind `{interpreted_string_literal | raw_string_literal}` ([anon_unions::InterpretedStringLiteral_RawStringLiteral])"]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        anon_unions::InterpretedStringLiteral_RawStringLiteral<'tree>,
    > {
        self . 0 . child_by_field_name ("path") . map (< anon_unions :: InterpretedStringLiteral_RawStringLiteral < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImportSpec<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "import_spec" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImportSpec<'tree> {
    const KIND: &'static str = "import_spec";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `import_spec_list`\n\nThis node has children: `import_spec*` ([ImportSpec])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImportSpecList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImportSpecList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ImportSpec<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, ImportSpec<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, ImportSpec<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, ImportSpec<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImportSpecList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "import_spec_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImportSpecList<'tree> {
    const KIND: &'static str = "import_spec_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `inc_statement`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IncStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IncStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for IncStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "inc_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IncStatement<'tree> {
    const KIND: &'static str = "inc_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `index_expression`\n\nThis node has these fields:\n- `index`: `_expression` ([Expression])\n- `operand`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IndexExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IndexExpression<'tree> {
    #[doc = "Get the field `index` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn index(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("index") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for IndexExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "index_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IndexExpression<'tree> {
    const KIND: &'static str = "index_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `interface_type`\n\nThis node has children: `{method_elem | type_elem}*`:\n- [MethodElem]\n- [TypeElem]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct InterfaceType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> InterfaceType<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MethodElem_TypeElem<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::MethodElem_TypeElem<'tree>> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::MethodElem_TypeElem<'tree>>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::MethodElem_TypeElem<'tree>> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for InterfaceType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "interface_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for InterfaceType<'tree> {
    const KIND: &'static str = "interface_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `interpreted_string_literal`\n\nThis node has children: `escape_sequence*` ([EscapeSequence])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct InterpretedStringLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> InterpretedStringLiteral<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for InterpretedStringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "interpreted_string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for InterpretedStringLiteral<'tree> {
    const KIND: &'static str = "interpreted_string_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `keyed_element`\n\nThis node has children: `literal_element+` ([LiteralElement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct KeyedElement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> KeyedElement<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, LiteralElement<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, LiteralElement<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, LiteralElement<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, LiteralElement<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for KeyedElement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "keyed_element" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for KeyedElement<'tree> {
    const KIND: &'static str = "keyed_element";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `labeled_statement`\n\nThis node has these fields:\n- `label`: `label_name` ([LabelName])\n\nAnd an additional (optional) child: `_statement?` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LabeledStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LabeledStatement<'tree> {
    #[doc = "Get the field `label` which has kind `label_name` ([LabelName])"]
    #[allow(dead_code)]
    #[inline]
    pub fn label(&self) -> type_sitter_lib::NodeResult<'tree, LabelName<'tree>> {
        self . 0 . child_by_field_name ("label") . map (< LabelName < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_LabelName<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_LabelName<'tree>> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_LabelName<'tree>>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_LabelName<'tree>> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LabeledStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "labeled_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LabeledStatement<'tree> {
    const KIND: &'static str = "labeled_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `literal_element`\n\nThis node has a child: `{_expression | literal_value}`:\n- [Expression]\n- [LiteralValue]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LiteralElement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LiteralElement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Expression_LiteralValue<'tree>> {
        self . 0 . named_child (0) . map (< anon_unions :: Expression_LiteralValue < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LiteralElement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "literal_element" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LiteralElement<'tree> {
    const KIND: &'static str = "literal_element";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `literal_value`\n\nThis node has children: `{keyed_element | literal_element}*`:\n- [KeyedElement]\n- [LiteralElement]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LiteralValue<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LiteralValue<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::KeyedElement_LiteralElement<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: KeyedElement_LiteralElement < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::KeyedElement_LiteralElement<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: KeyedElement_LiteralElement < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LiteralValue<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "literal_value" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LiteralValue<'tree> {
    const KIND: &'static str = "literal_value";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `map_type`\n\nThis node has these fields:\n- `key`: `_type` ([Type])\n- `value`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MapType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MapType<'tree> {
    #[doc = "Get the field `key` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn key(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("key") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `value` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for MapType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "map_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MapType<'tree> {
    const KIND: &'static str = "map_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `method_declaration`\n\nThis node has these fields:\n- `body`: `block?` ([Block])\n- `name`: `field_identifier` ([FieldIdentifier])\n- `parameters`: `parameter_list` ([ParameterList])\n- `receiver`: `parameter_list` ([ParameterList])\n- `result`: `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MethodDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MethodDeclaration<'tree> {
    #[doc = "Get the field `body` which has kind `block?` ([Block])"]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> Option<type_sitter_lib::NodeResult<'tree, Block<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `name` which has kind `field_identifier` ([FieldIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< FieldIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `parameters` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `receiver` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn receiver(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("receiver") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `result` which has kind `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn result(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::SimpleType_ParameterList<'tree>>>
    {
        self.0
            .child_by_field_name("result")
            .map(<anon_unions::SimpleType_ParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for MethodDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "method_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MethodDeclaration<'tree> {
    const KIND: &'static str = "method_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `method_elem`\n\nThis node has these fields:\n- `name`: `field_identifier` ([FieldIdentifier])\n- `parameters`: `parameter_list` ([ParameterList])\n- `result`: `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MethodElem<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MethodElem<'tree> {
    #[doc = "Get the field `name` which has kind `field_identifier` ([FieldIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< FieldIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `parameters` which has kind `parameter_list` ([ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, ParameterList<'tree>> {
        self . 0 . child_by_field_name ("parameters") . map (< ParameterList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `result` which has kind `{_simple_type | parameter_list}?` ([anon_unions::SimpleType_ParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn result(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::SimpleType_ParameterList<'tree>>>
    {
        self.0
            .child_by_field_name("result")
            .map(<anon_unions::SimpleType_ParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for MethodElem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "method_elem" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MethodElem<'tree> {
    const KIND: &'static str = "method_elem";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `negated_type`\n\nThis node has a child: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct NegatedType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> NegatedType<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . named_child (0) . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for NegatedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "negated_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for NegatedType<'tree> {
    const KIND: &'static str = "negated_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `package_clause`\n\nThis node has a child: `package_identifier` ([PackageIdentifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PackageClause<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PackageClause<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, PackageIdentifier<'tree>> {
        self . 0 . named_child (0) . map (< PackageIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for PackageClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "package_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PackageClause<'tree> {
    const KIND: &'static str = "package_clause";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `parameter_declaration`\n\nThis node has these fields:\n- `name`: `identifier*` ([Identifier])\n- `type`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParameterDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParameterDeclaration<'tree> {
    #[doc = "Get the field `name` which has kind `identifier*` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Identifier<'tree>>,
        >,
    > + 'a {
        self.0.children_by_field_name("name", c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Identifier<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ParameterDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameter_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParameterDeclaration<'tree> {
    const KIND: &'static str = "parameter_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `parameter_list`\n\nThis node has children: `{parameter_declaration | variadic_parameter_declaration}*`:\n- [ParameterDeclaration]\n- [VariadicParameterDeclaration]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParameterList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParameterList<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ParameterDeclaration_VariadicParameterDeclaration<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ParameterDeclaration_VariadicParameterDeclaration<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ParameterDeclaration_VariadicParameterDeclaration<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ParameterDeclaration_VariadicParameterDeclaration<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ParameterList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameter_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParameterList<'tree> {
    const KIND: &'static str = "parameter_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `parenthesized_expression`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParenthesizedExpression<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ParenthesizedExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParenthesizedExpression<'tree> {
    const KIND: &'static str = "parenthesized_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `parenthesized_type`\n\nThis node has a child: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParenthesizedType<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . named_child (0) . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ParenthesizedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parenthesized_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParenthesizedType<'tree> {
    const KIND: &'static str = "parenthesized_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `pointer_type`\n\nThis node has a child: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PointerType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PointerType<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . named_child (0) . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for PointerType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pointer_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PointerType<'tree> {
    const KIND: &'static str = "pointer_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `qualified_type`\n\nThis node has these fields:\n- `name`: `type_identifier` ([TypeIdentifier])\n- `package`: `package_identifier` ([PackageIdentifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct QualifiedType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> QualifiedType<'tree> {
    #[doc = "Get the field `name` which has kind `type_identifier` ([TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `package` which has kind `package_identifier` ([PackageIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn package(&self) -> type_sitter_lib::NodeResult<'tree, PackageIdentifier<'tree>> {
        self . 0 . child_by_field_name ("package") . map (< PackageIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for QualifiedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "qualified_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for QualifiedType<'tree> {
    const KIND: &'static str = "qualified_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `range_clause`\n\nThis node has these fields:\n- `left`: `expression_list?` ([ExpressionList])\n- `right`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RangeClause<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RangeClause<'tree> {
    #[doc = "Get the field `left` which has kind `expression_list?` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .child_by_field_name("left")
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `right` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RangeClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RangeClause<'tree> {
    const KIND: &'static str = "range_clause";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `receive_statement`\n\nThis node has these fields:\n- `left`: `expression_list?` ([ExpressionList])\n- `right`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReceiveStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReceiveStatement<'tree> {
    #[doc = "Get the field `left` which has kind `expression_list?` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .child_by_field_name("left")
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `right` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ReceiveStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "receive_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReceiveStatement<'tree> {
    const KIND: &'static str = "receive_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `return_statement`\n\nThis node has an (optional) child: `expression_list?` ([ExpressionList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReturnStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReturnStatement<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .named_child(0)
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ReturnStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReturnStatement<'tree> {
    const KIND: &'static str = "return_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `select_statement`\n\nThis node has children: `{communication_case | default_case}*`:\n- [CommunicationCase]\n- [DefaultCase]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SelectStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SelectStatement<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::CommunicationCase_DefaultCase<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: CommunicationCase_DefaultCase < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::CommunicationCase_DefaultCase<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: CommunicationCase_DefaultCase < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SelectStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "select_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SelectStatement<'tree> {
    const KIND: &'static str = "select_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `selector_expression`\n\nThis node has these fields:\n- `field`: `field_identifier` ([FieldIdentifier])\n- `operand`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SelectorExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SelectorExpression<'tree> {
    #[doc = "Get the field `field` which has kind `field_identifier` ([FieldIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn field(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self . 0 . child_by_field_name ("field") . map (< FieldIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SelectorExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "selector_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SelectorExpression<'tree> {
    const KIND: &'static str = "selector_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `send_statement`\n\nThis node has these fields:\n- `channel`: `_expression` ([Expression])\n- `value`: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SendStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SendStatement<'tree> {
    #[doc = "Get the field `channel` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn channel(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("channel") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `value` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SendStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "send_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SendStatement<'tree> {
    const KIND: &'static str = "send_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `short_var_declaration`\n\nThis node has these fields:\n- `left`: `expression_list` ([ExpressionList])\n- `right`: `expression_list` ([ExpressionList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ShortVarDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ShortVarDeclaration<'tree> {
    #[doc = "Get the field `left` which has kind `expression_list` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>> {
        self . 0 . child_by_field_name ("left") . map (< ExpressionList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `right` which has kind `expression_list` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>> {
        self . 0 . child_by_field_name ("right") . map (< ExpressionList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ShortVarDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "short_var_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ShortVarDeclaration<'tree> {
    const KIND: &'static str = "short_var_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `slice_expression`\n\nThis node has these fields:\n- `capacity`: `_expression?` ([Expression])\n- `end`: `_expression?` ([Expression])\n- `operand`: `_expression` ([Expression])\n- `start`: `_expression?` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SliceExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SliceExpression<'tree> {
    #[doc = "Get the field `capacity` which has kind `_expression?` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn capacity(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("capacity")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `end` which has kind `_expression?` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn end(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("end")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `start` which has kind `_expression?` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn start(&self) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("start")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SliceExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "slice_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SliceExpression<'tree> {
    const KIND: &'static str = "slice_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `slice_type`\n\nThis node has these fields:\n- `element`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SliceType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SliceType<'tree> {
    #[doc = "Get the field `element` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn element(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("element") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SliceType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "slice_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SliceType<'tree> {
    const KIND: &'static str = "slice_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `source_file`\n\nThis node has children: `{_statement | function_declaration | import_declaration | method_declaration | package_clause}*`:\n- [Statement]\n- [FunctionDeclaration]\n- [ImportDeclaration]\n- [MethodDeclaration]\n- [PackageClause]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SourceFile<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SourceFile<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut yak_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SourceFile<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SourceFile<'tree> {
    const KIND: &'static str = "source_file";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `struct_type`\n\nThis node has a child: `field_declaration_list` ([FieldDeclarationList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructType<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructType<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, FieldDeclarationList<'tree>> {
        self . 0 . named_child (0) . map (< FieldDeclarationList < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for StructType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructType<'tree> {
    const KIND: &'static str = "struct_type";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_alias`\n\nThis node has these fields:\n- `name`: `type_identifier` ([TypeIdentifier])\n- `type`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeAlias<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeAlias<'tree> {
    #[doc = "Get the field `name` which has kind `type_identifier` ([TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeAlias<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_alias" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeAlias<'tree> {
    const KIND: &'static str = "type_alias";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_arguments`\n\nThis node has children: `type_elem+` ([TypeElem])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeArguments<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeArguments<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, TypeElem<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, TypeElem<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, TypeElem<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, TypeElem<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeArguments<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_arguments" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeArguments<'tree> {
    const KIND: &'static str = "type_arguments";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_assertion_expression`\n\nThis node has these fields:\n- `operand`: `_expression` ([Expression])\n- `type`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeAssertionExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeAssertionExpression<'tree> {
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeAssertionExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_assertion_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeAssertionExpression<'tree> {
    const KIND: &'static str = "type_assertion_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_case`\n\nThis node has these fields:\n- `type`: `{, | _type}+` ([anon_unions::Comma_Type])\n\nAnd additional children: `_statement*` ([Statement])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeCase<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeCase<'tree> {
    #[doc = "Get the field `type` which has kind `{, | _type}+` ([anon_unions::Comma_Type])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Comma_Type<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("type" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Comma_Type < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_Comma_Type<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_Comma_Type<'tree>> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Statement_Comma_Type<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Statement_Comma_Type < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeCase<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_case" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeCase<'tree> {
    const KIND: &'static str = "type_case";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_constraint`\n\nThis node has children: `_type+` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeConstraint<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeConstraint<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeConstraint<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_constraint" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeConstraint<'tree> {
    const KIND: &'static str = "type_constraint";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_conversion_expression`\n\nThis node has these fields:\n- `operand`: `_expression` ([Expression])\n- `type`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeConversionExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeConversionExpression<'tree> {
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeConversionExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_conversion_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeConversionExpression<'tree> {
    const KIND: &'static str = "type_conversion_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_declaration`\n\nThis node has children: `{type_alias | type_spec}*`:\n- [TypeAlias]\n- [TypeSpec]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeDeclaration<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::TypeAlias_TypeSpec<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::TypeAlias_TypeSpec<'tree>> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::TypeAlias_TypeSpec<'tree>>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::TypeAlias_TypeSpec<'tree>> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeDeclaration<'tree> {
    const KIND: &'static str = "type_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_elem`\n\nThis node has children: `_type+` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeElem<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeElem<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeElem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_elem" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeElem<'tree> {
    const KIND: &'static str = "type_elem";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_instantiation_expression`\n\nThis node has these fields:\n- `type`: `_type` ([Type])\n\nAnd additional children: `_type+` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeInstantiationExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeInstantiationExpression<'tree> {
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeInstantiationExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_instantiation_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeInstantiationExpression<'tree> {
    const KIND: &'static str = "type_instantiation_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_parameter_declaration`\n\nThis node has these fields:\n- `name`: `identifier+` ([Identifier])\n- `type`: `type_constraint` ([TypeConstraint])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeParameterDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeParameterDeclaration<'tree> {
    #[doc = "Get the field `name` which has kind `identifier+` ([Identifier])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Identifier<'tree>>,
        >,
    > + 'a {
        self.0.children_by_field_name("name", c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Identifier<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the field `type` which has kind `type_constraint` ([TypeConstraint])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, TypeConstraint<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< TypeConstraint < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeParameterDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_parameter_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeParameterDeclaration<'tree> {
    const KIND: &'static str = "type_parameter_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_parameter_list`\n\nThis node has children: `type_parameter_declaration+` ([TypeParameterDeclaration])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeParameterList<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeParameterList<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, TypeParameterDeclaration<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , TypeParameterDeclaration < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, TypeParameterDeclaration<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , TypeParameterDeclaration < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeParameterList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_parameter_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeParameterList<'tree> {
    const KIND: &'static str = "type_parameter_list";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_spec`\n\nThis node has these fields:\n- `name`: `type_identifier` ([TypeIdentifier])\n- `type`: `_type` ([Type])\n- `type_parameters`: `type_parameter_list?` ([TypeParameterList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeSpec<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeSpec<'tree> {
    #[doc = "Get the field `name` which has kind `type_identifier` ([TypeIdentifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< TypeIdentifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type_parameters` which has kind `type_parameter_list?` ([TypeParameterList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameterList<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameterList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeSpec<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_spec" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeSpec<'tree> {
    const KIND: &'static str = "type_spec";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_switch_statement`\n\nThis node has these fields:\n- `alias`: `expression_list?` ([ExpressionList])\n- `initializer`: `_simple_statement?` ([SimpleStatement])\n- `value`: `_expression` ([Expression])\n\nAnd additional children: `{default_case | type_case}*`:\n- [DefaultCase]\n- [TypeCase]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeSwitchStatement<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeSwitchStatement<'tree> {
    #[doc = "Get the field `alias` which has kind `expression_list?` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .child_by_field_name("alias")
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `initializer` which has kind `_simple_statement?` ([SimpleStatement])"]
    #[allow(dead_code)]
    #[inline]
    pub fn initializer(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, SimpleStatement<'tree>>> {
        self.0
            .child_by_field_name("initializer")
            .map(<SimpleStatement<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `value` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("value") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeSwitchStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_switch_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeSwitchStatement<'tree> {
    const KIND: &'static str = "type_switch_statement";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `unary_expression`\n\nThis node has these fields:\n- `operand`: `_expression` ([Expression])\n- `operator`: `{! | & | * | + | - | <- | ^}` ([anon_unions::Not_And_Mul_Add_Sub_LtSub_BitXor])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnaryExpression<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnaryExpression<'tree> {
    #[doc = "Get the field `operand` which has kind `_expression` ([Expression])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operand(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . child_by_field_name ("operand") . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `operator` which has kind `{! | & | * | + | - | <- | ^}` ([anon_unions::Not_And_Mul_Add_Sub_LtSub_BitXor])"]
    #[allow(dead_code)]
    #[inline]
    pub fn operator(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Not_And_Mul_Add_Sub_LtSub_BitXor<'tree>>
    {
        self . 0 . child_by_field_name ("operator") . map (< anon_unions :: Not_And_Mul_Add_Sub_LtSub_BitXor < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for UnaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnaryExpression<'tree> {
    const KIND: &'static str = "unary_expression";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `var_declaration`\n\nThis node has children: `var_spec*` ([VarSpec])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VarDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VarDeclaration<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, VarSpec<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, VarSpec<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, VarSpec<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, VarSpec<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for VarDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "var_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VarDeclaration<'tree> {
    const KIND: &'static str = "var_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `var_spec`\n\nThis node has these fields:\n- `name`: `{, | identifier}+` ([anon_unions::Comma_Identifier])\n- `type`: `_type?` ([Type])\n- `value`: `expression_list?` ([ExpressionList])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VarSpec<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VarSpec<'tree> {
    #[doc = "Get the field `name` which has kind `{, | identifier}+` ([anon_unions::Comma_Identifier])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut yak_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Comma_Identifier<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("name" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Comma_Identifier < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `type` which has kind `_type?` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `value` which has kind `expression_list?` ([ExpressionList])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, ExpressionList<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<ExpressionList<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for VarSpec<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "var_spec" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VarSpec<'tree> {
    const KIND: &'static str = "var_spec";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `variadic_argument`\n\nThis node has a child: `_expression` ([Expression])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VariadicArgument<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VariadicArgument<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self . 0 . named_child (0) . map (< Expression < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for VariadicArgument<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "variadic_argument" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VariadicArgument<'tree> {
    const KIND: &'static str = "variadic_argument";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `variadic_parameter_declaration`\n\nThis node has these fields:\n- `name`: `identifier?` ([Identifier])\n- `type`: `_type` ([Type])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VariadicParameterDeclaration<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VariadicParameterDeclaration<'tree> {
    #[doc = "Get the field `name` which has kind `identifier?` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> Option<type_sitter_lib::NodeResult<'tree, Identifier<'tree>>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `type` which has kind `_type` ([Type])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< Type < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for VariadicParameterDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "variadic_parameter_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VariadicParameterDeclaration<'tree> {
    const KIND: &'static str = "variadic_parameter_declaration";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `blank_identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BlankIdentifier<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BlankIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BlankIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "blank_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BlankIdentifier<'tree> {
    const KIND: &'static str = "blank_identifier";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `comment`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Comment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Comment<'tree> {
    const KIND: &'static str = "comment";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `escape_sequence`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for EscapeSequence<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EscapeSequence<'tree> {
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `false`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct False<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> False<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for False<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for False<'tree> {
    const KIND: &'static str = "false";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `field_identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldIdentifier<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FieldIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldIdentifier<'tree> {
    const KIND: &'static str = "field_identifier";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `float_literal`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FloatLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FloatLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for FloatLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "float_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FloatLiteral<'tree> {
    const KIND: &'static str = "float_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Identifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier<'tree> {
    const KIND: &'static str = "identifier";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `imaginary_literal`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImaginaryLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImaginaryLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImaginaryLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "imaginary_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImaginaryLiteral<'tree> {
    const KIND: &'static str = "imaginary_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `int_literal`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IntLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IntLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for IntLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "int_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IntLiteral<'tree> {
    const KIND: &'static str = "int_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `iota`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Iota<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Iota<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Iota<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "iota" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Iota<'tree> {
    const KIND: &'static str = "iota";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `label_name`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LabelName<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LabelName<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LabelName<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "label_name" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LabelName<'tree> {
    const KIND: &'static str = "label_name";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `nil`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Nil<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Nil<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Nil<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "nil" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Nil<'tree> {
    const KIND: &'static str = "nil";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `package_identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PackageIdentifier<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PackageIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for PackageIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "package_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PackageIdentifier<'tree> {
    const KIND: &'static str = "package_identifier";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `raw_string_literal`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RawStringLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RawStringLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RawStringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "raw_string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RawStringLiteral<'tree> {
    const KIND: &'static str = "raw_string_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `rune_literal`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RuneLiteral<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RuneLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RuneLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "rune_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RuneLiteral<'tree> {
    const KIND: &'static str = "rune_literal";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `true`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct True<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> True<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for True<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for True<'tree> {
    const KIND: &'static str = "true";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `type_identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeIdentifier<'tree>(yak_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeIdentifier<'tree> {
    const KIND: &'static str = "type_identifier";
    #[inline]
    fn node(&self) -> &yak_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> yak_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `\0`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct U0<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> U0<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for U0<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\0" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for U0<'tree> {
        const KIND: &'static str = "\0";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `break`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Break<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Break<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Break<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "break" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Break<'tree> {
        const KIND: &'static str = "break";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `case`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Case<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Case<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Case<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "case" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Case<'tree> {
        const KIND: &'static str = "case";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `chan`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Chan<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Chan<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Chan<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "chan" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Chan<'tree> {
        const KIND: &'static str = "chan";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `const`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Const<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Const<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Const<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "const" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Const<'tree> {
        const KIND: &'static str = "const";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `continue`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Continue<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Continue<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Continue<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "continue" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Continue<'tree> {
        const KIND: &'static str = "continue";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `default`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Default<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Default<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Default<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "default" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Default<'tree> {
        const KIND: &'static str = "default";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `defer`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Defer<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Defer<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Defer<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "defer" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Defer<'tree> {
        const KIND: &'static str = "defer";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `else`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Else<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Else<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Else<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "else" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Else<'tree> {
        const KIND: &'static str = "else";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `fallthrough`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Fallthrough<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Fallthrough<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Fallthrough<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "fallthrough" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Fallthrough<'tree> {
        const KIND: &'static str = "fallthrough";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `for`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct For<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> For<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for For<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "for" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for For<'tree> {
        const KIND: &'static str = "for";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `func`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Func<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Func<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Func<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "func" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Func<'tree> {
        const KIND: &'static str = "func";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `go`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Go<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Go<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Go<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "go" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Go<'tree> {
        const KIND: &'static str = "go";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `goto`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Goto<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Goto<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Goto<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "goto" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Goto<'tree> {
        const KIND: &'static str = "goto";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `if`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct If<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> If<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for If<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "if" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for If<'tree> {
        const KIND: &'static str = "if";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `import`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Import<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Import<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Import<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "import" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Import<'tree> {
        const KIND: &'static str = "import";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `interface`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Interface<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Interface<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Interface<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "interface" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Interface<'tree> {
        const KIND: &'static str = "interface";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `map`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Map<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Map<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Map<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "map" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Map<'tree> {
        const KIND: &'static str = "map";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `package`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Package<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Package<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Package<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "package" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Package<'tree> {
        const KIND: &'static str = "package";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `range`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Range<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Range<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Range<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "range" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Range<'tree> {
        const KIND: &'static str = "range";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `return`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Return<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Return<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Return<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "return" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Return<'tree> {
        const KIND: &'static str = "return";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `select`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Select<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Select<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Select<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "select" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Select<'tree> {
        const KIND: &'static str = "select";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `struct`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Struct<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Struct<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Struct<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "struct" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Struct<'tree> {
        const KIND: &'static str = "struct";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `switch`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Switch<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Switch<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Switch<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "switch" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Switch<'tree> {
        const KIND: &'static str = "switch";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `type`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Type<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Type<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "type" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
        const KIND: &'static str = "type";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `var`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Var<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Var<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Var<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "var" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Var<'tree> {
        const KIND: &'static str = "var";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `\n`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Newline<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Newline<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Newline<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\n" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Newline<'tree> {
        const KIND: &'static str = "\n";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `!`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Not<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not<'tree> {
        const KIND: &'static str = "!";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `!=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> NotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for NotEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for NotEq<'tree> {
        const KIND: &'static str = "!=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\"`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DoubleQuote<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DoubleQuote<'tree> {
        const KIND: &'static str = "\"";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `%`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Mod<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod<'tree> {
        const KIND: &'static str = "%";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `%=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct ModEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> ModEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ModEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "%=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ModEq<'tree> {
        const KIND: &'static str = "%=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for And<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for And<'tree> {
        const KIND: &'static str = "&";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&&`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndAnd<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndAnd<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AndAnd<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndAnd<'tree> {
        const KIND: &'static str = "&&";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AndEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndEq<'tree> {
        const KIND: &'static str = "&=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&^`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndBitXor<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndBitXor<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AndBitXor<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&^" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndBitXor<'tree> {
        const KIND: &'static str = "&^";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&^=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AndBitXorEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AndBitXorEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AndBitXorEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&^=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AndBitXorEq<'tree> {
        const KIND: &'static str = "&^=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `(`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LParen<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen<'tree> {
        const KIND: &'static str = "(";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `)`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RParen<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RParen<'tree> {
        const KIND: &'static str = ")";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `*`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Mul<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mul<'tree> {
        const KIND: &'static str = "*";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `*=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct MulEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> MulEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for MulEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "*=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MulEq<'tree> {
        const KIND: &'static str = "*=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `+`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Add<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add<'tree> {
        const KIND: &'static str = "+";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `++`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AddAdd<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AddAdd<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AddAdd<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "++" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AddAdd<'tree> {
        const KIND: &'static str = "++";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `+=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct AddEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> AddEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for AddEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AddEq<'tree> {
        const KIND: &'static str = "+=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `,`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Comma<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma<'tree> {
        const KIND: &'static str = ",";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `-`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Sub<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub<'tree> {
        const KIND: &'static str = "-";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `--`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct SubSub<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> SubSub<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SubSub<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "--" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for SubSub<'tree> {
        const KIND: &'static str = "--";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `-=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct SubEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> SubEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SubEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for SubEq<'tree> {
        const KIND: &'static str = "-=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `.`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Dot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dot<'tree> {
        const KIND: &'static str = ".";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `...`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DotDotDot<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DotDotDot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DotDotDot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "..." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDotDot<'tree> {
        const KIND: &'static str = "...";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `/`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Div<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Div<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Div<'tree> {
        const KIND: &'static str = "/";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `/=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct DivEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> DivEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for DivEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "/=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DivEq<'tree> {
        const KIND: &'static str = "/=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `:`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Colon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Colon<'tree> {
        const KIND: &'static str = ":";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `:=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct ColonEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> ColonEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ColonEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ":=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ColonEq<'tree> {
        const KIND: &'static str = ":=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `;`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Semicolon<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Semicolon<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Semicolon<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ";" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Semicolon<'tree> {
        const KIND: &'static str = ";";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Lt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt<'tree> {
        const KIND: &'static str = "<";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<-`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtSub<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtSub<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LtSub<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<-" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtSub<'tree> {
        const KIND: &'static str = "<-";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<<`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtLt<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtLt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LtLt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<<" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLt<'tree> {
        const KIND: &'static str = "<<";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<<=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtLtEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtLtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LtLtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<<=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLtEq<'tree> {
        const KIND: &'static str = "<<=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LtEq<'tree> {
        const KIND: &'static str = "<=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Eq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Eq<'tree> {
        const KIND: &'static str = "=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `==`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> EqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for EqEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for EqEq<'tree> {
        const KIND: &'static str = "==";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Gt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Gt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Gt<'tree> {
        const KIND: &'static str = ">";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtEq<'tree> {
        const KIND: &'static str = ">=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>>`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtGt<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GtGt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">>" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGt<'tree> {
        const KIND: &'static str = ">>";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>>=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct GtGtEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> GtGtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for GtGtEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">>=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGtEq<'tree> {
        const KIND: &'static str = ">>=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `[`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBracket<'tree> {
        const KIND: &'static str = "[";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `]`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RBracket<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBracket<'tree> {
        const KIND: &'static str = "]";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `^`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitXor<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitXor<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BitXor<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXor<'tree> {
        const KIND: &'static str = "^";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `^=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitXorEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitXorEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BitXorEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "^=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXorEq<'tree> {
        const KIND: &'static str = "^=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `{`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for LBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBrace<'tree> {
        const KIND: &'static str = "{";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `|`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Or<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Or<'tree> {
        const KIND: &'static str = "|";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `|=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct OrEq<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> OrEq<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for OrEq<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "|=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for OrEq<'tree> {
        const KIND: &'static str = "|=";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `||`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct OrOr<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> OrOr<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for OrOr<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "||" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for OrOr<'tree> {
        const KIND: &'static str = "||";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `}`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for RBrace<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBrace<'tree> {
        const KIND: &'static str = "}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `~`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitNot<'tree>(yak_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitNot<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BitNot<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitNot<'tree> {
        const KIND: &'static str = "~";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: yak_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "one of `{_expression | _type | variadic_argument}`:\n- [Expression]\n- [Type]\n- [VariadicArgument]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Type_VariadicArgument<'tree> {
        Expression(Expression<'tree>),
        Type(Type<'tree>),
        VariadicArgument(VariadicArgument<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_Type_VariadicArgument<'tree> {
        #[doc = "Returns the node if it is of kind `_expression` ([Expression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_type` ([Type]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `variadic_argument` ([VariadicArgument]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn variadic_argument(self) -> Option<VariadicArgument<'tree>> {
            match self {
                Self::VariadicArgument(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Expression_Type_VariadicArgument<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <VariadicArgument<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::VariadicArgument(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_Type_VariadicArgument<'tree> {
        const KIND: &'static str = "{_expression | _type | variadic_argument}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::VariadicArgument(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::VariadicArgument(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_node(),
                Self::Type(x) => x.into_node(),
                Self::VariadicArgument(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{%= | &= | &^= | *= | += | -= | /= | <<= | = | >>= | ^= | |=}`:\n- [symbols::ModEq]\n- [symbols::AndEq]\n- [symbols::AndBitXorEq]\n- [symbols::MulEq]\n- [symbols::AddEq]\n- [symbols::SubEq]\n- [symbols::DivEq]\n- [symbols::LtLtEq]\n- [symbols::Eq]\n- [symbols::GtGtEq]\n- [symbols::BitXorEq]\n- [symbols::OrEq]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq<'tree> {
        ModEq(symbols::ModEq<'tree>),
        AndEq(symbols::AndEq<'tree>),
        AndBitXorEq(symbols::AndBitXorEq<'tree>),
        MulEq(symbols::MulEq<'tree>),
        AddEq(symbols::AddEq<'tree>),
        SubEq(symbols::SubEq<'tree>),
        DivEq(symbols::DivEq<'tree>),
        LtLtEq(symbols::LtLtEq<'tree>),
        Eq(symbols::Eq<'tree>),
        GtGtEq(symbols::GtGtEq<'tree>),
        BitXorEq(symbols::BitXorEq<'tree>),
        OrEq(symbols::OrEq<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq<'tree> {
        #[doc = "Returns the node if it is of kind `%=` ([symbols::ModEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mod_eq(self) -> Option<symbols::ModEq<'tree>> {
            match self {
                Self::ModEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&=` ([symbols::AndEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_eq(self) -> Option<symbols::AndEq<'tree>> {
            match self {
                Self::AndEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&^=` ([symbols::AndBitXorEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_bit_xor_eq(self) -> Option<symbols::AndBitXorEq<'tree>> {
            match self {
                Self::AndBitXorEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `*=` ([symbols::MulEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul_eq(self) -> Option<symbols::MulEq<'tree>> {
            match self {
                Self::MulEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `+=` ([symbols::AddEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add_eq(self) -> Option<symbols::AddEq<'tree>> {
            match self {
                Self::AddEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `-=` ([symbols::SubEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_eq(self) -> Option<symbols::SubEq<'tree>> {
            match self {
                Self::SubEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `/=` ([symbols::DivEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn div_eq(self) -> Option<symbols::DivEq<'tree>> {
            match self {
                Self::DivEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<<=` ([symbols::LtLtEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_lt_eq(self) -> Option<symbols::LtLtEq<'tree>> {
            match self {
                Self::LtLtEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `=` ([symbols::Eq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eq(self) -> Option<symbols::Eq<'tree>> {
            match self {
                Self::Eq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `>>=` ([symbols::GtGtEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_gt_eq(self) -> Option<symbols::GtGtEq<'tree>> {
            match self {
                Self::GtGtEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `^=` ([symbols::BitXorEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bit_xor_eq(self) -> Option<symbols::BitXorEq<'tree>> {
            match self {
                Self::BitXorEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `|=` ([symbols::OrEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or_eq(self) -> Option<symbols::OrEq<'tree>> {
            match self {
                Self::OrEq(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "%=" => Ok(unsafe {
                    Self::ModEq(<symbols::ModEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "&=" => Ok(unsafe {
                    Self::AndEq(<symbols::AndEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "&^=" => Ok(unsafe {
                    Self :: AndBitXorEq (< symbols :: AndBitXorEq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "*=" => Ok(unsafe {
                    Self::MulEq(<symbols::MulEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "+=" => Ok(unsafe {
                    Self::AddEq(<symbols::AddEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "-=" => Ok(unsafe {
                    Self::SubEq(<symbols::SubEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "/=" => Ok(unsafe {
                    Self::DivEq(<symbols::DivEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "<<=" => Ok(unsafe {
                    Self::LtLtEq(<symbols::LtLtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "=" => Ok(unsafe {
                    Self :: Eq (< symbols :: Eq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                ">>=" => Ok(unsafe {
                    Self::GtGtEq(<symbols::GtGtEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "^=" => Ok(unsafe {
                    Self::BitXorEq(<symbols::BitXorEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "|=" => Ok(unsafe {
                    Self::OrEq(<symbols::OrEq<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ModEq_AndEq_AndBitXorEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_Eq_GtGtEq_BitXorEq_OrEq<'tree>
    {
        const KIND: &'static str = "{%= | &= | &^= | *= | += | -= | /= | <<= | = | >>= | ^= | |=}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.node(),
                Self::AndEq(x) => x.node(),
                Self::AndBitXorEq(x) => x.node(),
                Self::MulEq(x) => x.node(),
                Self::AddEq(x) => x.node(),
                Self::SubEq(x) => x.node(),
                Self::DivEq(x) => x.node(),
                Self::LtLtEq(x) => x.node(),
                Self::Eq(x) => x.node(),
                Self::GtGtEq(x) => x.node(),
                Self::BitXorEq(x) => x.node(),
                Self::OrEq(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.node_mut(),
                Self::AndEq(x) => x.node_mut(),
                Self::AndBitXorEq(x) => x.node_mut(),
                Self::MulEq(x) => x.node_mut(),
                Self::AddEq(x) => x.node_mut(),
                Self::SubEq(x) => x.node_mut(),
                Self::DivEq(x) => x.node_mut(),
                Self::LtLtEq(x) => x.node_mut(),
                Self::Eq(x) => x.node_mut(),
                Self::GtGtEq(x) => x.node_mut(),
                Self::BitXorEq(x) => x.node_mut(),
                Self::OrEq(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::ModEq(x) => x.into_node(),
                Self::AndEq(x) => x.into_node(),
                Self::AndBitXorEq(x) => x.into_node(),
                Self::MulEq(x) => x.into_node(),
                Self::AddEq(x) => x.into_node(),
                Self::SubEq(x) => x.into_node(),
                Self::DivEq(x) => x.into_node(),
                Self::LtLtEq(x) => x.into_node(),
                Self::Eq(x) => x.into_node(),
                Self::GtGtEq(x) => x.into_node(),
                Self::BitXorEq(x) => x.into_node(),
                Self::OrEq(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{!= | % | & | && | &^ | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}`:\n- [symbols::NotEq]\n- [symbols::Mod]\n- [symbols::And]\n- [symbols::AndAnd]\n- [symbols::AndBitXor]\n- [symbols::Mul]\n- [symbols::Add]\n- [symbols::Sub]\n- [symbols::Div]\n- [symbols::Lt]\n- [symbols::LtLt]\n- [symbols::LtEq]\n- [symbols::EqEq]\n- [symbols::Gt]\n- [symbols::GtEq]\n- [symbols::GtGt]\n- [symbols::BitXor]\n- [symbols::Or]\n- [symbols::OrOr]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<
        'tree,
    > {
        NotEq(symbols::NotEq<'tree>),
        Mod(symbols::Mod<'tree>),
        And(symbols::And<'tree>),
        AndAnd(symbols::AndAnd<'tree>),
        AndBitXor(symbols::AndBitXor<'tree>),
        Mul(symbols::Mul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Div(symbols::Div<'tree>),
        Lt(symbols::Lt<'tree>),
        LtLt(symbols::LtLt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        GtGt(symbols::GtGt<'tree>),
        BitXor(symbols::BitXor<'tree>),
        Or(symbols::Or<'tree>),
        OrOr(symbols::OrOr<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr<
            'tree,
        >
    {
        #[doc = "Returns the node if it is of kind `!=` ([symbols::NotEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not_eq(self) -> Option<symbols::NotEq<'tree>> {
            match self {
                Self::NotEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `%` ([symbols::Mod]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#mod(self) -> Option<symbols::Mod<'tree>> {
            match self {
                Self::Mod(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&` ([symbols::And]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and(self) -> Option<symbols::And<'tree>> {
            match self {
                Self::And(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&&` ([symbols::AndAnd]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_and(self) -> Option<symbols::AndAnd<'tree>> {
            match self {
                Self::AndAnd(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&^` ([symbols::AndBitXor]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and_bit_xor(self) -> Option<symbols::AndBitXor<'tree>> {
            match self {
                Self::AndBitXor(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `*` ([symbols::Mul]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul(self) -> Option<symbols::Mul<'tree>> {
            match self {
                Self::Mul(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `+` ([symbols::Add]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add(self) -> Option<symbols::Add<'tree>> {
            match self {
                Self::Add(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `-` ([symbols::Sub]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub(self) -> Option<symbols::Sub<'tree>> {
            match self {
                Self::Sub(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `/` ([symbols::Div]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn div(self) -> Option<symbols::Div<'tree>> {
            match self {
                Self::Div(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<` ([symbols::Lt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt(self) -> Option<symbols::Lt<'tree>> {
            match self {
                Self::Lt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<<` ([symbols::LtLt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_lt(self) -> Option<symbols::LtLt<'tree>> {
            match self {
                Self::LtLt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<=` ([symbols::LtEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_eq(self) -> Option<symbols::LtEq<'tree>> {
            match self {
                Self::LtEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `==` ([symbols::EqEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eq_eq(self) -> Option<symbols::EqEq<'tree>> {
            match self {
                Self::EqEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `>` ([symbols::Gt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt(self) -> Option<symbols::Gt<'tree>> {
            match self {
                Self::Gt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `>=` ([symbols::GtEq]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_eq(self) -> Option<symbols::GtEq<'tree>> {
            match self {
                Self::GtEq(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `>>` ([symbols::GtGt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt_gt(self) -> Option<symbols::GtGt<'tree>> {
            match self {
                Self::GtGt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `^` ([symbols::BitXor]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bit_xor(self) -> Option<symbols::BitXor<'tree>> {
            match self {
                Self::BitXor(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `|` ([symbols::Or]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or(self) -> Option<symbols::Or<'tree>> {
            match self {
                Self::Or(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `||` ([symbols::OrOr]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn or_or(self) -> Option<symbols::OrOr<'tree>> {
            match self {
                Self::OrOr(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl < 'tree > TryFrom < yak_sitter :: Node < 'tree >> for NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : yak_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "!=" => Ok (unsafe { Self :: NotEq (< symbols :: NotEq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "%" => Ok (unsafe { Self :: Mod (< symbols :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&" => Ok (unsafe { Self :: And (< symbols :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&&" => Ok (unsafe { Self :: AndAnd (< symbols :: AndAnd < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&^" => Ok (unsafe { Self :: AndBitXor (< symbols :: AndBitXor < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "*" => Ok (unsafe { Self :: Mul (< symbols :: Mul < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "+" => Ok (unsafe { Self :: Add (< symbols :: Add < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub (< symbols :: Sub < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "/" => Ok (unsafe { Self :: Div (< symbols :: Div < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<" => Ok (unsafe { Self :: Lt (< symbols :: Lt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<<" => Ok (unsafe { Self :: LtLt (< symbols :: LtLt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<=" => Ok (unsafe { Self :: LtEq (< symbols :: LtEq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "==" => Ok (unsafe { Self :: EqEq (< symbols :: EqEq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">" => Ok (unsafe { Self :: Gt (< symbols :: Gt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">=" => Ok (unsafe { Self :: GtEq (< symbols :: GtEq < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">>" => Ok (unsafe { Self :: GtGt (< symbols :: GtGt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "^" => Ok (unsafe { Self :: BitXor (< symbols :: BitXor < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "|" => Ok (unsafe { Self :: Or (< symbols :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "||" => Ok (unsafe { Self :: OrOr (< symbols :: OrOr < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for NotEq_Mod_And_AndAnd_AndBitXor_Mul_Add_Sub_Div_Lt_LtLt_LtEq_EqEq_Gt_GtEq_GtGt_BitXor_Or_OrOr < 'tree > { const KIND : & 'static str = "{!= | % | & | && | &^ | * | + | - | / | < | << | <= | == | > | >= | >> | ^ | | | ||}" ; # [inline] fn node (& self) -> & yak_sitter :: Node < 'tree > { match self { Self :: NotEq (x) => x . node () , Self :: Mod (x) => x . node () , Self :: And (x) => x . node () , Self :: AndAnd (x) => x . node () , Self :: AndBitXor (x) => x . node () , Self :: Mul (x) => x . node () , Self :: Add (x) => x . node () , Self :: Sub (x) => x . node () , Self :: Div (x) => x . node () , Self :: Lt (x) => x . node () , Self :: LtLt (x) => x . node () , Self :: LtEq (x) => x . node () , Self :: EqEq (x) => x . node () , Self :: Gt (x) => x . node () , Self :: GtEq (x) => x . node () , Self :: GtGt (x) => x . node () , Self :: BitXor (x) => x . node () , Self :: Or (x) => x . node () , Self :: OrOr (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut yak_sitter :: Node < 'tree > { match self { Self :: NotEq (x) => x . node_mut () , Self :: Mod (x) => x . node_mut () , Self :: And (x) => x . node_mut () , Self :: AndAnd (x) => x . node_mut () , Self :: AndBitXor (x) => x . node_mut () , Self :: Mul (x) => x . node_mut () , Self :: Add (x) => x . node_mut () , Self :: Sub (x) => x . node_mut () , Self :: Div (x) => x . node_mut () , Self :: Lt (x) => x . node_mut () , Self :: LtLt (x) => x . node_mut () , Self :: LtEq (x) => x . node_mut () , Self :: EqEq (x) => x . node_mut () , Self :: Gt (x) => x . node_mut () , Self :: GtEq (x) => x . node_mut () , Self :: GtGt (x) => x . node_mut () , Self :: BitXor (x) => x . node_mut () , Self :: Or (x) => x . node_mut () , Self :: OrOr (x) => x . node_mut () , } } # [inline] fn into_node (self) -> yak_sitter :: Node < 'tree > { match self { Self :: NotEq (x) => x . into_node () , Self :: Mod (x) => x . into_node () , Self :: And (x) => x . into_node () , Self :: AndAnd (x) => x . into_node () , Self :: AndBitXor (x) => x . into_node () , Self :: Mul (x) => x . into_node () , Self :: Add (x) => x . into_node () , Self :: Sub (x) => x . into_node () , Self :: Div (x) => x . into_node () , Self :: Lt (x) => x . into_node () , Self :: LtLt (x) => x . into_node () , Self :: LtEq (x) => x . into_node () , Self :: EqEq (x) => x . into_node () , Self :: Gt (x) => x . into_node () , Self :: GtEq (x) => x . into_node () , Self :: GtGt (x) => x . into_node () , Self :: BitXor (x) => x . into_node () , Self :: Or (x) => x . into_node () , Self :: OrOr (x) => x . into_node () , } } }
    #[doc = "one of `{_statement | receive_statement | send_statement}`:\n- [Statement]\n- [ReceiveStatement]\n- [SendStatement]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Statement_ReceiveStatement_SendStatement<'tree> {
        Statement(Statement<'tree>),
        ReceiveStatement(ReceiveStatement<'tree>),
        SendStatement(SendStatement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Statement_ReceiveStatement_SendStatement<'tree> {
        #[doc = "Returns the node if it is of kind `_statement` ([Statement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn statement(self) -> Option<Statement<'tree>> {
            match self {
                Self::Statement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `receive_statement` ([ReceiveStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn receive_statement(self) -> Option<ReceiveStatement<'tree>> {
            match self {
                Self::ReceiveStatement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `send_statement` ([SendStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn send_statement(self) -> Option<SendStatement<'tree>> {
            match self {
                Self::SendStatement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Statement_ReceiveStatement_SendStatement<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Statement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Statement(this));
            }
            if let Ok(this) = <ReceiveStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ReceiveStatement(this));
            }
            if let Ok(this) = <SendStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::SendStatement(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Statement_ReceiveStatement_SendStatement<'tree> {
        const KIND: &'static str = "{_statement | receive_statement | send_statement}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node(),
                Self::ReceiveStatement(x) => x.node(),
                Self::SendStatement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node_mut(),
                Self::ReceiveStatement(x) => x.node_mut(),
                Self::SendStatement(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.into_node(),
                Self::ReceiveStatement(x) => x.into_node(),
                Self::SendStatement(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{receive_statement | send_statement}`:\n- [ReceiveStatement]\n- [SendStatement]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ReceiveStatement_SendStatement<'tree> {
        ReceiveStatement(ReceiveStatement<'tree>),
        SendStatement(SendStatement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ReceiveStatement_SendStatement<'tree> {
        #[doc = "Returns the node if it is of kind `receive_statement` ([ReceiveStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn receive_statement(self) -> Option<ReceiveStatement<'tree>> {
            match self {
                Self::ReceiveStatement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `send_statement` ([SendStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn send_statement(self) -> Option<SendStatement<'tree>> {
            match self {
                Self::SendStatement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ReceiveStatement_SendStatement<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "receive_statement" => Ok(unsafe {
                    Self :: ReceiveStatement (< ReceiveStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "send_statement" => Ok(unsafe {
                    Self::SendStatement(<SendStatement<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ReceiveStatement_SendStatement<'tree> {
        const KIND: &'static str = "{receive_statement | send_statement}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::ReceiveStatement(x) => x.node(),
                Self::SendStatement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::ReceiveStatement(x) => x.node_mut(),
                Self::SendStatement(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::ReceiveStatement(x) => x.into_node(),
                Self::SendStatement(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{array_type | generic_type | implicit_length_array_type | map_type | qualified_type | slice_type | struct_type | type_identifier}`:\n- [ArrayType]\n- [GenericType]\n- [ImplicitLengthArrayType]\n- [MapType]\n- [QualifiedType]\n- [SliceType]\n- [StructType]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier<
        'tree,
    > {
        ArrayType(ArrayType<'tree>),
        GenericType(GenericType<'tree>),
        ImplicitLengthArrayType(ImplicitLengthArrayType<'tree>),
        MapType(MapType<'tree>),
        QualifiedType(QualifiedType<'tree>),
        SliceType(SliceType<'tree>),
        StructType(StructType<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier < 'tree > { # [doc = "Returns the node if it is of kind `array_type` ([ArrayType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn array_type (self) -> Option < ArrayType < 'tree > > { match self { Self :: ArrayType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `generic_type` ([GenericType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn generic_type (self) -> Option < GenericType < 'tree > > { match self { Self :: GenericType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `implicit_length_array_type` ([ImplicitLengthArrayType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn implicit_length_array_type (self) -> Option < ImplicitLengthArrayType < 'tree > > { match self { Self :: ImplicitLengthArrayType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `map_type` ([MapType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn map_type (self) -> Option < MapType < 'tree > > { match self { Self :: MapType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_type` ([QualifiedType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_type (self) -> Option < QualifiedType < 'tree > > { match self { Self :: QualifiedType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `slice_type` ([SliceType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn slice_type (self) -> Option < SliceType < 'tree > > { match self { Self :: SliceType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `struct_type` ([StructType]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn struct_type (self) -> Option < StructType < 'tree > > { match self { Self :: StructType (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn type_identifier (self) -> Option < TypeIdentifier < 'tree > > { match self { Self :: TypeIdentifier (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < yak_sitter :: Node < 'tree >> for ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : yak_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "array_type" => Ok (unsafe { Self :: ArrayType (< ArrayType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "generic_type" => Ok (unsafe { Self :: GenericType (< GenericType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "implicit_length_array_type" => Ok (unsafe { Self :: ImplicitLengthArrayType (< ImplicitLengthArrayType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "map_type" => Ok (unsafe { Self :: MapType (< MapType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_type" => Ok (unsafe { Self :: QualifiedType (< QualifiedType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "slice_type" => Ok (unsafe { Self :: SliceType (< SliceType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "struct_type" => Ok (unsafe { Self :: StructType (< StructType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "type_identifier" => Ok (unsafe { Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for ArrayType_GenericType_ImplicitLengthArrayType_MapType_QualifiedType_SliceType_StructType_TypeIdentifier < 'tree > { const KIND : & 'static str = "{array_type | generic_type | implicit_length_array_type | map_type | qualified_type | slice_type | struct_type | type_identifier}" ; # [inline] fn node (& self) -> & yak_sitter :: Node < 'tree > { match self { Self :: ArrayType (x) => x . node () , Self :: GenericType (x) => x . node () , Self :: ImplicitLengthArrayType (x) => x . node () , Self :: MapType (x) => x . node () , Self :: QualifiedType (x) => x . node () , Self :: SliceType (x) => x . node () , Self :: StructType (x) => x . node () , Self :: TypeIdentifier (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut yak_sitter :: Node < 'tree > { match self { Self :: ArrayType (x) => x . node_mut () , Self :: GenericType (x) => x . node_mut () , Self :: ImplicitLengthArrayType (x) => x . node_mut () , Self :: MapType (x) => x . node_mut () , Self :: QualifiedType (x) => x . node_mut () , Self :: SliceType (x) => x . node_mut () , Self :: StructType (x) => x . node_mut () , Self :: TypeIdentifier (x) => x . node_mut () , } } # [inline] fn into_node (self) -> yak_sitter :: Node < 'tree > { match self { Self :: ArrayType (x) => x . into_node () , Self :: GenericType (x) => x . into_node () , Self :: ImplicitLengthArrayType (x) => x . into_node () , Self :: MapType (x) => x . into_node () , Self :: QualifiedType (x) => x . into_node () , Self :: SliceType (x) => x . into_node () , Self :: StructType (x) => x . into_node () , Self :: TypeIdentifier (x) => x . into_node () , } } }
    #[doc = "one of `{, | identifier}`:\n- [symbols::Comma]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comma_Identifier<'tree> {
        Comma(symbols::Comma<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Comma_Identifier<'tree> {
        #[doc = "Returns the node if it is of kind `,` ([symbols::Comma]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comma(self) -> Option<symbols::Comma<'tree>> {
            match self {
                Self::Comma(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Comma_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "," => Ok(unsafe {
                    Self::Comma(<symbols::Comma<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma_Identifier<'tree> {
        const KIND: &'static str = "{, | identifier}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_statement | expression_list}`:\n- [Statement]\n- [ExpressionList]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Statement_ExpressionList<'tree> {
        Statement(Statement<'tree>),
        ExpressionList(ExpressionList<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Statement_ExpressionList<'tree> {
        #[doc = "Returns the node if it is of kind `_statement` ([Statement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn statement(self) -> Option<Statement<'tree>> {
            match self {
                Self::Statement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `expression_list` ([ExpressionList]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression_list(self) -> Option<ExpressionList<'tree>> {
            match self {
                Self::ExpressionList(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Statement_ExpressionList<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Statement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Statement(this));
            }
            if let Ok(this) = <ExpressionList<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ExpressionList(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Statement_ExpressionList<'tree> {
        const KIND: &'static str = "{_statement | expression_list}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node(),
                Self::ExpressionList(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node_mut(),
                Self::ExpressionList(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.into_node(),
                Self::ExpressionList(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{default_case | expression_case | _simple_statement | _expression}`:\n- [DefaultCase]\n- [ExpressionCase]\n- [SimpleStatement]\n- [Expression]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree> {
        DefaultCase(DefaultCase<'tree>),
        ExpressionCase(ExpressionCase<'tree>),
        SimpleStatement(SimpleStatement<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree> {
        #[doc = "Returns the node if it is of kind `default_case` ([DefaultCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn default_case(self) -> Option<DefaultCase<'tree>> {
            match self {
                Self::DefaultCase(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `expression_case` ([ExpressionCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression_case(self) -> Option<ExpressionCase<'tree>> {
            match self {
                Self::ExpressionCase(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_simple_statement` ([SimpleStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn simple_statement(self) -> Option<SimpleStatement<'tree>> {
            match self {
                Self::SimpleStatement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_expression` ([Expression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <DefaultCase<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::DefaultCase(this));
            }
            if let Ok(this) = <ExpressionCase<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ExpressionCase(this));
            }
            if let Ok(this) = <SimpleStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::SimpleStatement(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for DefaultCase_ExpressionCase_SimpleStatement_Expression<'tree>
    {
        const KIND: &'static str =
            "{default_case | expression_case | _simple_statement | _expression}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.node(),
                Self::ExpressionCase(x) => x.node(),
                Self::SimpleStatement(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.node_mut(),
                Self::ExpressionCase(x) => x.node_mut(),
                Self::SimpleStatement(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.into_node(),
                Self::ExpressionCase(x) => x.into_node(),
                Self::SimpleStatement(x) => x.into_node(),
                Self::Expression(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{interpreted_string_literal | raw_string_literal}`:\n- [InterpretedStringLiteral]\n- [RawStringLiteral]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum InterpretedStringLiteral_RawStringLiteral<'tree> {
        InterpretedStringLiteral(InterpretedStringLiteral<'tree>),
        RawStringLiteral(RawStringLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> InterpretedStringLiteral_RawStringLiteral<'tree> {
        #[doc = "Returns the node if it is of kind `interpreted_string_literal` ([InterpretedStringLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn interpreted_string_literal(self) -> Option<InterpretedStringLiteral<'tree>> {
            match self {
                Self::InterpretedStringLiteral(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `raw_string_literal` ([RawStringLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
            match self {
                Self::RawStringLiteral(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for InterpretedStringLiteral_RawStringLiteral<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "interpreted_string_literal" => Ok(unsafe {
                    Self :: InterpretedStringLiteral (< InterpretedStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "raw_string_literal" => Ok(unsafe {
                    Self :: RawStringLiteral (< RawStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for InterpretedStringLiteral_RawStringLiteral<'tree> {
        const KIND: &'static str = "{interpreted_string_literal | raw_string_literal}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::InterpretedStringLiteral(x) => x.node(),
                Self::RawStringLiteral(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::InterpretedStringLiteral(x) => x.node_mut(),
                Self::RawStringLiteral(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::InterpretedStringLiteral(x) => x.into_node(),
                Self::RawStringLiteral(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_type | generic_type | qualified_type | type_identifier}`:\n- [Type]\n- [GenericType]\n- [QualifiedType]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type_GenericType_QualifiedType_TypeIdentifier<'tree> {
        Type(Type<'tree>),
        GenericType(GenericType<'tree>),
        QualifiedType(QualifiedType<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type_GenericType_QualifiedType_TypeIdentifier<'tree> {
        #[doc = "Returns the node if it is of kind `_type` ([Type]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `generic_type` ([GenericType]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn generic_type(self) -> Option<GenericType<'tree>> {
            match self {
                Self::GenericType(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `qualified_type` ([QualifiedType]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn qualified_type(self) -> Option<QualifiedType<'tree>> {
            match self {
                Self::QualifiedType(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for Type_GenericType_QualifiedType_TypeIdentifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            if let Ok(this) = <GenericType<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::GenericType(this));
            }
            if let Ok(this) = <QualifiedType<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::QualifiedType(this));
            }
            if let Ok(this) = <TypeIdentifier<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeIdentifier(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Type_GenericType_QualifiedType_TypeIdentifier<'tree>
    {
        const KIND: &'static str = "{_type | generic_type | qualified_type | type_identifier}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node(),
                Self::GenericType(x) => x.node(),
                Self::QualifiedType(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.node_mut(),
                Self::GenericType(x) => x.node_mut(),
                Self::QualifiedType(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Type(x) => x.into_node(),
                Self::GenericType(x) => x.into_node(),
                Self::QualifiedType(x) => x.into_node(),
                Self::TypeIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_expression | for_clause | range_clause | block}`:\n- [Expression]\n- [ForClause]\n- [RangeClause]\n- [Block]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_ForClause_RangeClause_Block<'tree> {
        Expression(Expression<'tree>),
        ForClause(ForClause<'tree>),
        RangeClause(RangeClause<'tree>),
        Block(Block<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_ForClause_RangeClause_Block<'tree> {
        #[doc = "Returns the node if it is of kind `_expression` ([Expression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `for_clause` ([ForClause]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn for_clause(self) -> Option<ForClause<'tree>> {
            match self {
                Self::ForClause(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `range_clause` ([RangeClause]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn range_clause(self) -> Option<RangeClause<'tree>> {
            match self {
                Self::RangeClause(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `block` ([Block]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Expression_ForClause_RangeClause_Block<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <ForClause<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ForClause(this));
            }
            if let Ok(this) = <RangeClause<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::RangeClause(this));
            }
            if let Ok(this) = <Block<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Block(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_ForClause_RangeClause_Block<'tree> {
        const KIND: &'static str = "{_expression | for_clause | range_clause | block}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::ForClause(x) => x.node(),
                Self::RangeClause(x) => x.node(),
                Self::Block(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::ForClause(x) => x.node_mut(),
                Self::RangeClause(x) => x.node_mut(),
                Self::Block(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_node(),
                Self::ForClause(x) => x.into_node(),
                Self::RangeClause(x) => x.into_node(),
                Self::Block(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_simple_type | parameter_list}`:\n- [SimpleType]\n- [ParameterList]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum SimpleType_ParameterList<'tree> {
        SimpleType(SimpleType<'tree>),
        ParameterList(ParameterList<'tree>),
    }
    #[automatically_derived]
    impl<'tree> SimpleType_ParameterList<'tree> {
        #[doc = "Returns the node if it is of kind `_simple_type` ([SimpleType]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn simple_type(self) -> Option<SimpleType<'tree>> {
            match self {
                Self::SimpleType(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `parameter_list` ([ParameterList]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn parameter_list(self) -> Option<ParameterList<'tree>> {
            match self {
                Self::ParameterList(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for SimpleType_ParameterList<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <SimpleType<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::SimpleType(this));
            }
            if let Ok(this) = <ParameterList<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ParameterList(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for SimpleType_ParameterList<'tree> {
        const KIND: &'static str = "{_simple_type | parameter_list}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::SimpleType(x) => x.node(),
                Self::ParameterList(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::SimpleType(x) => x.node_mut(),
                Self::ParameterList(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::SimpleType(x) => x.into_node(),
                Self::ParameterList(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{negated_type | qualified_type | type_identifier}`:\n- [NegatedType]\n- [QualifiedType]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NegatedType_QualifiedType_TypeIdentifier<'tree> {
        NegatedType(NegatedType<'tree>),
        QualifiedType(QualifiedType<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> NegatedType_QualifiedType_TypeIdentifier<'tree> {
        #[doc = "Returns the node if it is of kind `negated_type` ([NegatedType]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn negated_type(self) -> Option<NegatedType<'tree>> {
            match self {
                Self::NegatedType(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `qualified_type` ([QualifiedType]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn qualified_type(self) -> Option<QualifiedType<'tree>> {
            match self {
                Self::QualifiedType(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for NegatedType_QualifiedType_TypeIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "negated_type" => Ok(unsafe {
                    Self :: NegatedType (< NegatedType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "qualified_type" => Ok(unsafe {
                    Self::QualifiedType(<QualifiedType<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for NegatedType_QualifiedType_TypeIdentifier<'tree> {
        const KIND: &'static str = "{negated_type | qualified_type | type_identifier}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::NegatedType(x) => x.node(),
                Self::QualifiedType(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::NegatedType(x) => x.node_mut(),
                Self::QualifiedType(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::NegatedType(x) => x.into_node(),
                Self::QualifiedType(x) => x.into_node(),
                Self::TypeIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{block | if_statement}`:\n- [Block]\n- [IfStatement]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Block_IfStatement<'tree> {
        Block(Block<'tree>),
        IfStatement(IfStatement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Block_IfStatement<'tree> {
        #[doc = "Returns the node if it is of kind `block` ([Block]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block(self) -> Option<Block<'tree>> {
            match self {
                Self::Block(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `if_statement` ([IfStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn if_statement(self) -> Option<IfStatement<'tree>> {
            match self {
                Self::IfStatement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Block_IfStatement<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "block" => {
                    Ok(unsafe {
                        Self :: Block (< Block < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "if_statement" => Ok(unsafe {
                    Self :: IfStatement (< IfStatement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Block_IfStatement<'tree> {
        const KIND: &'static str = "{block | if_statement}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.node(),
                Self::IfStatement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.node_mut(),
                Self::IfStatement(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Block(x) => x.into_node(),
                Self::IfStatement(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{import_spec | import_spec_list}`:\n- [ImportSpec]\n- [ImportSpecList]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ImportSpec_ImportSpecList<'tree> {
        ImportSpec(ImportSpec<'tree>),
        ImportSpecList(ImportSpecList<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ImportSpec_ImportSpecList<'tree> {
        #[doc = "Returns the node if it is of kind `import_spec` ([ImportSpec]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn import_spec(self) -> Option<ImportSpec<'tree>> {
            match self {
                Self::ImportSpec(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `import_spec_list` ([ImportSpecList]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn import_spec_list(self) -> Option<ImportSpecList<'tree>> {
            match self {
                Self::ImportSpecList(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for ImportSpec_ImportSpecList<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "import_spec" => Ok(unsafe {
                    Self :: ImportSpec (< ImportSpec < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "import_spec_list" => {
                    Ok(unsafe {
                        Self :: ImportSpecList (< ImportSpecList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ImportSpec_ImportSpecList<'tree> {
        const KIND: &'static str = "{import_spec | import_spec_list}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::ImportSpec(x) => x.node(),
                Self::ImportSpecList(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::ImportSpec(x) => x.node_mut(),
                Self::ImportSpecList(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::ImportSpec(x) => x.into_node(),
                Self::ImportSpecList(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{blank_identifier | dot | package_identifier}`:\n- [BlankIdentifier]\n- [Dot]\n- [PackageIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BlankIdentifier_Dot_PackageIdentifier<'tree> {
        BlankIdentifier(BlankIdentifier<'tree>),
        Dot(Dot<'tree>),
        PackageIdentifier(PackageIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> BlankIdentifier_Dot_PackageIdentifier<'tree> {
        #[doc = "Returns the node if it is of kind `blank_identifier` ([BlankIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn blank_identifier(self) -> Option<BlankIdentifier<'tree>> {
            match self {
                Self::BlankIdentifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `dot` ([Dot]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn dot(self) -> Option<Dot<'tree>> {
            match self {
                Self::Dot(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `package_identifier` ([PackageIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn package_identifier(self) -> Option<PackageIdentifier<'tree>> {
            match self {
                Self::PackageIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for BlankIdentifier_Dot_PackageIdentifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "blank_identifier" => {
                    Ok(unsafe {
                        Self :: BlankIdentifier (< BlankIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "dot" => Ok(unsafe {
                    Self::Dot(
                        <Dot<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "package_identifier" => Ok(unsafe {
                    Self :: PackageIdentifier (< PackageIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BlankIdentifier_Dot_PackageIdentifier<'tree> {
        const KIND: &'static str = "{blank_identifier | dot | package_identifier}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::BlankIdentifier(x) => x.node(),
                Self::Dot(x) => x.node(),
                Self::PackageIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::BlankIdentifier(x) => x.node_mut(),
                Self::Dot(x) => x.node_mut(),
                Self::PackageIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::BlankIdentifier(x) => x.into_node(),
                Self::Dot(x) => x.into_node(),
                Self::PackageIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{method_elem | type_elem}`:\n- [MethodElem]\n- [TypeElem]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MethodElem_TypeElem<'tree> {
        MethodElem(MethodElem<'tree>),
        TypeElem(TypeElem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> MethodElem_TypeElem<'tree> {
        #[doc = "Returns the node if it is of kind `method_elem` ([MethodElem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn method_elem(self) -> Option<MethodElem<'tree>> {
            match self {
                Self::MethodElem(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_elem` ([TypeElem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_elem(self) -> Option<TypeElem<'tree>> {
            match self {
                Self::TypeElem(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for MethodElem_TypeElem<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "method_elem" => Ok(unsafe {
                    Self :: MethodElem (< MethodElem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_elem" => {
                    Ok(unsafe {
                        Self :: TypeElem (< TypeElem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for MethodElem_TypeElem<'tree> {
        const KIND: &'static str = "{method_elem | type_elem}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::MethodElem(x) => x.node(),
                Self::TypeElem(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::MethodElem(x) => x.node_mut(),
                Self::TypeElem(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::MethodElem(x) => x.into_node(),
                Self::TypeElem(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_statement | label_name}`:\n- [Statement]\n- [LabelName]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Statement_LabelName<'tree> {
        Statement(Statement<'tree>),
        LabelName(LabelName<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Statement_LabelName<'tree> {
        #[doc = "Returns the node if it is of kind `_statement` ([Statement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn statement(self) -> Option<Statement<'tree>> {
            match self {
                Self::Statement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `label_name` ([LabelName]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn label_name(self) -> Option<LabelName<'tree>> {
            match self {
                Self::LabelName(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Statement_LabelName<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Statement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Statement(this));
            }
            if let Ok(this) = <LabelName<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LabelName(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Statement_LabelName<'tree> {
        const KIND: &'static str = "{_statement | label_name}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node(),
                Self::LabelName(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node_mut(),
                Self::LabelName(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.into_node(),
                Self::LabelName(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_expression | literal_value}`:\n- [Expression]\n- [LiteralValue]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_LiteralValue<'tree> {
        Expression(Expression<'tree>),
        LiteralValue(LiteralValue<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Expression_LiteralValue<'tree> {
        #[doc = "Returns the node if it is of kind `_expression` ([Expression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `literal_value` ([LiteralValue]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn literal_value(self) -> Option<LiteralValue<'tree>> {
            match self {
                Self::LiteralValue(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Expression_LiteralValue<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <LiteralValue<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::LiteralValue(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression_LiteralValue<'tree> {
        const KIND: &'static str = "{_expression | literal_value}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node(),
                Self::LiteralValue(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.node_mut(),
                Self::LiteralValue(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_node(),
                Self::LiteralValue(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{keyed_element | literal_element}`:\n- [KeyedElement]\n- [LiteralElement]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum KeyedElement_LiteralElement<'tree> {
        KeyedElement(KeyedElement<'tree>),
        LiteralElement(LiteralElement<'tree>),
    }
    #[automatically_derived]
    impl<'tree> KeyedElement_LiteralElement<'tree> {
        #[doc = "Returns the node if it is of kind `keyed_element` ([KeyedElement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn keyed_element(self) -> Option<KeyedElement<'tree>> {
            match self {
                Self::KeyedElement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `literal_element` ([LiteralElement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn literal_element(self) -> Option<LiteralElement<'tree>> {
            match self {
                Self::LiteralElement(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for KeyedElement_LiteralElement<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "keyed_element" => Ok(unsafe {
                    Self :: KeyedElement (< KeyedElement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "literal_element" => {
                    Ok(unsafe {
                        Self :: LiteralElement (< LiteralElement < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for KeyedElement_LiteralElement<'tree> {
        const KIND: &'static str = "{keyed_element | literal_element}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::KeyedElement(x) => x.node(),
                Self::LiteralElement(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::KeyedElement(x) => x.node_mut(),
                Self::LiteralElement(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::KeyedElement(x) => x.into_node(),
                Self::LiteralElement(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{parameter_declaration | variadic_parameter_declaration}`:\n- [ParameterDeclaration]\n- [VariadicParameterDeclaration]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ParameterDeclaration_VariadicParameterDeclaration<'tree> {
        ParameterDeclaration(ParameterDeclaration<'tree>),
        VariadicParameterDeclaration(VariadicParameterDeclaration<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ParameterDeclaration_VariadicParameterDeclaration<'tree> {
        #[doc = "Returns the node if it is of kind `parameter_declaration` ([ParameterDeclaration]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn parameter_declaration(self) -> Option<ParameterDeclaration<'tree>> {
            match self {
                Self::ParameterDeclaration(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `variadic_parameter_declaration` ([VariadicParameterDeclaration]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn variadic_parameter_declaration(self) -> Option<VariadicParameterDeclaration<'tree>> {
            match self {
                Self::VariadicParameterDeclaration(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for ParameterDeclaration_VariadicParameterDeclaration<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "parameter_declaration" => Ok(unsafe {
                    Self :: ParameterDeclaration (< ParameterDeclaration < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "variadic_parameter_declaration" => Ok(unsafe {
                    Self::VariadicParameterDeclaration(
                        <VariadicParameterDeclaration<'tree> as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ParameterDeclaration_VariadicParameterDeclaration<'tree>
    {
        const KIND: &'static str = "{parameter_declaration | variadic_parameter_declaration}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::ParameterDeclaration(x) => x.node(),
                Self::VariadicParameterDeclaration(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::ParameterDeclaration(x) => x.node_mut(),
                Self::VariadicParameterDeclaration(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::ParameterDeclaration(x) => x.into_node(),
                Self::VariadicParameterDeclaration(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{communication_case | default_case}`:\n- [CommunicationCase]\n- [DefaultCase]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CommunicationCase_DefaultCase<'tree> {
        CommunicationCase(CommunicationCase<'tree>),
        DefaultCase(DefaultCase<'tree>),
    }
    #[automatically_derived]
    impl<'tree> CommunicationCase_DefaultCase<'tree> {
        #[doc = "Returns the node if it is of kind `communication_case` ([CommunicationCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn communication_case(self) -> Option<CommunicationCase<'tree>> {
            match self {
                Self::CommunicationCase(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `default_case` ([DefaultCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn default_case(self) -> Option<DefaultCase<'tree>> {
            match self {
                Self::DefaultCase(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for CommunicationCase_DefaultCase<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "communication_case" => Ok(unsafe {
                    Self :: CommunicationCase (< CommunicationCase < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "default_case" => Ok(unsafe {
                    Self :: DefaultCase (< DefaultCase < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for CommunicationCase_DefaultCase<'tree> {
        const KIND: &'static str = "{communication_case | default_case}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::CommunicationCase(x) => x.node(),
                Self::DefaultCase(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::CommunicationCase(x) => x.node_mut(),
                Self::DefaultCase(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::CommunicationCase(x) => x.into_node(),
                Self::DefaultCase(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_statement | function_declaration | import_declaration | method_declaration | package_clause}`:\n- [Statement]\n- [FunctionDeclaration]\n- [ImportDeclaration]\n- [MethodDeclaration]\n- [PackageClause]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause<'tree> {
        Statement(Statement<'tree>),
        FunctionDeclaration(FunctionDeclaration<'tree>),
        ImportDeclaration(ImportDeclaration<'tree>),
        MethodDeclaration(MethodDeclaration<'tree>),
        PackageClause(PackageClause<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause<'tree> {
        #[doc = "Returns the node if it is of kind `_statement` ([Statement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn statement(self) -> Option<Statement<'tree>> {
            match self {
                Self::Statement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `function_declaration` ([FunctionDeclaration]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn function_declaration(self) -> Option<FunctionDeclaration<'tree>> {
            match self {
                Self::FunctionDeclaration(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `import_declaration` ([ImportDeclaration]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn import_declaration(self) -> Option<ImportDeclaration<'tree>> {
            match self {
                Self::ImportDeclaration(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `method_declaration` ([MethodDeclaration]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn method_declaration(self) -> Option<MethodDeclaration<'tree>> {
            match self {
                Self::MethodDeclaration(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `package_clause` ([PackageClause]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn package_clause(self) -> Option<PackageClause<'tree>> {
            match self {
                Self::PackageClause(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Statement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Statement(this));
            }
            if let Ok(this) = <FunctionDeclaration<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::FunctionDeclaration(this));
            }
            if let Ok(this) = <ImportDeclaration<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ImportDeclaration(this));
            }
            if let Ok(this) = <MethodDeclaration<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::MethodDeclaration(this));
            }
            if let Ok(this) = <PackageClause<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::PackageClause(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Statement_FunctionDeclaration_ImportDeclaration_MethodDeclaration_PackageClause<'tree>
    {
        const KIND : & 'static str = "{_statement | function_declaration | import_declaration | method_declaration | package_clause}" ;
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node(),
                Self::FunctionDeclaration(x) => x.node(),
                Self::ImportDeclaration(x) => x.node(),
                Self::MethodDeclaration(x) => x.node(),
                Self::PackageClause(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node_mut(),
                Self::FunctionDeclaration(x) => x.node_mut(),
                Self::ImportDeclaration(x) => x.node_mut(),
                Self::MethodDeclaration(x) => x.node_mut(),
                Self::PackageClause(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.into_node(),
                Self::FunctionDeclaration(x) => x.into_node(),
                Self::ImportDeclaration(x) => x.into_node(),
                Self::MethodDeclaration(x) => x.into_node(),
                Self::PackageClause(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{_statement | , | _type}`:\n- [Statement]\n- [symbols::Comma]\n- [Type]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Statement_Comma_Type<'tree> {
        Statement(Statement<'tree>),
        Comma(symbols::Comma<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Statement_Comma_Type<'tree> {
        #[doc = "Returns the node if it is of kind `_statement` ([Statement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn statement(self) -> Option<Statement<'tree>> {
            match self {
                Self::Statement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `,` ([symbols::Comma]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comma(self) -> Option<symbols::Comma<'tree>> {
            match self {
                Self::Comma(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_type` ([Type]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Statement_Comma_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <Statement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Statement(this));
            }
            if let Ok(this) = <symbols::Comma<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Comma(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Statement_Comma_Type<'tree> {
        const KIND: &'static str = "{_statement | , | _type}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node(),
                Self::Comma(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.node_mut(),
                Self::Comma(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Statement(x) => x.into_node(),
                Self::Comma(x) => x.into_node(),
                Self::Type(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{, | _type}`:\n- [symbols::Comma]\n- [Type]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comma_Type<'tree> {
        Comma(symbols::Comma<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Comma_Type<'tree> {
        #[doc = "Returns the node if it is of kind `,` ([symbols::Comma]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comma(self) -> Option<symbols::Comma<'tree>> {
            match self {
                Self::Comma(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_type` ([Type]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Comma_Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <symbols::Comma<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Comma(this));
            }
            if let Ok(this) = <Type<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Type(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma_Type<'tree> {
        const KIND: &'static str = "{, | _type}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.node(),
                Self::Type(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_node(),
                Self::Type(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{type_alias | type_spec}`:\n- [TypeAlias]\n- [TypeSpec]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum TypeAlias_TypeSpec<'tree> {
        TypeAlias(TypeAlias<'tree>),
        TypeSpec(TypeSpec<'tree>),
    }
    #[automatically_derived]
    impl<'tree> TypeAlias_TypeSpec<'tree> {
        #[doc = "Returns the node if it is of kind `type_alias` ([TypeAlias]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_alias(self) -> Option<TypeAlias<'tree>> {
            match self {
                Self::TypeAlias(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_spec` ([TypeSpec]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_spec(self) -> Option<TypeSpec<'tree>> {
            match self {
                Self::TypeSpec(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for TypeAlias_TypeSpec<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "type_alias" => Ok(unsafe {
                    Self :: TypeAlias (< TypeAlias < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_spec" => {
                    Ok(unsafe {
                        Self :: TypeSpec (< TypeSpec < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeAlias_TypeSpec<'tree> {
        const KIND: &'static str = "{type_alias | type_spec}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::TypeAlias(x) => x.node(),
                Self::TypeSpec(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::TypeAlias(x) => x.node_mut(),
                Self::TypeSpec(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::TypeAlias(x) => x.into_node(),
                Self::TypeSpec(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{default_case | type_case | expression_list | _simple_statement | _expression}`:\n- [DefaultCase]\n- [TypeCase]\n- [ExpressionList]\n- [SimpleStatement]\n- [Expression]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree> {
        DefaultCase(DefaultCase<'tree>),
        TypeCase(TypeCase<'tree>),
        ExpressionList(ExpressionList<'tree>),
        SimpleStatement(SimpleStatement<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree> {
        #[doc = "Returns the node if it is of kind `default_case` ([DefaultCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn default_case(self) -> Option<DefaultCase<'tree>> {
            match self {
                Self::DefaultCase(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_case` ([TypeCase]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_case(self) -> Option<TypeCase<'tree>> {
            match self {
                Self::TypeCase(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `expression_list` ([ExpressionList]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression_list(self) -> Option<ExpressionList<'tree>> {
            match self {
                Self::ExpressionList(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_simple_statement` ([SimpleStatement]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn simple_statement(self) -> Option<SimpleStatement<'tree>> {
            match self {
                Self::SimpleStatement(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `_expression` ([Expression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn expression(self) -> Option<Expression<'tree>> {
            match self {
                Self::Expression(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>>
        for DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if let Ok(this) = <DefaultCase<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::DefaultCase(this));
            }
            if let Ok(this) = <TypeCase<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::TypeCase(this));
            }
            if let Ok(this) = <ExpressionList<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::ExpressionList(this));
            }
            if let Ok(this) = <SimpleStatement<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::SimpleStatement(this));
            }
            if let Ok(this) = <Expression<'tree> as TryFrom<_>>::try_from(node) {
                return Ok(Self::Expression(this));
            }
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for DefaultCase_TypeCase_ExpressionList_SimpleStatement_Expression<'tree>
    {
        const KIND: &'static str =
            "{default_case | type_case | expression_list | _simple_statement | _expression}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.node(),
                Self::TypeCase(x) => x.node(),
                Self::ExpressionList(x) => x.node(),
                Self::SimpleStatement(x) => x.node(),
                Self::Expression(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.node_mut(),
                Self::TypeCase(x) => x.node_mut(),
                Self::ExpressionList(x) => x.node_mut(),
                Self::SimpleStatement(x) => x.node_mut(),
                Self::Expression(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::DefaultCase(x) => x.into_node(),
                Self::TypeCase(x) => x.into_node(),
                Self::ExpressionList(x) => x.into_node(),
                Self::SimpleStatement(x) => x.into_node(),
                Self::Expression(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{! | & | * | + | - | <- | ^}`:\n- [symbols::Not]\n- [symbols::And]\n- [symbols::Mul]\n- [symbols::Add]\n- [symbols::Sub]\n- [symbols::LtSub]\n- [symbols::BitXor]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not_And_Mul_Add_Sub_LtSub_BitXor<'tree> {
        Not(symbols::Not<'tree>),
        And(symbols::And<'tree>),
        Mul(symbols::Mul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        LtSub(symbols::LtSub<'tree>),
        BitXor(symbols::BitXor<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Not_And_Mul_Add_Sub_LtSub_BitXor<'tree> {
        #[doc = "Returns the node if it is of kind `!` ([symbols::Not]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not(self) -> Option<symbols::Not<'tree>> {
            match self {
                Self::Not(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `&` ([symbols::And]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and(self) -> Option<symbols::And<'tree>> {
            match self {
                Self::And(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `*` ([symbols::Mul]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul(self) -> Option<symbols::Mul<'tree>> {
            match self {
                Self::Mul(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `+` ([symbols::Add]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add(self) -> Option<symbols::Add<'tree>> {
            match self {
                Self::Add(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `-` ([symbols::Sub]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub(self) -> Option<symbols::Sub<'tree>> {
            match self {
                Self::Sub(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<-` ([symbols::LtSub]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt_sub(self) -> Option<symbols::LtSub<'tree>> {
            match self {
                Self::LtSub(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `^` ([symbols::BitXor]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn bit_xor(self) -> Option<symbols::BitXor<'tree>> {
            match self {
                Self::BitXor(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<yak_sitter::Node<'tree>> for Not_And_Mul_Add_Sub_LtSub_BitXor<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: yak_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "!" => Ok(unsafe {
                    Self :: Not (< symbols :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "&" => Ok(unsafe {
                    Self :: And (< symbols :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "*" => Ok(unsafe {
                    Self :: Mul (< symbols :: Mul < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "+" => Ok(unsafe {
                    Self :: Add (< symbols :: Add < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "-" => Ok(unsafe {
                    Self :: Sub (< symbols :: Sub < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "<-" => Ok(unsafe {
                    Self::LtSub(<symbols::LtSub<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "^" => Ok(unsafe {
                    Self::BitXor(<symbols::BitXor<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not_And_Mul_Add_Sub_LtSub_BitXor<'tree> {
        const KIND: &'static str = "{! | & | * | + | - | <- | ^}";
        #[inline]
        fn node(&self) -> &yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => x.node(),
                Self::And(x) => x.node(),
                Self::Mul(x) => x.node(),
                Self::Add(x) => x.node(),
                Self::Sub(x) => x.node(),
                Self::LtSub(x) => x.node(),
                Self::BitXor(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => x.node_mut(),
                Self::And(x) => x.node_mut(),
                Self::Mul(x) => x.node_mut(),
                Self::Add(x) => x.node_mut(),
                Self::Sub(x) => x.node_mut(),
                Self::LtSub(x) => x.node_mut(),
                Self::BitXor(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> yak_sitter::Node<'tree> {
            match self {
                Self::Not(x) => x.into_node(),
                Self::And(x) => x.into_node(),
                Self::Mul(x) => x.into_node(),
                Self::Add(x) => x.into_node(),
                Self::Sub(x) => x.into_node(),
                Self::LtSub(x) => x.into_node(),
                Self::BitXor(x) => x.into_node(),
            }
        }
    }
}
