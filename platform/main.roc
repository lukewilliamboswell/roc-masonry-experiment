platform "masonry-experiment"
    requires { Model } { program : _ }
    exposes [Types]
    packages {}
    imports [Types.{ Bounds, Elem, Event }]
    provides [mainForHost]

# We box the model before passing to the Host and unbox when passed to Roc
ProgramForHost : {
    init : Bounds -> Box Model,
    update : Box Model, Event -> Box Model,
    render : Box Model -> { root : List U8, model : Box Model },
}

init : Bounds -> Box Model
init = \bounds -> Box.box (program.init bounds)

update : Box Model, Event -> Box Model
update = \boxedModel, event -> Box.box (program.update (Box.unbox boxedModel) event)

render : Box Model -> { root : List U8, model : Box Model }
render = \boxedModel -> { root: program.render (Box.unbox boxedModel), model: boxedModel }

mainForHost : ProgramForHost
mainForHost = { init, update, render }