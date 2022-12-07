# Scene Creation (v 0.1.0)

This documents explains the process of creating a `2dcl` scene and uploading it to the catalyst network. At the moment the process is very manual a bit laborious, but our hope is that we can continue working on the client and tools to make the process easier and more powerful.

That said we made sure to eliminate as much friction out of the process as we could during the development of the client and protocol, and hopefully you'll find our current tools easy to use.

## Overview

The overall process is divided in three main steps:

 1. Creating a scene using a `scene.json`.
 2. Compiling the scene with our provided compiler.
 3. Uploading the scene using the `dcl` command.

## What knowledge do I need?

Since we're just starting with the project, our tools are a bit rough, that's why there's some knowledge you need to have to successfully deploy a scene:
  * A minimum level of terminal/console usage knowledge. Mainly how to move across directories and how to call programs in a terminal program. That can be the `Command Prompt` in Windows, `Terminal` in macOS or Linux.
  * Some experience editing files, and familiarity with JSON format is also very helpful.
  * Some familiarity with deploying scenes to Decentraland using the `dcl` SDK command.

## What do I need to download?

To start working on your scene you need to download the `2dcl` client from [https://2dcl.org](https://2dcl.org). 

You'll find links to download the client for Windows, Linux and macOS in the homepage.

There you'll also find a link to the sample scene github repository, which you can fork if you're savvy with git, or you can download a zip containing the scene files.

In this guide I'll assume that you either added the `2dcl` command to your `PATH` or you know how to call the command wherever it is located. If you don't know how to do this, there are plenty of resources on the internet to learn this.

### Can I start from scratch?

Sure, if you don't want to deal with repositories or zip files, you just need to create a file called `scene.json` and a folder called `assets`.

The bare minimum content of `scene.json` is this:

```json
{
  "name": "The Name of My Scene",
  "levels": [
    {
      ""
      "name": "Main Level",
      "entities": []
    },
    
  ]
}
```

The `assets` folder should have all the images you need to render your scene.

## Ok, now what?

The following steps will assume you have the sample scene, things will look differently if you started from scratch, but the overall process still applies.

The first thing to do is, when standing on the scene folder, call `2dcl preview`. 

You should see something like this in the terminal: 
```
~/2dcl-sample-scene$ 2dcl preview
2022-12-07T17:43:16.600995Z  INFO winit::platform_impl::platform::x11::window: Guessed window scale factor: 1    
2022-12-07T17:43:16.633674Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3080 Ti", vendor: 4318, device: 8712, device_type: DiscreteGpu, backend: Vulkan }
```

And you should see a new window open up.

![preview window](./images/preview.png)

