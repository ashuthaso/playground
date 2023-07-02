from dataclasses import dataclass
from enum import Enum


class TokenType(Enum):
    ILLEGAL = "ILLEGAL"
    EOF = ""

    # Identifiers + literals
    IDENT = "IDENT"
    INT = "INT"

    # Operators
    ASSIGN = "="
    PLUS = "+"
    MINUS = "-"
    SLASH = "/"
    ASTERISK = "*"
    LT = "<"
    GT = ">"
    BANG = "!"
    EQ = "=="
    NOT_EQ = "!="

    # Delimiters
    COMMA = ","
    SEMICOLON = ";"

    LPAREN = "("
    RPAREN = ")"
    LBRACE = "{"
    RBRACE = "}"

    # keywords
    ELSE = "ELSE"
    FALSE = "FALSE"
    FUNCTION = "FUNCTION"
    IF = "IF"
    LET = "LET"
    RETURN = "RETURN"
    TRUE = "TRUE"


KEYWORDS = {
    "else": TokenType.ELSE,
    "false": TokenType.FALSE,
    "fn": TokenType.FUNCTION,
    "if": TokenType.IF,
    "let": TokenType.LET,
    "return": TokenType.RETURN,
    "true": TokenType.TRUE,
}


@dataclass
class Token:
    type: str
    literal: str
