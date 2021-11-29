import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');
        const spheres = [];

        const viewportWidthInput = document.getElementById("viewportWidth");
        const viewportHeightInput = document.getElementById("viewportHeight");
        const sphereRadius = document.getElementById("sphereRadius");
        const sphereCenterX = document.getElementById("sphereCenterX");
        const sphereCenterY = document.getElementById("sphereCenterY");
        const sphereCenterZ = document.getElementById("sphereCenterZ");
        const sphereColorR = document.getElementById("sphere-color-r");
        const sphereColorG = document.getElementById("sphere-color-g");
        const sphereColorB = document.getElementById("sphere-color-b");
        const sphereColorA = document.getElementById("sphere-color-a");
        const sphereSpecular = document.getElementById("sphere-specular")
        const sphereRelective = document.getElementById("sphere-reflective")
        const addSphereBtn = document.getElementById("add-sphere");

        const renderBtn = document.getElementById("render");
        const addPredefinedSceneBtn = document.getElementById("add-scene");

        const viewport = {
            width: 1.0,
            height: 1.0,
        }

        addSphereBtn.addEventListener("click", () => {
            spheres.push({
                radius: parseFloat(sphereRadius.value) || 1.0,
                center: {
                    x: parseFloat(sphereCenterX.value) || 0.0,
                    y: parseFloat(sphereCenterY.value) || 0.0,
                    z: parseFloat(sphereCenterZ.value) || 0.0,
                    w: 1.0,
                },
                color: {
                    r: parseFloat(sphereColorR.value) || 0.0,
                    g: parseFloat(sphereColorG.value) || 0.0,
                    b: parseFloat(sphereColorB.value) || 0.0,
                    a: parseFloat(sphereColorA.value) || 0.0,
                },
                specular: parseFloat(sphereSpecular.value) || 1.0,
                reflective: parseFloat(sphereRelective.value) || 1.0,
            });
            const viewport = {
                width: parseFloat(viewportWidthInput.value) || 1.0,
                height: parseFloat(viewportHeightInput.value) || 1.0,
            };

            wasm.draw(ctx, 1000, 1000, viewport, spheres);
        });

        renderBtn.addEventListener("click", () => {
            const viewport = {
                width: parseFloat(viewportWidthInput.value) || 1.0,
                height: parseFloat(viewportHeightInput.value) || 1.0,
            };

            wasm.draw(ctx, 1000, 1000, viewport, spheres);
        });

        addPredefinedSceneBtn.addEventListener("click", () => {
            spheres.push({
                radius: 1.0, 
                center: { x: 0, y: -1, z: 3, w: 1.0},
                color: { r: 1, g: 0, b: 0, a: 255 },
                specular: 500,
                reflective: 0.2,
            }, 
            {
                radius: 1.0, 
                center: { x: 2, y: 0, z: 4, w: 1.0},
                color: { r: 0, g: 0, b: 1, a: 255 },
                specular: 500,
                reflective: 0.3,
            },
            {
                radius: 1.0, 
                center: { x: -2, y: 0, z: 4, w: 1.0 },
                color: { r: 0, g: 1, b: 0, a: 255 },
                specular: 10,
                reflective: 0.4,
            },
            {
                radius: 5000, 
                center: { x: 0, y: -5001, z: 0, w: 1.0 },
                color: { r: 1, g: 1, b: 0, a: 255 },
                specular: 1000,
                reflective: 0.5,
            });
            const viewport = {
                width: parseFloat(viewportWidthInput.value) || 1.0,
                height: parseFloat(viewportHeightInput.value) || 1.0,
            };

            wasm.draw(ctx, 1000, 1000, viewport, spheres);
        });

        wasm.draw(ctx, 1000, 1000, viewport, spheres);
    })
    .catch(console.error);
