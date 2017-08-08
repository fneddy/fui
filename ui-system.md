# Types

## DSL

Component
	- children() -> Vector<Item> 

Event
	parent: Item
	eventType: ?
	eventValue: Value

Property
	parent: Item
	value: Value
	- bindOnChange (o: Object)
	- bindProperty (p: Property)

Item
    parent: Item
	visible: Property
	enabled: Property
	text: Property
	css: Property
	active: Property
	- onClick(e: Event)
	- onKeyPress(e: Event)
	- onKeyRelease(e: Event)
	- children() -> Vector<Item> 


String

Integer

Double

Boolean
	
TypeInfo

?? Index


Object
	?? parent: Item
    - onChange(e: Event) 
	- getValue(i: Index) -> Value
	- call(method: String)
	
Vector: < String, Integer, Double, Boolean, TypeInfo, Index, Vector >
	?? parent: Item
    - get() -> infer
	- set(v: infer)
	- del(i: Integer)

Value: < String, Integer, Double, Boolean, TypeInfo, Index, Vector >
	?? parent: Item
    - get() -> infer
	- set(v: infer)


### Object
Is a rust struct that imp the `Object trait`. An `Object` can not be created from `DSL` but pushed from rust at creation of a component.

### Component
It's a `file`. A logical unit. Everything inside an `Component` has the same lifetime. At creation time all Objects need to be passed. The while `AST` will be parsed and created.


## Rust

Context
- loadFromString(s: String)
- loafFromPath(p: Path)
- getComponent() -> Component
- getItem(path: String) -> json.path? 

__('a of Context)__:

Component
Event
Index
Property

Trait Object
- onChange(Event)
- getValue(i: i32) -> Value
- call(method: String) 

vector -> vec? 
Value -> TypeInfo<String, Integer, Double, Boolean, TypeInfo, Index, vec>



### Mappings
__DSL__ -> __RUST__
Context -> Context
Component -> Component
Event -> Event
Index -> Event
Property -> Property
Trait Object -> Object 
String -> String 
Integer -> i64 
Double -> f64 
Boolean -> bool 
TypeInfo -> enum 
vector -> vec? 
Value -> TypeInfo<String, Integer, Double, Boolean, TypeInfo, Index, vec>

# System
The complete UI system runs in one extra thread. The li

# AST

