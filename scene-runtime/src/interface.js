SceneInterface = {};
subscrite = function() {}
Component = function() {}
Camera = {
  instance: {
    feetPosition: 0
  }
}

AudioClip = function() {}
AudioSource = function() {}
Attachable = function() {}
MessageBus = function() {}
GLTFShape = function() {}
UICanvas = function() {}
UIText = function() {}
UIImage = function() {}
Font = function() {}
Fonts = {}
Texture = function() {}
Material = function() {}
BoxShape = function() {}
Transform = function() {}
Entity = function() {
  return {
    setParent: function(entity) {},
    addComponent: function(component) {},
    addComponentOrReplace: function(component) {},
    getComponentOrCreate: function(component) {},
    getComponent: function(component) {
      return {
        rotate: function() {}
      }
    },
    subscribe: function() {}
  }
}

class Vector3 {
  constructor(x, y, z) {
    return {
      x: x,
      y: y,
      z: z,
      rotate: function() { return this; },
      multiplyByFloats: function() { return this; },
      add: function(vector) { return this; }
    }
  }
}

Vector3.Forward = function () { return new Vector3(0,0,1); }


Quaternion = {
  Euler: function() {}
}

Color3 = {
  Red: function() {},
  FromHexString: function() {}
}

class Color4 {
  constructor(r,g,b,a) {}
}

Color4.FromHexString = function() {}
Color4.FromInts = function() {}
Color4.White = function() {}

Input = {
  instance: {
    subscribe: function() {}
  }
}
Observable = function() {}

engine = {
  getComponentGroup: function(component) {},
  addEntity: function(entity) {},
  addSystem: function(cb) {}
}

OnPointerDown = function() {}
ActionButton = {}

ObservableComponent = function () {}
WebSocket = function () {}


// Old ECS
dcl = {
  onStart: function(cb) {
    SceneInterface.onStart = cb;
  },

  addEntity: function(entityId) {
    Deno.core.opSync("op_add_entity", entityId);
  },

  onUpdate: function(cb) {
    SceneInterface.onUpdate = cb;
  },

  onEvent: function(cb) {
    SceneInterface.onEvent = cb;
  },

  subscribe: function(eventName) {

  },

  loadModule: function(moduleName) {
    return { 
      then: function(cb) {
        return {
          catch: function(cb) {}
        }
      } 
    }
  }
}

this.dcl = dcl;
