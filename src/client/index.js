'use strict';

require('./index.html');
require('./styles.scss');

import { run_expr } from './gluon_wasm'
import { booted } from './gluon_wasm_wasm'

booted.then(() => {
    var Elm = require('./Main.elm');
    var mountNode = document.getElementById('main');


    var app = Elm.Main.embed(mountNode);

    app.ports.runExpr.subscribe(function(expr) {
        let result = run_expr(expr);
        app.ports.runExprResult.send(result);
    });
});
