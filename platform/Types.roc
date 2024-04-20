interface Types 
    exposes [Config, Rgba, Bounds, Elem, KeyCode, Event]
    imports []

Config : {
    title : Str,
    resizable : Bool,
}

Rgba : { r : F32, g : F32, b : F32, a : F32 }

Bounds : { height : F32, width : F32 }

Elem : [
    FlexCol (List Elem),
    FlexRow (List Elem),
    Label {text: Str},
]

KeyCode : [Left, Right, Other, Up, Down]

Event : [Resize { width : F32, height : F32 }, KeyDown KeyCode, KeyUp KeyCode, Tick U64]
