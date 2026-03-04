#!/usr/bin/env python3
"""Converts parseit_test.lua checkParse calls into Rust #[test] functions.

Each Lua call:
    checkParse(t, "prog", good, done, ast, "Test name")

becomes a Rust test:
    #[test]
    fn test_name_slug() {
        check_parse("prog", good, done, expected_ast);
    }
"""

import re
import sys
from pathlib import Path

# ─── Lua AST constant → Rust tag mapping ────────────────────────────────────

# Maps the Lua symbolic constant names (as they appear in the test file with
# their 'x' suffix) to the string tags used by our Rust Node constructors.
CONST_MAP: dict[str, str] = {
    "PROGRAMx":     "Program",   # handled structurally, not as a tag
    "EMPTYxSTMT":   "Empty",
    "PRINTxSTMT":   "Print",
    "PRINTLNxSTMT": "Println",
    "RETURNxSTMT":  "Return",
    "INCxSTMT":     "Inc",
    "DECxSTMT":     "Dec",
    "ASSNxSTMT":    "Assn",
    "FUNCxCALL":    "FuncCall",
    "FUNCxDEF":     "FuncDef",
    "IFxSTMT":      "If",
    "WHILExLOOP":   "While",
    "STRLITxOUT":   "StrlitOut",
    "CHRxCALL":     "ChrCall",
    "BINxOP":       "BinOp",    # handled specially (expr node)
    "UNxOP":        "UnOp",     # handled specially (expr node)
    "NUMLITxVAL":   "NumLit",    # leaf value
    "READxCALL":    "ReadCall",
    "RNDxCALL":     "RndCall",
    "SIMPLExVAR":   "SimpleVar", # leaf value
    "ARRAYxVAR":    "ArrayVar",
}

# ─── Lua source tokeniser ────────────────────────────────────────────────────

def parse_lua_string(text: str, pos: int) -> tuple[str, int]:
    """Parse a Lua single- or double-quoted string literal starting at pos.
    Returns (python_string_value, next_pos) with Lua escape sequences interpreted."""
    quote = text[pos]
    i = pos + 1
    result = []
    while i < len(text):
        ch = text[i]
        if ch == "\\" and i + 1 < len(text):
            esc = text[i + 1]
            i += 2
            # Interpret common Lua escape sequences
            result.append({
                "n": "\n", "r": "\r", "t": "\t",
                "\\": "\\", '"': '"', "'": "'", "a": "\a",
            }.get(esc, "\\" + esc))
            continue
        if ch == quote:
            i += 1
            break
        result.append(ch)
        i += 1
    return "".join(result), i


def skip_ws(text: str, i: int) -> int:
    while i < len(text) and text[i] in " \t\n\r":
        i += 1
    return i


# ─── Lua table (AST) parser ──────────────────────────────────────────────────

def parse_lua_value(text: str, i: int) -> tuple[object, int]:
    """Parse a Lua value (string, identifier/constant, or table) at position i.
    Returns (value, next_pos).  Tables become lists."""
    i = skip_ws(text, i)
    if i >= len(text):
        return None, i

    ch = text[i]

    if ch in ('"', "'"):
        s, i = parse_lua_string(text, i)
        return s, i

    if ch == "{":
        return parse_lua_table(text, i)

    if ch == "-" and i + 1 < len(text) and text[i + 1].isdigit():
        j = i + 1
        while j < len(text) and (text[j].isdigit() or text[j] == "."):
            j += 1
        return int(text[i:j]), j

    if ch.isdigit():
        j = i
        while j < len(text) and (text[j].isdigit() or text[j] == "."):
            j += 1
        return int(text[i:j]), j

    # Identifier / keyword
    if ch.isalpha() or ch == "_":
        j = i
        while j < len(text) and (text[j].isalnum() or text[j] == "_"):
            j += 1
        word = text[i:j]
        if word == "nil":
            return None, j
        if word == "true":
            return True, j
        if word == "false":
            return False, j
        return word, j  # Lua constant name

    return None, i + 1


def parse_lua_table(text: str, i: int) -> tuple[list, int]:
    """Parse a Lua array table { v, v, ... } starting at the '{'.
    Returns (list_of_values, next_pos)."""
    assert text[i] == "{"
    i += 1
    items = []
    while i < len(text):
        i = skip_ws(text, i)
        if i >= len(text) or text[i] == "}":
            i += 1
            break
        if text[i] == ",":
            i += 1
            continue
        val, i = parse_lua_value(text, i)
        if val is not None:
            items.append(val)
    return items, i


# ─── Lua table → Rust Node expression ───────────────────────────────────────

def lua_table_to_rust(table: list) -> str:
    """Recursively convert a parsed Lua AST table into a Rust Node expression."""
    if not table:
        return "Node::program(vec![])"

    head = table[0]

    # ── PROGRAM ──────────────────────────────────────────────────────────
    if head == "PROGRAMx":
        stmts = ", ".join(lua_table_to_rust(c) for c in table[1:])
        return f"Node::program(vec![{stmts}])"

    # ── Leaves that carry a single string value ───────────────────────────
    if head in ("NUMLITxVAL", "SIMPLExVAR"):
        val = table[1] if len(table) > 1 else ""
        return f'Node::value({rust_str(val)})'

    # ── READ_CALL (no children) ───────────────────────────────────────────
    if head == "READxCALL":
        return 'Node::stmt("readint", vec![])'

    # ── BIN_OP: {{BINxOP, op}, lhs, rhs} ─────────────────────────────────
    # In Lua the BIN_OP head is itself a sub-table: { BINxOP, "+" }
    if isinstance(head, list) and head and head[0] == "BINxOP":
        op = head[1] if len(head) > 1 else "?"
        lhs = lua_table_to_rust(table[1]) if len(table) > 1 else "Node::value(\"\")"
        rhs = lua_table_to_rust(table[2]) if len(table) > 2 else "Node::value(\"\")"
        return f'Node::expr({rust_str(op)}, vec![{lhs}, {rhs}])'

    # ── UN_OP: {{UNxOP, op}, operand} ────────────────────────────────────
    if isinstance(head, list) and head and head[0] == "UNxOP":
        op = head[1] if len(head) > 1 else "?"
        operand = lua_table_to_rust(table[1]) if len(table) > 1 else "Node::value(\"\")"
        # Matches parser.rs: expr(op, [value("unary"), operand])
        return f'Node::expr({rust_str(op)}, vec![Node::value("unary"), {operand}])'

    # ── BINxOP / UNxOP used as a direct string constant (shouldn't happen) ─
    if head in ("BINxOP", "UNxOP"):
        op = table[1] if len(table) > 1 else "?"
        children = [lua_table_to_rust(c) for c in table[2:]]
        children_str = ", ".join(children)
        return f'Node::expr({rust_str(op)}, vec![{children_str}])'

    # ── ARRAYxVAR: {ARRAYxVAR, name, index_expr} ─────────────────────────
    if head == "ARRAYxVAR":
        name = table[1] if len(table) > 1 else ""
        idx  = lua_table_to_rust(table[2]) if len(table) > 2 else 'Node::value("")'
        return f'Node::expr("array_var", vec![Node::value({rust_str(name)}), {idx}])'

    # ── FUNCxCALL: {FUNCxCALL, name} ─────────────────────────────────────
    if head == "FUNCxCALL":
        name = table[1] if len(table) > 1 else ""
        return f'Node::stmt("func_call", vec![Node::value({rust_str(name)})])'

    # ── FUNCxDEF: {FUNCxDEF, name, program} ──────────────────────────────
    if head == "FUNCxDEF":
        name = rust_str(table[1]) if len(table) > 1 else '""'
        body = lua_table_to_rust(table[2]) if len(table) > 2 else "Node::program(vec![])"
        return f'Node::stmt("func_def", vec![Node::value({name}), {body}])'

    # ── STRLITxOUT: {STRLITxOUT, raw_string} ────────────────────────────
    if head == "STRLITxOUT":
        val = table[1] if len(table) > 1 else ""
        return f'Node::stmt("strlit_out", vec![Node::value({rust_str(val)})])'

    # ── Generic stmt nodes with variadic children ─────────────────────────
    tag = CONST_MAP.get(head, head.lower().replace("x", "_", 1))
    children = [lua_table_to_rust(c) for c in table[1:]]
    children_str = ", ".join(children)
    return f'Node::stmt(ParseToken::{tag}, vec![{children_str}])'


def lua_table_to_rust_value(v: object) -> str:
    """Convert a top-level parsed Lua value to a Rust Option<Node> expression."""
    if v is None:
        return "Node::none()"
    if isinstance(v, list):
        return f"{lua_table_to_rust(v)}"
    return "None"


def rust_str(s: object) -> str:
    """Emit a Rust &str literal from a Python string value.

    The Python string is already the logical value (Lua escape sequences have
    been interpreted by parse_lua_string).  We only need to re-escape what
    Rust string literals require: backslashes and double-quotes.
    """
    s = str(s)
    escaped = s.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def to_rust_input(s: str) -> str:
    """Wrap a program string for use as a Rust string literal.

    Prefers a regular string literal; falls back to raw only when there are
    no double-quote characters (since `r#"..."#` cannot contain `"`).
    """
    # Regular string — just escape backslash and double-quote
    escaped = s.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


# ─── checkParse extractor ────────────────────────────────────────────────────

def extract_check_parse_calls(source: str) -> list[dict]:
    """Extract all checkParse(...) calls from Lua source.

    Returns list of dicts with keys:
        suite, prog, good, done, ast (parsed list or None), name
    """
    results = []
    current_suite = "unknown"

    # Strip Lua comments (-- to end of line), but not inside strings
    # We'll do a simple approach: parse call-by-call
    suite_re = re.compile(r'function (test_\w+)\s*\(')
    call_start_re = re.compile(r'\bcheckParse\s*\(')

    i = 0
    while i < len(source):
        # Track current suite
        m = suite_re.match(source, i)
        if m:
            current_suite = m.group(1)
            i = m.end()
            continue

        # Skip Lua line comments
        if source[i:i+2] == "--":
            end = source.find("\n", i)
            i = end + 1 if end != -1 else len(source)
            continue

        # Skip strings
        if source[i] in ('"', "'"):
            _, i = parse_lua_string(source, i)
            continue

        # Find checkParse call
        m = call_start_re.match(source, i)
        if not m:
            i += 1
            continue

        i = m.end()

        # Now parse the 6 arguments manually
        try:
            # arg1: t (skip)
            i = skip_ws(source, i)
            # skip "t"
            while i < len(source) and source[i] not in (",",):
                i += 1
            i += 1  # skip comma

            # arg2: prog (string)
            i = skip_ws(source, i)
            if source[i] not in ('"', "'"):
                continue
            prog, i = parse_lua_string(source, i)

            # skip comma
            i = skip_ws(source, i)
            if i < len(source) and source[i] == ",":
                i += 1

            # arg3: good (true/false)
            good_val, i = parse_lua_value(source, i)
            i = skip_ws(source, i)
            if i < len(source) and source[i] == ",":
                i += 1

            # arg4: done (true/false)
            done_val, i = parse_lua_value(source, i)
            i = skip_ws(source, i)
            if i < len(source) and source[i] == ",":
                i += 1

            # arg5: ast (nil or table)
            ast_val, i = parse_lua_value(source, i)
            i = skip_ws(source, i)
            if i < len(source) and source[i] == ",":
                i += 1

            # arg6: test name (string)
            i = skip_ws(source, i)
            if source[i] not in ('"', "'"):
                continue
            name, i = parse_lua_string(source, i)

            # skip closing paren
            i = skip_ws(source, i)
            if i < len(source) and source[i] == ")":
                i += 1

            results.append({
                "suite": current_suite,
                "prog": prog,
                "good": bool(good_val),
                "done": bool(done_val),
                "ast": ast_val,
                "name": name,
            })
        except Exception as e:
            print(f"Warning: failed to parse checkParse at offset {i}: {e}", file=sys.stderr)
            continue

    return results


# ─── Code generation ─────────────────────────────────────────────────────────

def fn_name(suite: str) -> str:
    """Convert a Lua test suite function name to a Rust test function name."""
    return re.sub(r"[^a-z0-9]+", "_", suite.lower()).strip("_")


def generate(calls: list[dict]) -> str:
    lines = [
        "//! Parser tests — auto-generated from parseit_test.lua",
        "//! Do not edit by hand.",
        "",
        "use rstest::rstest;",
        "use std::time::Duration;",
        "use tamandua_rs::parser::{Node, Parser, ParseToken};",
        "use tamandua_rs::lexer::Lexer;",
        "",
    ]

    # Group by suite, preserving order
    suites: dict[str, list[dict]] = {}
    for c in calls:
        suites.setdefault(c["suite"], []).append(c)

    for suite, suite_calls in suites.items():
        lines.append("    #[rstest]")
        for c in suite_calls:
            prog  = to_rust_input(c["prog"])
            good  = "true" if c["good"] else "false"
            done  = "true" if c["done"] else "false"
            ast   = lua_table_to_rust_value(c["ast"])
            lines.append(f"    #[case({prog}, {good}, {done}, {ast})]")

        lines.append("    #[timeout(Duration::from_secs(1))]")
        lines.append(f"    fn {fn_name(suite)}(")
        lines.append("        #[case] input: &str,")
        lines.append("        #[case] exp_good: bool,")
        lines.append("        #[case] exp_done: bool,")
        lines.append("        #[case] exp_ast: Node,")
        lines.append("    ) {")
        lines.append("        let tokens = Lexer::new(input.to_string()).lex_input();")
        lines.append("        let (good, done, ast) = Parser::new(tokens).parse();")
        lines.append("        assert_eq!(good, exp_good, \"good flag mismatch\\ninput: {input:?}\");")
        lines.append("        assert_eq!(done, exp_done, \"done flag mismatch\\ninput: {input:?}\");")
        lines.append("        if exp_good && exp_done {")
        lines.append("            assert_eq!(ast, exp_ast, \"ast mismatch\\ninput: {input:?}\");")
        lines.append("        }")
        lines.append("    }")
        lines.append("")

    return "\n".join(lines)


# ─── Main ────────────────────────────────────────────────────────────────────

def main() -> None:
    lua_path = Path("parseit_test.lua")
    source = lua_path.read_text()

    calls = extract_check_parse_calls(source)
    print(f"Extracted {len(calls)} checkParse calls across {len({c['suite'] for c in calls})} suites", file=sys.stderr)

    rust = generate(calls)

    out = Path("tests/parser_tests.rs")
    out.write_text(rust)
    print(f"Wrote {out}", file=sys.stderr)


if __name__ == "__main__":
    main()
