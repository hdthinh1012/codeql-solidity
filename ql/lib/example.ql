import codeql.solidity.ast.internal.TreeSitter

from Solidity::AstNode node
select node, "Low-level tree-sitter node"
