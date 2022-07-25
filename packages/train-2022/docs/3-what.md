---
layout: statement
---


# What are types?

<div class="relative w-2/3 h-[540px] m-auto pt-8 text-left">

<div v-click-hide class="absolute h-full">

> Types are a way to tell correct programs from incorrect before we run them by describing in our code how we plan to use our data.

</div>

<div v-after class="w-full h-full" >

<Youtube id="C5fr0LZLMAs" class="w-full h-full" />

</div>


</div>


<!--

类型是描述，在程序运行之前我们如何使用数据，来以此区分正确程序和错误程序。

类型可以是简单类型，也可以是我们问题领域建模出来的复杂数据结构

编程语言分为静态类型与动态类型

带有静态类型语言的程序，在编译时就需要知道变量的类型。声明一个变量，就要告诉编译器它是个什么类型

动态类型语言只有在运行时才知道变量类型。

-->

---

## Type Annotations

<div class="pt-8">

```ts {1-2|4|6-8|10-12}
let age: number;
age = 18;

const isTruthy = !!1;

function equals(x: number, y: number): boolean {
    return x === y;
}

function compose<T extends Function>(...funcs: Function[]): T {
    return funcs.reverse().reduce((a, b) => (...args) => b(a(...args)));
}
```

</div>

<div class="pt-8">

```ts {0|1-3|4-6}
function printPoint(point: { x: number; y: number; z?: number }) {
    console.table(point);
}
function printAnyhow(anyhow: Record<string | number | symbol, unknown>) {
    console.table(anyhow);
}
```

</div>

<!--

当你使用const let var 声明变量的时候，我们可添加类型注解来制定这个变量的类型.
但在大多数情况下，你都没必要定义类型，ts编译器会自动帮你推导出来

下面是函数参数和返回值的类型注解。
和变量类型注解一样，在大多数情况下你都不需要添加返回值类型，ts会自动根据代码中的`return`语句推导出返回值类型。

定义对象类型的参数，只需简单列举他的属性和类型，属性之间使用 分号或者逗号都可以 可选属性使用`?`
另外一种定义对象类型的方式，使用内置工具类型`Record`或者 `key in`语法，后面会提到

-->

---
layout: two-cols
---

# Types Overview

<div class="mt-8" />

- null
- undefined
- boolean
- bigint
- number
- string
- symbol

::right::

<div class="mt-16" />

- any
- unknown
- never
- void(a function which returns `undefined` or has no return value)

- interface
- type


---

# Unions

```ts
function isTruthy(value: true | false) {}
function isActive(value: 'active' | 'inactive' | 'idle') {}
```

<!--

联合类型
ts类型系统允许你从已有的一组类型中合并组成新的类型
-->

---
layout: two-cols
---

```ts
type MissingNo = 404;
type Location = {
    x: number;
    y: number;
}
type Data = [
    location: Location,
    timestamp: string,
]
type Size = "small" | "medium" | "large";
```

```ts
type Location = { x: number } & { y: number }
type Response = { data: {} };
type Data = Response['data'];
```

```ts
const data = {};
const Data = typeof Data;
```
```ts
const createFixtures = () => ({});
type Fixtures = ReturnType<typeof createFixtures>;
```

::right::


<div class="pl-5">

```ts
type Artist = { name: string; bio: string; }
type Subscriber<Type> = {
    [Property in keyof Type]: (newValue: Type[Property]) => void;
}
type ArtistSub = Subscriber<Artist>;
```

```ts
type HasFourLegs<Animal> = Animal extends { legs: 4 } ? Animal: never;
type Animals = Bird | Dog | Ant | Wolf;
type FourLegs = HasFourLegs<Animal>;
```

```ts
type SupportedLangs = 'en' | 'pt' | 'zh';
type FooterLocaleIDs = 'header' | 'footer';

type AllLocaleIDs = `${SupportedLangs}_${FooterLocaleIDs}`;
```

</div>

<!--
类型别名使用一种简单的方式定义类型。
可以用它来定义类型，并在多处复用

可以用它给一组类型一个有意义的名字。

定义元组类型、联合类型、交叉类型、类型索引

使用typeof 操作符获取js运行时值的类型
-->

---

# Interfaces

```ts
interface JSONResponse extends Response, HTTPAble {
    version: number;
    payloadSize: number;

    update: (retryTimes: number) => void;
    update(retryTimes: number): void;

    (): JSONResponse;

    new(s: string): JSONResponse;

    [key: string]: number;
    readonly body: string;
}

interface APICall<Response extends { status: number }> {
    data: Response;
}

interface Expect {
    (matcher: boolean): string;
    (matcher: string): boolean;
}
```

<!--
interface 是另一种定义对象类型数据的方式


可执行的interface（[[call]]、[[constructor]]）

定义多个interface会合并
-->

---
layout: two-cols
---


# Interface

<div class="pr-6">

```ts
interface Animal {
  name: string
}

interface Bear extends Animal {
  honey: boolean
}

const bear = getBear()
bear.name
bear.honey
```
</div>

::right::

# Type

```ts
type Animal = {
  name: string
}

type Bear = Animal & {
  honey: boolean
}

const bear = getBear();
bear.name;
bear.honey;
```

<!--

interface 和 type 继承的方式不一样

一个type不能被重复定义，但interface可以

-->

---

# Type Assertions

```ts {1|2-4|5|6|8-10}
const canvasEle = document.getElementById('a-canvas') as HTMLCanvasElement;
const handleCellTouched = (e: MouseEvent) => {
    const cell = e.target as HTMLTableCellElement;
}
const a = 1 as unknown as string;
const radioEleList = document.querySelectorAll<HTMLSelectElement>('input[type=radio]');

function getName(user: { name?: string }): string {
    return user.name!;
}
```


<!--
类型断言: 当ts推断的类型是不准确的时候，可以使用它指定一个确切的类型
例如获取canvas元素，你知道一定是canvas但返回类型是 HTMLElement | null

有时你想强制转换类型，这时候可以将它推断为顶级类型如any/unknown，在推断为你想要的类型

也有些情况下类型会提供范型给你

还有一种特殊的情况，非空断言。
-->

---

# Literal Types

```ts
const PLATFORM = 'LINUX';
let name: 'Lily' = 'Lily';
// name = 'Lisa'; // ERROR

type LiteralTypes = 'string' | 100 | false;
const aList = ['A', 'B', 'C'] as const;
```

<!--
字面量类型
如果一个类型能确定一个类型，且不会改变，那么他的类型会被作为字面量类型

`as const` 后缀与js中的const功能相同，但作用与类型系统，它将确保转换为字面量类型
-->

---
layout: image-left
image: https://images.unsplash.com/photo-1592285733872-b70552ecf665?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=774&q=80
---

# Enums

<div>

> Enums are a feature added to JavaScript by TypeScript which allows for describing a value which could be one of a set of possible named constants. Unlike most TypeScript features, this is not a type-level addition to JavaScript but something added to the language and runtime. Because of this, it’s a feature which you should know exists, but maybe hold off on using unless you are sure.


```ts
enum Direction {
  Up = 1,
  Down,
  Left,
  Right,
}
```

```js
"use strict";
var Direction;
(function (Direction) {
    Direction[Direction["Up"] = 1] = "Up";
    Direction[Direction["Down"] = 2] = "Down";
    Direction[Direction["Left"] = 3] = "Left";
    Direction[Direction["Right"] = 4] = "Right";
})(Direction || (Direction = {}));
```

</div>


<!--
枚举类型与其他类型不同，他在运行时会被添加，因此，在大多数情况下你都不应该再使用改类型(ts官方建议)
-->

---

# Generics

```ts {1|2|3-5|6-13|14-16|17-22|23}
type StringArray = Array<string>;
type ObjectWithNameArray = Array<{ name: string }>;
function identify<T> (arg: T) {
    return arg;
}
interface HttpResponse<T = unknown> {
    status: number;
    data: {
        code: number;
        message: string;
        result?: T
    };
}
interface GenericIdentityFn {
    <T>(arg: T): T;
}
class DragableObject { }
class Component<T extends Component> {
    draw(arg: T) {
        // ...
    }
}
const getProperty = <T, K extends keyof T>(obj: T, key: K) => obj[key];
```

<!--

我们可以使用范型来创建可重用的类型

如上代码展示了如何使用范型，使用尖括号明确传入的类型，但多数情况下，可以自动推导出范型类型。

定义范型时，可以传入范型默认值

我们也可以使用范型约束来约束范型类型，当约束后它不再适用于任何类型

在范型约束中，也是可以使用类型参数的，如上所示

-->

---

# Structural Type (Duck Typing)

```ts {1-17|18-23}
interface Point {
    x: number;
    y: number;
}

class VirtualPoint {
    constructor(public x: number, public y: number) {}
}

function printPoint(point: Point) {
    console.table(point);
}

const point1 = { x: 1, y: 1 };
const point2 = new VirtualPoint(1, 1);
printPoint(point1);
printPoint(point2);

type Coordinate1 = { x: number; y: number };
type Coordinate2 = { x: number; y: number; z: number };
const printCoordinate = <T extends Coordinate1>(v: T) => console.log(v);
printCoordinate({ x: 1, y: 2 });
printCoordinate({ x: 1, y: 2, z: 3 });
```

<!--
鸭子类型：如果类型A有类型B所有需要的属性，那么它可以作为类型B去使用!
-->

---

# [Type Compatibility](https://www.typescriptlang.org/docs/handbook/type-compatibility.html#subtype-vs-assignment)

```ts
interface Pet {
    name: string;
}
class Dog {
    name!: string;
}
let dog: Pet = new Dog;
```

<v-click>

```ts
interface Pet {
    name: string;
}
let dog = { name: 'Julia', owner: 'Michael' }

function greet(pet: Pet) {/* ... */}
greet(dog);
```

</v-click>
<v-click>

```ts
let x = (a: number) => 0;
let y = (a: number, b: number) => 0;

y = x; // Yep
x = y; // Nope
```

</v-click>
<v-click>

```ts
let x = () => ({ x: number, y: number });
let y = () => ({ x: number, y: number, z: number });
x = y; // Yep
y = x; // Nope
```

</v-click>

<!--
类型兼容性：结构类型是一种仅基于类型成员来关联类型的方法。兼容性包含两方面，赋值兼容性与子类型兼容性

ts是一种基于结构的类型系统

在编译器执行时，为了检查是否能将Dog分配给Pet，编译器会检查Pet每一个属性是否在dog上有与之对应的属性，在上面的例子中，Dog必须有name这个成员并且类型为string

虽然上面的例子中，Dog类型有一个owner属性，只要有目标类型的所有成员，即认为类型可兼容

函数参数的类型兼容性，正好与之相反。
在x的每一个参数类型，必须在y中有与之对应的，所以第一个是正确的。(在js中可选参数是很普遍的，所以x的参数数量可以小于y的参数数量，但多于就不行了)

ts的类型系统强制要求，必须是目标函数的返回值类型的子类型

-->

---

# Class Members

```ts
class Customer {}

class Pallet {}

class Greeter {
    readonly name = 'Lily';

    #id = 0
    // private id = 0;

    // Overloads
    constructor(customer: Custom);
    constructor(pallet: Pallet) {}

    get id() {
        return this.#id;
    }

    set id(value) {
        this.#id = value
    }

    sayHi() {
        console.log('Hi, ' + this.name);
    }
}
const greeter: Greeter = new Greeter();
class G implements Greeter {} // Nope
```

<!--
ts的class支持所有最新的js类语法

ts提供了private 前缀来声明私有属性或方法，但只是类型的额外声明，不作用于运行时。实际还是能使用该私有属性

#开头的属性不会在类以外被使用。所以private和枚举一样，都不推荐使用!

一个类可以既作为类型也作为值，但不要这样使用
-->

---

# Class Heritage

```ts
interface Pingable {}
interface Pongable {}
class Radar {}
class Sonar extends Radar implements Pingable, Pongable {}
```

<!--
类继承

可以继承一个类，使用extends关键字
实现多个接口。使用implements关键字

-->

---

# Member Visibility

```ts
class Account {}
class User extends Account {
    #attributes: Map<any, any>;
    name!: string;
    roles = ['user'];
    readonly createAt = Date.now();

    constructor(public id: string, public email: string) {
        super(id);
    }

    verifyName = (name: string) => {}

    sync(): Promise<boolean>;
    sync(cb: ((res: string) => void)): void;
    sync(cb?: ((res: string) => void)): void | Promise<boolean> {
        // ...
    }

    public greet() {
        console.log(`Hi, ${this.name}, I'm a ${Greeter.alias}!`);
    }

    private makeRequest() {}

    // only accessible within class 'Greeter' and its subclasses.
    protected getName() {
        return this.name + this.#petName;
    }

    // invalid properties: name/length/call
    static alias = 'receptionist';
    static #counter = 0;

    // static block
    static {
        try {
            Greeter.#counter = 100;
        } catch (error) {}
    }
}
```

<!--

下面我们来看下如何定义类成员的可见性

属性默认为public
#private/private 只允许当前class访问
protected 允许当前class和子class访问

-->

---

# Abstract classes & members

```ts
abstract class Base {
    abstract getName(): string;

    getAge() {
        return 18;
    }
}
class Derived extends Base {
    getName() {
        return 'Sid';
    }
}
function greet(ctor: new () => Base) {
    const instance = new ctor();
    instance.getName();
}
// greet(Base);
greet(Derived);
```

<!--
抽象类是一个无法被实例化的特殊class

可以定义抽象方法，或者给某个方法添加默认实现
-->

---

# [Decorators](https://github.com/tc39/proposal-decorators) and Attributes

```ts
import {
    Syncable,
    triggersSync,
    preferCache,
    required
} from 'mylib';
@Syncable
class User {
    @triggersSync()
    save() {
        // ...
    }

    @preferCache(false)
    get displayName() {
        // ...
    }

    update(@required info: Partial<User>) {
        // ...
    }
}
```

<!--
上面展示了装饰器的用法，如何自定义装饰器。大家可课下去了解🫡
-->

---
layout: two-cols
---

# Narrowing

```ts
function handleInput(input: string | number | boolean | symbol | Date | { id: number }) {
    if (typeof input === 'string') {/** ... */}
    else if (input instanceof Date) {/** ... */}
    else if ('id' in input) {/** ... */}
    else if (Array.isArray(input)) {/** ... */}
}

type Response =
    | { statis: 200, data: any }
    | { status: 301, to: string }
    | { status: 400, error: Error }

function isElement(node: Node): node is Element {
    return node instanceof Element;
}
function assertElement(obj: any): asserts obj is Element {
    if (!(obj instanceof Element)) {
        throw new Error("Not a Element");
    }
}
```

::right::

<div class="pt-13 pl-4">

```ts
const data1 = {
    name: 'Misky'
};
const data2 = {
    name: 'Misky'
} as const;
```

```ts
type Shape = Circle | Square;

function getArea(shape: Shape) {
    switch (shape.kind) {
        case "circle":
            return Math.PI * shape.radius ** 2;
        case "square":
            return shape.sideLength ** 2;
        default:
            const _exhaustiveCheck: never = shape;
            return _exhaustiveCheck;
    }
}
```

</div>

<!--
类型收窄

使用if 语法，在if 语句块内能获取到正确的类型
可通过 typeof / instanceof / in

对于联合类型，某些情况下可通过 switch...case 语法

还可以使用类型守卫函数、断言函数

-->

---

# [Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html)
