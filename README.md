# Rend-ox

/!\ Under heavy developpement /!\

Rust basic 3D rendering librairy.
As long as the Nannou dependency is used for setting up WGPU and Winnit, Rendox is just a layer on top of Nannou to add 3D Rendering. The end goal is to remove Nannou completely and use WGPU and Winnit directly.

## Getting Started

### Setup

Add the depedency to your cargo file:

```
rend_ox = { git = "https://github.com/Iddeko/Rend-ox.git", branch = "main" }
```

Here is the basic setup for a rend-ox project:

```rust
struct Model {  // Struct holding game states and ressources

}

fn create_app(nannou_app: &rend_ox::nannou::App) -> App<Model> {
    app(nannou_app, Model{})
        .update(...)        // function executed each frame
        .key_pressed(...)   // function executed when the key is pressed
        .event(...)         // function executed when the nannou event is recieved
}

fn main() {
    rend_ox::app::launch_rendox_app(create_app);
}
```

Check the [examples](./examples/) folder for more in depth examples

## Contributing

The library does not have the immediate goal of continuing development, however any addition is welcome, do not hesitate to make a pull-request

## Authors

See also the list of contributors to find who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for
details
