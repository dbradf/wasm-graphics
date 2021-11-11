import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        wasm.draw(ctx, 1000, 1000);
    })
    .catch(console.error);
