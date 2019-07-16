// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq)]
pub enum Go {
    End = 0,
    Identifier = 1,
    LF = 2,
    SEMI = 3,
    Package = 4,
    Import = 5,
    DOT = 6,
    BlankIdentifier = 7,
    LPAREN = 8,
    RPAREN = 9,
    Const = 10,
    COMMA = 11,
    EQ = 12,
    Var = 13,
    Func = 14,
    DOTDOTDOT = 15,
    Type = 16,
    STAR = 17,
    LBRACK = 18,
    RBRACK = 19,
    Struct = 20,
    LBRACE = 21,
    RBRACE = 22,
    Interface = 23,
    Map = 24,
    Chan = 25,
    LTDASH = 26,
    COLONEQ = 27,
    PLUSPLUS = 28,
    DASHDASH = 29,
    STAREQ = 30,
    SLASHEQ = 31,
    PERCENTEQ = 32,
    LTLTEQ = 33,
    GTGTEQ = 34,
    AMPEQ = 35,
    AMPCARETEQ = 36,
    PLUSEQ = 37,
    DASHEQ = 38,
    PIPEEQ = 39,
    CARETEQ = 40,
    COLON = 41,
    Fallthrough = 42,
    Break = 43,
    Continue = 44,
    Goto = 45,
    Return = 46,
    Go = 47,
    Defer = 48,
    If = 49,
    Else = 50,
    For = 51,
    Range = 52,
    Switch = 53,
    Case = 54,
    Default = 55,
    Select = 56,
    Identifier2 = 57,
    Identifier3 = 58,
    PLUS = 59,
    DASH = 60,
    BANG = 61,
    CARET = 62,
    AMP = 63,
    SLASH = 64,
    PERCENT = 65,
    LTLT = 66,
    GTGT = 67,
    AMPCARET = 68,
    PIPE = 69,
    EQEQ = 70,
    BANGEQ = 71,
    LT = 72,
    LTEQ = 73,
    GT = 74,
    GTEQ = 75,
    AMPAMP = 76,
    PIPEPIPE = 77,
    RawStringLiteral = 78,
    DQUOTE = 79,
    InterpretedStringLiteralToken1 = 80,
    EscapeSequence = 81,
    IntLiteral = 82,
    FloatLiteral = 83,
    ImaginaryLiteral = 84,
    RuneLiteral = 85,
    Nil = 86,
    True = 87,
    False = 88,
    Comment = 89,
    SourceFile = 90,
    TopLevelDeclaration = 91,
    PackageClause = 92,
    ImportDeclaration = 93,
    ImportSpec = 94,
    Dot = 95,
    ImportSpecList = 96,
    Declaration = 97,
    ConstDeclaration = 98,
    ConstSpec = 99,
    VarDeclaration = 100,
    VarSpec = 101,
    FunctionDeclaration = 102,
    MethodDeclaration = 103,
    ParameterList = 104,
    ParameterDeclaration = 105,
    VariadicParameterDeclaration = 106,
    TypeAlias = 107,
    TypeDeclaration = 108,
    TypeSpec = 109,
    ExpressionList = 110,
    ParenthesizedType = 111,
    SimpleType = 112,
    PointerType = 113,
    ArrayType = 114,
    ImplicitLengthArrayType = 115,
    SliceType = 116,
    StructType = 117,
    FieldDeclarationList = 118,
    FieldDeclaration = 119,
    InterfaceType = 120,
    MethodSpecList = 121,
    MethodSpec = 122,
    MapType = 123,
    ChannelType = 124,
    FunctionType = 125,
    Block = 126,
    StatementList = 127,
    Statement = 128,
    EmptyStatement = 129,
    SimpleStatement = 130,
    SendStatement = 131,
    ReceiveStatement = 132,
    IncStatement = 133,
    DecStatement = 134,
    AssignmentStatement = 135,
    ShortVarDeclaration = 136,
    LabeledStatement = 137,
    LabeledStatement2 = 138,
    FallthroughStatement = 139,
    BreakStatement = 140,
    ContinueStatement = 141,
    GotoStatement = 142,
    ReturnStatement = 143,
    GoStatement = 144,
    DeferStatement = 145,
    IfStatement = 146,
    IfInitializer = 147,
    ElseClause = 148,
    ForStatement = 149,
    ForClause = 150,
    RangeClause = 151,
    ExpressionSwitchStatement = 152,
    ExpressionCaseClause = 153,
    ExpressionCase = 154,
    DefaultCase = 155,
    TypeSwitchStatement = 156,
    TypeSwitchGuard = 157,
    TypeCaseClause = 158,
    TypeCase = 159,
    SelectStatement = 160,
    CommunicationClause = 161,
    CommunicationCase = 162,
    Expression = 163,
    ParenthesizedExpression = 164,
    CallExpression = 165,
    VariadicArgument = 166,
    ArgumentList = 167,
    ArgumentList2 = 168,
    SelectorExpression = 169,
    IndexExpression = 170,
    SliceExpression = 171,
    TypeAssertionExpression = 172,
    TypeConversionExpression = 173,
    CompositeLiteral = 174,
    LiteralValue = 175,
    KeyedElement = 176,
    Element = 177,
    FuncLiteral = 178,
    UnaryExpression = 179,
    BinaryExpression = 180,
    QualifiedType = 181,
    StringLiteral = 182,
    InterpretedStringLiteral = 183,
    SourceFileRepeat1 = 184,
    ImportSpecListRepeat1 = 185,
    ConstDeclarationRepeat1 = 186,
    ConstSpecRepeat1 = 187,
    VarDeclarationRepeat1 = 188,
    ParameterListRepeat1 = 189,
    TypeDeclarationRepeat1 = 190,
    FieldNameListRepeat1 = 191,
    ExpressionListRepeat1 = 192,
    FieldDeclarationListRepeat1 = 193,
    MethodSpecListRepeat1 = 194,
    StatementListRepeat1 = 195,
    ExpressionSwitchStatementRepeat1 = 196,
    TypeSwitchStatementRepeat1 = 197,
    TypeCaseRepeat1 = 198,
    SelectStatementRepeat1 = 199,
    ArgumentListRepeat1 = 200,
    LiteralValueRepeat1 = 201,
    InterpretedStringLiteralRepeat1 = 202,
    PackageIdentifier = 203,
    FieldIdentifier = 204,
    LabelName = 205,
    TypeIdentifier = 206,
    Error = 207,
}

impl Into<&'static str> for Go {
    fn into(self) -> &'static str {
        match self {
            Go::End => "end",
            Go::Identifier => "identifier",
            Go::LF => "\n",
            Go::SEMI => ";",
            Go::Package => "package",
            Go::Import => "import",
            Go::DOT => ".",
            Go::BlankIdentifier => "blank_identifier",
            Go::LPAREN => "(",
            Go::RPAREN => ")",
            Go::Const => "const",
            Go::COMMA => ",",
            Go::EQ => "=",
            Go::Var => "var",
            Go::Func => "func",
            Go::DOTDOTDOT => "...",
            Go::Type => "type",
            Go::STAR => "*",
            Go::LBRACK => "[",
            Go::RBRACK => "]",
            Go::Struct => "struct",
            Go::LBRACE => "{",
            Go::RBRACE => "}",
            Go::Interface => "interface",
            Go::Map => "map",
            Go::Chan => "chan",
            Go::LTDASH => "<-",
            Go::COLONEQ => ":=",
            Go::PLUSPLUS => "++",
            Go::DASHDASH => "--",
            Go::STAREQ => "*=",
            Go::SLASHEQ => "/=",
            Go::PERCENTEQ => "%=",
            Go::LTLTEQ => "<<=",
            Go::GTGTEQ => ">>=",
            Go::AMPEQ => "&=",
            Go::AMPCARETEQ => "&^=",
            Go::PLUSEQ => "+=",
            Go::DASHEQ => "-=",
            Go::PIPEEQ => "|=",
            Go::CARETEQ => "^=",
            Go::COLON => ":",
            Go::Fallthrough => "fallthrough",
            Go::Break => "break",
            Go::Continue => "continue",
            Go::Goto => "goto",
            Go::Return => "return",
            Go::Go => "go",
            Go::Defer => "defer",
            Go::If => "if",
            Go::Else => "else",
            Go::For => "for",
            Go::Range => "range",
            Go::Switch => "switch",
            Go::Case => "case",
            Go::Default => "default",
            Go::Select => "select",
            Go::Identifier2 => "identifier",
            Go::Identifier3 => "identifier",
            Go::PLUS => "+",
            Go::DASH => "-",
            Go::BANG => "!",
            Go::CARET => "^",
            Go::AMP => "&",
            Go::SLASH => "/",
            Go::PERCENT => "%",
            Go::LTLT => "<<",
            Go::GTGT => ">>",
            Go::AMPCARET => "&^",
            Go::PIPE => "|",
            Go::EQEQ => "==",
            Go::BANGEQ => "!=",
            Go::LT => "<",
            Go::LTEQ => "<=",
            Go::GT => ">",
            Go::GTEQ => ">=",
            Go::AMPAMP => "&&",
            Go::PIPEPIPE => "||",
            Go::RawStringLiteral => "raw_string_literal",
            Go::DQUOTE => "\"",
            Go::InterpretedStringLiteralToken1 => "interpreted_string_literal_token1",
            Go::EscapeSequence => "escape_sequence",
            Go::IntLiteral => "int_literal",
            Go::FloatLiteral => "float_literal",
            Go::ImaginaryLiteral => "imaginary_literal",
            Go::RuneLiteral => "rune_literal",
            Go::Nil => "nil",
            Go::True => "true",
            Go::False => "false",
            Go::Comment => "comment",
            Go::SourceFile => "source_file",
            Go::TopLevelDeclaration => "_top_level_declaration",
            Go::PackageClause => "package_clause",
            Go::ImportDeclaration => "import_declaration",
            Go::ImportSpec => "import_spec",
            Go::Dot => "dot",
            Go::ImportSpecList => "import_spec_list",
            Go::Declaration => "_declaration",
            Go::ConstDeclaration => "const_declaration",
            Go::ConstSpec => "const_spec",
            Go::VarDeclaration => "var_declaration",
            Go::VarSpec => "var_spec",
            Go::FunctionDeclaration => "function_declaration",
            Go::MethodDeclaration => "method_declaration",
            Go::ParameterList => "parameter_list",
            Go::ParameterDeclaration => "parameter_declaration",
            Go::VariadicParameterDeclaration => "variadic_parameter_declaration",
            Go::TypeAlias => "type_alias",
            Go::TypeDeclaration => "type_declaration",
            Go::TypeSpec => "type_spec",
            Go::ExpressionList => "expression_list",
            Go::ParenthesizedType => "parenthesized_type",
            Go::SimpleType => "_simple_type",
            Go::PointerType => "pointer_type",
            Go::ArrayType => "array_type",
            Go::ImplicitLengthArrayType => "implicit_length_array_type",
            Go::SliceType => "slice_type",
            Go::StructType => "struct_type",
            Go::FieldDeclarationList => "field_declaration_list",
            Go::FieldDeclaration => "field_declaration",
            Go::InterfaceType => "interface_type",
            Go::MethodSpecList => "method_spec_list",
            Go::MethodSpec => "method_spec",
            Go::MapType => "map_type",
            Go::ChannelType => "channel_type",
            Go::FunctionType => "function_type",
            Go::Block => "block",
            Go::StatementList => "_statement_list",
            Go::Statement => "_statement",
            Go::EmptyStatement => "empty_statement",
            Go::SimpleStatement => "_simple_statement",
            Go::SendStatement => "send_statement",
            Go::ReceiveStatement => "receive_statement",
            Go::IncStatement => "inc_statement",
            Go::DecStatement => "dec_statement",
            Go::AssignmentStatement => "assignment_statement",
            Go::ShortVarDeclaration => "short_var_declaration",
            Go::LabeledStatement => "labeled_statement",
            Go::LabeledStatement2 => "labeled_statement",
            Go::FallthroughStatement => "fallthrough_statement",
            Go::BreakStatement => "break_statement",
            Go::ContinueStatement => "continue_statement",
            Go::GotoStatement => "goto_statement",
            Go::ReturnStatement => "return_statement",
            Go::GoStatement => "go_statement",
            Go::DeferStatement => "defer_statement",
            Go::IfStatement => "if_statement",
            Go::IfInitializer => "if_initializer",
            Go::ElseClause => "else_clause",
            Go::ForStatement => "for_statement",
            Go::ForClause => "for_clause",
            Go::RangeClause => "range_clause",
            Go::ExpressionSwitchStatement => "expression_switch_statement",
            Go::ExpressionCaseClause => "expression_case_clause",
            Go::ExpressionCase => "expression_case",
            Go::DefaultCase => "default_case",
            Go::TypeSwitchStatement => "type_switch_statement",
            Go::TypeSwitchGuard => "type_switch_guard",
            Go::TypeCaseClause => "type_case_clause",
            Go::TypeCase => "type_case",
            Go::SelectStatement => "select_statement",
            Go::CommunicationClause => "communication_clause",
            Go::CommunicationCase => "communication_case",
            Go::Expression => "_expression",
            Go::ParenthesizedExpression => "parenthesized_expression",
            Go::CallExpression => "call_expression",
            Go::VariadicArgument => "variadic_argument",
            Go::ArgumentList => "argument_list",
            Go::ArgumentList2 => "argument_list",
            Go::SelectorExpression => "selector_expression",
            Go::IndexExpression => "index_expression",
            Go::SliceExpression => "slice_expression",
            Go::TypeAssertionExpression => "type_assertion_expression",
            Go::TypeConversionExpression => "type_conversion_expression",
            Go::CompositeLiteral => "composite_literal",
            Go::LiteralValue => "literal_value",
            Go::KeyedElement => "keyed_element",
            Go::Element => "element",
            Go::FuncLiteral => "func_literal",
            Go::UnaryExpression => "unary_expression",
            Go::BinaryExpression => "binary_expression",
            Go::QualifiedType => "qualified_type",
            Go::StringLiteral => "_string_literal",
            Go::InterpretedStringLiteral => "interpreted_string_literal",
            Go::SourceFileRepeat1 => "source_file_repeat1",
            Go::ImportSpecListRepeat1 => "import_spec_list_repeat1",
            Go::ConstDeclarationRepeat1 => "const_declaration_repeat1",
            Go::ConstSpecRepeat1 => "const_spec_repeat1",
            Go::VarDeclarationRepeat1 => "var_declaration_repeat1",
            Go::ParameterListRepeat1 => "parameter_list_repeat1",
            Go::TypeDeclarationRepeat1 => "type_declaration_repeat1",
            Go::FieldNameListRepeat1 => "field_name_list_repeat1",
            Go::ExpressionListRepeat1 => "expression_list_repeat1",
            Go::FieldDeclarationListRepeat1 => "field_declaration_list_repeat1",
            Go::MethodSpecListRepeat1 => "method_spec_list_repeat1",
            Go::StatementListRepeat1 => "_statement_list_repeat1",
            Go::ExpressionSwitchStatementRepeat1 => "expression_switch_statement_repeat1",
            Go::TypeSwitchStatementRepeat1 => "type_switch_statement_repeat1",
            Go::TypeCaseRepeat1 => "type_case_repeat1",
            Go::SelectStatementRepeat1 => "select_statement_repeat1",
            Go::ArgumentListRepeat1 => "argument_list_repeat1",
            Go::LiteralValueRepeat1 => "literal_value_repeat1",
            Go::InterpretedStringLiteralRepeat1 => "interpreted_string_literal_repeat1",
            Go::PackageIdentifier => "package_identifier",
            Go::FieldIdentifier => "field_identifier",
            Go::LabelName => "label_name",
            Go::TypeIdentifier => "type_identifier",
            Go::Error => "ERROR",
        }
    }
}

static KEYS: phf::Map<&'static str, Go> = ::phf::Map {
    key: 8726423004985182586,
    disps: ::phf::Slice::Static(&[
        (0, 57),
        (0, 30),
        (0, 0),
        (0, 0),
        (3, 153),
        (5, 89),
        (1, 51),
        (0, 34),
        (0, 11),
        (0, 69),
        (0, 8),
        (7, 52),
        (0, 149),
        (0, 152),
        (0, 13),
        (0, 27),
        (4, 22),
        (2, 52),
        (1, 9),
        (3, 93),
        (0, 46),
        (0, 1),
        (0, 86),
        (1, 0),
        (0, 1),
        (0, 14),
        (1, 161),
        (0, 66),
        (0, 0),
        (0, 159),
        (0, 31),
        (0, 184),
        (0, 0),
        (12, 182),
        (6, 173),
        (0, 104),
        (70, 21),
        (4, 126),
        (9, 9),
        (0, 1),
        (3, 181),
    ]),
    entries: ::phf::Slice::Static(&[
        ("interface", Go::Interface),
        ("float_literal", Go::FloatLiteral),
        ("_simple_type", Go::SimpleType),
        ("const", Go::Const),
        ("select_statement_repeat1", Go::SelectStatementRepeat1),
        ("package", Go::Package),
        ("default_case", Go::DefaultCase),
        ("call_expression", Go::CallExpression),
        ("end", Go::End),
        ("break", Go::Break),
        ("parameter_list", Go::ParameterList),
        ("raw_string_literal", Go::RawStringLiteral),
        ("source_file_repeat1", Go::SourceFileRepeat1),
        ("continue", Go::Continue),
        ("field_declaration_list", Go::FieldDeclarationList),
        ("_statement_list", Go::StatementList),
        ("method_spec_list_repeat1", Go::MethodSpecListRepeat1),
        ("for_clause", Go::ForClause),
        ("for", Go::For),
        ("-=", Go::DASHEQ),
        ("pointer_type", Go::PointerType),
        ("continue_statement", Go::ContinueStatement),
        (":=", Go::COLONEQ),
        ("]", Go::RBRACK),
        ("imaginary_literal", Go::ImaginaryLiteral),
        ("return", Go::Return),
        ("-", Go::DASH),
        ("receive_statement", Go::ReceiveStatement),
        ("interpreted_string_literal", Go::InterpretedStringLiteral),
        ("<<", Go::LTLT),
        ("go", Go::Go),
        ("return_statement", Go::ReturnStatement),
        ("argument_list", Go::ArgumentList),
        ("slice_type", Go::SliceType),
        ("||", Go::PIPEPIPE),
        ("&^=", Go::AMPCARETEQ),
        ("|", Go::PIPE),
        ("package_identifier", Go::PackageIdentifier),
        ("type_conversion_expression", Go::TypeConversionExpression),
        ("keyed_element", Go::KeyedElement),
        ("/=", Go::SLASHEQ),
        (
            "expression_switch_statement_repeat1",
            Go::ExpressionSwitchStatementRepeat1,
        ),
        ("fallthrough_statement", Go::FallthroughStatement),
        ("label_name", Go::LabelName),
        ("&=", Go::AMPEQ),
        ("communication_case", Go::CommunicationCase),
        ("range", Go::Range),
        ("++", Go::PLUSPLUS),
        ("*", Go::STAR),
        ("<-", Go::LTDASH),
        ("==", Go::EQEQ),
        ("&&", Go::AMPAMP),
        (">>", Go::GTGT),
        ("argument_list_repeat1", Go::ArgumentListRepeat1),
        ("short_var_declaration", Go::ShortVarDeclaration),
        ("case", Go::Case),
        ("const_declaration", Go::ConstDeclaration),
        ("_string_literal", Go::StringLiteral),
        (
            "interpreted_string_literal_repeat1",
            Go::InterpretedStringLiteralRepeat1,
        ),
        ("element", Go::Element),
        (":", Go::COLON),
        ("comment", Go::Comment),
        ("field_name_list_repeat1", Go::FieldNameListRepeat1),
        ("index_expression", Go::IndexExpression),
        ("expression_list", Go::ExpressionList),
        ("parameter_list_repeat1", Go::ParameterListRepeat1),
        ("--", Go::DASHDASH),
        ("|=", Go::PIPEEQ),
        ("var", Go::Var),
        ("func", Go::Func),
        ("type_identifier", Go::TypeIdentifier),
        ("defer_statement", Go::DeferStatement),
        ("blank_identifier", Go::BlankIdentifier),
        ("method_declaration", Go::MethodDeclaration),
        ("identifier", Go::Identifier),
        ("for_statement", Go::ForStatement),
        ("...", Go::DOTDOTDOT),
        ("const_spec_repeat1", Go::ConstSpecRepeat1),
        ("function_declaration", Go::FunctionDeclaration),
        ("dot", Go::Dot),
        ("{", Go::LBRACE),
        ("_statement", Go::Statement),
        ("function_type", Go::FunctionType),
        ("+", Go::PLUS),
        ("var_spec", Go::VarSpec),
        ("type", Go::Type),
        ("defer", Go::Defer),
        ("channel_type", Go::ChannelType),
        ("type_declaration", Go::TypeDeclaration),
        ("ERROR", Go::Error),
        ("selector_expression", Go::SelectorExpression),
        ("!=", Go::BANGEQ),
        ("else_clause", Go::ElseClause),
        ("parenthesized_type", Go::ParenthesizedType),
        ("implicit_length_array_type", Go::ImplicitLengthArrayType),
        ("func_literal", Go::FuncLiteral),
        ("fallthrough", Go::Fallthrough),
        ("array_type", Go::ArrayType),
        ("_declaration", Go::Declaration),
        ("(", Go::LPAREN),
        ("type_spec", Go::TypeSpec),
        ("[", Go::LBRACK),
        ("field_declaration", Go::FieldDeclaration),
        ("dec_statement", Go::DecStatement),
        ("^", Go::CARET),
        ("unary_expression", Go::UnaryExpression),
        ("\\n", Go::LF),
        ("go_statement", Go::GoStatement),
        ("expression_switch_statement", Go::ExpressionSwitchStatement),
        (
            "type_switch_statement_repeat1",
            Go::TypeSwitchStatementRepeat1,
        ),
        ("break_statement", Go::BreakStatement),
        ("package_clause", Go::PackageClause),
        ("type_alias", Go::TypeAlias),
        ("type_assertion_expression", Go::TypeAssertionExpression),
        ("+=", Go::PLUSEQ),
        ("block", Go::Block),
        ("parameter_declaration", Go::ParameterDeclaration),
        ("%=", Go::PERCENTEQ),
        ("int_literal", Go::IntLiteral),
        ("nil", Go::Nil),
        ("type_case_clause", Go::TypeCaseClause),
        ("=", Go::EQ),
        ("select_statement", Go::SelectStatement),
        ("field_identifier", Go::FieldIdentifier),
        ("_simple_statement", Go::SimpleStatement),
        ("if", Go::If),
        ("communication_clause", Go::CommunicationClause),
        (".", Go::DOT),
        ("chan", Go::Chan),
        ("literal_value", Go::LiteralValue),
        ("type_declaration_repeat1", Go::TypeDeclarationRepeat1),
        ("parenthesized_expression", Go::ParenthesizedExpression),
        ("method_spec", Go::MethodSpec),
        ("var_declaration", Go::VarDeclaration),
        ("labeled_statement", Go::LabeledStatement),
        ("<", Go::LT),
        ("literal_value_repeat1", Go::LiteralValueRepeat1),
        ("type_case_repeat1", Go::TypeCaseRepeat1),
        ("assignment_statement", Go::AssignmentStatement),
        ("import_spec_list_repeat1", Go::ImportSpecListRepeat1),
        (
            "interpreted_string_literal_token1",
            Go::InterpretedStringLiteralToken1,
        ),
        (")", Go::RPAREN),
        ("var_declaration_repeat1", Go::VarDeclarationRepeat1),
        ("type_switch_statement", Go::TypeSwitchStatement),
        ("default", Go::Default),
        ("expression_list_repeat1", Go::ExpressionListRepeat1),
        ("_statement_list_repeat1", Go::StatementListRepeat1),
        ("goto", Go::Goto),
        (">=", Go::GTEQ),
        ("if_statement", Go::IfStatement),
        ("source_file", Go::SourceFile),
        ("struct", Go::Struct),
        ("import_declaration", Go::ImportDeclaration),
        ("rune_literal", Go::RuneLiteral),
        (
            "variadic_parameter_declaration",
            Go::VariadicParameterDeclaration,
        ),
        ("range_clause", Go::RangeClause),
        ("empty_statement", Go::EmptyStatement),
        ("map", Go::Map),
        ("method_spec_list", Go::MethodSpecList),
        ("<<=", Go::LTLTEQ),
        (
            "field_declaration_list_repeat1",
            Go::FieldDeclarationListRepeat1,
        ),
        ("import", Go::Import),
        ("interface_type", Go::InterfaceType),
        ("struct_type", Go::StructType),
        ("false", Go::False),
        ("goto_statement", Go::GotoStatement),
        ("inc_statement", Go::IncStatement),
        ("import_spec", Go::ImportSpec),
        ("select", Go::Select),
        ("slice_expression", Go::SliceExpression),
        ("!", Go::BANG),
        ("if_initializer", Go::IfInitializer),
        ("send_statement", Go::SendStatement),
        ("\\\"", Go::DQUOTE),
        ("switch", Go::Switch),
        ("_expression", Go::Expression),
        ("else", Go::Else),
        (";", Go::SEMI),
        ("}", Go::RBRACE),
        ("/", Go::SLASH),
        ("&^", Go::AMPCARET),
        ("<=", Go::LTEQ),
        ("map_type", Go::MapType),
        ("type_case", Go::TypeCase),
        ("^=", Go::CARETEQ),
        ("expression_case_clause", Go::ExpressionCaseClause),
        ("*=", Go::STAREQ),
        ("import_spec_list", Go::ImportSpecList),
        ("variadic_argument", Go::VariadicArgument),
        ("&", Go::AMP),
        ("binary_expression", Go::BinaryExpression),
        ("const_spec", Go::ConstSpec),
        (">>=", Go::GTGTEQ),
        (">", Go::GT),
        ("true", Go::True),
        ("qualified_type", Go::QualifiedType),
        ("escape_sequence", Go::EscapeSequence),
        (",", Go::COMMA),
        ("expression_case", Go::ExpressionCase),
        ("composite_literal", Go::CompositeLiteral),
        ("type_switch_guard", Go::TypeSwitchGuard),
        ("const_declaration_repeat1", Go::ConstDeclarationRepeat1),
        ("%", Go::PERCENT),
        ("_top_level_declaration", Go::TopLevelDeclaration),
    ]),
};

impl From<&str> for Go {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Go {
    #[inline(always)]
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x as u8) }
    }
}

// Go == u16
impl PartialEq<u16> for Go {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Go::from(*x)
    }
}

// u16 == Go
impl PartialEq<Go> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Go) -> bool {
        *x == *self
    }
}
