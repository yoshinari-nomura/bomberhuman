(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/bomberhuman.js":
/*!*****************************!*\
  !*** ../pkg/bomberhuman.js ***!
  \*****************************/
/*! exports provided: initialize, Key, ActorId, GameState, __wbg_screenclearrect_3ca97735de5553db, __wbg_screenputsprite_962e70243a8629a3, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_getRandomValues_3ac1b33c90b52596, __wbg_randomFillSync_6f956029658662ec, __wbg_self_1c83eb4471d9eb9b, __wbg_static_accessor_MODULE_abf5ae284bffdf45, __wbg_require_5b2b5b594d809d9f, __wbg_crypto_c12f14e810edcaa2, __wbg_msCrypto_679be765111ba775, __wbindgen_is_undefined, __wbg_getRandomValues_05a60bf171bfc2be, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./bomberhuman_bg.wasm */ \"../pkg/bomberhuman_bg.wasm\");\n/* harmony import */ var _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./bomberhuman_bg.js */ \"../pkg/bomberhuman_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"initialize\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"initialize\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Key\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Key\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"ActorId\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"ActorId\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"GameState\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"GameState\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_screenclearrect_3ca97735de5553db\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_screenclearrect_3ca97735de5553db\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_screenputsprite_962e70243a8629a3\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_screenputsprite_962e70243a8629a3\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_3ac1b33c90b52596\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_3ac1b33c90b52596\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_6f956029658662ec\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_6f956029658662ec\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1c83eb4471d9eb9b\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_1c83eb4471d9eb9b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_abf5ae284bffdf45\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_static_accessor_MODULE_abf5ae284bffdf45\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_5b2b5b594d809d9f\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_5b2b5b594d809d9f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_c12f14e810edcaa2\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_c12f14e810edcaa2\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_679be765111ba775\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_msCrypto_679be765111ba775\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_05a60bf171bfc2be\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_05a60bf171bfc2be\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _bomberhuman_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n_bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]();\n\n\n//# sourceURL=webpack:///../pkg/bomberhuman.js?");

/***/ }),

/***/ "../pkg/bomberhuman_bg.js":
/*!********************************!*\
  !*** ../pkg/bomberhuman_bg.js ***!
  \********************************/
/*! exports provided: initialize, Key, ActorId, GameState, __wbg_screenclearrect_3ca97735de5553db, __wbg_screenputsprite_962e70243a8629a3, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_getRandomValues_3ac1b33c90b52596, __wbg_randomFillSync_6f956029658662ec, __wbg_self_1c83eb4471d9eb9b, __wbg_static_accessor_MODULE_abf5ae284bffdf45, __wbg_require_5b2b5b594d809d9f, __wbg_crypto_c12f14e810edcaa2, __wbg_msCrypto_679be765111ba775, __wbindgen_is_undefined, __wbg_getRandomValues_05a60bf171bfc2be, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"initialize\", function() { return initialize; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Key\", function() { return Key; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"ActorId\", function() { return ActorId; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"GameState\", function() { return GameState; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_screenclearrect_3ca97735de5553db\", function() { return __wbg_screenclearrect_3ca97735de5553db; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_screenputsprite_962e70243a8629a3\", function() { return __wbg_screenputsprite_962e70243a8629a3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_3ac1b33c90b52596\", function() { return __wbg_getRandomValues_3ac1b33c90b52596; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_6f956029658662ec\", function() { return __wbg_randomFillSync_6f956029658662ec; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1c83eb4471d9eb9b\", function() { return __wbg_self_1c83eb4471d9eb9b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_abf5ae284bffdf45\", function() { return __wbg_static_accessor_MODULE_abf5ae284bffdf45; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_5b2b5b594d809d9f\", function() { return __wbg_require_5b2b5b594d809d9f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_c12f14e810edcaa2\", function() { return __wbg_crypto_c12f14e810edcaa2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_679be765111ba775\", function() { return __wbg_msCrypto_679be765111ba775; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_05a60bf171bfc2be\", function() { return __wbg_getRandomValues_05a60bf171bfc2be; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _snippets_bomberhuman_3fb6ca739e397268_src_javascripts_screen_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./snippets/bomberhuman-3fb6ca739e397268/src/javascripts/screen.js */ \"../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascripts/screen.js\");\n/* harmony import */ var _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./bomberhuman_bg.wasm */ \"../pkg/bomberhuman_bg.wasm\");\n\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* Main for WASM\n*/\nfunction initialize() {\n    _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"initialize\"]();\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n* Keyboard\n*/\nconst Key = Object.freeze({ Button1:0,\"0\":\"Button1\",Left:1,\"1\":\"Left\",Right:2,\"2\":\"Right\",Up:4,\"4\":\"Up\",Down:8,\"8\":\"Down\", });\n/**\n* Character ID for mapping to Sprite\n*/\nconst ActorId = Object.freeze({ Player1:0,\"0\":\"Player1\",Player2:1,\"1\":\"Player2\",Player3:2,\"2\":\"Player3\",Player4:3,\"3\":\"Player4\",Bomb:4,\"4\":\"Bomb\",Block:5,\"5\":\"Block\",Fire:6,\"6\":\"Fire\",BombUp:7,\"7\":\"BombUp\",BombPowerUp:8,\"8\":\"BombPowerUp\",SpeedUp:9,\"9\":\"SpeedUp\", });\n/**\n* Game State\n*/\nclass GameState {\n\n    static __wrap(ptr) {\n        const obj = Object.create(GameState.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_gamestate_free\"](ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    get width() {\n        var ret = _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_get_gamestate_width\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set width(arg0) {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_gamestate_width\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get height() {\n        var ret = _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_get_gamestate_height\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @param {number} arg0\n    */\n    set height(arg0) {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_gamestate_height\"](this.ptr, arg0);\n    }\n    /**\n    * @param {number} width\n    * @param {number} height\n    * @returns {GameState}\n    */\n    static new(width, height) {\n        var ret = _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"gamestate_new\"](width, height);\n        return GameState.__wrap(ret);\n    }\n    /**\n    * Update status of actors in the game.\n    *\n    * `delta` is in ms. In general, one frame takes 16.6 ms.\n    * @param {number} delta\n    */\n    update(delta) {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"gamestate_update\"](this.ptr, delta);\n    }\n    /**\n    * Draw all actors in the game.\n    */\n    draw() {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"gamestate_draw\"](this.ptr);\n    }\n    /**\n    * Callback function on change the key-input status\n    * @param {number} bind\n    * @param {number} key\n    * @param {boolean} state\n    */\n    toggle_key(bind, key, state) {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"gamestate_toggle_key\"](this.ptr, bind, key, state);\n    }\n}\n\nconst __wbg_screenclearrect_3ca97735de5553db = function(arg0, arg1, arg2, arg3) {\n    Object(_snippets_bomberhuman_3fb6ca739e397268_src_javascripts_screen_js__WEBPACK_IMPORTED_MODULE_0__[\"screen_clear_rect\"])(arg0, arg1, arg2 >>> 0, arg3 >>> 0);\n};\n\nconst __wbg_screenputsprite_962e70243a8629a3 = function(arg0, arg1, arg2, arg3) {\n    Object(_snippets_bomberhuman_3fb6ca739e397268_src_javascripts_screen_js__WEBPACK_IMPORTED_MODULE_0__[\"screen_put_sprite\"])(arg0, arg1, arg2 >>> 0, arg3 >>> 0);\n};\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_malloc\"], _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _bomberhuman_bg_wasm__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_getRandomValues_3ac1b33c90b52596 = function(arg0, arg1, arg2) {\n    getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_randomFillSync_6f956029658662ec = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_self_1c83eb4471d9eb9b = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_static_accessor_MODULE_abf5ae284bffdf45 = function() {\n    var ret = module;\n    return addHeapObject(ret);\n};\n\nconst __wbg_require_5b2b5b594d809d9f = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_c12f14e810edcaa2 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbg_msCrypto_679be765111ba775 = function(arg0) {\n    var ret = getObject(arg0).msCrypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_05a60bf171bfc2be = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/bomberhuman_bg.js?");

/***/ }),

/***/ "../pkg/bomberhuman_bg.wasm":
/*!**********************************!*\
  !*** ../pkg/bomberhuman_bg.wasm ***!
  \**********************************/
/*! exports provided: memory, __wbg_gamestate_free, __wbg_get_gamestate_width, __wbg_set_gamestate_width, __wbg_get_gamestate_height, __wbg_set_gamestate_height, gamestate_new, gamestate_update, gamestate_draw, gamestate_toggle_key, initialize, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_exn_store, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./bomberhuman_bg.js */ \"../pkg/bomberhuman_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/bomberhuman_bg.wasm?");

/***/ }),

/***/ "../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascripts/screen.js":
/*!******************************************************************************!*\
  !*** ../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascripts/screen.js ***!
  \******************************************************************************/
/*! exports provided: screen_clear_rect, screen_put_sprite */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"screen_clear_rect\", function() { return screen_clear_rect; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"screen_put_sprite\", function() { return screen_put_sprite; });\nconst debug = false;\n\n////////////////////////////////////////////////////////////////\n/// Sprite\n////////////////////////////////////////////////////////////////\n\nconst dimension = 60;\n\nclass Sprite {\n  constructor(image_path) {\n    this.image = new Image();\n    this.image.src = image_path;\n  }\n\n  draw_on(ctx, x, y, action = 0) {\n    ctx.drawImage(this.image,\n                  dimension * action, 0,\n                  dimension, dimension,\n                  x, y,\n                  dimension, dimension);\n  }\n}\n\n////////////////////////////////////////////////////////////////\n/// export functions to WASM\n////////////////////////////////////////////////////////////////\n\nlet ctx = document.getElementById('canvas').getContext('2d');\nlet sprites = new Sprite('assets/sprites.png');\n\nfunction screen_clear_rect(x, y, width, height) {\n  ctx.clearRect(x, y, width, height);\n}\n\n/// put sprite on x,y\nfunction screen_put_sprite(x, y, class_id, action = 0) {\n  ctx.drawImage(sprites.image,\n                dimension * action, dimension * class_id,\n                dimension, dimension,\n                x, y,\n                dimension, dimension);\n}\n\n\n//# sourceURL=webpack:///../pkg/snippets/bomberhuman-3fb6ca739e397268/src/javascripts/screen.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var bomberhuman__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! bomberhuman */ \"../pkg/bomberhuman.js\");\nconst debug = true;\n\n\n\n////////////////////////////////////////////////////////////////\n// Key handling\n////////////////////////////////////////////////////////////////\n\n/// `Key` is imported from WASM\nconst KeyBind = {\n  // Player 1\n  \"ArrowLeft\":  [0, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Left],\n  \"ArrowRight\": [0, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Right],\n  \"ArrowUp\":    [0, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Up],\n  \"ArrowDown\":  [0, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Down],\n  \" \":          [0, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Button1],\n\n  // Player 2\n  \"a\":          [1, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Left],\n  \"d\":          [1, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Right],\n  \"w\":          [1, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Up],\n  \"s\":          [1, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Down],\n  \"q\":          [1, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Button1],\n\n  // Player 3\n  \"h\":          [2, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Left],\n  \"l\":          [2, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Right],\n  \"k\":          [2, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Up],\n  \"j\":          [2, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Down],\n  \"u\":          [2, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Button1],\n\n  // Player 4\n  \"1\":          [3, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Left],\n  \"2\":          [3, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Right],\n  \"3\":          [3, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Up],\n  \"4\":          [3, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Down],\n  \"5\":          [3, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Button1],\n};\n\nfunction process_key(key, state) {\n  if (debug) console.log(key);\n  let bind = KeyBind[key];\n  if (bind)\n    gs.toggle_key(bind[0], bind[1], state);\n}\n\n////////////////////////////////////////////////////////////////\n// Gamepads handling\n////////////////////////////////////////////////////////////////\n\n// Needs \"gamepadconnected\" handler even if empty.\nfunction init_gamepads(gp) {\n  if (debug) {\n    console.log(\"Gamepad connected at index:%d buttons:%d axes:%d [%s]\",\n                gp.index, gp.buttons.length, gp.axes.length, gp.id);\n  }\n}\n\nfunction scan_gamepads() {\n  // Chrome should refresh gamepads everytime you read.\n  var gamepads = navigator.getGamepads ? navigator.getGamepads() : [];\n\n  for (var i = 0; i < gamepads.length; i++) {\n    var pad = gamepads[i];\n\n    if (pad) {\n      // Send state to WASM\n      gs.toggle_key(i, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Left,    pad.axes[0] < -0.5);\n      gs.toggle_key(i, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Right,   pad.axes[0] >  0.5);\n      gs.toggle_key(i, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Up,      pad.axes[1] < -0.5);\n      gs.toggle_key(i, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Down,    pad.axes[1] >  0.5);\n      gs.toggle_key(i, bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"Key\"].Button1, pad.buttons[0].pressed);\n    }\n  }\n}\n\n////////////////////////////////////////////////////////////////\n// Main loop\n////////////////////////////////////////////////////////////////\n\nlet gs = null;\nlet start = null;\nlet prev_timestamp = null;\n\nlet game_loop = (timestamp) => {\n  if (!prev_timestamp) {\n    start = timestamp;\n    prev_timestamp = timestamp;\n    requestAnimationFrame(game_loop);\n    return;\n  }\n\n  let delta = (timestamp - prev_timestamp);\n\n  scan_gamepads();\n  gs.update(delta);  // WASM\n  gs.draw();  // WASM\n\n  prev_timestamp = timestamp;\n  requestAnimationFrame(game_loop);\n};\n\n////////////////////////////////////////////////////////////////\n// Main\n////////////////////////////////////////////////////////////////\n\nfunction start_game() {\n  gs = bomberhuman__WEBPACK_IMPORTED_MODULE_0__[\"GameState\"].new(900, 780); // WASM\n  document.addEventListener('keydown', e => process_key(e.key, true));\n  document.addEventListener('keyup',   e => process_key(e.key, false));\n  document.addEventListener(\"gamepadconnected\", e => init_gamepads(e.gamepad));\n  game_loop();\n}\n\nstart_game();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);