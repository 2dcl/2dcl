# RFC-1: Scene File Definition

In this first iteration of the project (`v1`), `2dcl` uses a static serialization of scenes compiled in a file called `scene.2dcl`. 

`scene.2dcl` is compiled from a hand-written JSON file (`scene.json`) using [MessagePack](https://msgpack.org/).

We provide an reference implementation in Rust, but, if you wanted to create a compiler in another language, you should be able to do so with the information below.

For a rich example of a scene with multiple entities and components, check the [sample scene](https://github.com/hiddenpeopleclub/2dcl-sample-scene) we created.

## `scene.json`

The structure of the `scene.json` file is pretty simple, it includes only three attributes: `name` (name of the scene), `entities` (an array of serialized entities), and `parcels` (the list of all the parcels that this scene contains).

Parcels use the same serialization used for the 3d explorer: a string of comma separated integers (`"0,0"`).

```json
{
  "name": "My Awesome Scene",
  "entities": [
    // See entities below
    // ... 
  ],
  "parcels": [
    "0,1",
    "1,1"
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

Each component you attach to an entity gives some features to it. For now we're just supporting a small set of components: `BoxCollider`, `SpriteRenderer`, and `Transform`.

We're open to discussing with the community which components we all would like to see in the future. If we add additional components, we'll update this document to include them.

### BoxCollider
```json
{
  "center": [ 0, 0 ],
  "size": [ 1, 1 ]
}
```

### SpriteRenderer
```json
{
  "sprite": "a_pixel.png",
  "color": [ 1.0, 1.0, 1.0, 1.0 ], // RGBA
  "layer": 0, // Z-Index
  "flip_x": false,
  "flip_y": false,
  "anchor": "Center"
}
```

Anchor:
```json
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    Custom([i16; 2]),
}
```

### Transform
```json
{
  "location": [ 0.0, 0.0 ],
  "rotation": [ 90.0, 90.0, 90.0 ],
  "scale": [ 1.0, 1.0 ]
}
```

## Full Example

Here you can see an example scene with a bunch of entities with common components.

```json
{
  "name": "My Awesome Scene",
  "parcels": [ "0,1" ],
  "entities": [
    {
      "name": "Floor",
      "components": [
        {
          "type": "SpriteRenderer",
          "sprite": "floor.png",
          "color": [ 1.0, 1.0, 1.0, 1.0 ],
          "layer": -2,
          "flip_x": false,
          "flip_y": false,
          "anchor": "Center"
        },
        { 
          "type": "Transform",
          "location": [ 0.0, 0.0 ],
          "rotation": [ 0.0, 0.0, 0.0 ],
          "scale": [ 1.0, 1.0 ]
        }
      ]
    },
    {
      "name": "Tree1",
      "components": [
        {
          "type": "SpriteRenderer",
          "sprite": "assets/Tree.png",
          "color": [ 1.0, 1.0, 1.0, 1.0 ],
          "layer": 0,
          "flip_x": false,
          "flip_y": false,
          "anchor": "BottomCenter"
        },
        { 
          "type": "Transform",
          "location": [ 88.0, 105.0 ],
          "rotation": [ 0.0, 0.0, 0.0 ],
          "scale": [ 1.0, 1.0 ]
        },
        {
          "type": "BoxCollider",
          "center": [ 0, 0 ],
          "size": [ 15, 15 ]
        }
      ]
    },
    {
      "name": "Tree2",
      "components": [
        {
          "type": "SpriteRenderer",
          "sprite": "assets/Tree.png",
          "color": [ 1.0, 1.0, 1.0, 1.0 ],
          "layer": 0,
          "flip_x": true,
          "flip_y": false,
          "anchor": "BottomCenter"
        },
        { 
          "type": "Transform",
          "location": [ -19.0, -113.0 ],
          "rotation": [ 0.0, 0.0, 0.0 ],
          "scale": [ 1.0, 1.0 ]
        },
        {
          "type": "BoxCollider",
          "center": [ 0, 0 ],
          "size": [ 15, 15 ]
        }
      ]
    },
    {
      "name": "Rock",
      "components": [
        {
          "type": "SpriteRenderer",
          "sprite": "assets/Rock.png",
          "color": [ 1.0, 1.0, 1.0, 1.0 ],
          "layer": 0,
          "flip_x": false,
          "flip_y": false,
          "anchor": "BottomCenter"
        },
        { 
          "type": "Transform",
          "location": [ -133.0, -156.0 ],
          "rotation": [ 0.0, 0.0, 0.0 ],
          "scale": [ 1.0, 1.0 ]
        },
        {
          "type": "BoxCollider",
          "center": [ 0, 0 ],
          "size": [ 15, 40 ]
        }
      ]
    }
  ]
}
```
