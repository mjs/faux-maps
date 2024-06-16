import('../pkg')
    .then(wasm => {
        let seed = window.performance.now();
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const redraw = () => {
            const dim = Math.max(window.innerWidth, window.innerHeight);
            canvas.width = dim;
            canvas.height = dim;
            wasm.draw(ctx, dim, dim, seed);
        };
        redraw();

        window.onresize = redraw;
        canvas.onclick = () => {
            seed = window.performance.now();
            redraw();
        };
    })
    .catch(console.error);
