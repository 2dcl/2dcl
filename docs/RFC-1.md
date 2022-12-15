# RFC-1: Scene File Definition

In this first iteration of the project (`v1`), `2dcl` uses a static serialization of scenes compiled in a file called `scene.2dcl`. 

`scene.2dcl` is compiled from a hand-written JSON file (`scene.json`) using [MessagePack](https://msgpack.org/).

We provide an reference implementation in Rust, but, if you wanted to create a compiler in another language, you should be able to do so with the information below.

For a rich example of a scene with multiple entities and components, check the [sample scene](https://github.com/hiddenpeopleclub/2dcl-sample-scene) we created.

## `scene.json`

The structure of the `scene.json` file is pretty simple, it includes only two attributes: `name` (name of the scene)and `levels` (an array of different levels available in the scene)

```json
{
  "name": "My Awesome Scene",
  "levels": [
    // See levels below
    // ... 
  ]
}
```

## Levels

A single scene can include multilpe levels, each level contains a `name`, `dimensions` (the size in pixels for the level), `entities` (an array of entities available in that level, see Entities below), a `spawn_point` (the location where a player appears when teleporting to the level) and `player_layer` that determines in which z-order layer the player should be rendered in this level.

The first level in the scene is the one that gets rendered when walking around decentraland, and its dimensions get automatically set by the parcels (each parcel is 500x500) available, so anything outside the boundaries of the parcels will be ignored.

```json
{
  "name" : "My Level Name",
  "dimensions" : { "x" : 200, "y" : 200 }, // optional, defaults to the parcel settings in the scene
  "player_layer" : 5, // optional, defaults to 0,
  "spawn_point": { "x" : 50, "y" : 50 } // optional, defaults to { "x" : 0, "y" : 0 }
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

An entity also has `children`, which are entities that inherit the `Transform` component of the parent.

```json
{
  "name": "My Entity",
  "components": [
    // See components below
    // ...
  ],
  "children" : [
    // Child entities
    //... 
  ]
}
```

## Components

Each component you attach to an entity gives some features to it. For now we're just supporting a small set of components.

We're open to discussing with the community which components we all would like to see in the future. If we add additional components, we'll update this document to include them.

### Transform
The `Transform` component lets you set the `location` (in pixels), `rotation` (in degrees) and `scale` (as a multiplicative factor) of your entity.

```json
{
  "type": "Transform",
  "location": { "x" : 0, "y": 0 },
  "rotation": { "x" : 90.0, "y": 90.0, "z" : 90.0 }, // optional, defaults to (0,0,0)
  "scale": { "x" : 1.0, "y": 1.0 } // optional, defaults to (1.0,1.0)
}
```

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
  "type" : "SpriteRenderer",
  "sprite": "a_pixel.png",
  "color": { "r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0 }, // optional, defaults to no coloring (white)
  "layer": 0, // optional, defaults to `player_layer` in level
  "flip": { "x": true, "y": false }, // optional, defaults to false
  "anchor": "Center", // optional, defaults to "Center"
  "blend_mode" : "Multiply" // optional, defaults to "AlphaBlend"
}
```

### BoxCollider
In order to avoid the player to walk on top of things we can use colliders. A `BoxCollider` is a bounding box with a `center` and `size`.

It has a `center` (`x` and `y` are pixel coordinates) and a `size` (with `width` and `height` in pixels).

You can make this collider `Solid` (prevent the player from moving across it), or `Trigger` (player can walk across it) by setting the `collision_type`.

```json
{
  "type": "BoxCollider",
  "collision_type" : "Trigger", // optional, defaults to Solid
  "center": { "x" : 1, "y" : 0 }, // optional, defaults to 0,0
  "size": { "width" : 1, "height" : 2 } // optional, defaults to 1,1
}
```

### CircleCollider
The `CircleCollider`, has a `center` and a `radius` (in pixels) to define the boundaries where it collides.

You can make this collider `Solid` (prevent the player from moving across it), or `Trigger` (player can walk across it) by setting the `collision_type`.

```json
{
  "type": "CircleCollider",
  "collision_type" : "Trigger", // optional, defaults to Solid
  "center": { "x" : 1, "y" : 0 }, // optional, defaults to 0,0
  "radius": 2 // optional, defaults to 1
}
```

### MaskCollider
The `MaskCollider` is used to create a pixel perfect collision using one channel of an image. You can usually just use the `alpha` channel of a sprite for this.
You can set the `sprite` to be used, which color `channel`, and the `anchor`. Characters will not collide with pixels set to 0, but will collide to pixels set to anything else.

You can make this collider `Solid` (prevent the player from moving across it), or `Trigger` (player can walk across it) by setting the `collision_type`.

```json
{
  "type": "MaskCollider",
  "collision_type" : "Trigger", // optional, defaults to Solid
  "sprite": "a_pixel.png",
  "channel": "R", // Optional, defaults to 'A' (for the alpha channel)
  "anchor": "Center" // Optional, defaults to "BottomCenter"
}
```


### LevelChange

When a player interacts with an entity with a `LevelChange` component, they get teleported to that level.

The attributes are `level` (the name of the level), and `spawn_point` (the `x`,`y` coordinate where the player should appear in the level).

```json
{
  "type": "LevelChange",
  "level": "MyLevel",
  "spawn_point" : { "x": 0, "y": 0 }
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
 - `{ "Custom" : { "x": 0, "y: 0} }` for custom anchor, `x` and `y` are pixels using the center of the asset as the origin coordinate.

### BlendModes

When rendering a sprite, the graphic card gives us multiple ways of merging existing pixel colors with the ones we're adding.

The options are:
 - `Add`: additive, the existing pixel is added to the new one.
 - `AlphaBlend`: the pixels are added using the alpha channel to reduce the opacity.
 - `Multiply`: bot pixels are multiplied.
 - `{ "Custom" : { "color": { "src": "One", "dst": "One" }, "alpha": { "src": "One", "dst": "One" } } }`: This is a custom blend state for advanced users. You can use this to set custom blend factors.

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
