# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > regression > non-octal-float-strict-mode`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "core/regression/non-octal-float-strict-mode/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array []
	loc: Object {
		filename: "core/regression/non-octal-float-strict-mode/input.js"
		end: Object {
			column: 0
			line: 2
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Legacy octal literals are not allowed in strict mode"}]}
			}
			location: Object {
				filename: "core/regression/non-octal-float-strict-mode/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 2
					line: 1
				}
				start: Object {
					column: 2
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "core/regression/non-octal-float-strict-mode/input.js"
				end: Object {
					column: 4
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSNumericLiteral {
				value: 9.5
				format: undefined
				loc: Object {
					filename: "core/regression/non-octal-float-strict-mode/input.js"
					end: Object {
						column: 4
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```

 core/regression/non-octal-float-strict-mode/input.js:1:2 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Legacy octal literals are not allowed in strict mode

    09.5
      ^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```