platform "masonry-experiment"
    requires {} { main : _ }
    exposes [Types]
    packages {}
    imports []
    provides [mainForHost]

mainForHost : Str
mainForHost = main.title