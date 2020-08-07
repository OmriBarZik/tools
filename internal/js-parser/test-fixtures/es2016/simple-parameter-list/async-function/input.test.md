# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2016 > simple-parameter-list > async-function`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "es2016/simple-parameter-list/async-function/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "es2016/simple-parameter-list/async-function/input.js"
		end: Object {
			column: 0
			line: 4
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
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Illegal 'use strict' directive in function with non-simple parameter list"}]}
			}
			location: Object {
				filename: "es2016/simple-parameter-list/async-function/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 15
					line: 2
				}
				start: Object {
					column: 2
					line: 2
				}
			}
		}
	]
	body: Array [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "a"
				loc: Object {
					filename: "es2016/simple-parameter-list/async-function/input.js"
					identifierName: "a"
					end: Object {
						column: 16
						line: 1
					}
					start: Object {
						column: 15
						line: 1
					}
				}
			}
			loc: Object {
				filename: "es2016/simple-parameter-list/async-function/input.js"
				end: Object {
					column: 1
					line: 3
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			body: JSBlockStatement {
				body: Array []
				loc: Object {
					filename: "es2016/simple-parameter-list/async-function/input.js"
					end: Object {
						column: 1
						line: 3
					}
					start: Object {
						column: 31
						line: 1
					}
				}
				directives: Array [
					JSDirective {
						value: "use strict"
						loc: Object {
							filename: "es2016/simple-parameter-list/async-function/input.js"
							end: Object {
								column: 15
								line: 2
							}
							start: Object {
								column: 2
								line: 2
							}
						}
					}
				]
			}
			head: JSFunctionHead {
				async: true
				generator: false
				hasHoistedVars: false
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "es2016/simple-parameter-list/async-function/input.js"
					end: Object {
						column: 30
						line: 1
					}
					start: Object {
						column: 16
						line: 1
					}
				}
				params: Array [
					JSBindingAssignmentPattern {
						loc: Object {
							filename: "es2016/simple-parameter-list/async-function/input.js"
							end: Object {
								column: 29
								line: 1
							}
							start: Object {
								column: 17
								line: 1
							}
						}
						right: JSObjectExpression {
							properties: Array []
							loc: Object {
								filename: "es2016/simple-parameter-list/async-function/input.js"
								end: Object {
									column: 29
									line: 1
								}
								start: Object {
									column: 27
									line: 1
								}
							}
						}
						left: JSBindingIdentifier {
							name: "options"
							loc: Object {
								filename: "es2016/simple-parameter-list/async-function/input.js"
								identifierName: "options"
								end: Object {
									column: 24
									line: 1
								}
								start: Object {
									column: 17
									line: 1
								}
							}
							meta: JSPatternMeta {
								optional: undefined
								typeAnnotation: undefined
								loc: Object {
									filename: "es2016/simple-parameter-list/async-function/input.js"
									end: Object {
										column: 24
										line: 1
									}
									start: Object {
										column: 17
										line: 1
									}
								}
							}
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```

 es2016/simple-parameter-list/async-function/input.js:2:2 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Illegal 'use strict' directive in function with non-simple parameter list

    1 │ async function a(options = {}) {
  > 2 │   "use strict";
      │   ^^^^^^^^^^^^^
    3 │ }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```