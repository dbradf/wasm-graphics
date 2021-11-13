import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const viewportWidthInput = document.getElementById("viewportWidth");
        const viewportHeightInput = document.getElementById("viewportHeight");
        const renderBtn = document.getElementById("render");

        const viewport = {
            width: 1.0,
            height: 1.0,
        }

        renderBtn.addEventListener("click", () => {
            const viewport = {
                width: parseFloat(viewportWidthInput.value) || 1.0,
                height: parseFloat(viewportHeightInput.value) || 1.0,
            };

            wasm.draw(ctx, 1000, 1000, viewport);
        });

        wasm.draw(ctx, 1000, 1000, viewport);
    })
    .catch(console.error);
