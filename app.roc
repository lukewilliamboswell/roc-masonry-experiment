app "hello"
    packages { 
        pf: "platform/main.roc",
        json: "https://github.com/lukewilliamboswell/roc-json/releases/download/0.7.0/xuaMzXRVG_SEhOFZucS3iBozlRdObWsfKaYZMHVE_q0.tar.br",
     }
    imports [pf.Types.{ Bounds, Elem, Event }, json.Core.{jsonWithOptions}]
    provides [program] { Model } to pf

program = { init, update, render }

Model : { text : Str }

init : Bounds -> Model
init = \_ ->
    { text: "Hello, World!" }

update : Model, Event -> Model
update = \model, event ->
    when event is
        Resize _ -> { model & text: "Resized" }
        KeyDown _ -> { model & text: "KeyDown" }
        KeyUp _ -> { model & text: "KeyUp" }
        Tick _ -> model

render : Model -> List U8
render = \model ->
    root = FlexCol [
        Label {text: "Foo"},
        Label {text: model.text},
        Label {text: "Bar"}
    ]

    encoder = jsonWithOptions {
        fieldNameMapping: SnakeCase,
    }

    Encode.toBytes root encoder
    
