parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"Jugo":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.run=exports.default=exports.__wbindgen_throw=exports.__wbindgen_string_new=exports.__wbindgen_object_drop_ref=exports.__wbindgen_object_clone_ref=exports.__wbindgen_number_new=exports.__wbindgen_number_get=exports.__wbindgen_is_undefined=exports.__wbindgen_debug_string=exports.__wbindgen_closure_wrapper616=exports.__wbindgen_closure_wrapper435=exports.__wbindgen_cb_drop=exports.__wbg_window_b4be7f48b24ac56e=exports.__wbg_warn_ca021eeadd0df9cd=exports.__wbg_warn_2aa0e7178e1d35f6=exports.__wbg_value_fc1c354d1a0e9714=exports.__wbg_value_d3a30bc2c7caf357=exports.__wbg_valueOf_39e0d6bc7e4232b9=exports.__wbg_target_e560052e31e4567c=exports.__wbg_stack_0ddaca5d1abfb52f=exports.__wbg_setvalue_ce4a23f487065c07=exports.__wbg_setvalue_6a34bab301f38bf2=exports.__wbg_setnodeValue_f175b74a390f8fda=exports.__wbg_setchecked_f6ead3490df88a7f=exports.__wbg_set_c42875065132a932=exports.__wbg_setAttribute_1776fcc9b98d464e=exports.__wbg_self_e23d74ae45fb17d1=exports.__wbg_removeEventListener_9cd36e5806463d5d=exports.__wbg_removeChild_f4a83c9698136bbb=exports.__wbg_removeAttribute_1adaecf6b4d35a09=exports.__wbg_querySelector_cc714d0aa0b868ed=exports.__wbg_pathname_d0014089875ea691=exports.__wbg_pathname_7affbcff36f35c0e=exports.__wbg_parentElement_96e1e07348340043=exports.__wbg_newnoargs_f579424187aa1717=exports.__wbg_new_d3138911a89329b0=exports.__wbg_new_693216e109162396=exports.__wbg_new_4473c9af1cac368b=exports.__wbg_namespaceURI_e9a971e6c1ce68db=exports.__wbg_log_681299aef22afa27=exports.__wbg_location_11472bb76bf5bbca=exports.__wbg_lastChild_e2b014abab089e08=exports.__wbg_is_3d73f4d91adacc37=exports.__wbg_instanceof_Window_434ce1849eb4e0fc=exports.__wbg_instanceof_Element_c9423704dd5d9b1d=exports.__wbg_insertBefore_4f09909023feac91=exports.__wbg_info_8bed0988e7416289=exports.__wbg_href_cad8f02caf39f2fb=exports.__wbg_history_52cfc93c824e772b=exports.__wbg_global_e7669da72fd7f239=exports.__wbg_globalThis_d61b1f48a57191ae=exports.__wbg_get_8bbb82393651dd9c=exports.__wbg_error_ca520cb687b085a1=exports.__wbg_error_644d3bc8c0537e80=exports.__wbg_error_09919627ac0992f5=exports.__wbg_document_5edd43643d1060d9=exports.__wbg_debug_6df4b1a327dd2e94=exports.__wbg_createTextNode_39a0de25d14bcde5=exports.__wbg_createElement_d017b8d2af99bab9=exports.__wbg_createElementNS_fd4a7e49f74039e1=exports.__wbg_cancelBubble_17d7988ab2fbe4c9=exports.__wbg_call_89558c3e96703ca1=exports.__wbg_body_7538539844356c1c=exports.__wbg_appendChild_3fe5090c665d3bb4=exports.__wbg_addEventListener_55682f77717d7665=void 0;var _=e(require("./pkg/yew_app_bg.wasm"));function e(_){return _&&_.__esModule?_:{default:_}}var a=_.default;exports.default=a;var b=_.default.run;exports.run=b;var r=_.default.__wbindgen_object_drop_ref;exports.__wbindgen_object_drop_ref=r;var t=_.default.__wbindgen_object_clone_ref;exports.__wbindgen_object_clone_ref=t;var d=_.default.__wbindgen_string_new;exports.__wbindgen_string_new=d;var f=_.default.__wbindgen_cb_drop;exports.__wbindgen_cb_drop=f;var w=_.default.__wbindgen_number_new;exports.__wbindgen_number_new=w;var o=_.default.__wbindgen_number_get;exports.__wbindgen_number_get=o;var g=_.default.__wbg_new_693216e109162396;exports.__wbg_new_693216e109162396=g;var c=_.default.__wbg_stack_0ddaca5d1abfb52f;exports.__wbg_stack_0ddaca5d1abfb52f=c;var s=_.default.__wbg_error_09919627ac0992f5;exports.__wbg_error_09919627ac0992f5=s;var n=_.default.__wbg_warn_2aa0e7178e1d35f6;exports.__wbg_warn_2aa0e7178e1d35f6=n;var p=_.default.__wbg_instanceof_Window_434ce1849eb4e0fc;exports.__wbg_instanceof_Window_434ce1849eb4e0fc=p;var l=_.default.__wbg_document_5edd43643d1060d9;exports.__wbg_document_5edd43643d1060d9=l;var x=_.default.__wbg_location_11472bb76bf5bbca;exports.__wbg_location_11472bb76bf5bbca=x;var u=_.default.__wbg_history_52cfc93c824e772b;exports.__wbg_history_52cfc93c824e772b=u;var i=_.default.__wbg_body_7538539844356c1c;exports.__wbg_body_7538539844356c1c=i;var v=_.default.__wbg_createElement_d017b8d2af99bab9;exports.__wbg_createElement_d017b8d2af99bab9=v;var m=_.default.__wbg_createElementNS_fd4a7e49f74039e1;exports.__wbg_createElementNS_fd4a7e49f74039e1=m;var h=_.default.__wbg_createTextNode_39a0de25d14bcde5;exports.__wbg_createTextNode_39a0de25d14bcde5=h;var E=_.default.__wbg_querySelector_cc714d0aa0b868ed;exports.__wbg_querySelector_cc714d0aa0b868ed=E;var y=_.default.__wbg_parentElement_96e1e07348340043;exports.__wbg_parentElement_96e1e07348340043=y;var C=_.default.__wbg_lastChild_e2b014abab089e08;exports.__wbg_lastChild_e2b014abab089e08=C;var j=_.default.__wbg_setnodeValue_f175b74a390f8fda;exports.__wbg_setnodeValue_f175b74a390f8fda=j;var k=_.default.__wbg_appendChild_3fe5090c665d3bb4;exports.__wbg_appendChild_3fe5090c665d3bb4=k;var A=_.default.__wbg_insertBefore_4f09909023feac91;exports.__wbg_insertBefore_4f09909023feac91=A;var B=_.default.__wbg_removeChild_f4a83c9698136bbb;exports.__wbg_removeChild_f4a83c9698136bbb=B;var L=_.default.__wbg_pathname_d0014089875ea691;exports.__wbg_pathname_d0014089875ea691=L;var N=_.default.__wbg_value_d3a30bc2c7caf357;exports.__wbg_value_d3a30bc2c7caf357=N;var S=_.default.__wbg_setvalue_6a34bab301f38bf2;exports.__wbg_setvalue_6a34bab301f38bf2=S;var T=_.default.__wbg_pathname_7affbcff36f35c0e;exports.__wbg_pathname_7affbcff36f35c0e=T;var q=_.default.__wbg_new_4473c9af1cac368b;exports.__wbg_new_4473c9af1cac368b=q;var O=_.default.__wbg_instanceof_Element_c9423704dd5d9b1d;exports.__wbg_instanceof_Element_c9423704dd5d9b1d=O;var I=_.default.__wbg_namespaceURI_e9a971e6c1ce68db;exports.__wbg_namespaceURI_e9a971e6c1ce68db=I;var R=_.default.__wbg_removeAttribute_1adaecf6b4d35a09;exports.__wbg_removeAttribute_1adaecf6b4d35a09=R;var U=_.default.__wbg_setAttribute_1776fcc9b98d464e;exports.__wbg_setAttribute_1776fcc9b98d464e=U;var V=_.default.__wbg_debug_6df4b1a327dd2e94;exports.__wbg_debug_6df4b1a327dd2e94=V;var W=_.default.__wbg_error_ca520cb687b085a1;exports.__wbg_error_ca520cb687b085a1=W;var M=_.default.__wbg_error_644d3bc8c0537e80;exports.__wbg_error_644d3bc8c0537e80=M;var P=_.default.__wbg_info_8bed0988e7416289;exports.__wbg_info_8bed0988e7416289=P;var z=_.default.__wbg_log_681299aef22afa27;exports.__wbg_log_681299aef22afa27=z;var D=_.default.__wbg_warn_ca021eeadd0df9cd;exports.__wbg_warn_ca021eeadd0df9cd=D;var F=_.default.__wbg_href_cad8f02caf39f2fb;exports.__wbg_href_cad8f02caf39f2fb=F;var G=_.default.__wbg_setchecked_f6ead3490df88a7f;exports.__wbg_setchecked_f6ead3490df88a7f=G;var H=_.default.__wbg_value_fc1c354d1a0e9714;exports.__wbg_value_fc1c354d1a0e9714=H;var J=_.default.__wbg_setvalue_ce4a23f487065c07;exports.__wbg_setvalue_ce4a23f487065c07=J;var K=_.default.__wbg_target_e560052e31e4567c;exports.__wbg_target_e560052e31e4567c=K;var Q=_.default.__wbg_cancelBubble_17d7988ab2fbe4c9;exports.__wbg_cancelBubble_17d7988ab2fbe4c9=Q;var X=_.default.__wbg_addEventListener_55682f77717d7665;exports.__wbg_addEventListener_55682f77717d7665=X;var Y=_.default.__wbg_removeEventListener_9cd36e5806463d5d;exports.__wbg_removeEventListener_9cd36e5806463d5d=Y;var Z=_.default.__wbg_newnoargs_f579424187aa1717;exports.__wbg_newnoargs_f579424187aa1717=Z;var $=_.default.__wbg_get_8bbb82393651dd9c;exports.__wbg_get_8bbb82393651dd9c=$;var __=_.default.__wbg_call_89558c3e96703ca1;exports.__wbg_call_89558c3e96703ca1=__;var e_=_.default.__wbg_new_d3138911a89329b0;exports.__wbg_new_d3138911a89329b0=e_;var a_=_.default.__wbg_valueOf_39e0d6bc7e4232b9;exports.__wbg_valueOf_39e0d6bc7e4232b9=a_;var b_=_.default.__wbg_is_3d73f4d91adacc37;exports.__wbg_is_3d73f4d91adacc37=b_;var r_=_.default.__wbg_self_e23d74ae45fb17d1;exports.__wbg_self_e23d74ae45fb17d1=r_;var t_=_.default.__wbg_window_b4be7f48b24ac56e;exports.__wbg_window_b4be7f48b24ac56e=t_;var d_=_.default.__wbg_globalThis_d61b1f48a57191ae;exports.__wbg_globalThis_d61b1f48a57191ae=d_;var f_=_.default.__wbg_global_e7669da72fd7f239;exports.__wbg_global_e7669da72fd7f239=f_;var w_=_.default.__wbindgen_is_undefined;exports.__wbindgen_is_undefined=w_;var o_=_.default.__wbg_set_c42875065132a932;exports.__wbg_set_c42875065132a932=o_;var g_=_.default.__wbindgen_debug_string;exports.__wbindgen_debug_string=g_;var c_=_.default.__wbindgen_throw;exports.__wbindgen_throw=c_;var s_=_.default.__wbindgen_closure_wrapper435;exports.__wbindgen_closure_wrapper435=s_;var n_=_.default.__wbindgen_closure_wrapper616;exports.__wbindgen_closure_wrapper616=n_;
},{"./pkg/yew_app_bg.wasm":"HmKK"}],"QCba":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0});var e=require("../Cargo.toml");(0,e.run)();
},{"../Cargo.toml":"Jugo"}],"FheM":[function(require,module,exports) {
var t=null;function e(){return t||(t=n()),t}function n(){try{throw new Error}catch(e){var t=(""+e.stack).match(/(https?|file|ftp|chrome-extension|moz-extension):\/\/[^)\n]+/g);if(t)return r(t[0])}return"/"}function r(t){return(""+t).replace(/^((?:https?|file|ftp|chrome-extension|moz-extension):\/\/.+)?\/[^/]+(?:\?.*)?$/,"$1")+"/"}exports.getBundleURL=e,exports.getBaseURL=r;
},{}],"TUK3":[function(require,module,exports) {
var r=require("./bundle-url").getBundleURL;function e(r){Array.isArray(r)||(r=[r]);var e=r[r.length-1];try{return Promise.resolve(require(e))}catch(n){if("MODULE_NOT_FOUND"===n.code)return new s(function(n,i){t(r.slice(0,-1)).then(function(){return require(e)}).then(n,i)});throw n}}function t(r){return Promise.all(r.map(u))}var n={};function i(r,e){n[r]=e}module.exports=exports=e,exports.load=t,exports.register=i;var o={};function u(e){var t;if(Array.isArray(e)&&(t=e[1],e=e[0]),o[e])return o[e];var i=(e.substring(e.lastIndexOf(".")+1,e.length)||e).toLowerCase(),u=n[i];return u?o[e]=u(r()+e).then(function(r){return r&&module.bundle.register(t,r),r}).catch(function(r){throw delete o[e],r}):void 0}function s(r){this.executor=r,this.promise=null}s.prototype.then=function(r,e){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.then(r,e)},s.prototype.catch=function(r){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.catch(r)};
},{"./bundle-url":"FheM"}],"rDCW":[function(require,module,exports) {

},{}],"fISM":[function(require,module,exports) {
var global = arguments[3];
var __dirname = "/home/runner/work/Nojipiz.github.io/Nojipiz.github.io/node_modules/parcel-plugin-wasm.rs";
var n,e=arguments[3],t="/home/runner/work/Nojipiz.github.io/Nojipiz.github.io/node_modules/parcel-plugin-wasm.rs";const r={},_=new Array(32).fill(void 0);function o(n){return _[n]}_.push(void 0,null,!0,!1);let c=_.length;function u(n){n<36||(_[n]=c,c=n)}function i(n){const e=o(n);return u(n),e}function f(n){c===_.length&&_.push(_.length+1);const e=c;return c=_[e],_[e]=n,e}const a="undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder;let b=new a("utf-8",{ignoreBOM:!0,fatal:!0});b.decode();let l=null;function d(){return null!==l&&l.buffer===n.memory.buffer||(l=new Uint8Array(n.memory.buffer)),l}function s(n,e){return b.decode(d().subarray(n,n+e))}function g(n){return null==n}let w=null;function m(){return null!==w&&w.buffer===n.memory.buffer||(w=new Float64Array(n.memory.buffer)),w}let y=null;function h(){return null!==y&&y.buffer===n.memory.buffer||(y=new Int32Array(n.memory.buffer)),y}function p(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=p(n[0]));for(let r=1;r<e;r++)t+=", "+p(n[r]);return t+="]"}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let r;if(!(t.length>1))return toString.call(n);if("Object"==(r=t[1]))try{return"Object("+JSON.stringify(n)+")"}catch(_){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:r}let v=0;const E="undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder;let A=new E("utf-8");const x="function"==typeof A.encodeInto?function(n,e){return A.encodeInto(n,e)}:function(n,e){const t=A.encode(n);return e.set(t),{read:n.length,written:t.length}};function j(n,e,t){if(void 0===t){const t=A.encode(n),r=e(t.length);return d().subarray(r,r+t.length).set(t),v=t.length,r}let r=n.length,_=e(r);const o=d();let c=0;for(;c<r;c++){const e=n.charCodeAt(c);if(e>127)break;o[_+c]=e}if(c!==r){0!==c&&(n=n.slice(c)),_=t(_,r,r=c+3*n.length);const e=d().subarray(_+c,_+r);c+=x(n,e).written}return v=c,_}function k(e,t,r,_){const o={a:e,b:t,cnt:1,dtor:r},c=(...e)=>{o.cnt++;const t=o.a;o.a=0;try{return _(t,o.b,...e)}finally{0==--o.cnt?n.__wbindgen_export_2.get(o.dtor)(t,o.b):o.a=t}};return c.original=o,c}let O=32;function S(n){if(1==O)throw new Error("out of js stack");return _[--O]=n,O}function T(e,t,r){try{n._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc6ce875804db3960(e,t,S(r))}finally{_[O++]=void 0}}function C(e,t,r,_){const o={a:e,b:t,cnt:1,dtor:r},c=(...e)=>{o.cnt++;try{return _(o.a,o.b,...e)}finally{0==--o.cnt&&(n.__wbindgen_export_2.get(o.dtor)(o.a,o.b),o.a=0)}};return c.original=o,c}function W(e,t,r){n._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha84562168c00b055(e,t,f(r))}r.run=function(){try{const t=n.__wbindgen_add_to_stack_pointer(-16);n.run(t);var e=h()[t/4+0];if(h()[t/4+1])throw i(e)}finally{n.__wbindgen_add_to_stack_pointer(16)}};let F=null;function N(){return null!==F&&F.buffer===n.memory.buffer||(F=new Uint32Array(n.memory.buffer)),F}function R(n,e){const t=N().subarray(n/4,n/4+e),r=[];for(let _=0;_<t.length;_++)r.push(i(t[_]));return r}function $(e,t){try{return e.apply(this,t)}catch(r){n.__wbindgen_exn_store(f(r))}}function B(e){const t=fetch(e);let _;return(_="function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,{"./yew_app_bg.js":r}):t.then(n=>n.arrayBuffer()).then(n=>WebAssembly.instantiate(n,{"./yew_app_bg.js":r}))).then(({instance:e})=>{n=B.wasm=e.exports,r.wasm=n})}function q(e){const _=require("fs");return new Promise(function(n,r){_.readFile(t+e,function(e,t){e?r(e):n(t.buffer)})}).then(n=>WebAssembly.instantiate(n,{"./yew_app_bg":r})).then(({instance:e})=>{n=B.wasm=e.exports,r.wasm=n})}r.__wbindgen_object_drop_ref=function(n){i(n)},r.__wbindgen_object_clone_ref=function(n){return f(o(n))},r.__wbindgen_string_new=function(n,e){return f(s(n,e))},r.__wbindgen_cb_drop=function(n){const e=i(n).original;if(1==e.cnt--)return e.a=0,!0;return!1},r.__wbindgen_number_new=function(n){return f(n)},r.__wbindgen_number_get=function(n,e){const t=o(e);var r="number"==typeof t?t:void 0;m()[n/8+1]=g(r)?0:r,h()[n/4+0]=!g(r)},r.__wbg_new_693216e109162396=function(){return f(new Error)},r.__wbg_stack_0ddaca5d1abfb52f=function(e,t){var r=j(o(t).stack,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbg_error_09919627ac0992f5=function(e,t){try{console.error(s(e,t))}finally{n.__wbindgen_free(e,t)}},r.__wbg_warn_2aa0e7178e1d35f6=function(e,t){var r=R(e,t).slice();n.__wbindgen_free(e,4*t),console.warn(...r)},r.__wbg_instanceof_Window_434ce1849eb4e0fc=function(n){return o(n)instanceof Window},r.__wbg_document_5edd43643d1060d9=function(n){var e=o(n).document;return g(e)?0:f(e)},r.__wbg_location_11472bb76bf5bbca=function(n){return f(o(n).location)},r.__wbg_history_52cfc93c824e772b=function(){return $(function(n){return f(o(n).history)},arguments)},r.__wbg_body_7538539844356c1c=function(n){var e=o(n).body;return g(e)?0:f(e)},r.__wbg_createElement_d017b8d2af99bab9=function(){return $(function(n,e,t){return f(o(n).createElement(s(e,t)))},arguments)},r.__wbg_createElementNS_fd4a7e49f74039e1=function(){return $(function(n,e,t,r,_){return f(o(n).createElementNS(0===e?void 0:s(e,t),s(r,_)))},arguments)},r.__wbg_createTextNode_39a0de25d14bcde5=function(n,e,t){return f(o(n).createTextNode(s(e,t)))},r.__wbg_querySelector_cc714d0aa0b868ed=function(){return $(function(n,e,t){var r=o(n).querySelector(s(e,t));return g(r)?0:f(r)},arguments)},r.__wbg_parentElement_96e1e07348340043=function(n){var e=o(n).parentElement;return g(e)?0:f(e)},r.__wbg_lastChild_e2b014abab089e08=function(n){var e=o(n).lastChild;return g(e)?0:f(e)},r.__wbg_setnodeValue_f175b74a390f8fda=function(n,e,t){o(n).nodeValue=0===e?void 0:s(e,t)},r.__wbg_appendChild_3fe5090c665d3bb4=function(){return $(function(n,e){return f(o(n).appendChild(o(e)))},arguments)},r.__wbg_insertBefore_4f09909023feac91=function(){return $(function(n,e,t){return f(o(n).insertBefore(o(e),o(t)))},arguments)},r.__wbg_removeChild_f4a83c9698136bbb=function(){return $(function(n,e){return f(o(n).removeChild(o(e)))},arguments)},r.__wbg_pathname_d0014089875ea691=function(){return $(function(e,t){var r=j(o(t).pathname,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},arguments)},r.__wbg_value_d3a30bc2c7caf357=function(e,t){var r=j(o(t).value,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbg_setvalue_6a34bab301f38bf2=function(n,e,t){o(n).value=s(e,t)},r.__wbg_pathname_7affbcff36f35c0e=function(e,t){var r=j(o(t).pathname,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbg_new_4473c9af1cac368b=function(){return $(function(n,e){return f(new URL(s(n,e)))},arguments)},r.__wbg_instanceof_Element_c9423704dd5d9b1d=function(n){return o(n)instanceof Element},r.__wbg_namespaceURI_e9a971e6c1ce68db=function(e,t){var r=o(t).namespaceURI,_=g(r)?0:j(r,n.__wbindgen_malloc,n.__wbindgen_realloc),c=v;h()[e/4+1]=c,h()[e/4+0]=_},r.__wbg_removeAttribute_1adaecf6b4d35a09=function(){return $(function(n,e,t){o(n).removeAttribute(s(e,t))},arguments)},r.__wbg_setAttribute_1776fcc9b98d464e=function(){return $(function(n,e,t,r,_){o(n).setAttribute(s(e,t),s(r,_))},arguments)},r.__wbg_debug_6df4b1a327dd2e94=function(n,e,t,r){console.debug(o(n),o(e),o(t),o(r))},r.__wbg_error_ca520cb687b085a1=function(n){console.error(o(n))},r.__wbg_error_644d3bc8c0537e80=function(n,e,t,r){console.error(o(n),o(e),o(t),o(r))},r.__wbg_info_8bed0988e7416289=function(n,e,t,r){console.info(o(n),o(e),o(t),o(r))},r.__wbg_log_681299aef22afa27=function(n,e,t,r){console.log(o(n),o(e),o(t),o(r))},r.__wbg_warn_ca021eeadd0df9cd=function(n,e,t,r){console.warn(o(n),o(e),o(t),o(r))},r.__wbg_href_cad8f02caf39f2fb=function(e,t){var r=j(o(t).href,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbg_setchecked_f6ead3490df88a7f=function(n,e){o(n).checked=0!==e},r.__wbg_value_fc1c354d1a0e9714=function(e,t){var r=j(o(t).value,n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbg_setvalue_ce4a23f487065c07=function(n,e,t){o(n).value=s(e,t)},r.__wbg_target_e560052e31e4567c=function(n){var e=o(n).target;return g(e)?0:f(e)},r.__wbg_cancelBubble_17d7988ab2fbe4c9=function(n){return o(n).cancelBubble},r.__wbg_addEventListener_55682f77717d7665=function(){return $(function(n,e,t,r,_){o(n).addEventListener(s(e,t),o(r),o(_))},arguments)},r.__wbg_removeEventListener_9cd36e5806463d5d=function(){return $(function(n,e,t,r,_){o(n).removeEventListener(s(e,t),o(r),0!==_)},arguments)},r.__wbg_newnoargs_f579424187aa1717=function(n,e){return f(new Function(s(n,e)))},r.__wbg_get_8bbb82393651dd9c=function(){return $(function(n,e){return f(Reflect.get(o(n),o(e)))},arguments)},r.__wbg_call_89558c3e96703ca1=function(){return $(function(n,e){return f(o(n).call(o(e)))},arguments)},r.__wbg_new_d3138911a89329b0=function(){return f(new Object)},r.__wbg_valueOf_39e0d6bc7e4232b9=function(n){return o(n).valueOf()},r.__wbg_is_3d73f4d91adacc37=function(n,e){return Object.is(o(n),o(e))},r.__wbg_self_e23d74ae45fb17d1=function(){return $(function(){return f(self.self)},arguments)},r.__wbg_window_b4be7f48b24ac56e=function(){return $(function(){return f(window.window)},arguments)},r.__wbg_globalThis_d61b1f48a57191ae=function(){return $(function(){return f(globalThis.globalThis)},arguments)},r.__wbg_global_e7669da72fd7f239=function(){return $(function(){return f(e.global)},arguments)},r.__wbindgen_is_undefined=function(n){return void 0===o(n)},r.__wbg_set_c42875065132a932=function(){return $(function(n,e,t){return Reflect.set(o(n),o(e),o(t))},arguments)},r.__wbindgen_debug_string=function(e,t){var r=j(p(o(t)),n.__wbindgen_malloc,n.__wbindgen_realloc),_=v;h()[e/4+1]=_,h()[e/4+0]=r},r.__wbindgen_throw=function(n,e){throw new Error(s(n,e))},r.__wbindgen_closure_wrapper435=function(n,e,t){return f(k(n,e,203,T))},r.__wbindgen_closure_wrapper616=function(n,e,t){return f(C(n,e,245,W))};const I=Object.assign(B,r);module.exports=function(n){return I(n).then(()=>r)};
},{"fs":"rDCW"}],0:[function(require,module,exports) {
var b=require("TUK3");b.register("wasm",require("fISM"));b.load([["yew_app_bg.c7fd0f67.wasm","HmKK"]]).then(function(){require("QCba");});
},{}]},{},[0], null)
//# sourceMappingURL=static.5a81ed45.js.map