# TagMap

A small program that converts Rust's ML-style [sum types](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) into C-style
[tagged unions](https://en.wikipedia.org/wiki/Tagged_union).

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="display: flex; align-items: center;">
        <img src="./static/rust.png" style="width: 300px;">
        <span style="font-size: 50px;"> → </span>
        <img src="./static/c.png" style="width: 300px;">
    </div>
</div>

## Dependencies

For optional formatting of the generated C code, install [indent](https://www.gnu.org/software/indent/).
