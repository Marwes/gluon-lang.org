'use strict';

require('./index.html');
require('./styles.scss');

import { runExpr } from './gluon_wasm'
import { booted } from './gluon_wasm_wasm'

booted.then(() => {
    var Elm = require('./Main.elm');
    var mountNode = document.getElementById('main');


    var app = Elm.Main.embed(mountNode);

    app.ports.runExpr.subscribe(function(word) {
        let result = runExpr(expr)
        app.ports.suggestions.send(result);
    });
});
