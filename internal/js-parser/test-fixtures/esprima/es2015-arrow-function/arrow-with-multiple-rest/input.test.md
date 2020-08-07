# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-arrow-function > arrow-with-multiple-rest`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: true
	directives: Array []
	filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
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
				message: MARKUP {parts: Array [RAW_MARKUP {value: "The rest element has to be the last element when destructuring"}]}
			}
			location: Object {
				filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 5
					line: 1
				}
				start: Object {
					column: 1
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
				end: Object {
					column: 17
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSArrowFunctionExpression {
				loc: Object {
					filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
					end: Object {
						column: 17
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				body: JSNumericLiteral {
					value: 0
					format: undefined
					loc: Object {
						filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
						end: Object {
							column: 17
							line: 1
						}
						start: Object {
							column: 16
							line: 1
						}
					}
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					returnType: undefined
					thisType: undefined
					loc: Object {
						filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
						end: Object {
							column: 15
							line: 1
						}
						start: Object {
							column: 0
							line: 1
						}
					}
					rest: JSBindingIdentifier {
						name: "b"
						loc: Object {
							filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
							identifierName: "b"
							end: Object {
								column: 11
								line: 1
							}
							start: Object {
								column: 10
								line: 1
							}
						}
					}
					params: Array [
						JSBindingIdentifier {
							name: "INVALID_PLACEHOLDER"
							loc: Object {
								filename: "esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js"
								end: Object {
									column: 15
									line: 1
								}
								start: Object {
									column: 16
									line: 1
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```

 esprima/es2015-arrow-function/arrow-with-multiple-rest/input.js:1:1 parse/js ━━━━━━━━━━━━━━━━━━━━━━

  ✖ The rest element has to be the last element when destructuring

    (...a, ...b) => 0
     ^^^^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```