CargoQueryServer
================
2015/06/01
Hoon H.

Prints value for a path from `Cargo.toml`.
Written to be used from Eonil Editor.


Example
-------

Type this to query package name from `Cargo.toml` in same directory.

    $ CargoQueryServer package.name

The command prints this for case of this project.

    CargoQueryServer

Query-path will be passed to [toml-rs library's `lookup` function](http://alexcrichton.com/toml-rs/toml/enum.Value.html#method.lookup) as is. See there for more details.

Currently, this supports only string type values, and crash for any other type values.




License
-------
"MIT License".

Also, this is my first "proper" Rust program. Celerate me!
