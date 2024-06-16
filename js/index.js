import('../pkg')
    .then(wasm => {
        const dim = Math.max(window.innerWidth, window.innerHeight);

        const canvas = document.getElementById('drawing');
        canvas.width = dim;
        canvas.height = dim;

        const ctx = canvas.getContext('2d');
        wasm.draw(ctx, dim, dim, window.performance.now());
    })
    .catch(console.error);
