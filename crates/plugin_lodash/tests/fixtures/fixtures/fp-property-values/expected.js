"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "formatters", {
    enumerable: true,
    get: function() {
        return formatters;
    }
});
var _interopRequireDefault = require("@swc/helpers/lib/_interop_require_default.js").default;
var _snakeCase = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/snakeCase"));
var _kebabCase = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/kebabCase"));
var _camelCase = /*#__PURE__*/ _interopRequireDefault(require("lodash/fp/camelCase"));
var formatters = {
    camelCase: _camelCase.default,
    "kebabCase": _kebabCase.default,
    "snakeCase": _snakeCase.default
};
