## Project Goals:

### Near-Term:

1.  The first step starting from the Teapot example was to replace the teapot with an object of our choice, in this case a small civil jet.  **Done**.

2.  The second step was getting joystick input active in the main event loop, so we can alter model state between frames. **Done**.

3.  The third step is to read in a terrain height file and tessellate out some decent terrain.  **In progress**...

4.  Implementing a super-naive flight model, and chase-plane viewing the model across the sky.  Terrain collisions, etc.  Improve ambient lighting a bit for a tiny semblance of sky.  Some very basic, passable graphics and realism features.  Very basic.  **future**

With 1-4 done I would consider this project stage done, where the goal was to demonstrate basic physical system graphical output simulator features: the representation of a physical world and some mechanical system, with expected realism.  

The goals for the next steps might be:

- implementation of a library of relevant physics functions, methodologies for organizing model/object/world model systems.  

- Alternatively one could focus on optimizing the graphical pipeline from the implementation side, the Vulkan and hardware side,

- or the physics side, with ray-tracing algorithm and spherical harmonics and associated physics-based rendering techniques.


### Near-Term Ancillary:

1. Live shader reload on file watch with immediate screen render effect.  **in progress (in Studio example)**

2. Interface to adjust the application defined transformations (matrices generally, the ones that live in application/Rust logic rather than GPU/shader logic) live during runtime.  This is pretty much the same as input for a game, except we'll be using it in a studio way to enable the developer to tranfer through various relevant morphisms.  **in progress (in Studio example)**

3.  Refactor (clean up naming in particular) and spin out code to modules, April 2020 episode.  **in progress**





## Memium-Term:
So beyond those 4 near-term goals.
I'm looking to:

- make a flight simulator homage to the 1980s Spectrum Holobyte classic "Falcon".  So it's kind of like DCS (Digital Combat Simulator), but it will be simplified, with no multiplayer, no dynamic campaign, very limited features generally...  Kind of like an 80s game, befitting its modest development resource allocation.

- I want the most advanced physics modeling _I_ can develop, myself.  I'm starting with books like Gerald Sussman's "Structure and Interpretation of Classical Mechanics", and I'm thinking about different implementations of physics modeling, some of which work with classical mechanics and Euclidean space, whereas some implementations may work with relativity and deal with manifold effects even at local levels.  Will see, this is a big project and not everything can be done in medium term.

- Working with Vulkan and Rust, the sky is kind of the limit when optimizing the graphics representation.  I don't like any of the big game engines, I think their graphics look cheezy.  Since this is just my beginner project I'm not expecting to show them up or anything, but I'm happy to be working at the low-level, to have to learn all the basic underlying techniques involved in mapping information through a matrix of colored light bulbs.  I guess medium-term in this flight-sim project I just want the graphics to be interesting and physics-based mostly, and to give me an opportunity to learn more.

### Medium-Term Ancillary

1. Implement ray-tracing over the Vulkan extension.

    ##### Reference material:

    https://devblogs.nvidia.com/vulkan-raytracing/ (vulkan implementation specific)

    https://devblogs.nvidia.com/practical-real-time-ray-tracing-rtx/ (more background than implementation specific)

    https://github.com/maierfelix/tiny-rtx  (note to self: this is a good one for learning some new and exotic node.js stuff)



## Long-Term:

The general long-term goal is a game and simulation engine.  I'd like to work on that as a project for the rest of my career, it's so valuable and so interesting.  This kind of software can be leveraged for games or it can be leveraged for real critical systems validation, all that entails and implies...

I guess more specifically I could easily see something like this turning into an open-sourced competitor to DCS.  A kind of Linux for that enthusiast community to tune and represent world-systems.

I have some other ideas for ecological system simulation, also economic sysem simulation, which can provide a lattice or scaffolding for the development of a comprehensive formal ontological structure supporting a sophisticated economic science.
