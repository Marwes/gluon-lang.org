port module Gluon exposing (..)


port runExpr : String -> Cmd msg


port runExprResult : (String -> msg) -> Sub msg
