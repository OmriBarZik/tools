# `headingHasContent.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/compiler/lint/rules/jsx-a11y/headingHasContent.test.ts --update-snapshots` to update.

## `jsx-a11y heading has content`

### `0`

```

 unknown:1 lint/jsx-a11y/headingHasContent ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide screen reader accessible content when using heading elements.

    <h1 />
    ^^^^^^

  ℹ All headings on a page should have content that is accessible to screen readers.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `0: formatted`

```
<h1 />;

```

### `1`

```

 unknown:1 lint/jsx-a11y/headingHasContent ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide screen reader accessible content when using heading elements.

    <h1><TextWrapper aria-hidden /></h1>
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

  ℹ All headings on a page should have content that is accessible to screen readers.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `1: formatted`

```
<h1><TextWrapper aria-hidden /></h1>;

```

### `2`

```

 unknown:1 lint/jsx-a11y/headingHasContent ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Provide screen reader accessible content when using heading elements.

    <h1><div aria-hidden /></h1>
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

  ℹ All headings on a page should have content that is accessible to screen readers.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `2: formatted`

```
<h1><div aria-hidden /></h1>;

```

### `3`

```
✔ No known problems!

```

### `3: formatted`

```
<h1>heading</h1>;

```

### `4`

```
✔ No known problems!

```

### `4: formatted`

```
<h1><TextWrapper /></h1>;

```

### `5`

```
✔ No known problems!

```

### `5: formatted`

```
<h1 dangerouslySetInnerHTML={{__html: "heading"}} />;

```

### `6`

```
✔ No known problems!

```

### `6: formatted`

```
<h1><div aria-hidden />visible content</h1>;

```