---
class: text-white bg-black65
layout: two-cols
title: What is [TypeScript](https://www.typescriptlang.org/)?
---

<div class="relative w-full h-full pr-8">

<div v-click-hide class="absolute h-full">
<img class="h-full text-center" src="https://images.unsplash.com/photo-1608306448197-e83633f1261c?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=687&q=80" >
</div>

<div class="mb-4" v-after>

```js
function compact(arr) {
    if (orr.length > 10) return arr.trim(0, 10);
    return arr;
}
```

</div>

<div class="mb-4" v-click>

```js
// @ts-check
function compact(arr) {
    if (orr.length > 10) return arr.trim(0, 10);
    return arr;
}
```

</div>

<div class="mb-4" v-click>

```js
// @ts-check

/** @param {any[]} arr */
function compact(arr) {
    if (arr.length > 10) return arr.trim(0, 10);
    return arr;
}
```

</div>

<div class="mb-4" v-click>

```ts
function compact(arr: string[]) {
    if (arr.length > 10) return arr.slice(0, 10);
    return arr;
}
```

</div>

</div>

::right::

# What is [TypeScript](https://www.typescriptlang.org/)?

> TypeScript is a typed superset of JavaScript [...]

<div class="py-2 text-white/80">Or</div>

> TypeScript is JavaScript with syntax for types.

<ul class="mt-4">
    <li>
        JavaScript and More
        <br>
        <quote class="text-white/70 text-xs">
        TypeScript adds additional syntax to JavaScript to support a tighter integration with your editor. Catch errors early in your editor.
        </quote>
    </li>
    <li class="pt-4">
        A Result You Can Trust
        <br>
        <quote class="text-white/70 text-xs">
        TypeScript code converts to JavaScript, which runs anywhere JavaScript runs: In a browser, on Node.js or Deno and in your apps.
        </quote>
    </li>
    <li class="pt-4">
        Safety at Scale
        <br>
        <quote class="text-white/70 text-xs">
        TypeScript understands JavaScript and uses type inference to give you great tooling without additional code.
        </quote>
    </li>
</ul>

<!--
什么是 TypeScript？

ts官方对自身的定位转变：
ts是js的超集 vs ts是带类型语法的js


如何描述它呢？ --〉 摘自官网

1. 与编辑器深度集成，实时显示可能的类型错误
2. ts转换为js，支持各种环境
   1. 浏览器：babel/webpack/rollup...
   2. Node.js: 自定义加载器 ts-node/esm [示例](https://github.com/nodejs/loaders-test/blob/67d0dc2c27e3137c4601c1ee0e96aad0f85fb4e6/typescript-loader/loader.js)
   3. Deno/Bun: ts作为一等语言

总而言之，ts就是带静态类型的js！

下面从ts官网的例子来看ts与js的关系
-->
