
#[derive(Clone, Debug)]
enum TypeInfo {
	Item,
	Event,
	Property,
	Component,
	String,
	Integer,
	Double,
	Boolean,
	Index,
	Object,
	Enum,
	Vector,
	Value,
}

#[derive(Clone, Debug, Default)]
struct Event;

#[derive(Clone, Debug)]
struct Property {
	value: ValueContainer,
}

#[derive(Clone, Debug, Default)]
struct Index;

#[derive(Clone, Debug)]
struct Item{
	id: String,
	width: Property,
	length: Property,
	x: Property,
	y: Property,
	value: ValueContainer,
	event: Event,
	css: String,
	system_type: String,
}

#[derive(Clone, Debug)]
struct ValueContainer {
	type_info: TypeInfo,
	value: Value,
}

#[derive(Clone, Debug)]
enum Value {
	String(String),
	Integer(i64),
	Double(f64),
	Boolean(bool),
	TypeInfo(TypeInfo),
	Index(Index),
	Vector(Vec<Box<ValueContainer>>),
}

#[derive(Clone, Debug)]
enum Component {
	Object,
	Item(Item),
	Event(Event),
	Property(Property),
}


#[derive(Clone, Debug, Default)]
struct Node {
	component : Vec<Box<Component>>,
	children : Vec<Box<Node>>,
}



fn main() {
    let p = include_str!("test.fui");
    println!("{}",p);

    let mut c = Node::default();
    c.component.push ( Box::new( Component::Item( Item{ id: "mainWindow".to_owned() } ) ));    

    println!("{:#?}",c);

}

