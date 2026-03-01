#!/usr/bin/env lua
-- generate_rstest_tests.lua
-- Generates rstest-style parameterized tests from the Lua test suite

local KEYx = 1
local IDx = 2
local NUMLITx = 3
local STRLITx = 4
local OPx = 5
local PUNCTx = 6
local MALx = 7

local output = io.open("tests/lexer_tests.rs", "w")

-- Test categories and their cases
local test_suites = {}
local current_suite = nil

local function escape_rust_string(s)
	return s:gsub("\\", "\\\\"):gsub('"', '\\"'):gsub("\n", "\\n"):gsub("\r", "\\r"):gsub("\t", "\\t")
end

local function category_to_rust(cat)
	if cat == KEYx then
		return "Lexeme::Keyword"
	elseif cat == IDx then
		return "Lexeme::Identifier"
	elseif cat == NUMLITx then
		return "Lexeme::NumericLiteral"
	elseif cat == STRLITx then
		return "Lexeme::StringLiteral"
	elseif cat == OPx then
		return "Lexeme::Operator"
	elseif cat == PUNCTx then
		return "Lexeme::Punctuation"
	elseif cat == MALx then
		return "Lexeme::Malformed(LexingError::NoLexemeFound)"
	end
	return "Lexeme::None"
end

-- Start a new test suite
local function start_suite(name)
	current_suite = {
		name = name,
		cases = {},
	}
	table.insert(test_suites, current_suite)
end

-- Add a test case to the current suite
local function add_case(prog, expected)
	if not current_suite then
		error("No active test suite. Call start_suite() first.")
	end

	table.insert(current_suite.cases, {
		input = prog,
		expected = expected,
	})
end

-- Generate the #[case(...)] line for a single test
local function generate_case_line(test_case)
	local input_str = string.format('"%s"', escape_rust_string(test_case.input))

	-- Build expected vector
	local expected_parts = {}
	local i = 1
	while i * 2 <= #test_case.expected do
		local lexeme_str = test_case.expected[2 * i - 1]
		local category = test_case.expected[2 * i]
		local rust_cat = category_to_rust(category)

		table.insert(expected_parts, string.format('(%s, "%s".to_string())', rust_cat, escape_rust_string(lexeme_str)))
		i = i + 1
	end

	local expected_str = "vec![" .. table.concat(expected_parts, ", ") .. "]"

	return string.format("    #[case(%s, %s)]", input_str, expected_str)
end

-- Write file header
output:write([[
//! Generated tests for the Lexer module using rstest
//! Auto-generated from the Lua test suite

#[cfg(test)]
mod lexer_tests {
    use rstest::rstest;
    use tamandua_rs::lexer::{Lexer, Lexeme, LexingError};
    use std::time::Duration;

    fn assert_lex_eq(input: &str, expected: &[(Lexeme, String)], given: &[(Lexeme, String)]) {
        if expected != given {
            panic!(
                "\nInput: {:?}\n\nExpected: {:#?}\n\nGiven: {:#?}",
                input, expected, given
            );
        }
    }

]])

-- ==================================================================
-- DEFINE ALL TEST SUITES AND THEIR CASES
-- ==================================================================

-- Identifiers and Keywords
start_suite("identifiers")
add_case("a", { "a", IDx })
add_case(" a", { "a", IDx })
add_case("_", { "_", IDx })
add_case(" _", { "_", IDx })
add_case("bx", { "bx", IDx })
add_case("b3", { "b3", IDx })
add_case("_n", { "_n", IDx })
add_case("_4", { "_4", IDx })
add_case("abc_39xyz", { "abc_39xyz", IDx })
add_case("abc eol_3", { "abc", IDx, "eol_3", IDx })
add_case("a  ", { "a", IDx })
add_case("a#", { "a", IDx })
add_case("a #", { "a", IDx })
add_case("#\na", { "a", IDx })
add_case("#\n a", { "a", IDx })
add_case("ab", { "ab", IDx })
add_case("_chr", { "_chr", IDx })

start_suite("keywords")
add_case("chr", { "chr", KEYx })
add_case("else", { "else", KEYx })
add_case("elsif", { "elsif", KEYx })
add_case("func", { "func", KEYx })
add_case("if", { "if", KEYx })
add_case("print", { "print", KEYx })
add_case("println", { "println", KEYx })
add_case("readint", { "readint", KEYx })
add_case("return", { "return", KEYx })
add_case("rnd", { "rnd", KEYx })
add_case("while", { "while", KEYx })

start_suite("keyword_variations")
add_case("funcc", { "funcc", IDx })
add_case("chr2", { "chr2", IDx })
add_case("Print", { "Print", IDx })
add_case("elsE", { "elsE", IDx })
add_case("IF", { "IF", IDx })
add_case("elsi", { "elsi", IDx })
add_case("printl", { "printl", IDx })
add_case("els if", { "els", IDx, "if", KEYx })
add_case("re#\nturn", { "re", IDx, "turn", IDx })
add_case("whi2le", { "whi2le", IDx })
add_case("pri_nt", { "pri_nt", IDx })
add_case("ifreturn", { "ifreturn", IDx })

start_suite("numeric_literals")
add_case("3", { "3", NUMLITx })
add_case("3a", { "3", NUMLITx, "a", IDx })
add_case("123456", { "123456", NUMLITx })
add_case(".123456", { ".", PUNCTx, "123456", NUMLITx })
add_case("123456.", { "123456", NUMLITx, ".", PUNCTx })
add_case("123.456", { "123", NUMLITx, ".", PUNCTx, "456", NUMLITx })
add_case("1.2.3", { "1", NUMLITx, ".", PUNCTx, "2", NUMLITx, ".", PUNCTx, "3", NUMLITx })
add_case("123 456", { "123", NUMLITx, "456", NUMLITx })

start_suite("numeric_with_signs")
add_case("+123456", { "+", OPx, "123456", NUMLITx })
add_case("+.123456", { "+", OPx, ".", PUNCTx, "123456", NUMLITx })
add_case("+123456.", { "+", OPx, "123456", NUMLITx, ".", PUNCTx })
add_case("+123.456", { "+", OPx, "123", NUMLITx, ".", PUNCTx, "456", NUMLITx })
add_case("+1.2.3", { "+", OPx, "1", NUMLITx, ".", PUNCTx, "2", NUMLITx, ".", PUNCTx, "3", NUMLITx })
add_case("-123456", { "-", OPx, "123456", NUMLITx })
add_case("-.123456", { "-", OPx, ".", PUNCTx, "123456", NUMLITx })
add_case("-123456.", { "-", OPx, "123456", NUMLITx, ".", PUNCTx })
add_case("-123.456", { "-", OPx, "123", NUMLITx, ".", PUNCTx, "456", NUMLITx })
add_case("-1.2.3", { "-", OPx, "1", NUMLITx, ".", PUNCTx, "2", NUMLITx, ".", PUNCTx, "3", NUMLITx })
add_case("+-123456", { "+", OPx, "-", OPx, "123456", NUMLITx })
add_case("-+123456", { "-", OPx, "+", OPx, "123456", NUMLITx })

start_suite("scientific_notation")
add_case("123e456", { "123e456", NUMLITx })
add_case("123e+456", { "123e+456", NUMLITx })
add_case("123e-456", { "123", NUMLITx, "e", IDx, "-", OPx, "456", NUMLITx })
add_case("+123e456", { "+", OPx, "123e456", NUMLITx })
add_case("+123e+456", { "+", OPx, "123e+456", NUMLITx })
add_case("+123e-456", { "+", OPx, "123", NUMLITx, "e", IDx, "-", OPx, "456", NUMLITx })
add_case("-123e456", { "-", OPx, "123e456", NUMLITx })
add_case("-123e+456", { "-", OPx, "123e+456", NUMLITx })
add_case("-123e-456", { "-", OPx, "123", NUMLITx, "e", IDx, "-", OPx, "456", NUMLITx })
add_case("123E456", { "123E456", NUMLITx })
add_case("123E+456", { "123E+456", NUMLITx })
add_case("123E-456", { "123", NUMLITx, "E", IDx, "-", OPx, "456", NUMLITx })
add_case("+123E456", { "+", OPx, "123E456", NUMLITx })
add_case("+123E+456", { "+", OPx, "123E+456", NUMLITx })
add_case("+123E-456", { "+", OPx, "123", NUMLITx, "E", IDx, "-", OPx, "456", NUMLITx })
add_case("-123E456", { "-", OPx, "123E456", NUMLITx })
add_case("-123E+456", { "-", OPx, "123E+456", NUMLITx })
add_case("-123E-456", { "-", OPx, "123", NUMLITx, "E", IDx, "-", OPx, "456", NUMLITx })

start_suite("scientific_notation_edge_cases")
add_case("e", { "e", IDx })
add_case("E", { "E", IDx })
add_case("e3", { "e3", IDx })
add_case("E3", { "E3", IDx })
add_case("e+3", { "e", IDx, "+", OPx, "3", NUMLITx })
add_case("E+3", { "E", IDx, "+", OPx, "3", NUMLITx })
add_case("1e3", { "1e3", NUMLITx })
add_case("123e", { "123", NUMLITx, "e", IDx })
add_case("123E", { "123", NUMLITx, "E", IDx })
add_case("123ee", { "123", NUMLITx, "ee", IDx })
add_case("123Ee", { "123", NUMLITx, "Ee", IDx })
add_case("123eE", { "123", NUMLITx, "eE", IDx })
add_case("123EE", { "123", NUMLITx, "EE", IDx })
add_case("1.2e34", { "1", NUMLITx, ".", PUNCTx, "2e34", NUMLITx })
add_case("12e3.4", { "12e3", NUMLITx, ".", PUNCTx, "4", NUMLITx })
add_case("123e+", { "123", NUMLITx, "e", IDx, "+", OPx })
add_case("123E+", { "123", NUMLITx, "E", IDx, "+", OPx })

start_suite("string_literals")
add_case("''", { "''", STRLITx })
add_case('""', { '""', STRLITx })
add_case("'a'", { "'a'", STRLITx })
add_case('"b"', { '"b"', STRLITx })
add_case("'abc eol'", { "'abc eol'", STRLITx })
add_case('"The quick brown fox."', { '"The quick brown fox."', STRLITx })
add_case("'aa\"bb'", { "'aa\"bb'", STRLITx })
add_case('"cc\'dd"', { '"cc\'dd"', STRLITx })
add_case('"\\a"', { '"\\a"', STRLITx })

start_suite("malformed_strings")
add_case("'aabbcc", { "'aabbcc", MALx })
add_case("'aabbcc\"", { "'aabbcc\"", MALx })
add_case("'aabbcc\n", { "'aabbcc", MALx })
add_case('"aabbcc', { '"aabbcc', MALx })
add_case("\"aabbcc'", { "\"aabbcc'", MALx })
add_case('"aabbcc\n', { '"aabbcc', MALx })
add_case('"aabbcc\nd', { '"aabbcc', MALx, "d", IDx })
add_case("'\"'\"'\"", { "'\"'", STRLITx, '"\'"', STRLITx })

start_suite("operators")
add_case("!", { "!", OPx })
add_case("&&", { "&&", OPx })
add_case("||", { "||", OPx })
add_case("==", { "==", OPx })
add_case("!=", { "!=", OPx })
add_case("<", { "<", OPx })
add_case("<=", { "<=", OPx })
add_case(">", { ">", OPx })
add_case(">=", { ">=", OPx })
add_case("+", { "+", OPx })
add_case("-", { "-", OPx })
add_case("*", { "*", OPx })
add_case("/", { "/", OPx })
add_case("%", { "%", OPx })
add_case("[", { "[", OPx })
add_case("]", { "]", OPx })
add_case("=", { "=", OPx })
add_case("++", { "++", OPx })
add_case("--", { "--", OPx })

start_suite("operator_sequences")
add_case("+-++--", { "+", OPx, "-", OPx, "++", OPx, "--", OPx })
add_case("+++++", { "++", OPx, "++", OPx, "+", OPx })
add_case("-----", { "--", OPx, "--", OPx, "-", OPx })
add_case("=====", { "==", OPx, "==", OPx, "=", OPx })
add_case("=<<==", { "=", OPx, "<", OPx, "<=", OPx, "=", OPx })
add_case("**/ ", { "*", OPx, "*", OPx, "/", OPx })
add_case("= =", { "=", OPx, "=", OPx })
add_case("+++1+", { "++", OPx, "+", OPx, "1", NUMLITx, "+", OPx })
add_case("---2-", { "--", OPx, "-", OPx, "2", NUMLITx, "-", OPx })

start_suite("operators_with_following")
add_case("!1", { "!", OPx, "1", NUMLITx })
add_case("&&1", { "&&", OPx, "1", NUMLITx })
add_case("==1", { "==", OPx, "1", NUMLITx })
add_case("+1", { "+", OPx, "1", NUMLITx })
add_case("-1", { "-", OPx, "1", NUMLITx })
add_case("!a", { "!", OPx, "a", IDx })
add_case("&&a", { "&&", OPx, "a", IDx })
add_case("==a", { "==", OPx, "a", IDx })
add_case("+a", { "+", OPx, "a", IDx })
add_case("-a", { "-", OPx, "a", IDx })

start_suite("punctuation")
add_case("$(),.:;?@\\^`{}~", {
	"$",
	PUNCTx,
	"(",
	PUNCTx,
	")",
	PUNCTx,
	",",
	PUNCTx,
	".",
	PUNCTx,
	":",
	PUNCTx,
	";",
	PUNCTx,
	"?",
	PUNCTx,
	"@",
	PUNCTx,
	"\\",
	PUNCTx,
	"^",
	PUNCTx,
	"`",
	PUNCTx,
	"{",
	PUNCTx,
	"}",
	PUNCTx,
	"~",
	PUNCTx,
})

start_suite("whitespace")
add_case(" ", {})
add_case("\t", {})
add_case("\n", {})
add_case("ab 12", { "ab", IDx, "12", NUMLITx })
add_case("ab\t12", { "ab", IDx, "12", NUMLITx })
add_case("ab\n12", { "ab", IDx, "12", NUMLITx })

start_suite("comments")
add_case("#abcd\n", {})
add_case("12#abcd\nab", { "12", NUMLITx, "ab", IDx })
add_case("12#abcd", { "12", NUMLITx })
add_case("12#abcd#", { "12", NUMLITx })
add_case('ab#"', { "ab", IDx })
add_case("ab#'", { "ab", IDx })
add_case("a##\nb", { "a", IDx, "b", IDx })
add_case("a##b", { "a", IDx })

start_suite("complete_programs")
add_case("", {})
add_case("a_1[0]=1;", { "a_1", IDx, "[", OPx, "0", NUMLITx, "]", OPx, "=", OPx, "1", NUMLITx, ";", PUNCTx })
add_case('if(x==5){print("hello");}', {
	"if",
	KEYx,
	"(",
	PUNCTx,
	"x",
	IDx,
	"==",
	OPx,
	"5",
	NUMLITx,
	")",
	PUNCTx,
	"{",
	PUNCTx,
	"print",
	KEYx,
	"(",
	PUNCTx,
	'"hello"',
	STRLITx,
	")",
	PUNCTx,
	";",
	PUNCTx,
	"}",
	PUNCTx,
})
add_case("func test(){return 42;}", {
	"func",
	KEYx,
	"test",
	IDx,
	"(",
	PUNCTx,
	")",
	PUNCTx,
	"{",
	PUNCTx,
	"return",
	KEYx,
	"42",
	NUMLITx,
	";",
	PUNCTx,
	"}",
	PUNCTx,
})
add_case("while(i<n){++i;}", {
	"while",
	KEYx,
	"(",
	PUNCTx,
	"i",
	IDx,
	"<",
	OPx,
	"n",
	IDx,
	")",
	PUNCTx,
	"{",
	PUNCTx,
	"++",
	OPx,
	"i",
	IDx,
	";",
	PUNCTx,
	"}",
	PUNCTx,
})

-- ==================================================================
-- GENERATE OUTPUT
-- ==================================================================

-- Generate each test suite
for _, suite in ipairs(test_suites) do
	-- Generate the #[rstest] macro
	output:write("    #[rstest]\n")
	-- Generate all #[case(...)] lines
	for _, test_case in ipairs(suite.cases) do
		output:write(generate_case_line(test_case) .. "\n")
	end

	-- Generate test function
	output:write("    #[timeout(Duration::from_secs(1))]\n")
	output:write(string.format("    fn test_%s(\n", suite.name))
	output:write("        #[case] input: &str,\n")
	output:write("        #[case] expected: Vec<(Lexeme, String)>,\n")
	output:write("    ) {\n")
	output:write("        let mut lexer = Lexer::new(input.to_string());\n")
	output:write("        let result = lexer.lex_input();\n")
	output:write("        assert_lex_eq(input, &expected, &result);\n")
	output:write("    }\n\n")
end

output:write("}\n")
output:close()

print("Generated lexer_rstest.rs with rstest parameterized tests")
print("Test suites created: " .. #test_suites)

local total_cases = 0
for _, suite in ipairs(test_suites) do
	total_cases = total_cases + #suite.cases
end
print("Total test cases: " .. total_cases)
