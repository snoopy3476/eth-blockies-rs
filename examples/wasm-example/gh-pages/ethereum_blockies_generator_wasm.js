let wasm;const heap=new Array(32).fill(undefined);heap.push(undefined,null,true,false);function getObject(idx){return heap[idx];}
let heap_next=heap.length;function dropObject(idx){if(idx<36)return;heap[idx]=heap_next;heap_next=idx;}
function takeObject(idx){const ret=getObject(idx);dropObject(idx);return ret;}
function addHeapObject(obj){if(heap_next===heap.length)heap.push(heap.length+1);const idx=heap_next;heap_next=heap[idx];heap[idx]=obj;return idx;}
let cachedTextDecoder=new TextDecoder('utf-8',{ignoreBOM:true,fatal:true});cachedTextDecoder.decode();let cachegetUint8Memory0=null;function getUint8Memory0(){if(cachegetUint8Memory0===null||cachegetUint8Memory0.buffer!==wasm.memory.buffer){cachegetUint8Memory0=new Uint8Array(wasm.memory.buffer);}
return cachegetUint8Memory0;}
function getStringFromWasm0(ptr,len){return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr,ptr+len));}
function makeMutClosure(arg0,arg1,dtor,f){const state={a:arg0,b:arg1,cnt:1,dtor};const real=(...args)=>{state.cnt++;const a=state.a;state.a=0;try{return f(a,state.b,...args);}finally{if(--state.cnt===0){wasm.__wbindgen_export_0.get(state.dtor)(a,state.b);}else{state.a=a;}}};real.original=state;return real;}
function __wbg_adapter_10(arg0,arg1){wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h70fd950a52adbe44(arg0,arg1);}
export function init_blockies(){wasm.init_blockies();}
function handleError(f,args){try{return f.apply(this,args);}catch(e){wasm.__wbindgen_exn_store(addHeapObject(e));}}
function isLikeNone(x){return x===undefined||x===null;}
let WASM_VECTOR_LEN=0;let cachedTextEncoder=new TextEncoder('utf-8');const encodeString=(typeof cachedTextEncoder.encodeInto==='function'?function(arg,view){return cachedTextEncoder.encodeInto(arg,view);}:function(arg,view){const buf=cachedTextEncoder.encode(arg);view.set(buf);return{read:arg.length,written:buf.length};});function passStringToWasm0(arg,malloc,realloc){if(realloc===undefined){const buf=cachedTextEncoder.encode(arg);const ptr=malloc(buf.length);getUint8Memory0().subarray(ptr,ptr+buf.length).set(buf);WASM_VECTOR_LEN=buf.length;return ptr;}
let len=arg.length;let ptr=malloc(len);const mem=getUint8Memory0();let offset=0;for(;offset<len;offset++){const code=arg.charCodeAt(offset);if(code>0x7F)break;mem[ptr+offset]=code;}
if(offset!==len){if(offset!==0){arg=arg.slice(offset);}
ptr=realloc(ptr,len,len=offset+arg.length*3);const view=getUint8Memory0().subarray(ptr+offset,ptr+len);const ret=encodeString(arg,view);offset+=ret.written;}
WASM_VECTOR_LEN=offset;return ptr;}
let cachegetInt32Memory0=null;function getInt32Memory0(){if(cachegetInt32Memory0===null||cachegetInt32Memory0.buffer!==wasm.memory.buffer){cachegetInt32Memory0=new Int32Array(wasm.memory.buffer);}
return cachegetInt32Memory0;}
async function load(module,imports){if(typeof Response==='function'&&module instanceof Response){if(typeof WebAssembly.instantiateStreaming==='function'){try{return await WebAssembly.instantiateStreaming(module,imports);}catch(e){if(module.headers.get('Content-Type')!='application/wasm'){console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e);}else{throw e;}}}
const bytes=await module.arrayBuffer();return await WebAssembly.instantiate(bytes,imports);}else{const instance=await WebAssembly.instantiate(module,imports);if(instance instanceof WebAssembly.Instance){return{instance,module};}else{return instance;}}}
async function init(input){if(typeof input==='undefined'){input=new URL('ethereum_blockies_generator_wasm_bg.wasm',import.meta.url);}
const imports={};imports.wbg={};imports.wbg.__wbindgen_object_drop_ref=function(arg0){takeObject(arg0);};imports.wbg.__wbg_value_fc1c354d1a0e9714=function(arg0,arg1){var ret=getObject(arg1).value;var ptr0=passStringToWasm0(ret,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc);var len0=WASM_VECTOR_LEN;getInt32Memory0()[arg0/4+1]=len0;getInt32Memory0()[arg0/4+0]=ptr0;};imports.wbg.__wbindgen_object_clone_ref=function(arg0){var ret=getObject(arg0);return addHeapObject(ret);};imports.wbg.__wbg_removeAttribute_1adaecf6b4d35a09=function(){return handleError(function(arg0,arg1,arg2){getObject(arg0).removeAttribute(getStringFromWasm0(arg1,arg2));},arguments)};imports.wbg.__wbg_createElement_d017b8d2af99bab9=function(){return handleError(function(arg0,arg1,arg2){var ret=getObject(arg0).createElement(getStringFromWasm0(arg1,arg2));return addHeapObject(ret);},arguments)};imports.wbg.__wbg_settextContent_07dfb193b5deabbc=function(arg0,arg1,arg2){getObject(arg0).textContent=arg1===0?undefined:getStringFromWasm0(arg1,arg2);};imports.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc=function(arg0){var ret=getObject(arg0)instanceof Window;return ret;};imports.wbg.__wbg_document_5edd43643d1060d9=function(arg0){var ret=getObject(arg0).document;return isLikeNone(ret)?0:addHeapObject(ret);};imports.wbg.__wbg_instanceof_HtmlInputElement_8969541a2a0bded0=function(arg0){var ret=getObject(arg0)instanceof HTMLInputElement;return ret;};imports.wbg.__wbg_nextElementSibling_5c8c4ce9e9005c1c=function(arg0){var ret=getObject(arg0).nextElementSibling;return isLikeNone(ret)?0:addHeapObject(ret);};imports.wbg.__wbg_setoninput_14a20ff951dac6ec=function(arg0,arg1){getObject(arg0).oninput=getObject(arg1);};imports.wbg.__wbg_children_8c75bed97e79b5f7=function(arg0){var ret=getObject(arg0).children;return addHeapObject(ret);};imports.wbg.__wbg_length_e7da05fb6ffe5b28=function(arg0){var ret=getObject(arg0).length;return ret;};imports.wbg.__wbg_remove_b67ae06e76683b10=function(arg0){getObject(arg0).remove();};imports.wbg.__wbg_self_e23d74ae45fb17d1=function(){return handleError(function(){var ret=self.self;return addHeapObject(ret);},arguments)};imports.wbg.__wbg_window_b4be7f48b24ac56e=function(){return handleError(function(){var ret=window.window;return addHeapObject(ret);},arguments)};imports.wbg.__wbg_globalThis_d61b1f48a57191ae=function(){return handleError(function(){var ret=globalThis.globalThis;return addHeapObject(ret);},arguments)};imports.wbg.__wbg_global_e7669da72fd7f239=function(){return handleError(function(){var ret=global.global;return addHeapObject(ret);},arguments)};imports.wbg.__wbindgen_is_undefined=function(arg0){var ret=getObject(arg0)===undefined;return ret;};imports.wbg.__wbg_newnoargs_f579424187aa1717=function(arg0,arg1){var ret=new Function(getStringFromWasm0(arg0,arg1));return addHeapObject(ret);};imports.wbg.__wbg_call_89558c3e96703ca1=function(){return handleError(function(arg0,arg1){var ret=getObject(arg0).call(getObject(arg1));return addHeapObject(ret);},arguments)};imports.wbg.__wbindgen_throw=function(arg0,arg1){throw new Error(getStringFromWasm0(arg0,arg1));};imports.wbg.__wbg_appendChild_3fe5090c665d3bb4=function(){return handleError(function(arg0,arg1){var ret=getObject(arg0).appendChild(getObject(arg1));return addHeapObject(ret);},arguments)};imports.wbg.__wbg_getwithindex_5caaba1b5b3e6e18=function(arg0,arg1){var ret=getObject(arg0)[arg1>>>0];return isLikeNone(ret)?0:addHeapObject(ret);};imports.wbg.__wbg_firstElementChild_f626b9fe89401ac6=function(arg0){var ret=getObject(arg0).firstElementChild;return isLikeNone(ret)?0:addHeapObject(ret);};imports.wbg.__wbg_setAttribute_1776fcc9b98d464e=function(){return handleError(function(arg0,arg1,arg2,arg3,arg4){getObject(arg0).setAttribute(getStringFromWasm0(arg1,arg2),getStringFromWasm0(arg3,arg4));},arguments)};imports.wbg.__wbg_getElementById_b30e88aff96f66a1=function(arg0,arg1,arg2){var ret=getObject(arg0).getElementById(getStringFromWasm0(arg1,arg2));return isLikeNone(ret)?0:addHeapObject(ret);};imports.wbg.__wbindgen_closure_wrapper72=function(arg0,arg1,arg2){var ret=makeMutClosure(arg0,arg1,11,__wbg_adapter_10);return addHeapObject(ret);};if(typeof input==='string'||(typeof Request==='function'&&input instanceof Request)||(typeof URL==='function'&&input instanceof URL)){input=fetch(input);}
const{instance,module}=await load(await input,imports);wasm=instance.exports;init.__wbindgen_wasm_module=module;return wasm;}
export default init;