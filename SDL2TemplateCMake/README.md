2.5D SDL CXX Scene
===
This part of the project was built on the [UWindsor COMP-3520 SDL2 Project Template (CMake version)](https://github.com/InBetweenNames/SDL2TemplateCMake). Some features of the project come from the template and other were added on, a full list of the features in accordance with the [Project Document]() is provided below.
- 2D Objects
- Computed Surfaces: Cubic Bezier and Fractal
- Basic Scene of Sprites, Double Sided Walls and "wallscreens" to display the computed surfaces
- Basic Drawing of Objects 
- Surface Textures
- User Interface

The implementation of the Cubic Bezier is in the function BezierCurve and is as follows:
```cpp
// References: https://www.geeksforgeeks.org/cubic-bezier-curve-implementation-in-c/
void BezierCurve(SDL_Surface* surface, int x[] , int y[]) {
    SDL_SetSurfaceRLE(surface, 1);
    if (SDL_LockSurface(surface)) {
        // TODO: Error surface can't be locked
    }
    uint32_t (*pixels)[surface->w] = (uint32_t(*)[surface->w]) surface->pixels;
    uint32_t red = SDL_MapRGB(surface->format, 255, 0, 0);
    //uint32_t green = SDL_MapRGB(surface->format, 0, 255, 0);
    uint32_t blue = SDL_MapRGB(surface->format, 0, 0, 255);

    circleBres(x[0], y[0], 8, surface, blue);
    circleBres(x[1], y[1], 8, surface, blue);
    circleBres(x[2], y[2], 8, surface, blue);
    circleBres(x[3], y[3], 8, surface, blue);
    // Bezier
    double xu = 0.0 , yu = 0.0 , u = 0.0 ;
    for (u = 0.0 ; u <= 1.0 ; u += 0.0001) {
        xu = pow(1 - u, 3) * x[0] + 3 * u * pow(1 - u, 2) * x[1] + 3 * pow(u, 2) * (1 - u) * x[2]
             + pow(u, 3) * x[3];
        yu = pow(1 - u, 3) * y[0] + 3 * u * pow(1 - u, 2) * y[1] + 3 * pow(u, 2) * (1 - u) * y[2]
             + pow(u, 3) * y[3];

        pixels[(int)xu][(int)yu] = red;
    }

    SDL_UnlockSurface(surface);
    // return surface;
}
```
The function takes in a surface to which the pixels are directly modified with the Bezier Curve drawn onto it. The surface is then applied to the front side of the two sided wall and rendered in the scene.
```cpp
SDL_SetSurfaceRLE(surface, 1);
SDL_LockSurface(surface);
SDL_UnlockSurface(surface);
```
The SetSurface was used to speed up direct pixel access which in turn required the surface to be locked when being edited. This was done because direct pixel access to my knowledge isn't recommended because it is slow and has to be optimized manually. The computing of the Cubic Bezier curve is done in the following parts.
```cpp
    double xu = 0.0 , yu = 0.0 , u = 0.0 ;
    for (u = 0.0 ; u <= 1.0 ; u += 0.0001) {
        xu = pow(1 - u, 3) * x[0] + 3 * u * pow(1 - u, 2) * x[1] + 3 * pow(u, 2) * (1 - u) * x[2]
             + pow(u, 3) * x[3];
        yu = pow(1 - u, 3) * y[0] + 3 * u * pow(1 - u, 2) * y[1] + 3 * pow(u, 2) * (1 - u) * y[2]
             + pow(u, 3) * y[3];

        pixels[(int)xu][(int)yu] = red;
    }
```
A note on the usage of pow: I recently discovered when using the pow function that it is better to simply perform the multiplication directly when you are powering less than 4 in cxx as they perform a Taylor Series expansion according to Paul Preney.

Two curves are setup to be displayed in main as follows
```cpp
	res.brezScreen = SDL_CreateRGBSurfaceWithFormat(0, 500, 400, 32,
                     SDL_PIXELFORMAT_RGBA8888);
    SDL_FillRect(res.brezScreen, NULL, SDL_MapRGB(res.brezScreen->format, 0, 0, 0));
    int brezX[4] = {100, 200, 250, 350};
    int brezY[4] = {300, 250, 100, 50};
    BezierCurve(res.brezScreen, brezX, brezY);

    res.brezScreen2 = SDL_CreateRGBSurfaceWithFormat(0, 500, 400, 32,
                      SDL_PIXELFORMAT_RGBA8888);
    SDL_FillRect(res.brezScreen2, NULL, SDL_MapRGB(res.brezScreen2->format, 128, 128, 128));
    int brezX2[4] = {150, 300, 150, 50};
    int brezY2[4] = {100, 50, 300, 250};
    BezierCurve(res.brezScreen2, brezX2, brezY2);
```
The two arrays represent a set of coordinates on the X and Y axis of the screen. The curve is computed and then presented to the screen as shown below.

![](Bezier.png)

## Planned Features
Implementation work was attempted to produce and display fractals through direct pixel editing of surfaces as with the Bezier function but I couldn't get it to work within the alloted time frame. BSP rendering was also attempted but it was looking like it would take too much time so it was dropped pretty early on (this was before the due date was extended). Fractal attempt is still in the code and displaying on one of the walls.

UWindsor COMP-3520 SDL2 Project Template (CMake version)
===

** For a live demo of THE TEMPLATE code, [click here](https://inbetweennames.github.io/SDL2TemplateCMake/) **

This template is intended for students in the COMP-3520 Introduction to Computer Graphics course
at the University of Windsor, however it should serve as a useful template for anyone interested in
getting started with the [SDL2](http://libsdl.org/) library quickly on non-Windows platforms.
In contrast to the [Visual Studio template](https://github.com/InBetweenNames/SDL2Template), this version
uses dynamic linking by default and uses your installed system libraries.

The following dependencies are required:
* [CMake](https://cmake.org/)
* [SDL2](http://libsdl.org/) 
* [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/) library for easy text rendering
* [freetype2](https://www.freetype.org/) which `SDL2_ttf` uses for the actual work

It's very easy to get started with SDL2 on most linux distributions.  Just ensure you have the above packages installed along with their header files (usually requires a `-dev` package)

On Debian based systems, the packages will probably be called `libsdl2-dev` and `libsdl2-ttf-dev`.
On Arch Linux, look for the packages `sdl2` and `sdl2_ttf`.

Mac users should be able to use Homebrew or a similar package manager to install the needed dependencies.

Demo
---

A live demo compiled using Emscripten and WebAssembly is available [here](https://inbetweennames.github.io/SDL2TemplateCMake/)

Controls:
---

Mouse - look left/right
W, A, S, D - move forwards, backwards, upwards, downwards
T, G -- raise and lower height
Y, H -- adjust focal distance (distance to screen plane)

Setup
---

Once you've installed these packages, you'll be all set to start your first assignment.
Clone this repository somewhere using Git (in a shell):

~~~
git clone https://github.com/InBetweenNames/SDL2TemplateCMake.git
cd SDL2TemplateCMake
~~~

Next, enter the build directory:

~~~
cd build
~~~

Run CMake:

~~~
cmake ..
~~~

If this runs without errors, you're ready to build:

~~~
cmake --build .
~~~

Now, run the demo:

~~~
./main
~~~

Note that `iosevka-regular.ttf` must be in the working directory of `main` for it to work.
In practice, this means you need to be in the `build` directory when running `main`.
I would welcome a pull request that removes this restriction.

Setup for Emscripten
---

This template now supports building using [Emscripten](https://kripken.github.io/emscripten-site/) for compiling your SDL2-based
C or C++ code directly to your web browser using WebAssembly.  To use it, ensure you have the [Emscripten SDK](https://github.com/emscripten-core/emsdk)
installed and in your PATH (use `emsdk install` and `emsdk activate`, following all instructions), and then:

~~~
cd build-wasm
./build_with_emscripten.sh
~~~

This will configure, compile, link, and run your project directly in your web browser.
