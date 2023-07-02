from tokens import KEYWORDS, Token, TokenType


class Lexer:
    def __init__(self, input: str):
        self.input = input
        self.position = 0
        self.read_position = 0
        self.char = ""

        # Call read_char when initializing the lexer so that is is in a working state before it is used
        self.read_char()

    def read_char(self) -> None:
        if self.read_position >= len(self.input):
            self.char = ""
        else:
            self.char = self.input[self.read_position]
        self.position = self.read_position
        self.read_position += 1

    def next_token(self):
        tkn = None
        self.skip_whitespace()

        match self.char:
            case "=":
                if self.peek_char() == "=":
                    ch = self.char
                    self.read_char()
                    tkn = Token(TokenType.EQ, ch + self.char)
                else:
                    tkn = Token(TokenType.ASSIGN, self.char)
            case "!":
                if self.peek_char() == "=":
                    ch = self.char
                    self.read_char()
                    tkn = Token(TokenType.NOT_EQ, ch + self.char)
                else:
                    tkn = Token(TokenType.BANG, self.char)
            case ";":
                tkn = Token(TokenType.SEMICOLON, self.char)
            case "(":
                tkn = Token(TokenType.LPAREN, self.char)
            case ")":
                tkn = Token(TokenType.RPAREN, self.char)
            case "{":
                tkn = Token(TokenType.LBRACE, self.char)
            case "}":
                tkn = Token(TokenType.RBRACE, self.char)
            case ",":
                tkn = Token(TokenType.COMMA, self.char)
            case "+":
                tkn = Token(TokenType.PLUS, self.char)
            case "-":
                tkn = Token(TokenType.MINUS, self.char)
            case "*":
                tkn = Token(TokenType.ASTERISK, self.char)
            case "/":
                tkn = Token(TokenType.SLASH, self.char)
            case "<":
                tkn = Token(TokenType.LT, self.char)
            case ">":
                tkn = Token(TokenType.GT, self.char)
            case "":
                tkn = Token(TokenType.EOF, self.char)
            case other:
                if self.is_letter(self.char):
                    ident = self.read_identifier()
                    tkn = Token(self.lookup_ident(ident), ident)
                    return tkn  # Early return bc read_identifier calls `read_char` repeatedly.
                elif self.is_digit(self.char):
                    number = self.read_number()
                    tkn = Token(TokenType.INT, number)
                    return tkn
                else:
                    tkn = Token(TokenType.ILLEGAL, self.char)

        self.read_char()
        return tkn

    def read_identifier(self) -> str:
        start_pos = self.position
        while self.is_letter(self.char):
            self.read_char()
        return self.input[start_pos : self.position]

    def read_number(self) -> str:
        start_pos = self.position
        while self.is_digit(self.char):
            self.read_char()
        return self.input[start_pos : self.position]

    def peek_char(self) -> str:
        if self.read_position >= len(self.input):
            return ""
        else:
            return self.input[self.read_position]

    @staticmethod
    def is_letter(char: str) -> bool:
        return char.isalpha() or char == "_"

    @staticmethod
    def is_digit(char: str) -> bool:
        return char.isdigit()

    @staticmethod
    def lookup_ident(ident: str) -> TokenType:
        return KEYWORDS.get(ident, TokenType.IDENT)

    def skip_whitespace(self):
        while self.char.isspace():
            self.read_char()
