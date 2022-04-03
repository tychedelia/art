<script>
    import {onMount} from "svelte";

    const ENTER_KEY = 13;
    const ESCAPE_KEY = 27;

    let image;

    async function generateImage(evt) {
        const params = {
            seed: document.getElementById('seed').value,
            x: document.getElementById('x').value,
            y: document.getElementById('y').value,
            z: document.getElementById('z').value,
        }


        const res = await fetch(`http://localhost:5000?seed=${params.seed}`)
        const body = await res.json()

        document.getElementById("pic").src = `data:image/jpeg;base64,${body.image}`
    }
</script>

<style>

</style>

<header class="params">
    <h1>params</h1>
    <div>
        <label>Seed</label>
        <input id="seed" type="number" autofocus>
    </div>
    <div>
        <label>X</label>
        <input id="x" type="number" autofocus>
    </div>
    <div>
        <label>Y</label>
        <input id="y" type="number" autofocus>
    </div>
    <div>
        <label>Z</label>
        <input id="z" type="number" autofocus>
    </div>
    <div>
        <button on:click={generateImage}>Generate</button>
    </div>
</header>

<img id="pic" src="" alt="generated image"/>