(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./target/js/canvas.js":
/*!*****************************!*\
  !*** ./target/js/canvas.js ***!
  \*****************************/
/*! exports provided: run, __widl_instanceof_CanvasRenderingContext2D, __widl_f_begin_path_CanvasRenderingContext2D, __widl_f_stroke_CanvasRenderingContext2D, __widl_f_set_stroke_style_CanvasRenderingContext2D, __widl_f_set_fill_style_CanvasRenderingContext2D, __widl_f_set_line_width_CanvasRenderingContext2D, __widl_f_line_to_CanvasRenderingContext2D, __widl_f_move_to_CanvasRenderingContext2D, __widl_f_fill_rect_CanvasRenderingContext2D, __widl_f_get_element_by_id_Document, __widl_f_set_attribute_Element, __widl_f_add_event_listener_with_callback_EventTarget, __widl_instanceof_HTMLCanvasElement, __widl_f_get_context_with_context_options_HTMLCanvasElement, __widl_instanceof_Window, __widl_f_request_animation_frame_Window, __widl_f_document_Window, __widl_f_inner_width_Window, __widl_f_inner_height_Window, __widl_f_device_pixel_ratio_Window, __widl_f_log_1_, __wbg_newnoargs_6a80f84471205fc8, __wbg_call_582b20dfcad7fee4, __wbg_random_2cc0c8d054a5c72a, __wbg_error_cc95a3d302735ca3, __wbindgen_object_clone_ref, __wbindgen_object_drop_ref, __wbindgen_string_new, __wbindgen_number_get, __wbindgen_is_null, __wbindgen_is_undefined, __wbindgen_boolean_get, __wbindgen_is_symbol, __wbindgen_string_get, __wbindgen_cb_drop, __wbindgen_cb_forget, __wbindgen_json_parse, __wbindgen_closure_wrapper25, __wbindgen_defer_start, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_CanvasRenderingContext2D\", function() { return __widl_instanceof_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_begin_path_CanvasRenderingContext2D\", function() { return __widl_f_begin_path_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_stroke_CanvasRenderingContext2D\", function() { return __widl_f_stroke_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_stroke_style_CanvasRenderingContext2D\", function() { return __widl_f_set_stroke_style_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_fill_style_CanvasRenderingContext2D\", function() { return __widl_f_set_fill_style_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_line_width_CanvasRenderingContext2D\", function() { return __widl_f_set_line_width_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_line_to_CanvasRenderingContext2D\", function() { return __widl_f_line_to_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_move_to_CanvasRenderingContext2D\", function() { return __widl_f_move_to_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_fill_rect_CanvasRenderingContext2D\", function() { return __widl_f_fill_rect_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_get_element_by_id_Document\", function() { return __widl_f_get_element_by_id_Document; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_attribute_Element\", function() { return __widl_f_set_attribute_Element; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_add_event_listener_with_callback_EventTarget\", function() { return __widl_f_add_event_listener_with_callback_EventTarget; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_HTMLCanvasElement\", function() { return __widl_instanceof_HTMLCanvasElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_get_context_with_context_options_HTMLCanvasElement\", function() { return __widl_f_get_context_with_context_options_HTMLCanvasElement; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_instanceof_Window\", function() { return __widl_instanceof_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_request_animation_frame_Window\", function() { return __widl_f_request_animation_frame_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_document_Window\", function() { return __widl_f_document_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_inner_width_Window\", function() { return __widl_f_inner_width_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_inner_height_Window\", function() { return __widl_f_inner_height_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_device_pixel_ratio_Window\", function() { return __widl_f_device_pixel_ratio_Window; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_log_1_\", function() { return __widl_f_log_1_; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_6a80f84471205fc8\", function() { return __wbg_newnoargs_6a80f84471205fc8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_582b20dfcad7fee4\", function() { return __wbg_call_582b20dfcad7fee4; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_random_2cc0c8d054a5c72a\", function() { return __wbg_random_2cc0c8d054a5c72a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_cc95a3d302735ca3\", function() { return __wbg_error_cc95a3d302735ca3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_number_get\", function() { return __wbindgen_number_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_null\", function() { return __wbindgen_is_null; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_boolean_get\", function() { return __wbindgen_boolean_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_symbol\", function() { return __wbindgen_is_symbol; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_get\", function() { return __wbindgen_string_get; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_cb_drop\", function() { return __wbindgen_cb_drop; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_cb_forget\", function() { return __wbindgen_cb_forget; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_json_parse\", function() { return __wbindgen_json_parse; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_closure_wrapper25\", function() { return __wbindgen_closure_wrapper25; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_defer_start\", function() { return __wbindgen_defer_start; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _canvas_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./canvas_bg */ \"./target/js/canvas_bg.wasm\");\n/* tslint:disable */\n\n\n/**\n* This module handles the high-level initialization.\n* @returns {void}\n*/\nfunction run() {\n    return _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"run\"]();\n}\n\nconst heap = new Array(32);\n\nheap.fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction __widl_instanceof_CanvasRenderingContext2D(idx) {\n    return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0;\n}\n\nconst __widl_f_begin_path_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.beginPath || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.beginPath does not exist`);\n};\n\nfunction __widl_f_begin_path_CanvasRenderingContext2D(arg0) {\n    __widl_f_begin_path_CanvasRenderingContext2D_target.call(getObject(arg0));\n}\n\nconst __widl_f_stroke_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.stroke || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.stroke does not exist`);\n};\n\nfunction __widl_f_stroke_CanvasRenderingContext2D(arg0) {\n    __widl_f_stroke_CanvasRenderingContext2D_target.call(getObject(arg0));\n}\n\nfunction GetOwnOrInheritedPropertyDescriptor(obj, id) {\n    while (obj) {\n        let desc = Object.getOwnPropertyDescriptor(obj, id);\n        if (desc) return desc;\n        obj = Object.getPrototypeOf(obj);\n    }\nreturn {}\n}\n\nconst __widl_f_set_stroke_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'strokeStyle').set || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.strokeStyle does not exist`);\n};\n\nfunction __widl_f_set_stroke_style_CanvasRenderingContext2D(arg0, arg1) {\n    __widl_f_set_stroke_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));\n}\n\nconst __widl_f_set_fill_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'fillStyle').set || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillStyle does not exist`);\n};\n\nfunction __widl_f_set_fill_style_CanvasRenderingContext2D(arg0, arg1) {\n    __widl_f_set_fill_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));\n}\n\nconst __widl_f_set_line_width_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'lineWidth').set || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineWidth does not exist`);\n};\n\nfunction __widl_f_set_line_width_CanvasRenderingContext2D(arg0, arg1) {\n    __widl_f_set_line_width_CanvasRenderingContext2D_target.call(getObject(arg0), arg1);\n}\n\nconst __widl_f_line_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.lineTo || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineTo does not exist`);\n};\n\nfunction __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {\n    __widl_f_line_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);\n}\n\nconst __widl_f_move_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.moveTo || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.moveTo does not exist`);\n};\n\nfunction __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {\n    __widl_f_move_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);\n}\n\nconst __widl_f_fill_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillRect || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillRect does not exist`);\n};\n\nfunction __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {\n    __widl_f_fill_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? __webpack_require__(/*! util */ \"./node_modules/util/util.js\").TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nconst __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {\n    throw new Error(`wasm-bindgen: Document.getElementById does not exist`);\n};\n\nfunction __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {\n    let varg1 = getStringFromWasm(arg1, arg2);\n\n    const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);\n    return isLikeNone(val) ? 0 : addHeapObject(val);\n\n}\n\nconst __widl_f_set_attribute_Element_target = typeof Element === 'undefined' ? null : Element.prototype.setAttribute || function() {\n    throw new Error(`wasm-bindgen: Element.setAttribute does not exist`);\n};\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n\nfunction __widl_f_set_attribute_Element(arg0, arg1, arg2, arg3, arg4, exnptr) {\n    let varg1 = getStringFromWasm(arg1, arg2);\n    let varg3 = getStringFromWasm(arg3, arg4);\n    try {\n        __widl_f_set_attribute_Element_target.call(getObject(arg0), varg1, varg3);\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nconst __widl_f_add_event_listener_with_callback_EventTarget_target = typeof EventTarget === 'undefined' ? null : EventTarget.prototype.addEventListener || function() {\n    throw new Error(`wasm-bindgen: EventTarget.addEventListener does not exist`);\n};\n\nfunction __widl_f_add_event_listener_with_callback_EventTarget(arg0, arg1, arg2, arg3, exnptr) {\n    let varg1 = getStringFromWasm(arg1, arg2);\n    try {\n        __widl_f_add_event_listener_with_callback_EventTarget_target.call(getObject(arg0), varg1, getObject(arg3));\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __widl_instanceof_HTMLCanvasElement(idx) {\n    return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0;\n}\n\nconst __widl_f_get_context_with_context_options_HTMLCanvasElement_target = typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype.getContext || function() {\n    throw new Error(`wasm-bindgen: HTMLCanvasElement.getContext does not exist`);\n};\n\nfunction __widl_f_get_context_with_context_options_HTMLCanvasElement(arg0, arg1, arg2, arg3, exnptr) {\n    let varg1 = getStringFromWasm(arg1, arg2);\n    try {\n\n        const val = __widl_f_get_context_with_context_options_HTMLCanvasElement_target.call(getObject(arg0), varg1, getObject(arg3));\n        return isLikeNone(val) ? 0 : addHeapObject(val);\n\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __widl_instanceof_Window(idx) {\n    return getObject(idx) instanceof Window ? 1 : 0;\n}\n\nfunction __widl_f_request_animation_frame_Window(arg0, arg1, exnptr) {\n    try {\n        return getObject(arg0).requestAnimationFrame(getObject(arg1));\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __widl_f_document_Window(arg0) {\n\n    const val = getObject(arg0).document;\n    return isLikeNone(val) ? 0 : addHeapObject(val);\n\n}\n\nfunction __widl_f_inner_width_Window(arg0, exnptr) {\n    try {\n        return addHeapObject(getObject(arg0).innerWidth);\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __widl_f_inner_height_Window(arg0, exnptr) {\n    try {\n        return addHeapObject(getObject(arg0).innerHeight);\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __widl_f_device_pixel_ratio_Window(arg0) {\n    return getObject(arg0).devicePixelRatio;\n}\n\nconst __widl_f_log_1__target = console.log;\n\nfunction __widl_f_log_1_(arg0) {\n    __widl_f_log_1__target(getObject(arg0));\n}\n\nfunction __wbg_newnoargs_6a80f84471205fc8(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    return addHeapObject(new Function(varg0));\n}\n\nfunction __wbg_call_582b20dfcad7fee4(arg0, arg1, exnptr) {\n    try {\n        return addHeapObject(getObject(arg0).call(getObject(arg1)));\n    } catch (e) {\n        const view = getUint32Memory();\n        view[exnptr / 4] = 1;\n        view[exnptr / 4 + 1] = addHeapObject(e);\n\n    }\n}\n\nfunction __wbg_random_2cc0c8d054a5c72a() {\n    return Math.random();\n}\n\nfunction __wbg_error_cc95a3d302735ca3(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n\n    varg0 = varg0.slice();\n    _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1 * 1);\n\n    console.error(varg0);\n}\n\nfunction __wbindgen_object_clone_ref(idx) {\n    return addHeapObject(getObject(idx));\n}\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction __wbindgen_object_drop_ref(i) { dropObject(i); }\n\nfunction __wbindgen_string_new(p, l) {\n    return addHeapObject(getStringFromWasm(p, l));\n}\n\nfunction __wbindgen_number_get(n, invalid) {\n    let obj = getObject(n);\n    if (typeof(obj) === 'number') return obj;\n    getUint8Memory()[invalid] = 1;\n    return 0;\n}\n\nfunction __wbindgen_is_null(idx) {\n    return getObject(idx) === null ? 1 : 0;\n}\n\nfunction __wbindgen_is_undefined(idx) {\n    return getObject(idx) === undefined ? 1 : 0;\n}\n\nfunction __wbindgen_boolean_get(i) {\n    let v = getObject(i);\n    if (typeof(v) === 'boolean') {\n        return v ? 1 : 0;\n    } else {\n        return 2;\n    }\n}\n\nfunction __wbindgen_is_symbol(i) {\n    return typeof(getObject(i)) === 'symbol' ? 1 : 0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? __webpack_require__(/*! util */ \"./node_modules/util/util.js\").TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passStringToWasm(arg) {\n\n    const buf = cachedTextEncoder.encode(arg);\n    const ptr = _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](buf.length);\n    getUint8Memory().set(buf, ptr);\n    WASM_VECTOR_LEN = buf.length;\n    return ptr;\n}\n\nfunction __wbindgen_string_get(i, len_ptr) {\n    let obj = getObject(i);\n    if (typeof(obj) !== 'string') return 0;\n    const ptr = passStringToWasm(obj);\n    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;\n    return ptr;\n}\n\nfunction __wbindgen_cb_drop(i) {\n    const obj = getObject(i).original;\n    dropObject(i);\n    if (obj.cnt-- == 1) {\n        obj.a = 0;\n        return 1;\n    }\n    return 0;\n}\n\nconst __wbindgen_cb_forget = dropObject;\n\nfunction __wbindgen_json_parse(ptr, len) {\n    return addHeapObject(JSON.parse(getStringFromWasm(ptr, len)));\n}\n\nfunction __wbindgen_closure_wrapper25(a, b, _ignored) {\n    const f = _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_function_table\"].get(16);\n    const d = _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_function_table\"].get(17);\n    const cb = function() {\n        this.cnt++;\n        let a = this.a;\n        this.a = 0;\n        try {\n            return f(a, b);\n\n        } finally {\n            this.a = a;\n            if (this.cnt-- == 1) d(this.a, b);\n\n        }\n\n    };\n    cb.a = a;\n    cb.cnt = 1;\n    let real = cb.bind(cb);\n    real.original = cb;\n    return addHeapObject(real);\n}\n\nfunction __wbindgen_defer_start() {\n    Promise.resolve().then(() => _canvas_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]());\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./target/js/canvas.js?");

/***/ }),

/***/ "./target/js/canvas_bg.wasm":
/*!**********************************!*\
  !*** ./target/js/canvas_bg.wasm ***!
  \**********************************/
/*! exports provided: memory, run, __wbindgen_malloc, __wbindgen_free, __wbindgen_start, __wbg_function_table */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./canvas */ \"./target/js/canvas.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./target/js/canvas_bg.wasm?");

/***/ })

}]);