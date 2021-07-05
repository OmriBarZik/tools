# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/css-parser/index.test.ts --update-snapshots` to update.

## `invalid > grid > minmax > incorrect-flex`

### `ast`

```javascript
CSSRoot {
	body: [
		CSSRule {
			prelude: [
				CSSSelector {
					patterns: [
						CSSClassSelector {
							value: "style"
							loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 1:0-1:6
						}
					]
					loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 1:0-1:7
				}
			]
			block: CSSBlock {
				value: [
					CSSDeclaration {
						name: "grid-template-columns"
						value: [
							CSSFunction {
								name: "minmax"
								params: []
								loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:24-2:31
							}
							CSSDimension {
								value: 1
								unit: "fr"
								loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:31-2:34
							}
							CSSComma {
								loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:34-2:35
							}
							CSSDimension {
								value: 200
								unit: "px"
								loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:36-2:41
							}
							CSSRaw {
								loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:41-2:42
							}
						]
						important: false
						loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 2:1-2:42
					}
				]
				startingTokenValue: "{"
				loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 1:7-3:1
			}
			loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<css>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "css"
				message: RAW_MARKUP {
					value: "A flex argument is permitted only as <emphasis>second argument</emphasis> of the function <emphasis>minmax()</emphasis>"
				}
			}
			location: {
				language: "css"
				path: UIDPath<invalid/grid/minmax/incorrect-flex/input.css>
				end: Position 2:34
				start: Position 2:31
			}
		}
	]
	path: UIDPath<invalid/grid/minmax/incorrect-flex/input.css>
	loc: SourceLocation invalid/grid/minmax/incorrect-flex/input.css 1:0-3:1
}
```

### `diagnostics`

```

 invalid/grid/minmax/incorrect-flex/input.css:2:31 parse(css) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ A flex argument is permitted only as second argument of the function minmax()

    1 │ .style {
  > 2 │   grid-template-columns: minmax(1fr, 200px);
      │                                 ^^^
    3 │ }


```