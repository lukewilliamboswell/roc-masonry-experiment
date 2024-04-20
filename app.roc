app "hello"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf

main = {
    title : "FRUIT",
    resizable : Bool.true,
}
