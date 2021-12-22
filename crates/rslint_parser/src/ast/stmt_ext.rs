//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{ast::*, T};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JsVariableKind {
	Const,
	Let,
	Var,
}

impl JsVariableDeclarations {
	/// Whether the declaration is a const declaration
	pub fn is_const(&self) -> bool {
		self.variable_kind() == JsVariableKind::Const
	}

	/// Whether the declaration is a let declaration
	pub fn is_let(&self) -> bool {
		self.variable_kind() == JsVariableKind::Let
	}

	/// Whether the declaration is a let declaration
	pub fn is_var(&self) -> bool {
		self.variable_kind() == JsVariableKind::Const
	}

	pub fn variable_kind(&self) -> JsVariableKind {
		let token_kind = self.kind().kind();

		match token_kind {
			T![const] => JsVariableKind::Const,
			T![let] => JsVariableKind::Let,
			T![var] => JsVariableKind::Var,
			_ => unreachable!(),
		}
	}
}

impl JsAnySwitchClause {
	pub fn into_case(self) -> Option<JsCaseClause> {
		if let JsAnySwitchClause::JsCaseClause(clause) = self {
			Some(clause)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn var_decl_let_token() {
		let parsed = parse_text("/* */let a = 5;", 0).tree();
		let var_decl = parsed
			.statements()
			.iter()
			.find_map(|stmt| ast::JsVariableStatement::cast(stmt.syntax().clone()));

		assert!(var_decl.is_some());
	}
}
