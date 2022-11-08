# RFC-1: Scene File Definition

In this first iteration of the project (`v1`), `2dcl` uses a static serialization of scenes compiled in a file called `scene.2dcl`. 

`scene.2dcl` is compiled from a hand-written JSON file (`scene.json`) using [MessagePack](https://msgpack.org/).

We provide an reference implementation in Rust, but, if you wanted to create a compiler in another language, you should be able to do so with the information below.

For a rich example of a scene with multiple entities and components, check the [sample scene](https://github.com/hiddenpeopleclub/2dcl-sample-scene) we created.

## `scene.json`

The structure of the `scene.json` file is pretty simple, it includes only three attributes: `name` (name of the scene), `levels` (an array of different levels available in the scene), and `parcels` (the list of all the parcels that this scene contains).

Parcels use the same serialization used for the 3d explorer: a string of comma separated integers (`"0,0"`).

```json
{
  "name": "My Awesome Scene",
  "levels": [
    // See levels below
    // ... 
  ],
  "parcels": [
    "0,1",
    "1,1"
  ]
}
```


## Levels

A single scene can include multilpe levels, each level contains a `name`, `dimensions` (the size in pixels for the level), and `entities` (an array of entities available in that level, see Entities below), and `player_layer` that determines in which z-order layer the player should be rendered in this level.

The first level in the scene is the one that gets rendered when walking around decentraland, and its dimensions get automatically set by the parcels available, so anything outside the boundaries of the parcels will be ignored.

```json
{
  "name" : "My Level Name",
  "dimensions" : { "x" : 200, "y" : 200 }, // optional, defaults to the parcel settings in the scene
  "player_layer" : 5, // optional, defaults to 0
  "entities" : [
    // See entities below
    // ...
  ]
}
```

## Entities

As the 3d explorer does, we use an Entity Component System (ECS), because we think that long term this will be the most extensible architecture for the project.

That's why our scene serialization includes an array of entities.

Each entity is defined by an object with two attributes: `name` (name of the entity), and `components` (an array of components).

```json
{
  "name": "My Entity",
  "components": [
    // See components below
    // ...
  ] 
}
```

## Components

Each component you attach to an entity gives some features to it. For now we're just supporting a small set of components: `BoxCollider`, `SpriteRenderer`, `SpriteSheetRenderer` and `Transform`.

We're open to discussing with the community which components we all would like to see in the future. If we add additional components, we'll update this document to include them.


### SpriteRenderer

A `SpriteRenderer` component can be attached to an entity to render a static sprite (for animated sprites see `SpriteSheetRenderer`).

The attributes available are:
 - `sprite`: the path to the image file you want to render, it should be placed in the `assets` folder.
 - `color`: a color applied to the asset using a normalized (from `0` to `1`) RGBA format.
 - `layer`: a z-ordering layer that lets you decide what goes above what. Use the level's `player_layer` to determine which layers are below the player and which ones are above.
 - `flip`: used to determine if the asset should be flipped on the `x` or `y` axes.
 - `anchor`: This is used to determine where the center of the asset lies on. This will affect rotations and, in the case the sprite is in the same layer as the player, where it starts rendering below/above the player using the `y` coordinate. See `Anchor` below for available options.
 - `blend_mode`: Used to decide how the asset is blended with pixels existing below it. See `Blend Modes` below.

```json
{
  "sprite": "a_pixel.png",
  "color": { "r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0 }, // optional, defaults to no coloring (white)
  "layer": 0, // optional, defaults to `player_layer` in level
  "flip": { "x": true, "y": false }, // optional, defaults to false
  "anchor": "Center", // optional, defaults to "BottomCenter"
  "blend_mode" : "Multiply" // optional, defaults to "AlphaBlend"
}
```


### SpriteSheetRenderer

TODO: ???


### BoxCollider
In order to avoid the player to walk on top of things we can use colliders. A `BoxCollider` is a bounding box with a `center` and `size`.

It has a `center` (`x` and `y` are pixel coordinates) and a `size` (with `width` and `height` in pixels).

```json
{
  "type": "BoxCollider",
  "center": { "x" : 1, "y" : 0 }, // optional, defaults to 0,0
  "size": { "width" : 1, "height" : 2 } // optional, defaults to 1,1
}
```

### Transform
```json
{
  "type": "Transform",
  "location": [ 0.0, 0.0 ],
  "rotation": [ 90.0, 90.0, 90.0 ], // optional, defaults to (0,0,0)
  "scale": [ 1.0, 1.0 ] // optional, defaults to (1,1)
}
```

### Anchor

For rotation and z-ordering purposes, we can set the anchor of a sprite. Valid values for anchor are:
 - `Center`,
 - `BottomLeft`,
 - `BottomCenter`,
 - `BottomRight`,
 - `CenterLeft`,
 - `CenterRight`,
 - `TopLeft`,
 - `TopCenter`,
 - `TopRight`,
 - `{ "x": 0, "y: 0}` for custom anchor, `x` and `y` are pixels using the center of the asset as the origin coordinate.

### BlendModes

When rendering a sprite, the graphic card gives us multiple ways of merging existing pixel colors with the ones we're adding.

The options are:
 - `Add`: additive, the existing pixel is added to the new one.
 - `AlphaBlend`: the pixels are added using the alpha channel to reduce the opacity.
 - `Multiply`: bot pixels are multiplied.
 - `{ "color": { "src": "One", "dst": "One" }, "alpha": { "src": "One", "dst": "One" } }`: This is a custom blend state for advanced users. You can use this to set custom blend factors.

 Options for custom blend factors:
  - Zero,
  - One,
  - Src,
  - OneMinusSrc,
  - SrcAlpha,
  - OneMinusSrcAlpha,
  - Dst,
  - OneMinusDst,
  - DstAlpha,
  - OneMinusDstAlpha,
  - SrcAlphaSaturated,
  - Constant,
  - OneMinusConstant,

### Full Example

You can check our [Sample Scene](https://github.com/hiddenpeopleclub/2dcl-sample-scene) for examples of all these components in action.
