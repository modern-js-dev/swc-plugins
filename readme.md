# Modern.js swc-plugins binding

## Quick Start

```javascript
const { Compiler } = require('@modern-js/swc-plugins')
const { dirname } = require('path')

const compiler = new Compiler({
  // swc option
  swc: JSON.stringify({
    jsc: {
      transform: {}
    }
  }),

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

## Difference from @swc/core

When `swc.module` field is not set, `@modern-js/swc-plugins` will automatic detect module type and set `swc.module` for you.
