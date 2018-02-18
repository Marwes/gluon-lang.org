'use strict';

const gluon = {
    gluon_wasm: import('./gluon_wasm'),
    gluon_wasm_wasm: import('./gluon_wasm_wasm')
};

var Elm = require('./Main.elm');
var mountNode = document.getElementById('main');


var app = Elm.Main.embed(mountNode);

app.ports.runExpr.subscribe(function(expr) {
    gluon.gluon_wasm.then((gluon_wasm) => {
        gluon.gluon_wasm_wasm.then((wasm) => {
            wasm.booted.then(() => {
                let result = gluon_wasm.run_expr(expr);
                return app.ports.runExprResult.send(result);
            });
        });
    });
});

