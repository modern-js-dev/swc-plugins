# Modern.js swc-plugins binding

## Quick Start

```javascript
const { Compiler } = require('@modern-js/swc-plugins')
const { dirname } = require('path')

const compiler = new Compiler({
  // swc option
  jsc: {
    transform: {}
  },

  // native plugins
  extensions: {
    lockCorejsVersion: {
      corejs: dirname(require.resolve('core-js/package.json')),
      swcHelpers: dirname(require.resolve('@swc/helpers/package.json'))
    }
  }
})

compiler
  .transform("unknown.js", "let a = { ...obj }")
  .then(res => {
    console.log(res.code)
    console.log(res.map)
  })
```

## `extensions`

`Extensions` config is some native plugins ported from `Babel`.

### `extensions.pluginImport`

- type

```typescript
Array<{
  fromSource: string;
  replaceJs?: {
    ignoreEsComponent?: string[];
    replaceTpl?: string;
    replaceExpr?: (member: string) => string | false;
    transformToDefaultImport?: boolean;
  };
  replaceCss?: {
    ignoreStyleComponent?: string[];
    replaceTpl?: string;
    replaceExpr?: (member: string) => string | false;
  };
}>;
```

Ported from `@babel/plugin-import`.

`fromSource`

- Type: `string`

The package that need to be transformed，eg. in `import {a} from 'foo'`, `fromSource` should be `foo`.

`replaceJs.ignoreEsComponent`

- Type: `string[]`
- default: `[]`

The import specifiers which don't need to be transformed.

`replaceJs.replaceTpl`

- Type: `string`
- default: `undefineed`

Template that represents repacement, for example:

```javascript
import { MyButton as Btn } from 'foo';
```

If we set `replaceJs.replaceTpl = "foo/es/{{member}}"`, then the code above will be replaced to code below:

```javascript
import Btn from 'foo/es/MyButton';
```

We also put some naming conversion functions, take the above example again, if we set it to `"foo/es/{{ kebabCase member }}"`, it will be transformed to code below:

```javascript
import Btn from 'foo/es/my-button';
```

Besides `kebabCase`, there are also `camelCase`, `snakeCase`, `upperCase`, `lowerCase`

`replaceJs.replaceExpr`

- Type: `(member: string) => string`
- default: `undefineed`

This is also used to replace import specifiers. The argument is the specifier that imported. eg. `a` in `import { a as b } from 'foo'`.
This funtion is called by `Rust`，and it needs to be synchronous.
We recommend `replaceTpl` instead, because call `js` function through `node-api` will cause performance issue. `node-api` invokes `js` function actually put this `js` call inside a queue, and wait for this call to be executed, so if current `js` thread is busy, then this call will block `Rust` thread for a while.

`transformToDefaultImport`

- Type: `boolean`
- default: `true`

Whether transform specifier to default specifier.

### `extensions.reactUtils`

- Type: `Object`

Some little help utils for `React`.

`reactUtils.autoImportReact`

- Type: `boolean`

Automatically import `React` as global variable, eg: `import React from 'react'`.
Mostly used for generated `React.createElement`.

`reactUtils.rmEffect`

- Type: `boolean`

Remove `useEffect` call.

`reactUtils.rmPropTypes`

- Type:

```typescript
{
  mode?: "remove" | "unwrap" | "unsafe-wrap",
  removeImport?: bool,
  ignoreFilenames?: String[],
  additionalLibraries?: String[],
  classNameMatchers?: String[],
}
```

Remove `React` runtime type checking. This is ported from [@babel/plugin-react-transform-remove-prop-types](https://github.com/oliviertassinari/babel-plugin-transform-react-remove-prop-types), All the configurations remain the same.

### `extensions.lodash`

- Type: `{ cwd?: string, ids?: string,}`
- default: `{ cwd: process.cwd(), ids: [] }`

Ported from [@babel/plugin-lodash](https://github.com/lodash/babel-plugin-lodash).
