use crate::kinds_src::KindsSrc;

pub const CSS_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("#", "HASH"),
        // These are *not* question AND dot tokens, they are one
        // to distinguish between `? .3134` and `?.` per ecma specs
        ("&", "AMP"),
        ("|", "PIPE"),
        ("+", "PLUS"),
        ("++", "PLUS2"),
        ("*", "STAR"),
        ("**", "STAR2"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        (".", "DOT"),
        (":", "COLON"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("-", "MINUS"),
        ("--", "MINUS2"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("@", "AT"),
        ("$=", "DOLLAR_EQ"),
        ("~=", "TILDE_EQ"),
    ],
    keywords: &[
        "aliceblue",
        "antiquewhite",
        "aqua",
        "aquamarine",
        "azure",
        "beige",
        "bisque",
        "black",
        "blanchedalmond",
        "blue",
        "blueviolet",
        "brown",
        "burlywood",
        "cadetblue",
        "chartreuse",
        "chocolate",
        "coral",
        "cornflowerblue",
        "cornsilk",
        "crimson",
        "cyan",
        "darkblue",
        "darkcyan",
        "darkgoldenrod",
        "darkgray",
        "darkgreen",
        "darkkhaki",
        "darkmagenta",
        "darkolivegreen",
        "darkorange",
        "darkorchid",
        "darkred",
        "darksalmon",
        "darkseagreen",
        "darkslateblue",
        "darkslategray",
        "darkturquoise",
        "darkviolet",
        "deeppink",
        "deepskyblue",
        "dimgray",
        "dodgerblue",
        "firebrick",
        "floralwhite",
        "forestgreen",
        "fuchsia",
        "gainsboro",
        "ghostwhite",
        "gold",
        "goldenrod",
        "gray",
        "green",
        "greenyellow",
        "honeydew",
        "hotpink",
        "indianred",
        "indigo",
        "ivory",
        "khaki",
        "lavender",
        "lavenderblush",
        "lawngreen",
        "lemonchiffon",
        "lightblue",
        "lightcoral",
        "lightcyan",
        "lightgoldenrodyellow",
        "lightgreen",
        "lightgrey",
        "lightpink",
        "lightsalmon",
        "lightseagreen",
        "lightskyblue",
        "lightslategray",
        "lightsteelblue",
        "lightyellow",
        "lime",
        "limegreen",
        "linen",
        "magenta",
        "maroon",
        "mediumaquamarine",
        "mediumblue",
        "mediumorchid",
        "mediumpurple",
        "mediumseagreen",
        "mediumslateblue",
        "mediumspringgreen",
        "mediumturquoise",
        "mediumvioletred",
        "midnightblue",
        "mintcream",
        "mistyrose",
        "moccasin",
        "navajowhite",
        "navy",
        "navyblue",
        "oldlace",
        "olive",
        "olivedrab",
        "orange",
        "orangered",
        "orchid",
        "palegoldenrod",
        "palegreen",
        "paleturquoise",
        "palevioletred",
        "papayawhip",
        "peachpuff",
        "peru",
        "pink",
        "plum",
        "powderblue",
        "purple",
        "red",
        "rosybrown",
        "royalblue",
        "saddlebrown",
        "salmon",
        "sandybrown",
        "seagreen",
        "seashell",
        "sienna",
        "silver",
        "skyblue",
        "slateblue",
        "slategray",
        "snow",
        "springgreen",
        "steelblue",
        "tan",
        "teal",
        "thistle",
        "tomato",
        "turquoise",
        "violet",
        "wheat",
        "white",
        "whitesmoke",
        "yellow",
        "yellowgreen",
        "media",
        "keyframes",
        "not",
        "and",
        "only",
        "or",
        "i",
        "important",
        "from",
        "to",
        "var",
    ],
    literals: &[
        "CSS_STRING_LITERAL",
        "CSS_NUMBER_LITERAL",
        "CSS_CUSTOM_PROPERTY",
        "CSS_SPACE_LITERAL",
    ],
    tokens: &[
        "ERROR_TOKEN",
        "IDENT",
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
    ],
    nodes: &[
        "CSS_ROOT",
        "CSS_ID_SELECTOR_PATTERN",
        "CSS_RULE",
        "CSS_SELECTOR_LIST",
        "CSS_SELECTOR",
        "CSS_ANY_FUNCTION",
        "CSS_AT_KEYFRAMES",
        "CSS_AT_KEYFRAMES_BODY",
        "CSS_AT_MEDIA",
        "CSS_AT_MEDIA_QUERY",
        "CSS_AT_MEDIA_QUERY_CONSEQUENT",
        "CSS_AT_MEDIA_QUERY_FEATURE",
        "CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN",
        "CSS_AT_MEDIA_QUERY_FEATURE_COMPARE",
        "CSS_AT_MEDIA_QUERY_FEATURE_PLAIN",
        "CSS_AT_MEDIA_QUERY_FEATURE_RANGE",
        "CSS_AT_MEDIA_QUERY_RANGE",
        "CSS_ATTRIBUTE",
        "CSS_ATTRIBUTE_MATCHER",
        "CSS_ATTRIBUTE_META",
        "CSS_ATTRIBUTE_MODIFIER",
        "CSS_ATTRIBUTE_NAME",
        "CSS_ATTRIBUTE_SELECTOR_PATTERN",
        "CSS_BLOCK",
        "CSS_CLASS_SELECTOR_PATTERN",
        "CSS_COMBINATOR_SELECTOR_PATTERN",
        "CSS_DECLARATION",
        "CSS_DIMENSION",
        "CSS_IDENTIFIER",
        "CSS_KEYFRAMES_BLOCK",
        "CSS_KEYFRAMES_SELECTOR",
        "CSS_NUMBER",
        "CSS_PARAMETER",
        "CSS_PERCENTAGE",
        "CSS_PSEUDO_CLASS_SELECTOR_PATTERN",
        "CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS",
        "CSS_RATIO",
        "CSS_SIMPLE_FUNCTION",
        "CSS_STRING",
        "CSS_TYPE_SELECTOR_PATTERN",
        "CSS_UNIVERSAL_SELECTOR_PATTERN",
        "CSS_VAR_FUNCTION",
        "CSS_VAR_FUNCTION_VALUE",
        "CSS_ANY_SELECTOR_PATTERN_LIST",
        "CSS_AT_KEYFRAMES_ITEM_LIST",
        "CSS_AT_MEDIA_QUERY_LIST",
        "CSS_ATTRIBUTE_LIST",
        "CSS_DECLARATION_LIST",
        "CSS_KEYFRAMES_SELECTOR_LIST",
        "CSS_PARAMETER_LIST",
        "CSS_DECLARATION_IMPORTANT",
        // unknown nodes
        "CSS_UNKNOWN",
    ],
};
